// data structure which will be serialized and deserialized to and from the request body

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteInput {
    pub url: String,
}
