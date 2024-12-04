-- Create the planner_user table
CREATE TABLE planner_user (
    user_id SERIAL PRIMARY KEY
);

-- Create the classes table with a SERIAL primary key
CREATE TABLE classes (
    class_id SERIAL PRIMARY KEY,
    class_name VARCHAR NOT NULL,
    user_id INTEGER REFERENCES planner_user(user_id) NOT NULL
);

-- Create the schedule table with a SERIAL primary key
CREATE TABLE schedule (
    schedule_id SERIAL PRIMARY KEY,
    class_id INTEGER REFERENCES classes(class_id) NOT NULL,
    schedule_name VARCHAR NOT NULL
);

-- Create the block table with a SERIAL primary key
CREATE TABLE block (
    block_id SERIAL PRIMARY KEY,
    start_hour INTEGER NOT NULL,
    finish_hour INTEGER NOT NULL,
    day VARCHAR NOT NULL,
    schedule_id INTEGER REFERENCES schedule(schedule_id) NOT NULL
);
