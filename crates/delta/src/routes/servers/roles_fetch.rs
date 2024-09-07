use rocket::{serde::json::Json, State};
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_models::v0;
use upryzing_permissions::PermissionQuery;
use upryzing_result::{create_error, Result};

/// # Fetch Role
///
/// Fetch a role by its id.
#[openapi(tag = "Server Permissions")]
#[get("/<target>/roles/<role_id>")]
pub async fn fetch(
    db: &State<Database>,
    user: User,
    target: Reference,
    role_id: String,
) -> Result<Json<v0::Role>> {
    let mut server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    if !query.are_we_a_member().await {
        return Err(create_error!(NotFound));
    }

    let role = server.roles.remove(&role_id);

    if let Some(role) = role {
        Ok(Json(role.into()))
    } else {
        Err(create_error!(NotFound))
    }
}
