use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use sqlx::{MySql, Pool};

use crate::{
    rest::axum::{
        handlers::{create_user::create_user_handler, get_user_by_id::get_user_by_id_handler},
        shared::states::UserAppState,
    },
    repositories::user_repository::mysql_impl::MysqlUserRepository,
};

pub fn user_router(pool: Pool<MySql>) -> Router {
    let user_repo = MysqlUserRepository::new(pool);

    let user_app_state = UserAppState {
        user_repo: Arc::new(user_repo),
    };

    Router::new()
        .route("users", post(create_user_handler))
        .route("users/{id}", get(get_user_by_id_handler))
        .route("users", get(async || {}))
        .route("users/{id}", put(async || {}))
        .route("users/{id}", patch(async || {}))
        .route("users/{id}", delete(async || {}))
        .with_state(user_app_state)
}
