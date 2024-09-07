use rocket::serde::json::Json;
use upryzing_database::User;
use upryzing_models::v0;
use upryzing_result::Result;

/// # Fetch Self
///
/// Retrieve your user information.
#[openapi(tag = "User Information")]
#[get("/@me")]
pub async fn fetch(user: User) -> Result<Json<v0::User>> {
    Ok(Json(user.into_self(false).await))
}
