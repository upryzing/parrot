// Queue Type: Debounced
use crate::Database;

use deadqueue::limited::Queue;
use once_cell::sync::Lazy;
use std::{collections::HashMap, time::Duration};

use upryzing_result::Result;

use super::{
    apple_notifications::{self, ApnJob},
    DelayedTask,
};

/// Enumeration of possible events
#[derive(Debug, Eq, PartialEq)]
pub enum AckEvent {
    /// Add mentions for a user in a channel
    AddMention {
        /// Message IDs
        ids: Vec<String>,
    },

    /// Acknowledge message in a channel for a user
    AckMessage {
        /// Message ID
        id: String,
    },
}

/// Task information
struct Data {
    /// Channel to ack
    channel: String,
    /// User to ack for
    user: String,
    /// Event
    event: AckEvent,
}

#[derive(Debug)]
struct Task {
    event: AckEvent,
}

static Q: Lazy<Queue<Data>> = Lazy::new(|| Queue::new(10_000));

/// Queue a new task for a worker
pub async fn queue(channel: String, user: String, event: AckEvent) {
    Q.try_push(Data {
        channel,
        user,
        event,
    })
    .ok();

    info!("Queue is using {} slots from {}.", Q.len(), Q.capacity());
}

pub async fn handle_ack_event(
    event: &AckEvent,
    db: &Database,
    authifier_db: &authifier::Database,
    user: &str,
    channel: &str,
) -> Result<()> {
    match &event {
        #[allow(clippy::disallowed_methods)] // event is sent by higher level function
        AckEvent::AckMessage { id } => {
            let unread = db.fetch_unread(user, channel).await?;
            let updated = db.acknowledge_message(channel, user, id).await?;

            if let (Some(before), Some(after)) = (unread, updated) {
                let before_mentions = before.mentions.unwrap_or_default().len();
                let after_mentions = after.mentions.unwrap_or_default().len();

                let mentions_acked = before_mentions - after_mentions;

                if mentions_acked > 0 {
                    if let Ok(sessions) = authifier_db.find_sessions(user).await {
                        for session in sessions {
                            if let Some(sub) = session.subscription {
                                if sub.endpoint == "apn" {
                                    apple_notifications::queue(ApnJob::from_ack(
                                        session.id,
                                        user.to_string(),
                                        sub.auth,
                                    ))
                                    .await;
                                }
                            }
                        }
                    }
                };
            }
        }
        AckEvent::AddMention { ids } => {
            db.add_mention_to_unread(channel, user, ids).await?;
        }
    };

    Ok(())
}

/// Start a new worker
pub async fn worker(db: Database, authifier_db: authifier::Database) {
    let mut tasks = HashMap::<(String, String), DelayedTask<Task>>::new();
    let mut keys = vec![];

    loop {
        // Find due tasks.
        for (key, task) in &tasks {
            if task.should_run() {
                keys.push(key.clone());
            }
        }

        // Commit any due tasks to the database.
        for key in &keys {
            if let Some(task) = tasks.remove(key) {
                let Task { event } = task.data;
                let (user, channel) = key;

                if let Err(err) = handle_ack_event(&event, &db, &authifier_db, user, channel).await
                {
                    error!("{err:?} for {event:?}. ({user}, {channel})");
                } else {
                    info!("User {user} ack in {channel} with {event:?}");
                }
            }
        }

        // Clear keys
        keys.clear();

        // Queue incoming tasks.
        while let Some(Data {
            channel,
            user,
            mut event,
        }) = Q.try_pop()
        {
            let key = (user, channel);
            if let Some(task) = tasks.get_mut(&key) {
                task.delay();

                match &mut event {
                    AckEvent::AddMention { ids } => {
                        if let AckEvent::AddMention { ids: existing } = &mut task.data.event {
                            existing.append(ids);
                        } else {
                            task.data.event = event;
                        }
                    }
                    AckEvent::AckMessage { .. } => {
                        task.data.event = event;
                    }
                }
            } else {
                tasks.insert(key, DelayedTask::new(Task { event }));
            }
        }

        // Sleep for an arbitrary amount of time.
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
