use crate::{config::Config, db::init_pool};
use anyhow::Context;
use getset_scoped::Getters;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone, Getters)]
#[get = "pub"]
pub struct AppState {
    db: PgPool,
}

impl AppState {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            db: init_pool(config.database_url()).await?,
        })
    }
}
