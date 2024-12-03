use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, Invite, User,
};
use upryzing_models::v0;
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission};

use rocket::{serde::json::Json, State};
use upryzing_result::{create_error, Result};

/// # Create Invite
///
/// Creates an invite to this channel.
///
/// Channel must be a `TextChannel`.
#[openapi(tag = "Channel Invites")]
#[post("/<target>/invites")]
pub async fn create_invite(
    db: &State<Database>,
    user: User,
    target: Reference,
) -> Result<Json<v0::Invite>> {
    if user.bot.is_some() {
        return Err(create_error!(IsBot));
    }

    let channel = target.as_channel(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).channel(&channel);
    calculate_channel_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::InviteOthers)?;

    Invite::create_channel_invite(db, &user, &channel)
        .await
        .map(|invite| invite.into())
        .map(Json)
}
