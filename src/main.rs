// Import necessary modules from the DashMap, Tokio, and standard library
use dashmap::DashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;

// Entry point for the Tokio async runtime.
#[tokio::main]
async fn main() {
    // Create a new, thread-safe, shared DashMap database
    let db = Arc::new(DashMap::new());

    // Bind a TCP listener to localhost on port 6379 (commonly used by Redis as well)
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // Continuously accept incoming connections
    loop {
        // Accept a connection (blocking call until a connection is established)
        let (stream, _) = listener.accept().await.unwrap();

        // Clone the database Arc to share across threads
        let db = db.clone();

        // Spawn a new async task to handle the connection
        tokio::spawn(async move {
            handle_connection(stream, db).await;
        });
    }
}

// Asynchronously handle incoming TCP connection.
// `stream` is the socket connection between server and client.
// `db` is a thread-safe, shared database.
async fn handle_connection(mut stream: TcpStream, db: Arc<DashMap<String, String>>) {
    // Buffer for reading data from the connection
    let mut buffer = [0; 1024];

    loop {
        // Read data into the buffer. Exit loop if connection closed or error occurs.
        let n = match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(_) => return,
        };

        // Convert the buffer into a string and process the command
        let request = String::from_utf8_lossy(&buffer[..n]);
        let mut parts = request.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");

        // Match the command and execute database operations accordingly
        let response = match command {
            "SET" => {
                db.insert(key.to_string(), value.to_string());
                "OK\n".to_string()
            },
            "GET" => {
                match db.get(key) {
                    Some(value) => value.clone(),
                    None => "NotFound\n".to_string(),
                }
            },
            "DEL" => {
                if db.remove(key).is_some() {
                    "OK\n".to_string()
                } else {
                    "NotFound\n".to_string()
                }
            },
            _ => "Invalid Command\n".to_string(),
        };

        // Send response back to client. Exit loop if error occurs.
        if let Err(e) = stream.write_all(response.as_bytes()).await {
            println!("Failed to send response: {}", e);
            return;
        }
    }
}