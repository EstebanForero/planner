-- Add migration script here

CREATE TABLE planner_user (
    user_id SERIAL PRIMARY KEY
);

-- Create the classes table
CREATE TABLE classes (
    class_name VARCHAR PRIMARY KEY,
    user_id UUID REFERENCES planner_user(user_id)
);

-- Create the schedule table
CREATE TABLE schedule (
    schedule_id VARCHAR PRIMARY KEY,
    class_name VARCHAR REFERENCES classes(class_name)
);

-- Create the block table with an INTEGER primary key
CREATE TABLE block (
    block_id INTEGER PRIMARY KEY AUTOINCREMENT,
    start_hour INT,
    finish_hour INT,
    day VARCHAR,
    schedule_id TEXT REFERENCES schedule(schedule_id)
);

