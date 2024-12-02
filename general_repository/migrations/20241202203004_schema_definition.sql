-- Create the planner_user table
CREATE TABLE planner_user (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT
);

-- Create the classes table with an INTEGER primary key
CREATE TABLE classes (
    class_id INTEGER PRIMARY KEY AUTOINCREMENT,
    class_name VARCHAR NOT NULL,
    user_id INTEGER REFERENCES planner_user(user_id)
);

-- Create the schedule table with an INTEGER primary key
CREATE TABLE schedule (
    schedule_id INTEGER PRIMARY KEY AUTOINCREMENT,
    class_id INTEGER REFERENCES classes(class_id),
    schedule_name VARCHAR NOT NULL
);

-- Create the block table with an INTEGER primary key
CREATE TABLE block (
    block_id INTEGER PRIMARY KEY AUTOINCREMENT,
    start_hour INT,
    finish_hour INT,
    day VARCHAR,
    schedule_id INTEGER REFERENCES schedule(schedule_id)
);

