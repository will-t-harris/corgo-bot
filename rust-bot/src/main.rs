use std::env;
use std::fs;

use serde::{Deserialize, Serialize};
use serenity::{
    framework::standard::macros::{command, group},
    framework::standard::{Args, CommandResult, StandardFramework},
    model::{channel::Message},
    prelude::*,
};

#[group]
#[commands(create_event)]
struct General;

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    day: i32,
    month: i32,
    year: i32,
}

#[command]
fn create_event(_ctx: &mut Context, _msg: &Message, mut args: Args) -> CommandResult {
    let result: Vec<String> = args.iter::<String>().map(|c| c.unwrap()).collect();

    let event = Event {
        name: result[0].parse::<String>().unwrap(),
        day: result[1].parse::<i32>().unwrap(),
        month: result[2].parse::<i32>().unwrap(),
        year: result[3].parse::<i32>().unwrap(),
    };

    let event_json_string = serde_json::to_string(&event);

    write(event_json_string.unwrap()).unwrap_or_else(|err| println!("{:?}", err));
    Ok(())
}

fn write(s: String) -> std::io::Result<()> {
    fs::write("text.json", s)?;
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

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .group(&GENERAL_GROUP),
    );

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
