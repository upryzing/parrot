use rocket::State;
use rocket_empty::EmptyResponse;
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, RemovalIntention, User,
};
use upryzing_permissions::{calculate_server_permissions, ChannelPermission};
use upryzing_result::{create_error, Result};

/// # Kick Member
///
/// Removes a member from the server.
#[openapi(tag = "Server Members")]
#[delete("/<target>/members/<member>")]
pub async fn kick(
    db: &State<Database>,
    user: User,
    target: Reference,
    member: Reference,
) -> Result<EmptyResponse> {
    let server = target.as_server(db).await?;

    if member.id == user.id {
        return Err(create_error!(CannotRemoveYourself));
    }

    if member.id == server.owner {
        return Err(create_error!(InvalidOperation));
    }

    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    calculate_server_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::KickMembers)?;

    let member = member.as_member(db, &server.id).await?;
    if member.get_ranking(query.server_ref().as_ref().unwrap())
        <= query.get_member_rank().unwrap_or(i64::MIN)
    {
        return Err(create_error!(NotElevated));
    }

    member
        .remove(db, &server, RemovalIntention::Kick, false)
        .await
        .map(|_| EmptyResponse)
}
