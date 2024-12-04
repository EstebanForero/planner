-- Add migration script here

-- Drop existing foreign key constraints
ALTER TABLE classes DROP CONSTRAINT classes_user_id_fkey;
ALTER TABLE schedule DROP CONSTRAINT schedule_class_id_fkey;

-- Add foreign key constraints with ON DELETE CASCADE
ALTER TABLE classes
ADD CONSTRAINT classes_user_id_fkey FOREIGN KEY (user_id) REFERENCES planner_user(user_id) ON DELETE CASCADE;

ALTER TABLE schedule
ADD CONSTRAINT schedule_class_id_fkey FOREIGN KEY (class_id) REFERENCES classes(class_id) ON DELETE CASCADE;

