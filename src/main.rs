use std::{error::Error, println};

mod post;
use crate::post::Post;

mod credential;
use crate::credential::CredentialStore;

mod dev_to;
mod hacker_news;
mod mastodon;
mod medium;
mod twitter;
mod sendy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
  let credentials = CredentialStore::load();

  let post = Post::from_rss().await?;

  // sendy::syndicate(&post, credentials.sendy).await?;
  mastodon::syndicate(&post).await?;
  hacker_news::syndicate(&post, credentials.hacker_news).await?;
  medium::syndicate(&post).await?;
  twitter::syndicate(&post).await?;
  dev_to::syndicate(&post).await?;
  // TODO: linked_in

  Ok(())
}

