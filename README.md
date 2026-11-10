# LightFox

**A lightweight Rust desktop browser project targeting Servo — no Chromium or WebView2.**

> **Current status:** this is an early desktop UI prototype. Servo is not integrated yet, so this build does not render web pages and is not a usable browser or installer.

## Download the source

[LightFox source archive](releases/LightFox-source.zip)

The archive contains the Rust desktop shell, documentation, Windows helper scripts, and the static project website.

## Project goals

- Native Rust desktop application
- Servo as the intended web engine
- Avoid Chromium and WebView2
- Keep idle resource use low without weakening browser security
- No telemetry or background account services

## Build the current UI prototype

Install Rust using [rustup](https://rustup.rs/), then run in PowerShell:

```powershell
Expand-Archive .\LightFox-source.zip -DestinationPath .
cd .\LightFox-1.0
cargo build --release
.\target\release\lightfox.exe
```

This builds only the interface prototype. It does not render websites.

## Website

The GitHub Pages workflow extracts the landing page from the source archive and deploys it. Enable **Settings → Pages → GitHub Actions** if Pages is not already enabled.

## License

The application code is licensed under MPL-2.0. Servo and dependencies retain their own licenses.
