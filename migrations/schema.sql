PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS pages (
    id TEXT PRIMARY KEY,
    content TEXT NOT NULL,
    parent_id TEXT,
    CONSTRAINT fk_parent
        FOREIGN KEY (parent_id) 
        REFERENCES pages(id) 
        ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_pages_parent ON pages(parent_id);

CREATE TABLE IF NOT EXISTS links (
    source_id TEXT NOT NULL,
    target_id TEXT NOT NULL,
    PRIMARY KEY (source_id, target_id),
    CONSTRAINT fk_source
        FOREIGN KEY (source_id) 
        REFERENCES pages(id) 
        ON DELETE CASCADE,
    CONSTRAINT fk_target
        FOREIGN KEY (target_id) 
        REFERENCES pages(id) 
        ON DELETE CASCADE,
    CONSTRAINT check_self_loop 
        CHECK (source_id != target_id)
);

CREATE INDEX IF NOT EXISTS idx_links_target ON links(target_id);
