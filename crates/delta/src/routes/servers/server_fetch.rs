use rocket::{serde::json::Json, State};
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_models::v0;
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission, PermissionQuery};
use upryzing_result::{create_error, Result};

/// # Fetch Server
///
/// Fetch a server by its id.
#[openapi(tag = "Server Information")]
#[get("/<target>?<options..>")]
pub async fn fetch(
    db: &State<Database>,
    user: User,
    target: Reference,
    options: v0::OptionsFetchServer,
) -> Result<Json<v0::FetchServerResponse>> {
    let server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    if !query.are_we_a_member().await {
        return Err(create_error!(NotFound));
    }

    if let Some(true) = options.include_channels {
        let all_channels = db.fetch_channels(&server.channels).await?;
        let mut visible_channels: Vec<v0::Channel> = vec![];

        for channel in all_channels {
            let mut channel_query = query.clone().channel(&channel);
            if calculate_channel_permissions(&mut channel_query)
                .await
                .has_channel_permission(ChannelPermission::ViewChannel)
            {
                visible_channels.push(channel.into());
            }
        }

        Ok(Json(v0::FetchServerResponse::ServerWithChannels {
            server: server.into(),
            channels: visible_channels,
        }))
    } else {
        Ok(Json(v0::FetchServerResponse::JustServer(server.into())))
    }
}
