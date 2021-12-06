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

use serde::Deserialize;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
  let framework = StandardFramework::new()
    .configure(|c| c.prefix("-"))
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

#[derive(Deserialize, Debug)]
struct Task {
  id: usize,
  title: String,
  description: String,
  tbd: String,
  done: bool,
}

async fn apitest() -> Result<Vec<Task>, reqwest::Error> {

  let resp = reqwest::get("http://127.0.0.1:8000/tasks/get").await?;

  println!("Status: {}", resp.status());

  let tasks: Vec<Task> = resp.json().await?;

  Ok(tasks)
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {

  let mut tasks: Vec<Task> = vec![];

  match apitest().await {
    Ok(res) => tasks = res,
    Err(e) => println!("Error: {}", e),
  }

  for task in tasks {
    msg.channel_id.send_message(ctx, |m| m
      .embed(|e| e
        .title(task.title)
        .description(task.description))).await?;
  }

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