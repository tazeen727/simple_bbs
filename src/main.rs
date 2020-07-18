#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use chrono::prelude::*;
use rocket_contrib::templates::Template;
use rocket::response::Redirect;

use simple_bbs::{rocket_app, DbConn};
use simple_bbs::basic_types::*;
use simple_bbs::functions;
// use simple_bbs::forms::*;
use simple_bbs::models::*;
use simple_bbs::render::*;
use simple_bbs::globals;

type BoxError = Box<dyn std::error::Error>;

#[get("/")]
fn index(conn: DbConn) -> Result<Success, BoxError> {
    show_threads(conn, None, None)
}

#[get("/threads?<keyword>&<page>")]
fn show_threads(conn: DbConn, keyword :Option<NonEmptyString>, page: Option<Result<PageNumber, ()>>) -> Result<Success, BoxError> {
    let page = page.unwrap_or(Ok(PageNumber::default()));
    let keyword = keyword.map(|s| s.into());

    if let Err(_) = page {
        let query = keyword.map(|s| format!("?keyword={}", s)).unwrap_or(String::default());
        let uri = format!("/threads{}", query);
        return Ok(Success::Redirect(Redirect::to(uri)));
    }

    let page = page.unwrap();
    let (thread_count, threads) = functions::get_threads(&conn, page, &keyword)?;

    let now = Utc::now();
    let threads: Vec<_> = threads.into_iter().map(|t| ThreadCtx::from(t, now)).collect();
    let pagination = if thread_count > globals::THREADS_PER_PAGE {
        Some(PaginationItem::list(thread_count, page))
    } else {
        None
    };
    let ctx = ThreadsPageCtx {
        threads,
        pagination,
        keyword: keyword.map(|k| k.to_string())
    };

    Ok(Success::Template(Template::render("threads", &ctx)))
}

#[get("/thread/<thread_id>")]
fn show_posts(conn: DbConn, thread_id: i32) -> String {
    format!("thread_id is {}", thread_id)
}

#[get("/test")]
fn test_page(conn: DbConn) -> Template {
    #[derive(serde::Serialize)]
    struct Context {
        thread1: Thread,
        thread2: Thread,
        thread3: Thread,
        current_timestamp: DateTime<Utc>,
    }

    let mut thread1 = functions::get_thread(&conn, 59).unwrap();
    let mut thread2 = functions::get_thread(&conn, 37).unwrap();
    let mut thread3 = functions::get_thread(&conn, 50).unwrap();

    thread1.updated_at = Utc.datetime_from_str("2020-07-01T08:53:08.340Z", "%+").unwrap();
    thread2.updated_at = Utc.datetime_from_str("2020-06-30T22:53:08.123Z", "%+").unwrap();
    thread3.updated_at = Utc.datetime_from_str("2020-07-01T21:00:08.789Z", "%+").unwrap();
    let current_timestamp = Utc.datetime_from_str("2020-07-01T21:15:19.789Z", "%+").unwrap();

    let context = Context {
        thread1,
        thread2,
        thread3,
        current_timestamp
    };
    Template::render("test", &context)
}

#[post("/thread")]
fn create_new_thread(conn: DbConn) -> String {
    "new thread was created.".to_string()
}

#[post("/thread/<thread_id>")]
fn post_comment(conn: DbConn, thread_id: i32) -> String {
    format!("your comment was posted on thread[{}]", thread_id)
}

fn main() {
    rocket_app().mount("/", routes![
        index,
        show_threads,
        show_posts,
        create_new_thread,
        post_comment,
        test_page,
    ]).launch();
}
