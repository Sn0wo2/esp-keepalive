mod notfound;
mod v1;

use crate::db::Database;
use axum::Router;
use v1::device;

pub type AppRouter = Router<Database>;

pub trait RouterExt {
    fn init_v1<F>(self, f: F) -> Self
    where
        F: FnOnce(AppRouter) -> AppRouter;
    fn init_device(self) -> Self;
    fn init_notfound(self) -> Self;
}

impl RouterExt for AppRouter {
    fn init_v1<F>(self, f: F) -> Self
    where
        F: FnOnce(AppRouter) -> AppRouter,
    {
        v1::init_nest(self, f)
    }

    fn init_device(self) -> Self {
        device::init(self)
    }

    fn init_notfound(self) -> Self {
        notfound::init(self)
    }
}

pub fn init() -> AppRouter {
    Router::new().init_v1(|v1| v1.init_device()).init_notfound()
}
