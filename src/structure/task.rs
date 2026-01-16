use std::time::SystemTime;

use sqlx::types::time::Date;

pub struct Task {
  pub task_id: i64,
  pub task_name: String,
  pub task_description: String,
  pub task_deadline: Date,
  pub task_reward: f64,
  pub created_at: SystemTime,
  pub updated_at: SystemTime,
}