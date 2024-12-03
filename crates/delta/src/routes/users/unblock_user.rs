use rocket::serde::json::Json;
use rocket::State;
use upryzing_database::util::reference::Reference;
use upryzing_database::{Database, User};
use upryzing_models::v0;
use upryzing_result::Result;

/// # Unblock User
///
/// Unblock another user by their id.
#[openapi(tag = "Relationships")]
#[delete("/<target>/block")]
pub async fn unblock(
    db: &State<Database>,
    mut user: User,
    target: Reference,
) -> Result<Json<v0::User>> {
    let mut target = target.as_user(db).await?;

    user.unblock_user(db, &mut target).await?;
    Ok(Json(target.into(db, &user).await))
}
