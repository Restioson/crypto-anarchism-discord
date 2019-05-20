use serenity::client::Client;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::macros::{group, command};
use serenity::framework::standard::{CommandResult, StandardFramework, Args};
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
        .configure(|c|
            c.prefix("a!"))
            .group(&COMMANDS_GROUP)
    );

    println!("Starting bot...");

    if let Err(e) = client.start() {
        eprintln!("Error while running client: {:?}", e);
    }
}
#[command]
#[description = "Start a vote"]
#[usage("#vote <vote text>")]
fn ping(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    if let Err(e) = message.reply(&context, "Pong!") {
        eprintln!("Error returning ping: {:?}", e);
    }

    Ok(())
}

#[command]
#[description = "Start a vote"]
#[usage("#vote <vote text>")]
fn vote(context: &mut Context, message: &Message, _: Args) -> CommandResult {
    let res = catch! {
        let vote_content = message.content.split(' ').skip(1).collect::<Vec<&str>>().join(" ");
        let old_message = message;
        let message = message.channel_id.say(&context, format!("**Vote:** {}", vote_content))?;
        message.react(&context, '👍')?;
        message.react(&context, '👎')?;
        old_message.delete(&context)
    };

    if let Err(e) = res {
        eprintln!("Error starting vote: {:?}", e);
    }

    Ok(())
}

group!({
  name: "Commands",
  options: {},
  commands: [ping, vote],
});
