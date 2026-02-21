//! Route macro implementation
//!
//! Handles expansion of #[get], #[post], etc. macros into axum-compatible
//! handlers.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, ItemFn, LitStr,
};

/// HTTP method for route
#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl HttpMethod {
    // get the axum routing function name for this method
    #[allow(dead_code)]
    fn axum_method(&self) -> proc_macro2::TokenStream {
        match self {
            HttpMethod::Get => quote! { get },
            HttpMethod::Post => quote! { post },
            HttpMethod::Put => quote! { put },
            HttpMethod::Delete => quote! { delete },
            HttpMethod::Patch => quote! { patch },
        }
    }

    // get the method name as a string
    #[allow(dead_code)]
    fn as_str(&self) -> &'static str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
        }
    }
}

/// Arguments passed to route macro
pub struct RouteArgs {
    path: LitStr,
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let path: LitStr = input.parse()?;
        Ok(RouteArgs { path })
    }
}

/// Main expansion function for route macros
///
/// This transforms:
/// ```ignore
/// #[get("/users/{id}")]
/// async fn get_user(Path(id): Path<String>) -> Json<User> { ... }
/// ```
///
/// Into the original function plus a route info tuple constant:
/// ```ignore
/// async fn get_user(Path(id): Path<String>) -> Json<User> { ... }
/// const __get_user_route: (&'static str, &'static str) = ("/users/{id}", "GET");
/// ```
///
/// The tuple encodes both path and HTTP method, making the annotation a
/// binding contract — the verb from the annotation is the sole authority.
pub fn expand_route_macro(
    method: HttpMethod,
    args: TokenStream,
    input: TokenStream,
) -> TokenStream {
    // parse the route path argument
    let args = parse_macro_input!(args as RouteArgs);
    let path = args.path;

    // parse the function
    let func = parse_macro_input!(input as ItemFn);
    let func_name = &func.sig.ident;
    let func_vis = &func.vis;

    // generate route registration helper name
    let route_helper_name = syn::Ident::new(&format!("__{}_route", func_name), func_name.span());

    // encode the HTTP method as a string literal
    let method_str = method.as_str();

    let expanded = quote! {
        //original handler function
        #func

        // Route info tuple: (path, HTTP method).
        // The method comes from the macro annotation — this is the enforcement contract.
        // Use with Router::api_route() or RouterPipeline::route() to register correctly.
        #[allow(non_upper_case_globals)]
        #func_vis const #route_helper_name: (&'static str, &'static str) = (#path, #method_str);
    };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_as_str() {
        assert_eq!(HttpMethod::Get.as_str(), "GET");
        assert_eq!(HttpMethod::Post.as_str(), "POST");
        assert_eq!(HttpMethod::Put.as_str(), "PUT");
        assert_eq!(HttpMethod::Delete.as_str(), "DELETE");
        assert_eq!(HttpMethod::Patch.as_str(), "PATCH");
    }
}
