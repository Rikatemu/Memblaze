use dashmap::DashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;

async fn handle_connection(mut stream: TcpStream, db: Arc<DashMap<String, String>>) {
    let mut buffer = [0; 1024];

    loop {
        let n = match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(_) => return,
        };

        let request = String::from_utf8_lossy(&buffer[..n]);
        let mut parts = request.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let key = parts.next().unwrap_or("");
        let value = parts.next().unwrap_or("");

        let response = match command {
            "SET" => {
                db.insert(key.to_string(), value.to_string()); // Directly use DashMap here
                "OK\n".to_string()
            },
            "GET" => {
                match db.get(key) { // Correctly handle DashMap's return type here
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

        if let Err(e) = stream.write_all(response.as_bytes()).await {
            println!("Failed to send response: {}", e);
            return;
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let db = Arc::new(DashMap::new());

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move {
            handle_connection(stream, db).await;
        });
    }
}