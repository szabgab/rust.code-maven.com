CREATE TABLE IF NOT EXISTS grades (
    id INTEGER PRIMARY KEY,
    student TEXT NOT NULL UNIQUE,
    math INTEGER,
    chemistry INTEGER,
    biology INTEGER,
    physics INTEGER,
    literature INTEGER,
    sport INTEGER,
    drawing INTEGER,
    created_at TEXT NOT NULL DEFAULT (CURRENT_TIMESTAMP),
    CHECK (math IS NULL OR (math BETWEEN 0 AND 100)),
    CHECK (chemistry IS NULL OR (chemistry BETWEEN 0 AND 100)),
    CHECK (biology IS NULL OR (biology BETWEEN 0 AND 100)),
    CHECK (physics IS NULL OR (physics BETWEEN 0 AND 100)),
    CHECK (literature IS NULL OR (literature BETWEEN 0 AND 100)),
    CHECK (sport IS NULL OR (sport BETWEEN 0 AND 100)),
    CHECK (drawing IS NULL OR (drawing BETWEEN 0 AND 100))
);
