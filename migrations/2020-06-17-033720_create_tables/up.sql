CREATE TABLE threads (
    thread_id SERIAL PRIMARY KEY,
    thread_title VARCHAR(200) NOT NULL,
    post_count INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE posts (
    thread_id INTEGER,
    seq_number INTEGER,
    poster_name VARCHAR(200) NOT NULL,
    post_body VARCHAR(4000) NOT NULL,
    posted_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    PRIMARY KEY (thread_id, seq_number),
    FOREIGN KEY (thread_id) REFERENCES threads
);
