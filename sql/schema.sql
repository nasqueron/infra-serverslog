CREATE TABLE IF NOT EXISTS log (
    date VARCHAR(20),
    emitter VARCHAR(32),
    source VARCHAR(32),
    component VARCHAR(32),
    entry TEXT
);
