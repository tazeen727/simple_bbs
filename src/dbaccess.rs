use crate::Result;
use crate::forms::*;
use crate::models::*;
use crate::schema;
use crate::DbConn;
use diesel::prelude::*;

pub fn create_new_thread(conn: &DbConn, thread_form: &ThreadForm) -> Result<(Thread, Post)> {
    use schema::posts::dsl::*;
    use schema::threads::dsl::*;

    let new_thread = NewThread::new(thread_form.thread_title);
    let t: Thread = diesel::insert_into(threads)
        .values(&new_thread)
        .get_result(&**conn)?;

    let new_post = Post::new(
        t.thread_id,
        1,
        thread_form.poster_name,
        thread_form.post_body,
    );
    let p: Post = diesel::insert_into(posts)
        .values(&new_post)
        .get_result(&**conn)?;

    Ok((t, p))
}

pub fn post_to_thread(conn: &DbConn, post_form: &PostForm) -> Result<Post> {
    use schema::posts::dsl::*;
    use schema::threads::dsl::*;

    let mut t: Thread = threads.find(post_form.thread_id)
        .for_update()
        .get_result(&**conn)?;
    t.increment_post_count();
    let t: Thread = t.save_changes(&**conn)?;

    let p = Post::new(
        t.thread_id,
        t.post_count,
        post_form.poster_name,
        post_form.post_body,
    );
    let p: Post = diesel::insert_into(posts).values(&p).get_result(&**conn)?;
    Ok(p)
}

pub fn get_threads(conn: &DbConn, keyword: &Option<&str>) -> Result<Vec<Thread>> {
    use schema::threads::dsl::*;
    use crate::globals;

    let mut query = threads.into_boxed();

    let keyword = keyword
        .map(|k| k.replace("%", r"\%").replace("_", r"\_"))
        .map(|k| format!("%{}%", k));
    if let Some(k) = keyword {
        query = query.filter(thread_title.like(k));
    }

    let results = query
        .order(updated_at.desc())
        .limit((globals::THREADS_PER_PAGE * globals::MAX_PAGE_NUMBER) as i64)
        .load(&**conn)?;

    Ok(results)
}

pub fn get_thread(conn: &DbConn, target_thread_id: i32) -> Result<Thread> {
    use schema::threads::dsl::*;

    let thread: Thread = threads
        .find(target_thread_id)
        .get_result(&**conn)?;
    Ok(thread)
}

pub fn get_posts(conn: &DbConn, target_thread_id: i32) -> Result<Vec<Post>> {
    use schema::posts::dsl::*;

    let ps: Vec<Post> = posts
        .filter(thread_id.eq(target_thread_id))
        .order(seq_number)
        .load(&**conn)?;
    Ok(ps)
}
