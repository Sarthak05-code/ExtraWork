# Pingback — Realtime Chat App

A simple realtime chat app built with **React (Vite)** on the frontend and **Node.js + Express + Socket.io** on the backend. Users join a room with a username and room code, then chat in real time with live typing indicators and an online users list.

## Project structure

```
.
├── .github/workflows/ci.yml   # CI pipeline (lint, build, test)
├── client/                    # React frontend (Vite)
│   ├── src/
│   │   ├── App.jsx
│   │   ├── App.css
│   │   ├── socket.js          # shared socket.io-client instance
│   │   ├── main.jsx
│   │   └── index.css
│   ├── index.html
│   ├── package.json
│   └── .env.example
├── server/                    # Express + Socket.io backend
│   ├── server.js
│   ├── tests/health.test.js
│   ├── package.json
│   └── .env.example
└── .gitignore
```

## Getting started

### 1. Server

```bash
cd server
cp .env.example .env
npm install
npm run dev      # nodemon, restarts on changes
```

Runs on `http://localhost:5000` by default. `GET /health` returns a JSON status, useful for checking the server is up.

### 2. Client

```bash
cd client
cp .env.example .env
npm install
npm run dev
```

Runs on `http://localhost:5173` by default (Vite's default port).

Make sure `VITE_SERVER_URL` in `client/.env` matches where your server is running, and `CLIENT_ORIGIN` in `server/.env` matches where your client is running (so CORS doesn't block the connection).

## How it works

- `join_room` — client emits this with `{ username, room }` when joining. Server tracks who's in each room.
- `send_message` — broadcasts a chat message to everyone in the room.
- `typing` — broadcasts a "user is typing" indicator.
- `room_users` / `user_joined` / `user_left` — keep the online users list and join/leave system messages in sync.

## CI/CD

`.github/workflows/ci.yml` runs on every push and pull request to `main`, with two jobs:

- **client** — installs dependencies, runs ESLint, and builds the production bundle.
- **server** — installs dependencies and runs the test suite (a basic `/health` check using Node's built-in test runner, no extra dependencies needed).

> **Important:** the workflow uses `npm ci`, which requires a `package-lock.json` in each folder. Run `npm install` inside `client/` and `server/` locally at least once and commit the generated `package-lock.json` files — otherwise the CI jobs will fail with "no lock file found".

## Next steps / ideas

- Persist messages with a database (e.g. MySQL) instead of in-memory only.
- Add authentication so usernames are tied to accounts.
- Add a deploy job to the CI workflow (e.g. server to Render/Railway, client to Vercel/Netlify) once you've picked a host.
