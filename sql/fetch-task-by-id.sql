SELECT
    db_id,
    public_id,
    summary,
    status as "status: TaskStatus",
    details,
    alarm_time,
    created_time
FROM Tasks
WHERE db_id = ?1;