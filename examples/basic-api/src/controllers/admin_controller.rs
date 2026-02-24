use rust_api::prelude::*;

use crate::services::admin_service::{AdminService, AdminStatusResponse};

// ---------------------------------------------------------------------------
// Handler
// ---------------------------------------------------------------------------
//
// AdminController is a pure handler — it knows only about its service.
// Auth is a Tower layer applied to the route group in the pipeline:
//
//   RouterPipeline::new()
//       .group("/admin", |g| g
//           .mount::<AdminController>(admin_svc)
//           .map(|r| r.layer(require_bearer(key)))
//       )
//
// The handler never sees an unauthenticated request — the middleware rejects
// it before this code runs. No auth imports, no token comparisons here.

/// GET /status — admin subsystem health.
/// Mounted under the /admin group in the pipeline, so the full path is /admin/status.
/// Auth is applied by require_bearer on the /admin group — no token logic here.
#[get("/status")]
pub async fn admin_status(State(svc): State<Arc<AdminService>>) -> Json<AdminStatusResponse> {
    Json(svc.status())
}

// ---------------------------------------------------------------------------
// Controller registration
// ---------------------------------------------------------------------------

/// Controller marker for admin routes.
pub struct AdminController;

mount_handlers!(
    AdminController,
    AdminService,
    [(__admin_status_route, admin_status),]
);
