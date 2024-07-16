mod mercado_libre;

use dotenv::dotenv;
use std::env;

use mercado_libre::{
    client::mercado_libre_client::mercado_libre_client::MercadoLibreClient,
    model::notification::notification_mercado_libre::Notification,
};
use rocket::{response::status, serde::json::Json};

#[macro_use]
extern crate rocket;

#[post("/", data = "<notification>", format = "json")]
async fn received(notification: Json<Notification>) -> status::Accepted<String> {
    let noti = notification.into_inner();

    match noti.topic.as_str() {
        "stock-location" => {
            let access_token = env::var("ACCESS_TOKEN").expect("ACCESS_TOKEN must be set");
            let base_url = env::var("BASE_URL").expect("BASE_URL must be set");

            let client = MercadoLibreClient::new(base_url, access_token);

            let stock = client.get_stock(noti.resource).await;
            print!("{:?}", stock);
        }
        _ => panic!("unknown topic"),
    }

    status::Accepted("".to_string())
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![received])
}
