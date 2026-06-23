package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"strings"
	"sync"
	"time"
)

// ── CONFIG ────────────────────────────────────
const PORT = ":8080"

// ── CLIENT ────────────────────────────────────
// Represents one connected client
type Client struct {
	conn net.Conn
	name string
}

// ── SERVER STATE ──────────────────────────────
var (
	clients   = make(map[net.Conn]*Client) // all connected clients
	clientsMu sync.Mutex                   // mutex to protect the map
)

// ── MAIN ──────────────────────────────────────
func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage:")
		fmt.Println("  broadcast-server start    → start the server")
		fmt.Println("  broadcast-server connect  → connect as a client")
		os.Exit(1)
	}

	switch os.Args[1] {
	case "start":
		startServer()
	case "connect":
		startClient()
	default:
		fmt.Println("Unknown command:", os.Args[1])
		fmt.Println("Use 'start' or 'connect'")
		os.Exit(1)
	}
}

// ═════════════════════════════════════════════
//  SERVER
// ═════════════════════════════════════════════

func startServer() {
	listener, err := net.Listen("tcp", PORT)
	if err != nil {
		fmt.Println("Error starting server:", err)
		os.Exit(1)
	}
	defer listener.Close()

	fmt.Println("✅ Broadcast server started on port", PORT)
	fmt.Println("   Waiting for clients to connect...")

	// Accept clients in a loop
	for {
		conn, err := listener.Accept()
		if err != nil {
			fmt.Println("Connection error:", err)
			continue
		}

		// Each client gets its own goroutine
		go handleClient(conn)
	}
}

func handleClient(conn net.Conn) {
	defer conn.Close()

	// Ask for a name
	conn.Write([]byte("Enter your name: "))
	scanner := bufio.NewScanner(conn)

	if !scanner.Scan() {
		return
	}
	name := strings.TrimSpace(scanner.Text())
	if name == "" {
		name = "Anonymous"
	}

	// Register client
	client := &Client{conn: conn, name: name}
	clientsMu.Lock()
	clients[conn] = client
	clientsMu.Unlock()

	fmt.Printf("[+] %s connected (%s)\n", name, conn.RemoteAddr())
	broadcast(fmt.Sprintf("📢 %s has joined the chat!", name), conn)

	// Listen for messages from this client
	for scanner.Scan() {
		msg := strings.TrimSpace(scanner.Text())
		if msg == "" {
			continue
		}

		timestamp := time.Now().Format("15:04:05")
		formatted := fmt.Sprintf("[%s] %s: %s", timestamp, name, msg)

		fmt.Println(formatted)     // log on server
		broadcast(formatted, conn) // send to all other clients
	}

	// Client disconnected
	clientsMu.Lock()
	delete(clients, conn)
	clientsMu.Unlock()

	fmt.Printf("[-] %s disconnected\n", name)
	broadcast(fmt.Sprintf("📢 %s has left the chat.", name), conn)
}

// Send a message to all clients except the sender
func broadcast(msg string, sender net.Conn) {
	clientsMu.Lock()
	defer clientsMu.Unlock()

	for conn := range clients {
		if conn != sender {
			conn.Write([]byte(msg + "\n"))
		}
	}
}

// ═════════════════════════════════════════════
//  CLIENT
// ═════════════════════════════════════════════

func startClient() {
	conn, err := net.Dial("tcp", "localhost"+PORT)
	if err != nil {
		fmt.Println("Could not connect to server:", err)
		os.Exit(1)
	}
	defer conn.Close()

	fmt.Println("✅ Connected to server. Type messages and press Enter.")
	fmt.Println("   (Ctrl+C to quit)")

	// Goroutine: receive messages from server and print them
	go func() {
		scanner := bufio.NewScanner(conn)
		for scanner.Scan() {
			fmt.Println(scanner.Text())
		}
		fmt.Println("disconnected from server.")
		os.Exit(0)
	}()

	// Main: read from terminal and send to server
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		msg := scanner.Text()
		conn.Write([]byte(msg + "\n"))
	}
}
