use crate::credential::Credentials;
use std::error::Error;

use serde::Serialize;

use crate::Post;

#[derive(Debug, Serialize, Default)]
struct Login {
  acct: String,
  pw: String
}


#[derive(Debug, Serialize, Default)]
struct Submission {
  fnid: String,
  fnop: String,
  title: String,
  url: String
}

pub async fn syndicate(post: &Post, credentials: Credentials) -> Result<(), Box<dyn Error>>
{
  println!(" - Hacker News");

  let client = reqwest::Client::builder()
    .cookie_store(true)
    .build()?;

  println!("{:?}", credentials);

  let result = client.post("https://news.ycombinator.com/submit")
    .form(&Login {
      acct: credentials.user,
      pw: credentials.secret
    })
    .send()
    .await?;

  let result = client.get("https://news.ycombinator.com/submit")
    .send()
    .await?;

  let mut params = Submission {
    fnid: "".to_string(),
    fnop: "submit-page".to_string(),
    title: post.title.clone(),
    url: post.url.clone()
  };
  
  let fnids: Vec<String> = result.text().await?
    .lines()
    .filter(|s| s.contains("fnid"))
    .map(|x| x.split("name=\"fnid\" value=\"").nth(1).unwrap().to_string())
    .map(|x| x.split("\"><input type=\"hidden").nth(0).unwrap().to_string())
    .collect();

  if let Some(fnid) = fnids.get(0) {
    params.fnid = fnid.to_string();

    let _result = client.post("https://news.ycombinator.com/r")
      .form(&params)
      .send()
      .await?;

    let _ = open::that("https://news.ycombinator.com/submitted?id=frantufro");
  }
  else 
  {
    println!("  There Error syndicating to Hacker News.")
  }

  Ok(())
}