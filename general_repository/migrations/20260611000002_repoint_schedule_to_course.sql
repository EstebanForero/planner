-- Add migration script here

-- schedule now belongs to course (shared), not to a single user's class.
ALTER TABLE schedule DROP CONSTRAINT schedule_class_id_fkey;
ALTER TABLE schedule DROP COLUMN class_id;

-- The previous migration's "ADD COLUMN ... REFERENCES course(course_id)" already
-- created default-named FK constraints with no ON DELETE behavior; replace them.
ALTER TABLE schedule DROP CONSTRAINT schedule_course_id_fkey;
ALTER TABLE classes DROP CONSTRAINT classes_course_id_fkey;

-- If a course is ever deleted, its shared schedules/blocks go with it.
ALTER TABLE schedule
    ADD CONSTRAINT schedule_course_id_fkey FOREIGN KEY (course_id) REFERENCES course(course_id) ON DELETE CASCADE;

-- A class entry is meaningless without its course; deleting a course removes
-- every user's class entries that reference it. Deleting a single class (or
-- user) row never cascades back up to the shared course/schedule/block rows.
ALTER TABLE classes
    ADD CONSTRAINT classes_course_id_fkey FOREIGN KEY (course_id) REFERENCES course(course_id) ON DELETE CASCADE;
