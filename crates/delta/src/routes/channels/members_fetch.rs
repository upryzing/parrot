use futures::future::join_all;
use rocket::{serde::json::Json, State};
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Channel, Database, User,
};
use upryzing_models::v0;
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission};
use upryzing_result::{create_error, Result};

/// # Fetch Group Members
///
/// Retrieves all users who are part of this group.
///
/// This may not return full user information if users are not friends but have mutual connections.
#[openapi(tag = "Groups")]
#[get("/<target>/members")]
pub async fn fetch_members(
    db: &State<Database>,
    user: User,
    target: Reference,
) -> Result<Json<Vec<v0::User>>> {
    let channel = target.as_channel(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).channel(&channel);
    calculate_channel_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::ViewChannel)?;

    if let Channel::Group { recipients, .. } = channel {
        Ok(Json(
            User::fetch_many_ids_as_mutuals(db, &user, &recipients).await?,
        ))
    } else {
        Err(create_error!(InvalidOperation))
    }
}
