-- Add migration script here
CREATE TABLE notes (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  note TEXT NOT NULL,
  date DATE NOT NULL
);

CREATE INDEX idx_notes_date ON notes (date);