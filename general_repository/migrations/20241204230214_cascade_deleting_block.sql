-- Add migration script here

-- Drop the existing foreign key constraint on block
ALTER TABLE block DROP CONSTRAINT block_schedule_id_fkey;

-- Add the new foreign key with ON DELETE CASCADE
ALTER TABLE block
ADD CONSTRAINT block_schedule_id_fkey FOREIGN KEY (schedule_id) REFERENCES schedule(schedule_id) ON DELETE CASCADE;

