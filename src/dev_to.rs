use crate::dev_to::header::*;
use std::{error::Error, default, println};

use serde::Serialize;

use reqwest::header::{self, ACCEPT};

use crate::Post;

#[derive(Debug, Serialize, Default)]
struct Article {
  title: String,
  body_markdown: String,
  published: bool,
  series: String,
  main_image: String,
  canonical_url: String,
  description: String,
  tags: String,
  organization_id: u32  
}

#[derive(Debug, Serialize, Default)]
struct DevToPost {
  article: Article
}

pub async fn syndicate(post: &Post) -> Result<(), Box<dyn Error>>
{
  println!("TODO: Syndicating to DEV.to");
  let _ = open::that("https://dev.to/dashboard");
  
//   let client = reqwest::Client::new();  

//   let params = DevToPost {
//     article: Article {
//       title: post.title.clone(),
//       published: false,
//       main_image: post.image_url.clone().unwrap_or_default(),
//       ..Default::default()      
//     }
//   };

//   let result = client.post("https://dev.to/api/articles")
//       .header("api-key", "")
//       .header(CONTENT_TYPE, "application/json")
//       .header(ACCEPT, "application/vnd.forem.api-v1+json")
//       .json(&params)
//       .send()
//       .await?;

//   println!("{:?}", result);
//   println!("{}", result.text().await?);
  Ok(())
}