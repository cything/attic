use anyhow::Result;
use clap::Parser;

use humantime::Duration;

use crate::{cache::CacheRef, cli::Opts, config::Config};

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
    let (_, server, cache) = config.resolve_cache(&sub.cache)?;

    Ok(())
}
