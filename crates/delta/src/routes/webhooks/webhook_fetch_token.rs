use rocket::{serde::json::Json, State};
use upryzing_database::{util::reference::Reference, Database};
use upryzing_models::v0::Webhook;
use upryzing_result::Result;

/// # Gets a webhook
///
/// Gets a webhook with a token
#[openapi(tag = "Webhooks")]
#[get("/<webhook_id>/<token>")]
pub async fn webhook_fetch_token(
    db: &State<Database>,
    webhook_id: Reference,
    token: String,
) -> Result<Json<Webhook>> {
    let webhook = webhook_id.as_webhook(db).await?;
    webhook.assert_token(&token)?;
    Ok(Json(webhook.into()))
}
