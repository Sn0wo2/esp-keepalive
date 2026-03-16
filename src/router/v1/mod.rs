use crate::router::AppRouter;
use axum::Router;

pub mod device;

pub(crate) fn init_nest<F>(parent: AppRouter, f: F) -> AppRouter
where
    F: FnOnce(AppRouter) -> AppRouter,
{
    parent.nest("/v1", f(Router::new()))
}
