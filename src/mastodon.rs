use crate::Post;
use mastodon_async::helpers::toml;
use mastodon_async::prelude::*;
use mastodon_async::helpers::cli;
use isolang::Language;

use std::{error::Error, vec};

pub async fn syndicate(post: &Post) -> Result<(), Box<dyn Error>>
{
  println!("Syndicating to Mastodon");

  let mastodon = get_mastodon_data().await?;

  let mut media_ids = vec![];

  if let Some(path) = post.image_path.clone() {
    println!("  Uploading image...");
    let media = mastodon.media(path, None).await?;
    let media = mastodon
      .wait_for_processing(media, Default::default())
      .await?;
    media_ids.push(media.id);
    println!("  Photo uploaded to: {}", media.url);
  }

  let status = StatusBuilder::new()
    .status(format!(
        "{} - {}\n #gamedev #writing",
        post.title,
        post.url
      )
    )
    .media_ids(media_ids)
    // Uncomment for testing
    // .visibility(Visibility::Unlisted)
    .language(Language::Eng)
    .build()?;

  let status = mastodon.new_status(status).await?;

  println!("  Done: {}", status.uri);

  let _ = open::that("https://mastodon.gamedev.place/@onwritinggames");
    
  Ok(())
}

pub async fn get_mastodon_data() -> Result<Mastodon, Box<dyn Error>> {
    if let Ok(data) = toml::from_file("mastodon-data.toml") {
        Ok(Mastodon::from(data))
    } else {
      register().await
    }
}

pub async fn register() -> Result<Mastodon, Box<dyn Error>> {
    let website = "https://mastodon.gamedev.place/";
    let registration = Registration::new(website.trim())
        .client_name("elefren-examples")
        .scopes(Scopes::all())
        .website("https://github.com/dscottboggs/mastodon-async")
        .build()
        .await?;
    let mastodon = cli::authenticate(registration).await?;

    // Save app data for using on the next run.
    toml::to_file(&mastodon.data, "mastodon-data.toml")?;

    Ok(mastodon)
}