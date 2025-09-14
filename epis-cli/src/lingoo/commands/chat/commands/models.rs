use derive_more::{Display, From};
use epis::lingoo::handlers::list_conversations::ListLingooConversationsResponseData;

#[derive(Display)]
#[display(
  "{}{}",
  self.title.as_ref().map_or("untitled", |title| title),
  if self.title.is_none() { format!(" ({id:.5}...)") } else { "".into() }
)]
pub struct PartialConversation {
  pub id: String,
  pub title: Option<String>,
}

#[derive(From)]
pub struct PartialConversationsList(Vec<PartialConversation>);
impl PartialConversationsList {
  pub fn into_vec(self) -> Vec<PartialConversation> {
    self.0
  }
}

impl From<ListLingooConversationsResponseData> for PartialConversationsList {
  fn from(res_data: ListLingooConversationsResponseData) -> Self {
    res_data
      .data()
      .into_iter()
      .map(|conversation| PartialConversation {
        id: (conversation.id().to_string()),
        title: conversation.title(),
      })
      .collect::<Vec<PartialConversation>>()
      .into()
  }
}
