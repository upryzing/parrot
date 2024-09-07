use upryzing_database::util::permissions::DatabasePermissionQuery;
use upryzing_database::{util::reference::Reference, Channel, Database, User};
use upryzing_models::v0;
use upryzing_permissions::{calculate_server_permissions, ChannelPermission};
use upryzing_result::{create_error, Result};

use rocket::serde::json::Json;
use rocket::State;
use validator::Validate;

/// # Create Channel
///
/// Create a new Text or Voice channel.
#[openapi(tag = "Server Information")]
#[post("/<server>/channels", data = "<data>")]
pub async fn create_server_channel(
    db: &State<Database>,
    user: User,
    server: Reference,
    data: Json<v0::DataCreateServerChannel>,
) -> Result<Json<v0::Channel>> {
    let data = data.into_inner();
    data.validate().map_err(|error| {
        create_error!(FailedValidation {
            error: error.to_string()
        })
    })?;

    let mut server = server.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    calculate_server_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::ManageChannel)?;

    Channel::create_server_channel(db, &mut server, data, true)
        .await
        .map(|channel| channel.into())
        .map(Json)
}
