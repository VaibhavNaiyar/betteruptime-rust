use serde::{Deserialise,Serialize};

#[derive(Serialize , Deserialize)]
pub struct CreateWebsiteInput {
    pub url: String
}