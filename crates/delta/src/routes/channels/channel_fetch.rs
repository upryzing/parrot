use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};

use rocket::{serde::json::Json, State};
use upryzing_models::v0;
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission};
use upryzing_result::Result;

/// # Fetch Channel
///
/// Fetch channel by its id.
#[openapi(tag = "Channel Information")]
#[get("/<target>")]
pub async fn fetch(
    db: &State<Database>,
    user: User,
    target: Reference,
) -> Result<Json<v0::Channel>> {
    let channel = target.as_channel(db).await?;

    let mut query = DatabasePermissionQuery::new(db, &user).channel(&channel);
    calculate_channel_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::ViewChannel)?;

    Ok(Json(channel.into()))
}

#[cfg(test)]
mod test {
    use crate::{rocket, util::test::TestHarness};
    use rocket::http::{Header, Status};
    use upryzing_database::Channel;
    use upryzing_models::v0;

    #[rocket::async_test]
    async fn fetch_channel() {
        let harness = TestHarness::new().await;
        let (_, session, user) = harness.new_user().await;

        let group = Channel::create_group(
            &harness.db,
            v0::DataCreateGroup {
                name: TestHarness::rand_string(),
                ..Default::default()
            },
            user.id.to_string(),
        )
        .await
        .unwrap();

        let response = harness
            .client
            .get(format!("/channels/{}", group.id()))
            .header(Header::new("x-session-token", session.token.to_string()))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);

        let channel: v0::Channel = response.into_json().await.expect("`Channel`");
        assert_eq!(channel, group.into());
    }
}
