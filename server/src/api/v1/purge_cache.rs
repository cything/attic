use attic::api::v1::purge::{PurgeCacheRequest, PurgeCacheResult};
use axum::extract::{Extension, Json, Path};
use chrono::{TimeDelta, Utc};
use sea_orm::sea_query::Expr;
use tracing::instrument;

use crate::database::entity::cache;
use crate::database::entity::object::{self, Entity as Object};
use crate::error::{ErrorKind, ServerError, ServerResult};
use crate::{RequestState, State};
use attic::cache::CacheName;

#[instrument(skip_all, fields(cache_name, payload))]
pub(crate) async fn purge_cache(
    Extension(state): Extension<State>,
    Extension(req_state): Extension<RequestState>,
    Path(cache_name): Path<CacheName>,
    Json(payload): Json<PurgeCacheRequest>,
) -> ServerResult<Json<PurgeCacheResult>> {
    let database = state.database().await?;
    let now = Utc::now();

    let older_than_secs = payload.older_than.as_secs();
    let older_than_subsec_nanos = payload.older_than.subsec_nanos();
    let cutoff = now.checked_sub_signed(TimeDelta::new(older_than_secs.try_into().unwrap(), older_than_subsec_nanos).unwrap());

    let cache = req_state
        .auth
        .auth_cache(database, &cache_name, |cache, permission| {
            permission.require_destroy_cache()?;
            Ok(cache)
        })
        .await?;

    let deletion: u64;
    if payload.dry_run {
        deletion = Object::delete_many()
            .filter(object::Column::Id.eq(cache.id))
            .filter(object::Column::DeletedAt.is_null())
            .filter(
                object::Column::CreatedAt.lt(cutoff)
            )
            .exec(database)
            .await
            .map_err(ServerError::database_error)?.rows_affected;
    } else {
        deletion = Object::delete_many()
            .filter(object::Column::Id.eq(cache.id))
            .filter(object::Column::DeletedAt.is_null())
            .filter(
                object::Column::CreatedAt.lt(cutoff)
            )
            .exec(database)
            .await
            .map_err(ServerError::database_error)?.rows_affected;
    }

    if deletion == 0 {
        Err(ErrorKind::NoSuchCache.into())
    } else {
        Ok(Json(PurgeCacheResult{ objects_deleted: deletion }))
    }
}
