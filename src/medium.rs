// use tokio::fs::File;
// use tokio_util::codec::{BytesCodec, FramedRead};



use crate::Post;
use std::{error::Error, println};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct MediumPost {
  title: String,
  content_format: String,
  content: String,
  // tags: Vec<String>,
  canonical_url: String,
  publish_status: String,
  license: String,
  notify_followers: bool
}

pub async fn syndicate(post: &Post) -> Result<(), Box<dyn Error>>
{
  println!("TODO: Syndicating to Medium");
  let _ = open::that("https://medium.com/new-story");

//   let client = reqwest::Client::new();  

//   let image_url = post.image_url.clone().unwrap_or_default();

//   let content = format!("<h1>{}</h1>
//     <img src='{}' alt='post thumbnail'/>
//     {}
//     <hr>
//     <p>Like this post?</p>
// <p>I send out a short email every day to <strong>help writers create narrative games</strong>.</p>
// <p><a href='https://onwriting.games'>https://onwriting.games</a></p>", post.title, "https://onwriting.games/icon.png", post.content_html);

//   println!("{}", content);


//   let params = MediumPost{
//     title: post.title.clone(),
//     content_format: "html".to_string(),
//     content,
//     // tags: vec![
//     //   "Game Design".to_string(), 
//     //   "Game Development".to_string(), 
//     //   "Game Writing".to_string(), 
//     //   "Storytelling".to_string(), 
//     //   "Interactive Fiction".to_string()
//     // ],
//     canonical_url: post.url.clone(),
//     publish_status: "unlisted".to_string(),
//     license: "all-rights-reserved".to_string(),
//     notify_followers: false
//   };

//   let result = client.post("https://api.medium.com/v1/publications/eab187976b2e/posts")
//       .bearer_auth(TOKEN)
//       .form(&params)
//       .send()
//       .await?;

//   println!("{:?}", result.text().await);
  
  Ok(())
}