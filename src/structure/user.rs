use crate::structure::enums::{direction::Direction, group_list::GroupList};
use std::time::SystemTime;

pub struct User {
    pub user_id: i64,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub group: GroupList,
    pub direction: Direction,
    pub discord_id: i64,
    pub created_at: SystemTime,
}
