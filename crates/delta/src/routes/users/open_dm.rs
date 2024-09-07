use rocket::{serde::json::Json, State};
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Channel, Database, User,
};
use upryzing_models::v0;
use upryzing_permissions::{calculate_user_permissions, UserPermission};
use upryzing_result::Result;

/// # Open Direct Message
///
/// Open a DM with another user.
///
/// If the target is oneself, a saved messages channel is returned.
#[openapi(tag = "Direct Messaging")]
#[get("/<target>/dm")]
pub async fn open_dm(
    db: &State<Database>,
    user: User,
    target: Reference,
) -> Result<Json<v0::Channel>> {
    let target = target.as_user(db).await?;

    let mut query = DatabasePermissionQuery::new(db, &user).user(&target);
    calculate_user_permissions(&mut query)
        .await
        .throw_if_lacking_user_permission(UserPermission::SendMessage)?;

    Channel::create_dm(db, &user, &target)
        .await
        .map(Into::into)
        .map(Json)
}
