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

use chrono::{offset::TimeZone, DateTime, Local, NaiveDateTime};

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
  let mut task_string_today = String::from("");
  let mut task_string_upcoming = String::from("");
  let mut max_task_length = 0;
  let mut previous_tbd_str = String::from("");

  for task in &tasks {
    if task.title.len() > max_task_length {
      max_task_length = task.title.len();
    }
  }

  println!("{}",max_task_length);

  task_string_upcoming.push_str("```");
  
  let mut is_first = true;
  for task in tasks {

    let now: DateTime<Local> = Local::now();
    let tbd_naive = NaiveDateTime::parse_from_str(&task.tbd,"%Y-%m-%dT%H:%M:%S").unwrap();
    let tbd: DateTime<Local> = Local.from_local_datetime(&tbd_naive).unwrap();
    let tbd_str = tbd.format("%d.%m").to_string();

    let difference = now.signed_duration_since(tbd);

    if difference.num_days() < 1 {
      task_string_today.push_str("• ");
      task_string_today.push_str(&task.title);
      task_string_today.push_str("\n");

    } else {
      let inp: &str;
      let mut spaces = max_task_length - task.title.len() + 5;
      if previous_tbd_str != tbd_str {
        if ! is_first {
          task_string_upcoming = String::from(task_string_upcoming.split_at(task_string_upcoming.len()-2).0);
          task_string_upcoming.push_str("┘");
          task_string_upcoming.push_str("\n\n");
        }
        inp = &tbd_str;
      } else {
        spaces += 4;
        inp = "|";
      }
      
      task_string_upcoming.push_str("• ");
      task_string_upcoming.push_str(&task.title);
      for _ in 1..spaces {
        task_string_upcoming.push_str(" ");
      }
      task_string_upcoming.push_str(inp);
      task_string_upcoming.push_str("\n");
    }

    previous_tbd_str = tbd_str;
    is_first = false

  }

  if task_string_today == "" {
    task_string_today.push_str("-")
  }

  if task_string_upcoming == "" {
    task_string_upcoming.push_str("-")
  }

  task_string_upcoming.push_str("```");

  fields.push((String::from("Today"),task_string_today,true));
  fields.push((String::from("Upcoming"),task_string_upcoming,false));

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