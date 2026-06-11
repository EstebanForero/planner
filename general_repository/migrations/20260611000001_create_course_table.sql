-- Add migration script here

-- Shared course catalog: a course owns a set of shared schedule/block options
-- that multiple users' personal "classes" rows can link to.
CREATE TABLE course (
    course_id SERIAL PRIMARY KEY,
    course_name VARCHAR NOT NULL
);

-- Backfill: every existing class becomes its own course (1:1, reusing the
-- same id so existing schedule rows can be repointed below).
INSERT INTO course (course_id, course_name)
SELECT class_id, class_name FROM classes;

-- Make sure future course_id values don't collide with the borrowed class ids.
SELECT setval('course_course_id_seq', (SELECT COALESCE(MAX(course_id), 1) FROM course));

-- Link each class to its (backfilled) course.
ALTER TABLE classes ADD COLUMN course_id INTEGER REFERENCES course(course_id);
UPDATE classes SET course_id = class_id;
ALTER TABLE classes ALTER COLUMN course_id SET NOT NULL;

-- Repoint schedules at the course (still keeping class_id for now; dropped in
-- the next migration once the data is verified to be consistent).
ALTER TABLE schedule ADD COLUMN course_id INTEGER REFERENCES course(course_id);
UPDATE schedule s SET course_id = c.course_id FROM classes c WHERE s.class_id = c.class_id;
ALTER TABLE schedule ALTER COLUMN course_id SET NOT NULL;

CREATE INDEX idx_classes_course_id ON classes(course_id);
CREATE INDEX idx_schedule_course_id ON schedule(course_id);
CREATE INDEX idx_course_name ON course(course_name);
