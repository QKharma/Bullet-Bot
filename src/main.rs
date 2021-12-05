use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::framework::standard::{
  StandardFramework,
  CommandResult,
  macros::{
    command,
    group
  }
};

use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
  let framework = StandardFramework::new()
    .configure(|c| c.prefix("~"))
    .group(&GENERAL_GROUP);

  let token = env::var("BULLET_BOT_TOKEN").expect("token");
  let mut client = Client::builder(token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Error creating client");

  if let Err(why) = client.start().await {
    println!("An error occured while running the clinet: {:?}", why);

  }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {

  let mut reply = CreateMessage::default();
  reply
    .embed(|e| e
      .colour(0x00ff00)
      .title("bre")
      .description("beans")
    );

  msg.channel_id.send_message(ctx, |_| { &mut reply }).await?;

  Ok(())
}