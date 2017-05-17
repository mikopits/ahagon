use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use github_rs::github::Client as GhClient;

pub struct Client;

impl Client {
    pub fn new<P>(config_path: P) -> ::Result<(GhClient, Config)>
        where P: AsRef<Path>,
    {
        let config = Config::new(config_path)?;
        let gh_client = GhClient::new(config.clone().github.access_token)?;
        Ok((gh_client, config))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    name: String,
    github: GithubConfig,
    web: WebConfig,
    db: DbConfig,
    repos: Vec<RepoConfig>,
}

impl Config {
    pub fn url(&self) -> String {
        self.clone().web.host + ":" + &self.web.port
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GithubConfig {
    access_token: String,
    app_client_id: String,
    app_client_secret: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WebConfig {
    host: String,
    port: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DbConfig {
    file: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepoConfig {
    owner: String,
    name: String,
    reviewers: Vec<String>,
    try_users: Vec<String>,
    secret: String,
    travis_token: String,
}

impl Config {
    pub fn new<P>(config_path: P) -> ::Result<Config>
        where P: AsRef<Path>,
    {
        let file = File::open(config_path)?;
        let mut reader = BufReader::new(file);
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;

        let config: Config = ::toml::from_str(&buf)?;
        Ok(config)
    }
}
