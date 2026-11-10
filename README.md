# LightFox

**Экспериментальный лёгкий desktop-браузер на Rust. Цель — Servo, без Chromium и WebView2.**

> **Статус:** ранний прототип интерфейса. Servo пока не интегрирован; приложение не отображает веб-страницы и не является готовым браузером или установщиком.

## Скачать

[Скачать исходный код (ZIP)](https://github.com/maksimkramsaev58-prog/waterfox/archive/refs/heads/main.zip)

## Цели проекта

- Нативное приложение на Rust
- Servo как целевой браузерный движок
- Никакого Chromium или WebView2
- Контроль потребления ресурсов без ослабления безопасности
- Без встроенной телеметрии и фоновых аккаунтов

## Сборка прототипа на Windows 11

Установи Rust через [rustup](https://rustup.rs/), затем в PowerShell:

```powershell
cargo build --release
.\target\release\lightfox.exe
```

Это соберёт только интерфейсный прототип. Веб-страницы пока не отображаются.

## Сайт проекта

После включения GitHub Pages (Settings → Pages → GitHub Actions) сайт будет доступен по адресу:

https://maksimkramsaev58-prog.github.io/waterfox/

## Лицензия

Код приложения распространяется под MPL-2.0. Servo и сторонние зависимости сохраняют собственные лицензии.
