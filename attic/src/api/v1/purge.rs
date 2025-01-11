//! purge-cache v1
//!
//! `POST /_api/v1/purge-cache`
//!
//! Requires "destroy" permission.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::cache::CacheName;

#[derive(Debug, Serialize, Deserialize)]
pub struct PurgeCacheRequest {
    /// The name of the cache.
    pub cache: CacheName,

    /// Duration to purge
    pub older_than: Duration,

    pub dry_run: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PurgeCacheResult {
    /// Number of objects deleted
    pub objects_deleted: u64,
}
