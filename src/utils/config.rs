use std::str::FromStr;
use envconfig::Envconfig;


#[derive(Envconfig)]
pub struct Config {
  #[envconfig(from = "PORT")]
  pub port: u64,
  #[envconfig(from = "CORS_ORIGIN")]
  pub cors_config: CorsConfig,
  #[envconfig(from = "FIREBASE_API_KEY")]
  pub firebase_api_key: Option<String>,
}

pub struct CorsConfig {
  pub origin: Vec<String>,
}

impl FromStr for CorsConfig {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(Self {
      origin: s.split(",").map(|val| val.to_owned()).collect(),
    })
  }
}
