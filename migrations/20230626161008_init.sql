-- Add migration script here
CREATE TABLE notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  note TEXT NOT NULL,
  date_created DATE NOT NULL DEFAULT (DATETIME('now'))
);

CREATE INDEX idx_notes_date ON notes (date_created);