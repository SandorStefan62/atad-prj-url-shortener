# atad-prj-url-shortener
Service similar to bit.ly that turns long urls into short, memorable links that also provides analytics.

## Architecture
![alt text](/resources/atad-arch.png)

## Folder Structure

```bash
url_shortener/
├── migrations/ <- db operations
│   ├── 20240101_create_urls.sql
├── src/
│   ├── main.rs <- main app
│   ├── config.rs <- env var parsing
│   ├── error.rs <- custom err msgs
│   ├── models/ <- custom data types
│   │   ├── mod.rs
│   │   ├── url.rs
│   ├── db/ <- db queries
│   │   ├── mod.rs
│   │   └── queries.rs
│   ├── handlers/ <- http requests
│   │   ├── mod.rs
│   │   ├── shorten.rs
│   │   └── web.rs
│   ├── services/ <- business logic
│   │   ├── mod.rs
│   │   ├── shortener.rs
│   ├── templates/ <- html templating
│   │   ├── mod.rs
├── static/ <- static content
│   ├── css/
│   │   └── styles.css
│   └── js/
│       └── app.js
├── templates/ <- html ui content
│   ├── dashboard.html
│   └── index.html
├── .env.example
├── Cargo.lock
└── Cargo.toml
```

## Architecture Overview
<b>Layer Structure</b>

1. HTTP Layer (handlers/)
    - Receives HTTP requests
    - Validates Input
    - Returns responses
2. Service Layer (services/)
    - Business logic
    - Short Code Generation
    - and future logic such as rate limiting
3. Data Layer (db/)
    - Database queries
4. Models (models/)
    - Data structures that flow through the app
    - Request/Response types

<b>Features Implemented So Far</b>

- URL shortening with auto-generated codes.
- Custom short codes support.
- Web landing page.
- Web dashboard.
- Start of RESTful api.

<b>Next steps to implement</b>
- [] Rate limiting.
- [] Collision detection.
- [x] Click tracking with metadata.
- [x] Some sort of analytics (TBD).
- [] QR code generation.
- [x] Expiration dates.