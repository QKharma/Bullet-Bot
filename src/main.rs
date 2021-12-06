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

use chrono::{NaiveDateTime};
use chrono::format::ParseError;

mod models;
use models::task::Task;

#[group]
#[commands(todo)]
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

async fn apitest() -> Result<Vec<Task>, reqwest::Error> {

  let resp = reqwest::get("http://127.0.0.1:8000/tasks/get").await?;

  println!("Status: {}", resp.status());

  let tasks: Vec<Task> = resp.json().await?;

  Ok(tasks)
}

#[command]
async fn todo(ctx: &Context, msg: &Message) -> CommandResult {

  let mut tasks: Vec<Task> = vec![];

  match apitest().await {
    Ok(res) => tasks = res,
    Err(e) => println!("Error: {}", e),
  }

  let mut fields: Vec<(String,String,bool)> = vec![];
  for task in tasks {
    let tbd = NaiveDateTime::parse_from_str(&task.tbd,"%Y-%m-%dT%H:%M:%S").unwrap();
    let tbd_str = tbd.format("%d.%m.%Y").to_string();
    fields.push((task.title,task.description,true));
    fields.push((String::from("⠀"),tbd_str,true));
    fields.push((String::from("⠀"),String::from("⠀"),false));
  }

  let mut reply = CreateMessage::default();
  reply
    .embed(|e| e
      .colour(0x00ff00)
      .title("To-Do")
      .description("mach mol du bob")
      .fields(fields)
    );

  msg.channel_id.send_message(ctx, |_| { &mut reply }).await?;

  Ok(())
}