use domain::{
    entities::user::User,
    repositories::user_repository::{UserRepository, UserRepositoryError},
};
use sqlx::{MySql, Pool, Row};

#[derive(Debug, Clone)]
pub struct MysqlUserRepository {
    pool: Pool<MySql>,
}

impl MysqlUserRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for MysqlUserRepository {
    async fn save(&self, user: User) -> Result<u64, UserRepositoryError> {
        let sql = r#"
            INSERT INTO users (username, email, avatar_url, status, password_hash, created_at, updated_at, deleted_at) VALUES ()
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#;

        let query_result = sqlx::query(sql)
            .bind(user.username)
            .bind(user.email)
            .bind(user.avatar_url)
            .bind::<i8>(user.status.into())
            .bind(user.password_hash)
            .bind(user.created_at)
            .bind(user.updated_at)
            .bind(user.deleted_at)
            .execute(&self.pool)
            .await
            .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?;

        Ok(query_result.last_insert_id())
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<User>, UserRepositoryError> {
        let sql = r#"
            SELECT id, username, email, avatar_url, status, password_hash, created_at, updated_at, deleted_at
            FROM
                users
            WHERE
                deleted_at IS NULL,
                id = $1
        "#;

        let mysql_row_option = sqlx::query(sql)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?;

        if let Some(row) = mysql_row_option {
            let id = row
                .try_get::<u64, &str>("id")
                .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?;

            let user = User {
                id: Some(id),
                username: row
                    .try_get("username")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?,
                email: row
                    .try_get("email")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?,
                avatar_url: row
                    .try_get("avatar_url")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?,
                status: row
                    .try_get::<i8, _>("avatar_url")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?
                    .into(),
                password_hash: row
                    .try_get("password_hash")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?,
                created_at: row
                    .try_get("created_at")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?,
                updated_at: row
                    .try_get("updated_at")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?,
                deleted_at: row
                    .try_get("deleted_at")
                    .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?,
            };

            return Ok(Some(user));
        };

        Ok(None)
    }

    async fn soft_delete(&self, id: u64) -> Result<(), UserRepositoryError> {
        let some_user = self
            .get_by_id(id)
            .await
            .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?;

        if let Some(mut user) = some_user {
            user.soft_delete();

            self.save(user)
                .await
                .map_err(|err| UserRepositoryError::DatabaseAccessError(err.into()))?;
        };

        Ok(())
    }
}
