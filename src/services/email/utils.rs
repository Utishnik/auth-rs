#[doc = "создаем из host и порта полный адресс"]
pub fn build_addres(host: &str,port: &str) -> String{
    let result: String = format!("{host}::{port}");
    result
}