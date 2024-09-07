use rocket::State;
use rocket_empty::EmptyResponse;
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_permissions::PermissionQuery;
use upryzing_result::{create_error, Result};

/// # Mark Server As Read
///
/// Mark all channels in a server as read.
#[openapi(tag = "Server Information")]
#[put("/<target>/ack")]
pub async fn ack(db: &State<Database>, user: User, target: Reference) -> Result<EmptyResponse> {
    if user.bot.is_some() {
        return Err(create_error!(IsBot));
    }

    let server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    if !query.are_we_a_member().await {
        return Err(create_error!(NotFound));
    }

    db.acknowledge_channels(&user.id, &server.channels)
        .await
        .map(|_| EmptyResponse)
}
