use std::net::SocketAddr;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct BlueMessage {
    pub id: u8,
    pub kind: u16,
    pub content: [u8; 4],
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct RedMessage {
    pub key: [u8; 2],
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct GreenMessage {
    pub foo: [u8; 2],
    pub bar: [u8; 7],
}

pub async fn send_message<T: serde::Serialize>(dest: &SocketAddr, msg: T) {
    use tokio::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let buf = postcard::to_allocvec(&msg).unwrap();
    socket.send_to(&buf, dest).await.unwrap();
}
