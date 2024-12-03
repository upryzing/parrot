use rocket::serde::json::Json;
use rocket::State;
use upryzing_database::util::reference::Reference;
use upryzing_database::{Database, User, AMQP};
use upryzing_models::v0;
use upryzing_result::{create_error, Result};

/// # Accept Friend Request
///
/// Accept another user's friend request.
#[openapi(tag = "Relationships")]
#[put("/<target>/friend")]
pub async fn add(
    db: &State<Database>,
    amqp: &State<AMQP>,
    mut user: User,
    target: Reference,
) -> Result<Json<v0::User>> {
    let mut target = target.as_user(db).await?;

    if user.bot.is_some() || target.bot.is_some() {
        return Err(create_error!(IsBot));
    }

    user.add_friend(db, amqp, &mut target).await?;
    Ok(Json(target.into(db, &user).await))
}
