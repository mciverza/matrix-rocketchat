use diesel::sqlite::SqliteConnection;
use ruma_events::room::message::{MessageEvent, MessageEventContent, TextMessageEventContent};
use slog::Logger;

use api::RocketchatApi;
use db::{Room, UserOnRocketchatServer};
use errors::*;

/// Forwards messages
pub struct Forwarder<'a> {
    connection: &'a SqliteConnection,
    logger: &'a Logger,
}

impl<'a> Forwarder<'a> {
    /// Create a new `Forwarder`.
    pub fn new(connection: &'a SqliteConnection, logger: &'a Logger) -> Forwarder<'a> {
        Forwarder {
            connection: connection,
            logger: logger,
        }
    }

    /// Forwards messages to Rocket.Chat
    pub fn process(&self, event: &MessageEvent, room: &Room) -> Result<()> {
        match room.rocketchat_server(self.connection)? {
            Some(rocketchat_server) => {
                let user_on_rocketchat_server =
                    UserOnRocketchatServer::find(self.connection, &event.user_id, rocketchat_server.id)?;

                if user_on_rocketchat_server.is_virtual_user {
                    debug!(self.logger, "Skipping event, because it was sent by a virtual user");
                    return Ok(());
                }

                match event.content {
                    MessageEventContent::Text(ref text_content) => {

                        let rocketchat_api =
                            RocketchatApi::new(rocketchat_server.rocketchat_url, self.logger.clone())?
                                .with_credentials(user_on_rocketchat_server.rocketchat_user_id.clone().unwrap_or_default(),
                                                  user_on_rocketchat_server.rocketchat_auth_token.clone().unwrap_or_default());
                        self.forward_text_message(text_content, &rocketchat_api, room)?;
                    }
                    _ => info!(self.logger, format!("Forwarding the type {} is not implemented.", event.event_type)),
                }

                user_on_rocketchat_server.user(self.connection)?.set_last_message_sent(self.connection)?;
            }
            None => debug!(self.logger, "Skipping event, because the room is not bridged"),
        }

        Ok(())
    }

    /// Forward a text message
    pub fn forward_text_message(&self,
                                content: &TextMessageEventContent,
                                rocketchat_api: &Box<RocketchatApi>,
                                room: &Room)
                                -> Result<()> {
        rocketchat_api.post_chat_message(&content.body, &room.rocketchat_room_id.clone().unwrap_or_default())
    }
}
