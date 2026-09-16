INSERT INTO
    routines (
        id,
        title,
        start_time,
        duration_minutes,
        days_of_week,
        active,
        created_at,
        updated_at
    )
VALUES
    (
        'routine-wake-up',
        'Wake up',
        '05:00',
        NULL,
        X'5b312c322c332c342c352c362c375d', -- [1,2,3]
        1,
        datetime ('now'),
        datetime ('now')
    ),
    (
        'routine-quiet-time',
        'Have my Quiet Time',
        '05:20',
        45,
        X'5b312c322c332c342c352c362c375d', -- [1,2,3,4,5,6,7]
        1,
        datetime ('now'),
        datetime ('now')
    ),
    (
        'routine-ready-for-the-day',
        'Bathe, Freshen up, do some leetcode and any other activity I need to be ready by 7am',
        '06:00',
        60,
        X'5b312c322c332c342c352c362c375d', -- [1,2,3,4,5,6,7]
        1,
        datetime ('now'),
        datetime ('now')
    ),
    (
        'routine-study-rust',
        'Study Rust',
        '14:00',
        90,
        X'5b322c365d', -- [2,6]
        1,
        datetime ('now'),
        datetime ('now')
    ),
    (
        'routine-coding',
        'Build my developer skills',
        '21:00',
        60,
        X'5b312c322c332c342c355d', -- [1,2,3,4,5]
        1,
        datetime ('now'),
        datetime ('now')
    ),
    (
        'routine-bed',
        'Go to sleep with a 30minutes buffer',
        '22:30',
        30,
        X'5b312c322c332c342c352c362c375d', -- [1,2,3,4,5,6,7]
        1,
        datetime ('now'),
        datetime ('now')
    );