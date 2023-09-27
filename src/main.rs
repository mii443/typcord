mod event_handler;

use std::env;

use event_handler::Handler;
use serenity::{framework::StandardFramework, http::Http, prelude::GatewayIntents, Client};

#[tokio::main]
async fn main() -> Result<(), ()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let token = "";

    let http = Http::new(token);

    let bot_id = match http.get_current_user().await {
        Ok(bot_id) => bot_id.id,
        Err(why) => panic!("Could not access the bot id: {why:?}"),
    };

    let framework = StandardFramework::new();
    framework.configure(|c| {
        c.with_whitespace(true)
            .on_mention(Some(bot_id))
            .prefix("t.")
    });

    let mut client = Client::builder(
        &token,
        GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES,
    )
    .framework(framework)
    .event_handler(Handler)
    .await
    .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(())
}
