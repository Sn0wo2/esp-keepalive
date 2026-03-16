use crate::handler;
use crate::handler::notfound;
use crate::router::AppRouter;

pub(crate) fn init(router: AppRouter) -> AppRouter {
    router
        .route(
            "/device",
            axum::routing::get(handler::get_device::handle).post(handler::post_device::handle),
        )
        .method_not_allowed_fallback(notfound::init)
}
