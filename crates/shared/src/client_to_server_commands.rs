
pub enum ClientToServerCommand {
    Click(ClientToServerCommandClickType),
    ChatMessage(ClientToServerCommandChatMessageType)
}

pub struct ClientToServerCommandClickType {
    pub location: glam::Vec3,
}
pub struct ClientToServerCommandChatMessageType {
    pub messsage: String,
}