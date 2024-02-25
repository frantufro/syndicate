use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use atom_syndication::Feed;
use mdka::from_html;

#[derive(Debug, Default)]
pub struct Post {
  pub url: String,
  pub title: String,
  pub content_html: String,
  pub content_text: String,
  pub content_markdown: String,
  pub image_url: Option<String>,
  pub image_path: Option<String>
}

impl Post {
  pub async fn from_rss() -> Result<Post, Box<dyn Error>> {
    println!("Getting last post from rss.");
    let rss = get_rss().await.unwrap();
    let entry = rss.entries.first().unwrap();

    let url = entry.id.clone();
    let title = entry.title.value.clone();
    let content_html = entry.content.clone().unwrap().value.unwrap();
    // let content_text = html2text::from_read(&content_html[..], 2000);
    let content_markdown = from_html(&content_html);
    
    println!("Last post: {}", title);

    println!("Is this correct? You have 5 seconds to cancel...");

    // sleep(Duration::from_secs(5));

    let mut result = Post {
      url,
      title,
      content_html,
      // content_text,
      content_markdown,
      ..Default::default()
    };


    println!("Getting the post image...");

    let body = reqwest::get(&result.url)
      .await?
      .text()
      .await?;
    
    let images: Vec<String> = body
      .lines()
      .filter(|s| s.contains("meta property=image"))
      .map(|x| x.split("\"").nth(1).unwrap().to_string())
      .collect();
        
    if let Some(url) = images.get(0) {
      result.image_url = Some(url.to_string());

      println!("Downloading image...");

      let bytes = reqwest::get(url)
        .await?
        .bytes()
        .await?;    

      let filename = "/tmp/post_image.png";
      let mut file = File::create(&filename)?;
      file.write_all(&bytes)?;
      
      result.image_path = Some(filename.to_string());
    }

    Ok(result)
  }
}

async fn get_rss() -> Result<Feed, Box<dyn Error>> {
  let content = reqwest::get("https://onwriting.games/atom.xml")
    .await?
    .bytes()
    .await?;
  let feed = Feed::read_from(&content[..]).unwrap();
  Ok(feed)
}

