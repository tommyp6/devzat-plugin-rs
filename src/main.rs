use devzat_rs::Client;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance_host = match env::var("PLUGIN_HOST") {
        Ok(host) => host,
        Err(_) => panic!("Missing PLUGIN_HOST"),
    };

    let auth_token = match env::var("PLUGIN_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("Missing PLUGIN_TOKEN"),
    };

    let mut client = Client::new(instance_host, auth_token).await?;

    client
        .send_message(
            String::from("#main"),
            Some(String::from("Rusty")),
            String::from("Hello World from the devzat-rs API."),
            None,
        )
        .await?;

    Ok(())
}
