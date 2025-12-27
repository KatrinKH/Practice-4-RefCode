-- Basic schema

CREATE TABLE IF NOT EXISTS iss_fetch_log (
    id BIGSERIAL PRIMARY KEY,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source_url TEXT NOT NULL,
    payload JSONB NOT NULL
);

CREATE TABLE IF NOT EXISTS telemetry_legacy (
    id BIGSERIAL PRIMARY KEY,
    recorded_at TIMESTAMPTZ NOT NULL,
    voltage NUMERIC(6,2) NOT NULL,
    temp NUMERIC(6,2) NOT NULL,
    source_file TEXT NOT NULL
);

CREATE TABLE cms_blocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    content TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE cms_pages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    content TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1
);


-- Seed with deliberately unsafe content for XSS practice
INSERT INTO cms_pages(slug, title, body)
VALUES
('welcome', 'Добро пожаловать', '<h3>Демо контент</h3><p>Этот текст хранится в БД</p>'),
('unsafe', 'Небезопасный пример', '<script>console.log("XSS training")
</script><p>Если вы видите всплывашку значит защита не работает</p>')
ON CONFLICT DO NOTHING;


INSERT INTO cms_blocks (slug, title, content, is_active) VALUES
(
  'dashboard_experiment',
  'Астрономические события',
  '<h3>Астрономические события</h3><p>Данные загружены из SQLite</p>',
  1
),
(
  'astro',
  'Astro',
  '<h3>Astro</h3><p>Страница CMS на SQLite</p>',
  1
);
