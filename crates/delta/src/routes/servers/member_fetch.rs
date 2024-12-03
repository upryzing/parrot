use rocket::{serde::json::Json, State};
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_models::v0;
use upryzing_permissions::PermissionQuery;
use upryzing_result::{create_error, Result};

/// # Fetch Member
///
/// Retrieve a member.
#[openapi(tag = "Server Members")]
#[get("/<target>/members/<member>?<roles>")]
pub async fn fetch(
    db: &State<Database>,
    user: User,
    target: Reference,
    member: Reference,
    roles: Option<bool>,
) -> Result<Json<v0::MemberResponse>> {
    let server = target.as_server(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).server(&server);
    if !query.are_we_a_member().await {
        return Err(create_error!(NotFound));
    }

    let member = member.as_member(db, &server.id).await?;
    if let Some(true) = roles {
        Ok(Json(v0::MemberResponse::MemberWithRoles {
            roles: server
                .roles
                .into_iter()
                .filter(|(k, _)| member.roles.contains(k))
                .map(|(k, v)| (k, v.into()))
                .collect(),
            member: member.into(),
        }))
    } else {
        Ok(Json(v0::MemberResponse::Member(member.into())))
    }
}
