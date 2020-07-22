#![allow(unused_imports)]
use parking_lot::Mutex;

// DBにアクセスするtestは直列に動作するよう、ロックを使用
static DB_LOCK: Mutex<()> = Mutex::new(());

fn with_conn<T, E, F>(f: F) -> T
where F: FnOnce(&super::DbConn) -> Result<T, E>,
      E: std::error::Error,
{
    use diesel::connection::Connection;
    let _lock = DB_LOCK.lock();
    let rocket = super::rocket_app();
    let db = super::DbConn::get_one(&rocket);
    let conn: super::DbConn = db.expect("failed to get database connection for testing");
    conn.begin_test_transaction().expect("failed to begin transaction");
    f(&conn).expect("closure failed")
}

use std::convert::TryFrom;
use crate::basic_types::*;
use crate::models::*;
use crate::forms::*;
use crate::dbaccess::*;

#[test]
fn test_create_new_thread() {
    with_conn(|conn| {
        let thread_form = ThreadForm {
            thread_title: "チョコチップスナック Part1",
            poster_name: "名無しの顔も三度まで",
            post_body: "安くてお腹が膨れる、けどそんなにおいしくはない。<br>\
                        そんな愛すべきおやつ「チョコチップスナック」について語ろう！",
        };
        create_new_thread(conn, &thread_form)
    });
}

#[test]
fn test_create_new_post() {
    let (posts, threads) = with_conn::<_, crate::error::Error,_>(|conn| {
        let thread_id = 60;
        let post_form = PostForm {
            thread_id,
            poster_name: "アーモンド斉藤",
            post_body: "アーモンド入りのやつはもっとおいしいよ",
        };
        post_to_thread(conn, &post_form)?;

        let (_, threads) = get_threads(conn, PageNumber::default(), &None)?;
        let posts = get_posts(conn, thread_id)?;
        Ok((posts, threads))
    });
    
    println!("{:?}", threads[0]);
    assert_eq!(threads[0].post_count, 579);
    assert_eq!(posts.len(), 579);
    assert_eq!(posts.last().unwrap().post_body, "アーモンド入りのやつはもっとおいしいよ");
}

#[test]
fn test_get_threads() {
    let (_, threads) = with_conn(|conn| {
        get_threads(conn, PageNumber::try_from(2).unwrap(), &None)
    });
    assert_eq!(threads.len(), 20);
    assert_eq!(threads[0].thread_id, 56);
    assert!(threads[0].thread_title.contains("Chromebook Part42"));
}

#[test]
fn test_get_posts() {
    let posts = with_conn(|conn| {
        get_posts(conn, 60)
    });
    assert_eq!(posts.len(), 578);
    assert!(posts[577].post_body.contains("参考にします"));
}
