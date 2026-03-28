use crate::store::Store;
use std::str::from_utf8;
use std::sync::Arc;
use std::sync::mpsc::Sender;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn handle_connection(
    mut stream: TcpStream,
    store: Arc<Store>,
    tx: Sender<String>,
) -> Result<(), ()> {
    let mut buf = [0; 1024];

    loop {
        let n = match stream.read(&mut buf).await {
            Ok(n) => n,
            Err(_) => return Err(()),
        };
        if n == 0 {
            break;
        }
        let input = match from_utf8(&buf[0..n]) {
            Ok(val) => val,
            Err(_) => break,
        };
        for line in input.split('\n') {
            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }
            if parts[0].eq_ignore_ascii_case("set") {
                if parts.len() != 3 {
                    if stream
                        .write_all(b"error in number of arguments\n")
                        .await
                        .is_err()
                    {
                        break;
                    }
                    continue;
                }
                store.set(parts[1].to_string(), parts[2].to_string());

                if tx.send(line.to_string()).is_err() {
                    break;
                }
            } else if parts[0].eq_ignore_ascii_case("get") {
                if parts.len() != 2 {
                    if stream
                        .write_all(b"error in number of arguments\n")
                        .await
                        .is_err()
                    {
                        break;
                    }
                    continue;
                }
                if let Some(val) = store.get(parts[1]) {
                    if stream
                        .write_all(format!("{}\n", val).as_bytes())
                        .await
                        .is_err()
                    {
                        break;
                    }
                } else {
                    if stream.write_all(b"(nil)\n").await.is_err() {
                        break;
                    }
                }
            } else {
                if stream.write_all(b"invalid command\n").await.is_err() {
                    break;
                }
            }
        }
    }

    Ok(())
}
