pub struct Message(pub String);

pub enum ChatMessageRole {
    User,
    AI,
    System,
}

pub struct ChatMessage {
    pub role: ChatMessageRole,
    pub message: Message,
}
