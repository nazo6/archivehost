CREATE TABLE archives (
  url TEXT NOT NULL,
  mime TEXT NOT NULL,
  timestamp INTEGER NOT NULL,
  status INTEGER,
  save_path TEXT NOT NULL,
  PRIMARY KEY (url, timestamp)
);
