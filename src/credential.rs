use serde::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Debug, Deserialize)]
pub struct Credentials {
  pub user: String,
  pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct CredentialStore {
  pub dev_to: Credentials,
  pub hacker_news: Credentials,
  pub medium: Credentials,
  pub twitter: Credentials,
  pub sendy: Credentials
}

impl CredentialStore {
  pub fn load() -> CredentialStore {
    let filename = "credentials.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let store : CredentialStore = match toml::from_str(&contents) {
        // If successful, return data as `Data` struct.
        // `d` is a local variable.
        Ok(d) => d,
        // Handle the `error` case.
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Unable to load data from `{}`", filename);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    store
  }
}