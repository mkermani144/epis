use nutype::nutype;

#[nutype(derive(Debug, Clone, AsRef), validate(not_empty))]
pub struct ConversationTitle(String);
