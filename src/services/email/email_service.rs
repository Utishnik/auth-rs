use async_smtp::{Envelope, Message, SendableEmail, SmtpClient, SmtpTransport};
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, BufStream};
use tokio::net::TcpStream;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

async fn smtp_transport_simple() -> Result<()> {
    let tcp_stream: TcpStream = TcpStream::connect("127.0.0.1:2525").await?;
    let stream: BufStream<tokio::net::TcpStream> = BufStream::new(tcp_stream);
    let client: SmtpClient = SmtpClient::new();
    let mut transport: SmtpTransport<BufStream<TcpStream>> =
        SmtpTransport::new(client, stream).await?;

    let email = SendableEmail::new(
        Envelope::new(
            Some("user@localhost".parse().unwrap()),
            vec!["root@localhost".parse().unwrap()],
        )?,
        "Hello world",
    );
    transport.send(email).await?;
    /*
    let res: async_smtp::Message = email.message();
    let mut buffer: BytesMut = BytesMut::with_capacity(10);
    let read_b = res.read_buf(&mut buffer);
    println!("msg: {:?}",&buffer[..]);
    */
    Ok(())
}

#[test]
fn test1() {
    smtp_transport_simple();
}
