use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateWebsiteOutput {
    pub id: String
}

#[derive(Serialize, Deserialize)]
pub struct GetWebsite {
    pub id: String
}