//! Procedural macros for rust-api framework
//!
//! Provides route macros like #[get], #[post], etc. for defining HTTP endpoints
//! in a FastAPI-style syntax.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

mod route;

use route::{HttpMethod, RouteArgs};

/// Define a GET route handler
///
/// # Example
///
/// ```ignore
/// #[get("/users/:id")]
/// async fn get_user(path: Path<String>) -> Json<User> {
///     // handler code
/// }
/// ```
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand_route_macro(HttpMethod::Get, args, input)
}

/// Define a POST route handler
///
/// # Example
///
/// ```ignore
/// #[post("/users")]
/// async fn create_user(body: Json<CreateUser>) -> Json<User> {
///     // handler code
/// }
/// ```
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand_route_macro(HttpMethod::Post, args, input)
}

/// Define a PUT route handler
///
/// # Example
///
/// ```ignore
/// #[put("/users/:id")]
/// async fn update_user(path: Path<String>, body: Json<User>) -> Json<User> {
///     // handler code
/// }
/// ```
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand_route_macro(HttpMethod::Put, args, input)
}

/// Define a DELETE route handler
///
/// # Example
///
/// ```ignore
/// #[delete("/users/:id")]
/// async fn delete_user(path: Path<String>) -> StatusCode {
///     // handler code
/// }
/// ```
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand_route_macro(HttpMethod::Delete, args, input)
}

/// Define a PATCH route handler
///
/// # Example
///
/// ```ignore
/// #[patch("/users/:id")]
/// async fn patch_user(path: Path<String>, body: Json<UserPatch>) -> Json<User> {
///     // handler code
/// }
/// ```
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    route::expand_route_macro(HttpMethod::Patch, args, input)
}
