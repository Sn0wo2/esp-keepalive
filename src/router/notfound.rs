use crate::handler;
use crate::router::AppRouter;

pub(crate) fn init(router: AppRouter) -> AppRouter {
    router.fallback(handler::notfound::init)
}
