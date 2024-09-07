use rocket::{serde::json::Json, State};
use upryzing_database::{Database, User};
use upryzing_models::v0;
use upryzing_result::Result;

/// # Fetch Direct Message Channels
///
/// This fetches your direct messages, including any DM and group DM conversations.
#[openapi(tag = "Direct Messaging")]
#[get("/dms")]
pub async fn direct_messages(db: &State<Database>, user: User) -> Result<Json<Vec<v0::Channel>>> {
    db.find_direct_messages(&user.id)
        .await
        .map(|v| v.into_iter().map(Into::into).collect())
        .map(Json)
}
