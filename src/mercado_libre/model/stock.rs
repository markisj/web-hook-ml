pub mod stock {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Location {
        #[serde(rename = "type")]
        pub type_field: String,

        pub quantity: i64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Stock {
        pub locations: Vec<Location>,
        pub id: String,
        #[serde(rename = "user_id")]
        pub user_id: i64,
    }
}
