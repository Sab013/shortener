# QuickLink

**QuickLink** — это высокопроизводительный API для сокращения URL-адресов. Сервис позволяет создавать короткие ссылки, перенаправлять пользователей на оригинальные URL-адреса и собирать статистику по использованию сокращённых ссылок.

---

## Основные возможности

- **Создание коротких ссылок**: Преобразуйте длинные URL-адреса в короткие и удобные для использования.
- **Перенаправление**: Автоматическое перенаправление пользователей на оригинальные URL-адреса.
- **Статистика**: Получайте информацию о количестве переходов по сокращённым ссылкам.

---

## Как это работает

1. **Создание короткой ссылки**: Отправьте POST-запрос с оригинальным URL-адресом и (опционально) желаемым slug (идентификатором короткой ссылки).
2. **Перенаправление**: Используйте короткую ссылку для перенаправления на оригинальный URL.
3. **Статистика**: Получите статистику по количеству переходов по короткой ссылке.

---

## Примеры использования

### Создание короткой ссылки
**POST /api/v1/links/create-short-link**
```bash
curl -X 'POST' \
  'http://localhost:8090/api/v1/links/slug' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "slug": "vasya-999",
  "url": " https://www.google.com"
}'
```

### Перенаправление по короткой ссылке на оригинальный url
**GET /api/v1/links/{vasya-999}/redirect**
```bash
curl -X 'GET' \
  'http://localhost:8090/api/v1/links/vasya-999/redirect/' \
  -H 'accept: */*'
```

### Получить статистику
**GET /api/v1/links/{vasya-999}/stats**
```bash
curl -X 'GET' \
  'http://localhost:8090/api/v1/links/vasya-999/stats' \
  -H 'accept: application/json'
```

---

## Docker
```bash
docker compose up --build -d
```