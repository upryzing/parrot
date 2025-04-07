use rocket::{serde::json::Json, State};
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, PartialServer, User,
};
use upryzing_models::v0;
use upryzing_permissions::{
    calculate_server_permissions, ChannelPermission, DataPermissionsValue, Override,
};
use upryzing_result::Result;

/// # Set Default Permission
///
/// Sets permissions for the default role in this server.
#[openapi(tag = "Server Permissions")]
#[put("/<target>/permissions/default", data = "<data>", rank = 1)]
pub async fn set_default_permissions(
    db: &State<Database>,
    user: User,
    target: Reference,
    data: Json<DataPermissionsValue>,
) -> Result<Json<v0::Server>> {
    let data = data.into_inner();

    let mut server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    let permissions = calculate_server_permissions(&mut query).await;

    permissions.throw_if_lacking_channel_permission(ChannelPermission::ManagePermissions)?;

    // Ensure we have permissions to grant these permissions forwards
    permissions
        .throw_permission_override(
            None,
            &Override {
                allow: data.permissions,
                deny: 0,
            },
        )
        .await?;

    server
        .update(
            db,
            PartialServer {
                default_permissions: Some(data.permissions as i64),
                ..Default::default()
            },
            vec![],
        )
        .await?;

    Ok(Json(server.into()))
}
