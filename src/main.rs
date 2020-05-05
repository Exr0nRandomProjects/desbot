// modified from [serenity-rs example](https://github.com/serenity-rs/serenity#example-bot)
use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
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

fn log(msg: &Message) {
    println!("{time}, {name}#{tag}, {userid}, {channelid}\n    {content}",
             time=msg.timestamp,
             name=msg.author.name,
             tag=msg.author.discriminator,
             userid=msg.author.id,
             channelid=msg.channel_id,
             content=msg.content
             );
}

impl EventHandler for Handler {
    fn ready(&self, ctx: Context, _data: Ready) {
        // status: https://docs.rs/serenity/0.8.6/serenity/prelude/struct.Context.html#method.set_presence
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;
        let activity = Activity::playing("with your psyche...");
        let status = OnlineStatus::DoNotDisturb; // TODO: change to Offline
        ctx.set_presence(Some(activity), status);

        println!("Bot ready.");
    }
    fn message(&self, ctx: Context, msg: Message) {
        use serenity::model::error::Error::InvalidPermissions;
        log(&msg);
        if msg.is_own(&ctx) {
            return;
        }
        if msg.is_private() {
            msg.reply(ctx, "Pong!");
        } else {
            msg.react(ctx, '🥗');
        }
    }
}

#[command]
fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP));

    // start listening for events by starting a single shard
    println!("Starting bot...");
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

