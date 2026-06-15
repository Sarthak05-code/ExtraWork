const express = require('express');
const http = require('http');
const cors = require('cors');
const { Server } = require('socket.io');

const PORT = process.env.PORT || 5000;
const CLIENT_ORIGIN = process.env.CLIENT_ORIGIN || '*';

const app = express();
app.use(cors({ origin: CLIENT_ORIGIN }));
app.use(express.json());

// Simple health check, also useful for deployment platforms
app.get('/health', (req, res) => {
  res.json({ status: 'ok', uptime: process.uptime() });
});

const server = http.createServer(app);

const io = new Server(server, {
  cors: { origin: CLIENT_ORIGIN, methods: ['GET', 'POST'] },
});

// roomId -> Set of usernames currently in that room
const rooms = new Map();

io.on('connection', (socket) => {
  console.log(`[connect] ${socket.id}`);

  socket.on('join_room', ({ username, room }) => {
    if (!username || !room) return;

    socket.join(room);
    socket.data.username = username;
    socket.data.room = room;

    if (!rooms.has(room)) rooms.set(room, new Set());
    rooms.get(room).add(username);

    socket.to(room).emit('user_joined', { username, time: new Date().toISOString() });
    io.to(room).emit('room_users', Array.from(rooms.get(room)));
  });

  socket.on('send_message', ({ room, username, message }) => {
    if (!room || !message?.trim()) return;
    io.to(room).emit('receive_message', {
      username,
      message: message.trim(),
      time: new Date().toISOString(),
    });
  });

  socket.on('typing', ({ room, username, isTyping }) => {
    socket.to(room).emit('user_typing', { username, isTyping });
  });

  socket.on('disconnect', () => {
    const { username, room } = socket.data;
    if (room && rooms.has(room)) {
      rooms.get(room).delete(username);
      io.to(room).emit('room_users', Array.from(rooms.get(room)));
      socket.to(room).emit('user_left', { username, time: new Date().toISOString() });
    }
    console.log(`[disconnect] ${socket.id}`);
  });
});

// Only start listening when run directly (not when imported by tests)
if (require.main === module) {
  server.listen(PORT, () => {
    console.log(`Server listening on port ${PORT}`);
  });
}

module.exports = { app, server };
