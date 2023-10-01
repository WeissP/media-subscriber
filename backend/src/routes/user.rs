use super::youtube::types::ChannelID;
use axum_login::AuthUser;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

#[derive(Debug, Default, Clone, sqlx::FromRow)]
pub struct User {
    id: i32,
    username: String,
    hashed_password: String,
    subscribed_channels: Vec<String>,
}

pub async fn get_user(
    pool: &PgPool,
    username: &str,
) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
SELECT * FROM users WHERE username = $1
"#,
        username
    )
    .fetch_optional(pool)
    .await
}

pub async fn insert_user(
    pool: &PgPool,
    username: &str,
    hashed_password: &str,
    channels: Vec<String>,
) -> sqlx::Result<()> {
    sqlx::query!(
        r#"
INSERT INTO users (username, hashed_password, subscribed_channels)
VALUES ($1, $2, $3::text[]);
"#,
        username,
        hashed_password,
        channels.as_slice()
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use super::*;
    #[sqlx::test(migrator = "MIGRATOR")]
    async fn user_test(pool: PgPool) {
        let username = "Alice";
        let hashed_password =
            "95b89ee08b9009886f6f2163c2d2be2ff72cdad9142a569c61173ddcb8f7db9a";
        let channels = vec![
            "UCjuNibFJ21MiSNpu8LZyV4w".to_string(),
            "UC5facmu9-H1RqOI8GxfwPoQ".to_string(),
        ];
        insert_user(&pool, username, hashed_password, channels.clone())
            .await
            .unwrap();
        let user = get_user(&pool, username)
            .await
            .unwrap()
            .expect("could not find user with name Alice");
        assert_eq!(user.username, username);
        assert_eq!(user.hashed_password, hashed_password);
        assert_eq!(user.subscribed_channels, channels);
        let dup = insert_user(&pool, username, hashed_password, channels.clone())
            .await
            .unwrap();
    }
}
