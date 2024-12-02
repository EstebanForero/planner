-- Create the planner_user table
CREATE TABLE planner_user (
    user_id INTEGER PRIMARY KEY AUTOINCREMENT
);

-- Create the classes table with an INTEGER primary key
CREATE TABLE classes (
    class_id INTEGER PRIMARY KEY AUTOINCREMENT,
    class_name VARCHAR NOT NULL,
    user_id INTEGER REFERENCES planner_user(user_id) NOT NULL
);

-- Create the schedule table with an INTEGER primary key
CREATE TABLE schedule (
    schedule_id INTEGER PRIMARY KEY AUTOINCREMENT,
    class_id INTEGER REFERENCES classes(class_id) NOT NULL,
    schedule_name VARCHAR NOT NULL
);

-- Create the block table with an INTEGER primary key
CREATE TABLE block (
    block_id INTEGER PRIMARY KEY AUTOINCREMENT,
    start_hour INT NOT NULL,
    finish_hour INT NOT NULL,
    day VARCHAR NOT NULL,
    schedule_id INTEGER REFERENCES schedule(schedule_id) NOT NULL
);

