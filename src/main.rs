use serenity::command;
use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use std::env;

macro_rules! catch {
    ($($tt:tt)*) => {
        (|| { $($tt)* })()
    }
}

struct Handler;

impl EventHandler for Handler {}

fn main() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("a!"))
        .cmd("ping", ping)
        .cmd("vote", vote));

    println!("Starting bot...");

    if let Err(e) = client.start() {
        eprintln!("Error while running client: {:?}", e);
    }
}

command!(ping(_context, message) {
    if let Err(e) = message.reply("Pong!") {
        eprintln!("Error returning ping: {:?}", e);
    }
});

command!(vote(_context, message) {
    let res = catch! {
        let vote_content = message.content.split(' ').skip(1).collect::<Vec<&str>>().join(" ");
        let message = message.channel_id.say(format!("**Vote:** {}", vote_content))?;
        message.react( '👍')?;
        message.react('👎')
    };

    if let Err(e) = res {
        eprintln!("Error starting vote: {:?}", e);
    }
});
