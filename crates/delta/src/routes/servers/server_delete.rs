use rocket::State;
use upryzing_database::{util::reference::Reference, Database, RemovalIntention, User};
use upryzing_models::v0;
use upryzing_result::Result;

use rocket_empty::EmptyResponse;

/// # Delete / Leave Server
///
/// Deletes a server if owner otherwise leaves.
#[openapi(tag = "Server Information")]
#[delete("/<target>?<options..>")]
pub async fn delete(
    db: &State<Database>,
    user: User,
    target: Reference,
    options: v0::OptionsServerDelete,
) -> Result<EmptyResponse> {
    let server = target.as_server(db).await?;
    let member = db.fetch_member(&target.id, &user.id).await?;

    if server.owner == user.id {
        server.delete(db).await
    } else {
        member
            .remove(
                db,
                &server,
                RemovalIntention::Leave,
                options.leave_silently.unwrap_or_default(),
            )
            .await
    }
    .map(|_| EmptyResponse)
}
