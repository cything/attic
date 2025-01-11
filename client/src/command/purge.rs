use anyhow::Result;
use clap::Parser;

use humantime::Duration;

use crate::{api::ApiClient, cache::CacheRef, cli::Opts, config::Config};

#[derive(Debug, Parser)]
pub struct Purge {
    /// Name of the cache to purge
    ///
    /// This can either be `servername:cachename` or `cachename`
    cache: CacheRef,

    /// Duration to purge
    #[clap(short = 'd', long)]
    older_than: Duration,

    /// Dry-run
    ///
    /// Returns the number of objects that would be deleted instead of
    /// actually deleting them.
    #[clap(short = 'n', long)]
    dry_run: bool,
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

    let result = api.purge_cache(cache_name, sub.older_than, sub.dry_run).await?;
    if sub.dry_run {
        eprintln!("would delete objects: {}", result.objects_deleted);
    } else {
        eprintln!("objects deleted: {}", result.objects_deleted);
    }

    Ok(())
}
