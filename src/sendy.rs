
use crate::Error;
use crate::credential::Credentials;

use crate::Post;

pub async fn syndicate(post: &Post, credentials: Credentials) -> Result<(), Box<dyn Error>>
{
  let html_header = include_str!("header.html");
  let html_header = html_header.replace("{{title}}", &post.title);

  let html_signature = include_str!("signature.html");
  let plain_signature = include_str!("signature.txt");


  let html_signature = html_signature.replace("{{url}}", &post.url);
  let plain_signature = plain_signature.replace("{{url}}", &post.url);


  let html_text = html_header + &post.content_html.clone() + &html_signature;
  let plain_text = post.content_text.clone() + &plain_signature;

  let client = reqwest::Client::new();
  let params = [
    ("api_key", credentials.secret),
    ("from_name", "Fran Tufro | On Writing Games".to_string()),
    ("from_email", "fran@onwriting.games".to_string()),
    ("reply_to", "fran@onwriting.games".to_string()),
    ("send_campaign", "1".to_string()),
    ("track_opens", "2".to_string()),
    ("track_clicks", "0".to_string()),
    ("list_ids", "YHa47bHIPC3QUNDztxDaWw".to_string()),
    ("title", post.title.clone()),
    ("subject", post.title.clone()),
    ("plain_text", plain_text),
    ("html_text", html_text)
    
  ];
  let result = client.post("https://mail.hiddenpeople.club/api/campaigns/create.php")
      .form(&params)
      .send()
      .await?
      .text()
      .await?;

  println!("{}", result);

  Ok(())
}