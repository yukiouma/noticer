DROP TABLE IF EXISTS `task`;
CREATE TABLE IF NOT EXISTS `task` (
    `id` BIGINT AUTO_INCREMENT,
    `name` VARCHAR(255) NOT NULL,
    `description` VARCHAR(255) NOT NULL,
    `expect_times` INT NULL,
    `month` INT NULL,
    `day` INT NULL,
    `weekday` INT NULL,
    `timepoint` INT NULL,
    `time_gap` INT NULL,
    `duration_start` INT NULL,
    `duration_end` INT NULL,
    `execute_times` INT NULL,
    `last_executed_at` TIMESTAMP,
    `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    `deleted_at` TIMESTAMP,
    PRIMARY KEY(`id`)
);