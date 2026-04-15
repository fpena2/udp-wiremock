use std::net::SocketAddr;
use tokio::net::UdpSocket;
use udp_wiremock::{MockServer, MockTest};

fn main() {}

#[derive(serde::Serialize, serde::Deserialize)]
struct MyMessage {
    foo: u8,
    bar: u64,
}

async fn send_one_my_message(dest: &SocketAddr) {
    let message = MyMessage { foo: 8, bar: 3 };
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let buf = postcard::to_allocvec(&message).unwrap();
    socket.send_to(&buf, dest).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn example_test() {
        let server = MockServer::start().await;

        // The mock server will expect exactly one `MyMessage` to be sent to `server.address()`
        // before the mock is dropped.
        MockTest::matching::<MyMessage>()
            .named("test_my_message")
            .expect(1)
            .mount(&server)
            .await;

        // This is the code that will be tested.
        send_one_my_message(server.address()).await;

        // The test should pass because the code under test will send one `MyMessage`
        // and that will satisfy our expectations.
    }
}
