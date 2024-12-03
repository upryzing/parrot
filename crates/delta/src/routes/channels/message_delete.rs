use rocket::State;
use rocket_empty::EmptyResponse;
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission};
use upryzing_result::Result;

/// # Delete Message
///
/// Delete a message you've sent or one you have permission to delete.
#[openapi(tag = "Messaging")]
#[delete("/<target>/messages/<msg>", rank = 2)]
pub async fn delete(
    db: &State<Database>,
    user: User,
    target: Reference,
    msg: Reference,
) -> Result<EmptyResponse> {
    let message = msg.as_message_in_channel(db, &target.id).await?;

    if message.author != user.id {
        let channel = target.as_channel(db).await?;
        let mut query = DatabasePermissionQuery::new(db, &user).channel(&channel);
        calculate_channel_permissions(&mut query)
            .await
            .throw_if_lacking_channel_permission(ChannelPermission::ManageMessages)?;
    }

    message.delete(db).await.map(|_| EmptyResponse)
}
