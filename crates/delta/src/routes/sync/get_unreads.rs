use rocket::serde::json::Json;
use rocket::State;
use upryzing_database::{Database, User};
use upryzing_models::v0;
use upryzing_result::Result;

/// # Fetch Unreads
///
/// Fetch information about unread state on channels.
#[openapi(tag = "Sync")]
#[get("/unreads")]
pub async fn unreads(db: &State<Database>, user: User) -> Result<Json<Vec<v0::ChannelUnread>>> {
    db.fetch_unreads(&user.id)
        .await
        .map(|v| v.into_iter().map(|u| u.into()).collect())
        .map(Json)
}
