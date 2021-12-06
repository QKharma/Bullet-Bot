use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Task {
  pub id: usize,
  pub title: String,
  pub description: String,
  pub tbd: String,
  pub done: bool,
}