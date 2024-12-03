use rocket::State;
use rocket_empty::EmptyResponse;
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_permissions::{calculate_server_permissions, ChannelPermission};
use upryzing_result::Result;

/// # Unban user
///
/// Remove a user's ban.
#[openapi(tag = "Server Members")]
#[delete("/<server>/bans/<target>")]
pub async fn unban(
    db: &State<Database>,
    user: User,
    server: Reference,
    target: Reference,
) -> Result<EmptyResponse> {
    let server = server.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    calculate_server_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::BanMembers)?;

    let ban = target.as_ban(db, &server.id).await?;
    db.delete_ban(&ban.id).await.map(|_| EmptyResponse)
}
