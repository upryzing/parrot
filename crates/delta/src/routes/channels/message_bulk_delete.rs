use chrono::Utc;
use rocket::{serde::json::Json, State};
use rocket_empty::EmptyResponse;
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, Message, User,
};
use upryzing_models::v0;
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission};
use upryzing_result::{create_error, Result};
use validator::Validate;

/// # Bulk Delete Messages
///
/// Delete multiple messages you've sent or one you have permission to delete.
///
/// This will always require `ManageMessages` permission regardless of whether you own the message or not.
///
/// Messages must have been sent within the past 1 week.
#[openapi(tag = "Messaging")]
#[delete("/<target>/messages/bulk", data = "<options>", rank = 1)]
pub async fn bulk_delete_messages(
    db: &State<Database>,
    user: User,
    target: Reference,
    options: Json<v0::OptionsBulkDelete>,
) -> Result<EmptyResponse> {
    let options = options.into_inner();
    options.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })?;

    for id in &options.ids {
        if ulid::Ulid::from_string(id)
            .map_err(|_| create_error!(InvalidOperation))?
            .datetime()
            .signed_duration_since(Utc::now())
            .num_days()
            .abs()
            > 7
        {
            return Err(create_error!(InvalidOperation));
        }
    }

    let channel = target.as_channel(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).channel(&channel);
    calculate_channel_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::ManageMessages)?;

    Message::bulk_delete(db, &target.id, options.ids)
        .await
        .map(|_| EmptyResponse)
}
