use rocket::State;
use rocket_empty::EmptyResponse;
use upryzing_database::{
    util::{permissions::DatabasePermissionQuery, reference::Reference},
    Database, User,
};
use upryzing_permissions::{calculate_channel_permissions, ChannelPermission};
use upryzing_result::{create_error, Result};

/// # Acknowledge Message
///
/// Lets the server and all other clients know that we've seen this message id in this channel.
#[openapi(tag = "Messaging")]
#[put("/<target>/ack/<message>")]
pub async fn ack(
    db: &State<Database>,
    user: User,
    target: Reference,
    message: Reference,
) -> Result<EmptyResponse> {
    if user.bot.is_some() {
        return Err(create_error!(IsBot));
    }

    let channel = target.as_channel(db).await?;
    let mut query = DatabasePermissionQuery::new(db, &user).channel(&channel);
    calculate_channel_permissions(&mut query)
        .await
        .throw_if_lacking_channel_permission(ChannelPermission::ViewChannel)?;

    channel
        .ack(&user.id, &message.id)
        .await
        .map(|_| EmptyResponse)
}

#[cfg(test)]
mod test {
    use crate::{rocket, util::test::TestHarness};
    use rocket::http::{Header, Status};
    use upryzing_database::{events::client::EventV1, Channel};
    use upryzing_models::v0::DataCreateGroup;

    #[rocket::async_test]
    async fn success_ack_channel() {
        let mut harness = TestHarness::new().await;
        let (_, session, user) = harness.new_user().await;

        let group = Channel::create_group(
            &harness.db,
            DataCreateGroup {
                ..Default::default()
            },
            user.id.clone(),
        )
        .await
        .expect("`Channel`");

        let message_id = ulid::Ulid::new().to_string();
        let response = harness
            .client
            .put(format!("/channels/{}/ack/{}", group.id(), message_id))
            .header(Header::new("x-session-token", session.token.to_string()))
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::NoContent);
        drop(response);

        let event = harness
            .wait_for_event(&format!("{}!", user.id), |event| match event {
                EventV1::ChannelAck { id, .. } => id == group.id(),
                _ => false,
            })
            .await;

        match event {
            EventV1::ChannelAck {
                message_id: m_id, ..
            } => assert_eq!(m_id, message_id),
            _ => unreachable!(),
        };
    }
}
