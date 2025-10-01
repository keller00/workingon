ALTER TABLE `todos` DROP COLUMN `completed`;
ALTER TABLE `todos`
RENAME COLUMN `created` TO `created_on`
;

