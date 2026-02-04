use super::utils::build_addres;
use crate::repository::user::user_model::{self, User};
use crate::services::email::utils;
use async_smtp::authentication::{Credentials, Mechanism};
use async_smtp::util::get_all_mechanism;
use async_smtp::{Envelope, Message, SendableEmail, SmtpClient, SmtpTransport};
use bytes::BytesMut;
use log::debug;
use std::error::Error as StdErr;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, BufStream};
use tokio::net::TcpStream;
use native_tls::TlsConnector;

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
pub type TransportResult =
    std::result::Result<SmtpTransport<BufStream<TcpStream>>, async_smtp::error::Error>;

async fn create_transport(smtp_client: SmtpClient, tcp_stream: TcpStream) -> TransportResult {
    let stream: BufStream<tokio::net::TcpStream> = BufStream::new(tcp_stream);
    let res: TransportResult = SmtpTransport::new(smtp_client, stream).await;
    {
        if res.is_err() {
            println!("create_transport res is error");
            let unwrap_err: async_smtp::error::Error = unsafe { res.unwrap_err_unchecked() };
            let unwrap_err_fmt: String = format!("{unwrap_err}");
            println!("error msg: {}", unwrap_err_fmt);
            /*
            match unwrap_err {
                async_smtp::error::Error::Timeout(ref x) => {},
                _ => todo!(),
            }
            */
            return Err(unwrap_err);
        }
    }
    res
}

async fn smtp_transport_simple() -> Result<()> {
    let smtp_server: &str = "smtp.mail.ru";
    let smtp_port: i32 = 587;
    let full_addres: String = utils::build_addres(smtp_server, &smtp_port.to_string());
    println!("addres: {}", full_addres);
    let tcp_stream: std::result::Result<TcpStream, std::io::Error> =
        TcpStream::connect(full_addres).await;
    if tcp_stream.is_err() {
        println!("tcp stream connect is error");
        return unsafe { Err(Box::new(tcp_stream.unwrap_err_unchecked())) };
    }
    let tcp_stream_unwrap: TcpStream = unsafe { tcp_stream.unwrap_unchecked() };
    let creds: Credentials = Credentials::new(
        "utishnik@mail.ru".to_owned(),
        "".to_owned(),
    );
    let client: SmtpClient = SmtpClient::new();

    let transport: std::result::Result<
        SmtpTransport<BufStream<TcpStream>>,
        async_smtp::error::Error,
    > = create_transport(client.clone(), tcp_stream_unwrap).await;
    if transport.is_err() {
        println!("error create transport");
        return Err("error create transport".into());
    }
    let unwrap_transport: SmtpTransport<BufStream<TcpStream>> =
        unsafe { transport.unwrap_unchecked() };
    let start_tls: std::result::Result<BufStream<TcpStream>, async_smtp::error::Error> =
        unwrap_transport.starttls().await;
    if start_tls.is_err() {
        println!("error start_tls!");
        return Err("error auth".into());
    }
    let unwrap_tls_result: BufStream<TcpStream> = unsafe { start_tls.unwrap_unchecked() };
    let tls_tcp_stream: TcpStream = unwrap_tls_result.into_inner();
    let tls_connector: TlsConnector = TlsConnector::builder()
        .min_protocol_version(Some(Protocol::Tlsv12))
        .build()?;

    let transport: std::result::Result<
        SmtpTransport<BufStream<TcpStream>>,
        async_smtp::error::Error,
    > = create_transport(client.clone(), unwrap_tls_result.into_inner()).await;
    let mut unwrap_transport: SmtpTransport<BufStream<TcpStream>> =
        unsafe { transport.unwrap_unchecked() };
    println!("create tls transport");////

    let auth_res: std::result::Result<(), async_smtp::error::Error> = unwrap_transport
        .try_login(&creds, &get_all_mechanism())
        .await;
    if auth_res.is_err() {
        println!("error auth!");
        return Err("error auth".into());
    }

    let email: SendableEmail = SendableEmail::new(
        Envelope::new(
            Some("utishnik@mail.ru".parse().unwrap()),
            vec!["utishnik@mail.ru".parse().unwrap()],
        )?,
        "Hello world",
    );
    let send_res: std::result::Result<async_smtp::response::Response, async_smtp::error::Error> =
        unwrap_transport.send(email).await;
    if send_res.is_err() {
        println!("send error!");
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
fn test1() -> Result<()> {
    let rt: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let res: std::result::Result<(), Box<dyn StdErr + Send + Sync>> =
            smtp_transport_simple().await;
        if res.is_err() {
            println!("ошибка!!!");
        }
    });
    Ok(())
}
