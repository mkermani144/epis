use std::sync::Arc;

use anyhow::Result;
use colored::Colorize;
use inquire::Text;

use crate::{
  config::CONFIG,
  lingoo::{api::LingooHttpApi, utils::chat_round},
};

pub async fn handle_lingoo_new_chat() -> Result<()> {
  let epis_host = &*CONFIG.epis_host;
  let api = Arc::new(LingooHttpApi::new(epis_host.into()));

  let creation_res = api.create_conversation().await?;
  let cid = creation_res.cid().to_string();

  let user_message = Text::new("").prompt()?;

  // We don't care about setting title, and let it fail silently if needed
  {
    let cid = cid.clone();
    let lingoo_api = api.clone();
    let user_message = user_message.clone();

    tokio::spawn(async move {
      let title = lingoo_api
        .generate_conversation_title(user_message)
        .await?
        .into_title();

      lingoo_api.set_conversation_title(cid, title).await?;

      Ok::<_, anyhow::Error>(())
    });
  }

  let lingoo_api = api.clone();

  let ai_reply = lingoo_api
    .chat(cid.clone(), user_message)
    .await?
    .into_response();
  println!("{} {}", "Epis >".bold().cyan(), ai_reply);

  loop {
    let ai_reply = chat_round(cid.clone()).await?;
    println!("{} {}", "Epis >".bold().cyan(), ai_reply);
  }
}
