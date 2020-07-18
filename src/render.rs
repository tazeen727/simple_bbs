use chrono::prelude::*;
use chrono::Duration;
use rocket::fairing::Fairing;
use rocket::response::Responder;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

use crate::basic_types::*;
use crate::models::*;

pub fn fairing() -> impl Fairing {
    Template::custom(|_| {
    })
}

#[derive(Responder)]
pub enum Success {
    Template(Template),
    Redirect(Redirect),
}

#[derive(Debug, Serialize)]
pub struct ThreadCtx {
    pub id: i32,
    pub title: String,
    pub responses: i32,
    pub created_at: String,
    pub updated_at: String,
}

impl ThreadCtx {
    pub fn from(value: Thread, now: DateTime<Utc>) -> Self {
        ThreadCtx {
            id: value.thread_id,
            title: value.thread_title,
            responses: value.post_count,
            created_at: format_date(value.created_at, now),
            updated_at: format_date(value.updated_at, now),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ThreadsPageCtx {
    pub threads: Vec<ThreadCtx>,
    pub pagination: Option<Vec<PaginationItem>>,
    pub keyword: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PaginationItem {
    /// <
    Prev { number: Option<i32> },
    /// Number Link
    Number { number: i32, active: bool },
    /// >
    Next { number: Option<i32> },
    /// ...
    Dots,
}

impl PaginationItem {
    pub fn list(thread_count: i32, current_page: PageNumber) -> Vec<PaginationItem> {
        use std::cmp;
        use crate::globals;
        use PaginationItem::*;

        let first = 1;
        let last =
            (thread_count / globals::THREADS_PER_PAGE) +
            if thread_count % globals::THREADS_PER_PAGE == 0 { 0 } else { 1 };
        let current = current_page.number();

        // 現在のページの前後に表示する数字の範囲を計算
        let (start, end) =
            if first <= current && current <= first + 2 {
                (first, cmp::min(last, first + 4))
            } else if first + 2 < current && current < last - 2 {
                (current - 2, current + 2)
            } else if last - 2 <= current && current <= last {
                (cmp::max(last - 4, first), last)
            } else {
                unreachable!()
            };
        
        // 計算結果に従いPaginationItemをVecに詰める
        let mut items = Vec::new();
        items.push(Prev{
            number: if current == first { None } else { Some(current - 1) } 
        });
        if first < start {
            items.push(Number {
                number: first,
                active: current == first,
            });
        }
        if first + 1 < start {
            items.push(Dots);
        }
        for n in start..=end {
            items.push(Number {
                number: n,
                active: current == n,
            });
        }
        if end < last - 1 {
            items.push(Dots);
        }
        if end < last {
            items.push(Number {
                number: last,
                active: current == last,
            });
        }
        items.push(Next{
            number: if current == last { None } else { Some(current + 1) } 
        });
        items
    }
}

fn format_date(dt: DateTime<Utc>, now: DateTime<Utc>) -> String {
    let dr: Duration = now - dt;
    if dr.num_seconds() < 1 {
        "Now".to_string()
    } else if dr.num_seconds() < 60 {
        let unit = if dr.num_seconds() == 1 { "second" } else { "seconds" };
        format!("{} {} ago", dr.num_seconds(), unit)
    } else if dr.num_minutes() < 60 {
        let unit = if dr.num_minutes() == 1 { "minute" } else { "minutes" };
        format!("{} {} ago", dr.num_minutes(), unit)
    } else if dr.num_hours() < 24 {
        let unit = if dr.num_hours() == 1 { "hour" } else { "hours" };
        format!("{} {} ago", dr.num_hours(), unit)
    } else {
        let dt = dt.with_timezone(&Local);
        dt.format("%Y/%m/%d %H:%M:%S").to_string()
    }
}
