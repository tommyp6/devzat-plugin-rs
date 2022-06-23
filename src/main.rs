use devzat_rs::{Client, Listener};
use std::env;
use tokio::try_join;

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

    let client = Client::new(instance_host, auth_token).await?;

    client
        .send_message(
            String::from("#main"),
            Some(String::from("Rusty")),
            String::from("Hello World from the devzat-rs API."),
            None,
        )
        .await?;

    let listener = Listener {
        middleware: None,
        once: None,
        regex: None,
    };

    let fut = client.register_listener(listener, |event| async move {
        eprintln!(
            "room={}, from={}, msg={}",
            event.room, event.from, event.msg
        );

        None
    });

    let fut2 = client.register_cmd("greet", "Greet someone.", "<name>", |event| async move {
        format!("Hello {}!", event.args)
    });

    let fut3 = client.register_cmd("bye", "Say bye to someone.", "<name>", |event| async move {
        format!("Bye {}!", event.args)
    });

    let _ = try_join!(fut, fut2, fut3);

    Ok(())
}
