pub mod notification_mercado_libre {
    use chrono::{DateTime, Utc};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Notification {
        pub _id: String,
        pub resource: String,
        pub user_id: u64,
        pub topic: String,

        // #[serde(rename = "application_id")]
        pub application_id: u64,
        pub attempts: u16,
        pub sent: DateTime<Utc>,
        pub received: DateTime<Utc>,
    }
}
