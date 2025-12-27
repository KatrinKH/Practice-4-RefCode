# Практическая работа 4

Этот проект запускается с помощью Docker Compose и включает несколько сервисов: базу данных, веб-сервер на PHP, Rust-сервис и старое приложение на Pascal.

## Состав сервисов

- **iss_db** — PostgreSQL база данных (порт 5432)
- **php_web** — PHP веб-приложение (порт 8000 внутри контейнера, 8080 на хосте)
- **rust_iss** — Rust-сервис (порт 3000)
- **pascal_legacy** — старое приложение на Pascal

## Запуск проекта

**1. Сборка контейнеров**
```bash
docker compose build
```

**2. Запуск контейнеров**
```bash
docker compose up
```

**3. Проверка статуса контейнеров**
```bash
docker compose ps
```

**4. Запуск PHP-сервера вручную**
```bash
docker compose exec php php artisan serve --host=0.0.0.0 --port=8000
```

**5. Доступ к сайту**
```bash
http://localhost:8080/dashboard
```

**5. Остановка проекта**
```bash
docker compose down
```