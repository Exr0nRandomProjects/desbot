// modified from [serenity-rs example](https://github.com/serenity-rs/serenity#example-bot)
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{EventHandler, Context};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(ping)]
struct General;

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn log(msg: &Message) {
    println!("{}: Message from {}#{}\n    {}", msg.timestamp, msg.author.name, msg.author.discriminator, msg.content);
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    log(&msg);
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

