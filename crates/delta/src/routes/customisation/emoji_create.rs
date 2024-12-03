use upryzing_config::config;
use upryzing_database::{util::permissions::DatabasePermissionQuery, Database, Emoji, File, User};
use upryzing_models::v0;
use upryzing_permissions::{calculate_server_permissions, ChannelPermission};
use upryzing_result::{create_error, Result};
use validator::Validate;

use rocket::{serde::json::Json, State};

/// # Create New Emoji
///
/// Create an emoji by its Pigeon upload id.
#[openapi(tag = "Emojis")]
#[put("/emoji/<id>", data = "<data>")]
pub async fn create_emoji(
    db: &State<Database>,
    user: User,
    id: String,
    data: Json<v0::DataCreateEmoji>,
) -> Result<Json<v0::Emoji>> {
    let config = config().await;

    let data = data.into_inner();
    data.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })?;

    // Bots cannot manage emojis
    if user.bot.is_some() {
        return Err(create_error!(IsBot));
    }

    // Validate we have permission to write into parent
    match &data.parent {
        v0::EmojiParent::Server { id } => {
            let server = db.fetch_server(id).await?;

            // Check for permission
            let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
            calculate_server_permissions(&mut query)
                .await
                .throw_if_lacking_channel_permission(ChannelPermission::ManageCustomisation)?;

            // Check that we haven't hit the emoji limit
            let emojis = db.fetch_emoji_by_parent_id(&server.id).await?;
            if emojis.len() >= config.features.limits.global.server_emoji {
                return Err(create_error!(TooManyEmoji {
                    max: config.features.limits.global.server_emoji,
                }));
            }
        }
        v0::EmojiParent::Detached => return Err(create_error!(InvalidOperation)),
    };

    // Find the relevant attachment
    let attachment = File::use_emoji(db, &id, &id, &user.id).await?;

    // Create the emoji object
    let emoji = Emoji {
        id,
        parent: data.parent.into(),
        creator_id: user.id,
        name: data.name,
        animated: "image/gif" == &attachment.content_type,
        nsfw: data.nsfw,
    };

    // Save emoji
    emoji.create(db).await?;
    Ok(Json(emoji.into()))
}
