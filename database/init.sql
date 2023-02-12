CREATE TABLE IF NOT EXISTS idea (
    id              uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name            VARCHAR(64) NOT NULL,
    description     varchar(256)
);

INSERT INTO idea (name, description)
VALUES
    ('extract path (axum)', 'extract path parameter'),
    ('addition', null),
    ('share state (axum)', 'How to share data between routes'),
    ('middleware (axum)', 'apply a basic middleware')
    
