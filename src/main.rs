//! A proxy that forwards data to another server and forwards that server's
//! responses back to clients.
//!
//! Because the Tokio runtime uses a thread pool, each TCP connection is
//! processed concurrently with all other TCP connections across multiple
//! threads.
//!
//! You can showcase this by running this in one terminal:
//!
//!     cargo run --example proxy
//!
//! This in another terminal
//!
//!     cargo run --example echo
//!
//! And finally this in another terminal
//!
//!     cargo run --example connect 127.0.0.1:8081
//!
//! This final terminal will connect to our proxy, which will in turn connect to
//! the echo server, and you'll be able to see data flowing between them.

#![warn(rust_2018_idioms)]

extern crate num;

#[macro_use]
extern crate num_derive;

mod memcached_codec;
mod protocol;

use futures::{SinkExt, StreamExt};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};

use crate::memcached_codec::{MemcachedBinaryCodec, MemcachedBinaryCodecError};
use crate::protocol::memcached_binary::{Magic, PacketHeader};
use bytes::Bytes;
use futures::future::{try_join, IntoFuture};
use futures::FutureExt;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listen_addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server_addr = env::args()
        .nth(2)
        .unwrap_or_else(|| "10.236.107.20:11220".to_string());

    println!("Listening on: {}", listen_addr);
    println!("Proxying to: {}", server_addr);

    let mut listener = TcpListener::bind(listen_addr).await?;

    while let Ok((inbound, _)) = listener.accept().await {
        let transfer = transfer(inbound, server_addr.clone()).map(|r| {
            if let Err(e) = r {
                println!("Failed to transfer; error={}", e);
            }
        });

        tokio::spawn(transfer);
    }

    Ok(())
}

async fn transfer(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut transport: Framed<TcpStream, MemcachedBinaryCodec> =
        Framed::new(inbound, MemcachedBinaryCodec::new());

    while let Some(result) =
        transport.next().await as Option<Result<PacketHeader, MemcachedBinaryCodecError>>
    {
        match result {
            Ok(requestt) => {
                let mut request = requestt.clone();
                request.magic = 0x81;
                request.extras_length = 0;
                request.total_body_length = 0;
                request.key_length = 0;
                request.vbucket_id_or_status = 0;
                transport.send(request).await.unwrap();
            }
            Err(e) => {
                println!("error on decoding from socket; error = {:?}", e);
            }
        }
    }
    //let mut outbound = TcpStream::connect(proxy_addr).await?;

    //let (mut ri, mut wi) = inbound.split();
    //let (mut ro, mut wo) = outbound.split();

    //let client_to_server = io::copy(&mut ri, &mut wo);
    //let server_to_client = io::copy(&mut ro, &mut wi);

    //try_join(client_to_server, server_to_client).await?;

    Ok(())
}
