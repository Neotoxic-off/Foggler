use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Servers {
    #[serde(rename = "servers")]
    pub servers: Vec<Server>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub name: String,
    pub url: String,
}
