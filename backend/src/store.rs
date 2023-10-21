use crate::{config::Config, cornucopia};
use anyhow::Context;
use deadpool_postgres::{Manager, ManagerConfig, Object, Pool};
use getset_scoped::Getters;
use std::str::FromStr;
use tokio_postgres::NoTls;

#[derive(Clone, Debug)]
pub struct AppState {
    db: Pool,
}

impl AppState {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        Ok(Self {
            db: init_pool(config.database_url())?,
        })
    }

    pub async fn db(&self) -> anyhow::Result<deadpool_postgres::Object> {
        self.db.get().await.context("could not connect to database")
    }
}

fn init_pool(url: &str) -> anyhow::Result<Pool> {
    let config = tokio_postgres::Config::from_str(&url)
        .with_context(|| "could not construct config from database url {url}")?;
    let mgr = Manager::from_config(config, NoTls, ManagerConfig::default());
    Pool::builder(mgr)
        .max_size(16)
        .build()
        .context("could not init pool")
}
