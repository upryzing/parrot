use std::str::FromStr;

#[cfg(feature = "rocket-impl")]
use rocket::request::FromParam;
#[cfg(feature = "rocket-impl")]
use schemars::{
    schema::{InstanceType, Schema, SchemaObject, SingleOrVec},
    JsonSchema,
};
use upryzing_result::Result;

use crate::{
    Bot, Channel, Database, Emoji, Invite, Member, Message, Server, ServerBan, User, Webhook,
};

/// Reference to some object in the database
#[derive(Serialize, Deserialize)]
pub struct Reference {
    /// Id of object
    pub id: String,
}

impl Reference {
    /// Create a Ref from an unchecked string
    pub fn from_unchecked(id: String) -> Reference {
        Reference { id }
    }

    /// Fetch ban from Ref
    pub async fn as_ban(&self, db: &Database, server: &str) -> Result<ServerBan> {
        db.fetch_ban(server, &self.id).await
    }

    /// Fetch bot from Ref
    pub async fn as_bot(&self, db: &Database) -> Result<Bot> {
        db.fetch_bot(&self.id).await
    }

    /// Fetch emoji from Ref
    pub async fn as_emoji(&self, db: &Database) -> Result<Emoji> {
        db.fetch_emoji(&self.id).await
    }

    /// Fetch channel from Ref
    pub async fn as_channel(&self, db: &Database) -> Result<Channel> {
        db.fetch_channel(&self.id).await
    }

    /// Fetch invite from Ref or create invite to server if discoverable
    pub async fn as_invite(&self, db: &Database) -> Result<Invite> {
        if ulid::Ulid::from_str(&self.id).is_ok() {
            let server = self.as_server(db).await?;
            if !server.discoverable {
                return Err(create_error!(NotFound));
            }

            Ok(Invite::Server {
                code: self.id.to_string(),
                server: server.id,
                creator: server.owner,
                channel: server
                    .channels
                    .into_iter()
                    .next()
                    .ok_or(create_error!(NotFound))?,
            })
        } else {
            db.fetch_invite(&self.id).await
        }
    }

    /// Fetch message from Ref
    pub async fn as_message(&self, db: &Database) -> Result<Message> {
        db.fetch_message(&self.id).await
    }

    /// Fetch message from Ref and validate channel
    pub async fn as_message_in_channel(&self, db: &Database, channel: &str) -> Result<Message> {
        let msg = db.fetch_message(&self.id).await?;
        if msg.channel != channel {
            return Err(create_error!(NotFound));
        }

        Ok(msg)
    }

    /// Fetch member from Ref
    pub async fn as_member(&self, db: &Database, server: &str) -> Result<Member> {
        db.fetch_member(server, &self.id).await
    }

    /// Fetch server from Ref
    pub async fn as_server(&self, db: &Database) -> Result<Server> {
        db.fetch_server(&self.id).await
    }

    /// Fetch user from Ref
    pub async fn as_user(&self, db: &Database) -> Result<User> {
        db.fetch_user(&self.id).await
    }

    /// Fetch webhook from Ref
    pub async fn as_webhook(&self, db: &Database) -> Result<Webhook> {
        db.fetch_webhook(&self.id).await
    }
}

#[cfg(feature = "rocket-impl")]
impl<'r> FromParam<'r> for Reference {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Ok(Reference::from_unchecked(param.into()))
    }
}

#[cfg(feature = "rocket-impl")]
impl JsonSchema for Reference {
    fn schema_name() -> String {
        "Id".to_string()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> Schema {
        Schema::Object(SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            ..Default::default()
        })
    }
}
