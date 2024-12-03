use upryzing_database::util::permissions::DatabasePermissionQuery;
use upryzing_database::util::reference::Reference;
use upryzing_database::{Database, User};
use upryzing_models::v0;

use rocket::serde::json::Json;
use rocket::State;
use upryzing_permissions::{calculate_user_permissions, UserPermission};
use upryzing_result::{create_error, Result};

/// # Fetch Mutual Friends And Servers
///
/// Retrieve a list of mutual friends and servers with another user.
#[openapi(tag = "Relationships")]
#[get("/<target>/mutual")]
pub async fn mutual(
    db: &State<Database>,
    user: User,
    target: Reference,
) -> Result<Json<v0::MutualResponse>> {
    if target.id == user.id {
        return Err(create_error!(InvalidOperation));
    }

    let target = target.as_user(db).await?;

    let mut query = DatabasePermissionQuery::new(db, &user).user(&target);
    calculate_user_permissions(&mut query)
        .await
        .throw_if_lacking_user_permission(UserPermission::ViewProfile)?;

    Ok(Json(v0::MutualResponse {
        users: db.fetch_mutual_user_ids(&user.id, &target.id).await?,
        servers: db.fetch_mutual_server_ids(&user.id, &target.id).await?,
    }))
}
