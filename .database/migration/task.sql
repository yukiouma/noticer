START TRANSACTION;
DROP TABLE IF EXISTS `task`;
CREATE TABLE IF NOT EXISTS `task` (
    `id` BIGINT AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `description` VARCHAR(255) NOT NULL,
    `expect_times` INT NOT NULL,
    `month` SMALLINT NOT NULL,
    `day` INT NOT NULL,
    `weekday` TINYINT NOT NULL,
    `timepoint` INT NOT NULL,
    `time_gap` INT NOT NULL,
    `duration_start`: INT NOT NULL,
    `duration_end`: INT NOT NULL,
    `execute_times`: INT NOT NULL,
    `last_executed_at` TIMESTAMP,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    `deleted_at` TIMESTAMP,
    PRIMARY KEY(`id`)
);
COMMIT;