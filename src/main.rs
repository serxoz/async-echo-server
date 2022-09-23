/*
   Exemplo de servidor de eco asíncrono
*/

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use std::{env, io};

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:1337".to_string());

    let listener = TcpListener::bind(&addr).await?;
    println!("Escoitando en: {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(procesar_socket(socket));  // tokio::spawn """"abre un novo fío""""
    }
}

async fn procesar_socket(mut socket: TcpStream) -> io::Result<()>{
    println!("nova conexión!");

    let mut buf = vec![0; 1024];

    // en bucle ler datos do socket e escribir de volta: "eco"
    loop {
        let n = socket
            .read(&mut buf)
            .await?;

        if n == 0 {
            return Err(std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "peer desconectado"))
        }

        socket
            .write_all(&buf[0..n])
            .await?;
    }
}
