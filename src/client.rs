use std::env;

pub mod devzat {
    tonic::include_proto!("plugin");
}

use devzat::{plugin_client::PluginClient, Message};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let instance_host = match env::var("PLUGIN_HOST") {
        Ok(host) => host,
        Err(_) => panic!("Missing PLUGIN_HOST")
    };

    let channel = Channel::from_shared(instance_host)?.connect().await?;

    let auth_token = match env::var("PLUGIN_TOKEN") {
        Ok(token) => token,
        Err(_) => panic!("Missing PLUGIN_TOKEN")
    };

    let token: MetadataValue<_> = format!("Bearer {}", auth_token).parse()?;

    let mut client = PluginClient::with_interceptor(channel, move |mut req: Request<()>| {
        req.metadata_mut().insert("authorization", token.clone());
        Ok(req)
    });

    let request = tonic::Request::new(Message {
        room: "#main".into(),
        from: Some("Test Plugin".into()),
        msg: "Hello World from Rust!".into(),
        ephemeral_to: None,
    });

    let response = client.send_message(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}