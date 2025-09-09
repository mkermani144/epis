use anyhow::Result;
use colored::Colorize;

use crate::{
  config::config,
  lingoo::{api::LingooHttpApi, utils::chat_round},
};

pub async fn handle_lingoo_new_chat() -> Result<()> {
  let epis_host = &*config.epis_host;
  let lingoo_api = LingooHttpApi::new(epis_host.into());

  let res = lingoo_api.create_conversation().await?;
  let cid = res.cid();

  let ai_reply = chat_round(cid.to_string()).await?;
  println!("{} {}", "Epis >".bold().cyan(), ai_reply);

  // TODO: Set title

  loop {
    let ai_reply = chat_round(cid.to_string()).await?;
    println!("{} {}", "Epis >".bold().cyan(), ai_reply);
  }
}
