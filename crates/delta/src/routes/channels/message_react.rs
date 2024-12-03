use rocket::State;
use rocket_empty::EmptyResponse;
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission};
use upryzing_result::Result;

/// # Add Reaction to Message
///
/// React to a given message.
#[openapi(tag = "Interactions")]
#[put("/<target>/messages/<msg>/reactions/<emoji>")]
pub async fn react_message(
    db: &State<Database>,
    user: User,
    target: Reference,
    msg: Reference,
    emoji: Reference,
) -> Result<EmptyResponse> {
    let channel = target.as_channel(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).channel(&channel);
    calculate_channel_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::React)?;

    // Fetch relevant message
    let message = msg.as_message_in_channel(db, channel.id()).await?;

    // Add the reaction
    message
        .add_reaction(db, &user, &emoji.id)
        .await
        .map(|_| EmptyResponse)
}
