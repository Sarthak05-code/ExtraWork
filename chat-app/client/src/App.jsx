import { useEffect, useRef, useState } from 'react';
import { socket } from './socket';
import './App.css';

// Deterministic color per username so everyone gets a consistent avatar color
function avatarColor(name) {
  let hash = 0;
  for (let i = 0; i < name.length; i++) {
    hash = name.charCodeAt(i) + ((hash << 5) - hash);
  }
  const hue = Math.abs(hash) % 360;
  return `hsl(${hue}, 65%, 55%)`;
}

function formatTime(iso) {
  return new Date(iso).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

export default function App() {
  const [username, setUsername] = useState('');
  const [room, setRoom] = useState('');
  const [joined, setJoined] = useState(false);
  const [messages, setMessages] = useState([]);
  const [draft, setDraft] = useState('');
  const [onlineUsers, setOnlineUsers] = useState([]);
  const [typingUser, setTypingUser] = useState(null);

  const typingTimeout = useRef(null);
  const messagesEndRef = useRef(null);

  useEffect(() => {
    socket.on('receive_message', (data) => {
      setMessages((prev) => [...prev, { type: 'message', ...data }]);
    });

    socket.on('user_joined', ({ username: name, time }) => {
      setMessages((prev) => [...prev, { type: 'system', text: `${name} joined the room`, time }]);
    });

    socket.on('user_left', ({ username: name, time }) => {
      setMessages((prev) => [...prev, { type: 'system', text: `${name} left the room`, time }]);
    });

    socket.on('room_users', (users) => {
      setOnlineUsers(users);
    });

    socket.on('user_typing', ({ username: name, isTyping }) => {
      setTypingUser(isTyping ? name : null);
    });

    return () => {
      socket.off('receive_message');
      socket.off('user_joined');
      socket.off('user_left');
      socket.off('room_users');
      socket.off('user_typing');
    };
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages, typingUser]);

  const handleJoin = (e) => {
    e.preventDefault();
    if (!username.trim() || !room.trim()) return;
    socket.emit('join_room', { username: username.trim(), room: room.trim() });
    setJoined(true);
  };

  const handleSend = (e) => {
    e.preventDefault();
    if (!draft.trim()) return;
    socket.emit('send_message', { room, username, message: draft });
    setDraft('');
    socket.emit('typing', { room, username, isTyping: false });
  };

  const handleTyping = (value) => {
    setDraft(value);
    socket.emit('typing', { room, username, isTyping: true });

    clearTimeout(typingTimeout.current);
    typingTimeout.current = setTimeout(() => {
      socket.emit('typing', { room, username, isTyping: false });
    }, 1500);
  };

  if (!joined) {
    return (
      <div className="screen join-screen">
        <div className="card">
          <h1 className="logo">Pingback</h1>
          <p className="tagline">Drop a room code, bring your people in.</p>
          <form onSubmit={handleJoin}>
            <label>
              Your name
              <input
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                placeholder="e.g. Sarthak"
                maxLength={20}
                required
              />
            </label>
            <label>
              Room code
              <input
                value={room}
                onChange={(e) => setRoom(e.target.value)}
                placeholder="e.g. bca-2026"
                maxLength={30}
                required
              />
            </label>
            <button type="submit">Join room</button>
          </form>
        </div>
      </div>
    );
  }

  return (
    <div className="screen chat-screen">
      <header className="chat-header">
        <div>
          <h1>{room}</h1>
          <p>{onlineUsers.length} online</p>
        </div>
        <div className="avatars">
          {onlineUsers.map((user) => (
            <div
              key={user}
              className="avatar"
              style={{ backgroundColor: avatarColor(user) }}
              title={user}
            >
              {user.charAt(0).toUpperCase()}
            </div>
          ))}
        </div>
      </header>

      <main className="message-list">
        {messages.map((m, i) =>
          m.type === 'system' ? (
            <div key={i} className="system-message">
              {m.text}
            </div>
          ) : (
            <div key={i} className={`bubble-row ${m.username === username ? 'own' : ''}`}>
              <div className="avatar small" style={{ backgroundColor: avatarColor(m.username) }}>
                {m.username.charAt(0).toUpperCase()}
              </div>
              <div className="bubble">
                <div className="bubble-meta">
                  <span className="bubble-name">{m.username}</span>
                  <span className="bubble-time">{formatTime(m.time)}</span>
                </div>
                <p>{m.message}</p>
              </div>
            </div>
          )
        )}
        {typingUser && <div className="typing-indicator">{typingUser} is typing…</div>}
        <div ref={messagesEndRef} />
      </main>

      <form className="message-form" onSubmit={handleSend}>
        <input
          value={draft}
          onChange={(e) => handleTyping(e.target.value)}
          placeholder="Type a message"
          autoFocus
        />
        <button type="submit">Send</button>
      </form>
    </div>
  );
}
