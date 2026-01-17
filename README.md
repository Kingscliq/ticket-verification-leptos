# Ticket Verification App (Leptos + QR Scanner)

A **static, WASM-powered ticket verification frontend** built with **Leptos (Rust)** that integrates with an existing **NestJS public verification API**.  
The app scans QR codes using the device camera, sends the decoded ticket code to a backend API, and displays the verification result in real time.

---

## Table of Contents
- Overview
- Architecture
- Why Leptos?
- Rendering Strategy
- Project Structure
- QR Code Scanning Design
- JavaScript ↔ Rust Interop
- Verification Flow
- Security Considerations
- Local Development
- Building for Production
- Deployment
- Future Enhancements

---

## Overview

This application is a **frontend-only** ticket verification client:

- **Frontend:** Leptos (Rust → WebAssembly)
- **Backend:** Existing NestJS API (public)
- **Deployment:** Static hosting (Vercel, Netlify, Cloudflare Pages)

---

## Architecture

Leptos (WASM) runs entirely in the browser.  
JavaScript handles camera access and QR decoding.  
Rust handles state, UI rendering, and API communication.

---

## Why Leptos?

- Type safety end-to-end
- Small runtime footprint
- High performance on low-end devices
- Compile-time UI guarantees
- Clean async and state management

---

## Rendering Strategy

The app uses **Client-Side Rendering (CSR)** with static assets.

- No Rust backend required
- Works on any static host
- Simple deployment model

---

## Project Structure

```
ticket-verifier/
├── index.html
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── qr.rs
│   ├── qr.js
│   └── components/
│       ├── qr_scanner.rs
│       └── verify.rs
└── dist/
```

---

## QR Code Scanning Design

QR scanning is implemented using a JavaScript library (`html5-qrcode`) because browsers expose camera APIs only via JavaScript.

Rust communicates with JavaScript using `wasm-bindgen`.

---

## JavaScript ↔ Rust Interop

- JavaScript controls the camera and QR decoding
- Rust calls JavaScript functions and receives decoded values via callbacks
- This keeps Rust safe and browser logic isolated

---

## Verification Flow

1. User starts QR scan
2. Camera opens
3. QR code is decoded
4. Ticket code is sent to NestJS API
5. API response determines validity
6. UI updates immediately

---

## API Integration (Planning)

**Endpoint:** `POST {{BASE_URL}}/v1/vendor/tickets/validate`

**Payload:**
```json
{
    "ticketCode": "{{ticketCode}}"
}
```

**Requirements:**
- The application must be fully responsive and optimized for mobile devices (primary use case for scanning).
- Real-time feedback on validation status.

---

## Local Development

### Prerequisites
```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### Run
```bash
trunk serve
```

### Testing on Mobile (or other devices)

Browsers block camera access on non-secure contexts (HTTP), except for `localhost`. To test on your phone:

1.  **Start the app locally**:
    ```bash
    trunk serve
    ```
2.  **Expose via a secure tunnel** (using something like `ngrok`):
    ```bash
    ngrok http 8080
    ```
3.  **Open the HTTPS URL**:
    - Copy the `https://...` URL provided by ngrok.
    - Open it on your mobile phone's browser.
    - Grant camera permissions when prompted.

---

## Building for Production

```bash
trunk build --release
```

Output appears in `dist/`.

---

## Deployment

### Vercel
- Framework: Other
- Build command: `trunk build --release`
- Output directory: `dist`

---

## Future Enhancements

- Manual ticket input fallback
- Loading and error states
- Audio / vibration feedback
- Staff authentication
- Offline handling

---

## Summary

This project demonstrates how to build a secure, fast, and maintainable ticket verification frontend using Rust, WASM, and Leptos.
