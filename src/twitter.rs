// use tokio::fs::File;
// use tokio_util::codec::{BytesCodec, FramedRead};

use crate::Post;
use std::{error::Error, println};

use oauth2::*;
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;

pub async fn syndicate(post: &Post) -> Result<(), Box<dyn Error>>
{
  println!("TODO: Syndicating to Twitter");
  let twitter_share = format!("http://twitter.com/share?text={}&url={}&hashtags=gamedev,writing", post.title, post.url);
  let _ = open::that(twitter_share);
  
  // let api_key = ClientId::new("Vy2EQQVtwpFAAkhtKfYOOVopb".to_string());
  // let api_secret = ClientSecret::new("bYHxI4eiNyQvl4DdCKmLWopUAYaXo5SCXTn4dJDFAGEO4wMTaf".to_string());
  // let auth_url = AuthUrl::new("https://api.twitter.com/oauth/authenticate".to_string())?;

  // let client = BasicClient::new(
  //   api_key,
  //   Some(api_secret),
  //   auth_url,
  //   None,
  // )
  // .set_redirect_uri(
  //   RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
  // );

  // let (authorize_url, csrf_state) = client
  //   .authorize_url(CsrfToken::new_random)
  //   // This example is requesting access to the user's public repos and email.
  //   .add_scope(Scope::new("public_repo".to_string()))
  //   .add_scope(Scope::new("user:email".to_string()))
  //   .url();

  //   println!(
  //       "Open this URL in your browser:\n{}\n",
  //       authorize_url.to_string()
  //   );


  // // if let Some(path) = post.image_path.clone() {
  // //   let file = File::open(&path).await?;


  // //   let form = reqwest::multipart::Form::new()
  // //     // .text("command", "INIT")
  // //     // .text("total_bytes", file.metadata()?.len().to_string())
  // //     // .text("media_type", "image/png");
  // //     .text("media_category", "TWEET_IMAGE")
  // //     .text("additional_owners", "1723409529826648064");

  // //   // /1.1/media/upload.json?media_category=TWEET_IMAGE&additional_owners=3805104374" -f adsapi-heirarchy.png -F media

  // //   let stream = FramedRead::new(file, BytesCodec::new());
  // //   let body = Body::wrap_stream(stream);

  // //     println!(" Uploading image...");
  // //   let client = reqwest::Client::new();
  // //   let response = client.post("https://upload.twitter.com/1.1/media/upload.json")
  // //   .header("Authorization", BEARER)
  // //   .multipart(form)
  // //   .body(body)
  // //   .send().await?;
    
  // //   println!("{:?}", response);
  // //   println!("{}", response.text().await?);
  // // }

  Ok(())
}