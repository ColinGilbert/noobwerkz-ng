use crate::index_types::*;

pub enum ServerToClientCommand {
    Chats(ServerToClientCommandChatType),
}

pub struct PlayerChatMessage {
    pub player: Player3DUIDIndex,
    pub message: String,
}
pub struct ServerToClientCommandChatType {
    pub chats: PlayerChatMessages<PlayerChatMessage>
}