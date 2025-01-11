use anyhow::Result;
use clap::Parser;

use humantime::Duration;

use crate::{api::ApiClient, cache::CacheRef, cli::Opts, config::Config};

#[derive(Debug, Parser)]
pub struct Purge {
    /// Name of the cache to purge
    cache: CacheRef,

    /// Duration to purge
    older_than: Duration,
}

pub async fn run(opts: Opts) -> Result<()> {
    let sub = opts.command.as_purge().unwrap();
    let config = Config::load()?;
    let (_, server, cache_name) = config.resolve_cache(&sub.cache)?;
    let mut api = ApiClient::from_server_config(server.clone())?;

    // Confirm cache validity
    let cache_config = api.get_cache_config(cache_name).await?;

    if let Some(api_endpoint) = &cache_config.api_endpoint {
        // Use delegated API endpoint
        api.set_endpoint(api_endpoint)?;
    }

    api.purge_cache(cache_name, sub.older_than).await?;

    Ok(())
}
