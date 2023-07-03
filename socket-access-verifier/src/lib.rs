use futures_util::{SinkExt, StreamExt};
use log::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{
        Error,Result,
        protocol::Message,
    },
    
};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()>{
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");

    info!("New WebSocket connection: {}", peer);

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        let msg = msg.to_string();
        if msg == "check" {
            ws_stream.send("ok".into()).await?;
            break;
        }
    }

    Ok(())
}

async fn start_server(port: u16) {
    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");

        tokio::spawn(accept_connection(peer, stream));
    }
}

async fn try_send_check(server_addr: SocketAddr) -> Result<()> {
    let (ws_stream, _) = tokio_tungstenite::connect_async(format!("ws://{}", server_addr))
        .await
        .expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();

    let _ = write.send(Message::Text("check".into())).await.expect("Failed to write");

    let timeout = tokio::time::sleep(std::time::Duration::from_secs(10));

    tokio::pin!(timeout);

    loop {
        tokio::select! {
            _ = &mut timeout => {
                info!("Timeout waiting for response");
                return Err(Error::ConnectionClosed);
            }
            message = read.next() => {
                match message {
                    Some(Ok(msg)) if msg.is_text() => {
                        let text = msg.into_text().unwrap();
                        if text == "ok" {
                            println!("Received 'ok'");
                            return Ok(());
                        }
                    },
                    _ => return Err(Error::ConnectionClosed),
                }
            }
        }
    }
}

pub async fn verify_socket(addr: SocketAddr) -> Result<()> {
    let server = tokio::spawn(start_server(addr.port()));
    let client = tokio::spawn(try_send_check(addr));

    let (_server, client) = tokio::join!(server, client);

    client.unwrap()
}