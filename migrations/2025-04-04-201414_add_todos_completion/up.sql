ALTER TABLE `todos`
ADD COLUMN `completed_on` TIMESTAMPTZSQLITE DEFAULT NULL
;

ALTER TABLE `todos`
RENAME COLUMN `created_on` TO `created`
;
