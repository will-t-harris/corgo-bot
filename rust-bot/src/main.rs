use std::env;

use serenity::{
    framework::standard::macros::{check, command, group},
    framework::standard::{ArgError, Args, CommandResult, StandardFramework},
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

#[group]
#[commands(create_event)]
struct General;

// Result<String,ArgError<std::convert::Infallible>>

#[command]
fn create_event(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let result: Vec<String> = args.iter::<String>().map(|c| c.unwrap()).inspect(|c| println!"{:?}", c).collect();

    println!("{:?}", result);

    Ok(())
}

struct Handler;

impl EventHandler for Handler {}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    client.with_framework(StandardFramework::new().configure(|c| c.prefix("!")).group(&GENERAL_GROUP));

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
