-- Add migration script here

-- Create planner_user table
CREATE TABLE IF NOT EXISTS planner_user (
    user_id TEXT PRIMARY KEY
);

-- Create classes table
CREATE TABLE IF NOT EXISTS classes (
    class_name TEXT,
    user_id TEXT,
    PRIMARY KEY (class_name, user_id),
    FOREIGN KEY (user_id) REFERENCES planner_user(user_id)
);

-- Create schedule table
CREATE TABLE IF NOT EXISTS schedule (
    schedule_id TEXT PRIMARY KEY,
    class_name TEXT,
    FOREIGN KEY (class_name) REFERENCES classes(class_name)
);

-- Create block table
CREATE TABLE IF NOT EXISTS block (
    block_id TEXT PRIMARY KEY,
    start_hour INTEGER,
    finish_hour INTEGER,
    day TEXT,
    schedule_id TEXT,
    FOREIGN KEY (schedule_id) REFERENCES schedule(schedule_id)
);
