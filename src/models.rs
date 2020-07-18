use chrono::prelude::*;
use crate::schema::*;

#[derive(Debug, Queryable, Identifiable, AsChangeset, PartialEq, Serialize)]
#[table_name = "threads"]
#[primary_key(thread_id)]
pub struct Thread {
    pub thread_id: i32,
    pub thread_title: String,
    pub post_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Thread {
    pub fn increment_post_count(&mut self) -> i32 {
        self.post_count += 1;
        self.updated_at = Utc::now();
        self.post_count
    }
}

#[derive(Debug, Insertable, PartialEq, Serialize)]
#[table_name = "threads"]
pub struct NewThread {
    pub thread_title: String,
    pub post_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl NewThread {
    pub fn new(thread_title: &str) -> NewThread {
        let now = Utc::now();
        NewThread {
            thread_title: thread_title.into(),
            post_count: 1,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Queryable, Insertable, Identifiable, AsChangeset, PartialEq, Serialize)]
#[table_name = "posts"]
#[primary_key(thread_id)]
pub struct Post {
    pub thread_id: i32,
    pub seq_number: i32,
    pub poster_name: String,
    pub post_body: String,
    pub posted_at: DateTime<Utc>,
    pub is_deleted: bool,
}

impl Post {
    pub fn new(thread_id: i32, seq_number: i32, poster_name: &str, post_body: &str) -> Post {
        Post {
            thread_id,
            seq_number,
            poster_name: poster_name.into(),
            post_body: post_body.into(),
            posted_at: Utc::now(),
            is_deleted: false,
        }
    }
}
