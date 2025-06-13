use crate::rest::axum::routes::user_router::user_router;
use sqlx::{MySql, Pool};

pub mod user_router;

pub fn main_router(pool: Pool<MySql>) {
    let user_router = user_router(pool);

    Router::new().merge(user_router);
}
