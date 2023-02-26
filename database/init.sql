CREATE TABLE IF NOT EXISTS idea (
    id              uuid DEFAULT gen_random_uuid() PRIMARY KEY,
    name            VARCHAR(64) NOT NULL,
    context         VARCHAR(64),
    description     VARCHAR(512)
);

INSERT INTO idea (name, context, description)
VALUES
    ('extract path', 'axum', 'extract path parameter'),
    ('addition', null, null),
    ('share state', 'axum', 'How to share data between routes'),
    ('middleware', 'axum', 'apply a basic middleware')
    
