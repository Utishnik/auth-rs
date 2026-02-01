use crate::repository::user::user_model::{self, User};
use async_smtp::{Envelope, Message, SendableEmail, SmtpClient, SmtpTransport};
use async_smtp::authentication::Credentials;
use bytes::BytesMut;
use log::debug;
use std::error::Error as StdErr;
use tokio::io::{AsyncReadExt, BufStream};
use tokio::net::TcpStream;

impl User {
    fn get_email(&self) -> Option<String> {
        self.email.clone()
    }
    fn email_is_some(&self) -> bool {
        let get_email: Option<String> = self.get_email();
        if get_email.is_some() {
            return true;
        }
        false
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
pub type TransportErr =
    std::result::Result<SmtpTransport<BufStream<TcpStream>>, async_smtp::error::Error>;

async fn create_transport(smtp_client: SmtpClient, tcp_stream: TcpStream) -> TransportErr {
    let stream: BufStream<tokio::net::TcpStream> = BufStream::new(tcp_stream);
    let res: TransportErr = SmtpTransport::new(smtp_client, stream).await;
    {
        if res.is_err() {
            panic!("create_transport res is error");
        }
    }
    res
}

async fn smtp_transport_simple() -> Result<()> {
    let tcp_stream: TcpStream = TcpStream::connect("127.0.0.1:2525").await?;
    let client: SmtpClient = SmtpClient::new();
    let transport: std::result::Result<
        SmtpTransport<BufStream<TcpStream>>,
        async_smtp::error::Error,
    > = create_transport(client, tcp_stream).await;
    if transport.is_err() {
        debug!("error create transport");
        return Err("error create transport".into());
    }
    let mut unwrap_transport: SmtpTransport<BufStream<TcpStream>> =
        unsafe { transport.unwrap_unchecked() };

    let email: SendableEmail = SendableEmail::new(
        Envelope::new(
            Some("user@localhost".parse().unwrap()),
            vec!["root@localhost".parse().unwrap()],
        )?,
        "Hello world",
    );
    let send_res: std::result::Result<async_smtp::response::Response, async_smtp::error::Error> =
        unwrap_transport.send(email).await;
    if send_res.is_err(){
        debug!("send error!");
    }
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
    let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let res: std::result::Result<(), Box<dyn StdErr + Send + Sync>> =
            smtp_transport_simple().await;
        if res.is_err() {
            panic!("ошибка");
        }
    });
}
