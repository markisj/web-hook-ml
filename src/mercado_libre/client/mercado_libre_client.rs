pub mod mercado_libre_client {
    use reqwest::Client;

    use crate::mercado_libre::model::stock::stock::Stock;

    pub struct MercadoLibreClient {
        client: Client,
        base_url: String,
        accesss_token: String,
    }

    impl MercadoLibreClient {
        pub fn new(base_url: String, accesss_token: String) -> Self {
            MercadoLibreClient {
                client: Client::new(),
                base_url,
                accesss_token,
            }
        }

        // resource "/user-products/MLAU12345678/stock"
        pub async fn get_stock(&self, resource: String) -> Result<Stock, reqwest::Error> {
            let url: String = format!("{}{}", self.base_url, resource);

            let response = self
                .client
                .get(url)
                .bearer_auth(self.accesss_token.clone())
                .send()
                .await?;

            match response.status() {
                reqwest::StatusCode::OK => {
                    let stock = response.json::<Stock>().await?;

                    Ok(stock)
                }
                _ => Err(response.error_for_status().unwrap_err()),
            }
        }
    }
}
