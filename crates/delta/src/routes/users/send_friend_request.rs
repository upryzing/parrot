use rocket::serde::json::Json;
use rocket::State;
use upryzing_database::util::reference::Reference;
use upryzing_database::{Database, User};
use upryzing_models::v0;
use upryzing_result::{create_error, Result};

/// # Send Friend Request
///
/// Send a friend request to another user.
#[openapi(tag = "Relationships")]
#[post("/friend", data = "<data>")]
pub async fn send_friend_request(
    db: &State<Database>,
    mut user: User,
    data: Json<v0::DataSendFriendRequest>,
) -> Result<Json<v0::User>> {
    if let Some((username, discriminator)) = data.username.split_once('#') {
        let mut target = db.fetch_user_by_username(username, discriminator).await?;

        if user.bot.is_some() || target.bot.is_some() {
            return Err(create_error!(IsBot));
        }

        user.add_friend(db, &mut target).await?;
        Ok(Json(target.into(db, &user).await))
    } else {
        Err(create_error!(InvalidProperty))
    }
}
