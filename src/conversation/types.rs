use nutype::nutype;

#[nutype(derive(Debug, Clone), validate(not_empty))]
pub struct ConversationTitle(String);
