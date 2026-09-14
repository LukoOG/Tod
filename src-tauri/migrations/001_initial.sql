CREATE TABLE days (
    date TEXT PRIMARY KEY,

    mood INTEGER,
    reflection TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE routines (
    id TEXT PRIMARY KEY,

    title TEXT NOT NULL,

    start_time TEXT,
    duration_minutes INTEGER,

    days_of_week TEXT NOT NULL,

    active INTEGER NOT NULL DEFAULT 1,

    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE activities (
    id TEXT PRIMARY KEY,

    day_date TEXT NOT NULL,
    title TEXT NOT NULL,

    start_time TEXT,
    duration_minutes INTEGER,

    completed INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,

    notes TEXT,

    source_routine_id TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    FOREIGN KEY (day_date)
        REFERENCES days(date)
        ON DELETE CASCADE,

    FOREIGN KEY (source_routine_id)
        REFERENCES routines(id)
        ON DELETE SET NULL

    UNIQUE(day_date, source_routine_id)
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,

    day_date TEXT NOT NULL,
    title TEXT NOT NULL,

    completed INTEGER NOT NULL DEFAULT 0,
    completed_at TEXT,

    notes TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    FOREIGN KEY (day_date)
        REFERENCES days(date)
        ON DELETE CASCADE
);

CREATE INDEX idx_activities_day_date
    ON activities(day_date);

CREATE INDEX idx_tasks_day_date
    ON tasks(day_date);