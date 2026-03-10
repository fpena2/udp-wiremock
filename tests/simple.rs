use std::net::SocketAddr;
use udp_wiremock::{MockServer, MockTest};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct BlueMessage {
    id: u8,
    kind: u16,
    content: [u8; 4],
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RedMessage {
    key: [u8; 2],
}

async fn send_udp_packet<T: serde::Serialize>(dest: &SocketAddr, msg: T) {
    use tokio::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let buf = postcard::to_allocvec(&msg).unwrap();
    socket.send_to(&buf, dest).await.unwrap();
}

#[tokio::test]
async fn start_multiple_different_servers() {
    // Arrange
    let mock_server_one = MockServer::start().await;
    let mock_server_two = MockServer::start().await;

    // Act - no-op

    // Assert
    assert!(mock_server_one.address() != mock_server_two.address());
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn panics_if_the_expectation_is_not_satisfied() {
    // Arrange
    let server = MockServer::start().await;
    MockTest::matching::<BlueMessage>()
        .named("panics_if_the_expectation_is_not_satisfied")
        .expect(1)
        .mount(&server)
        .await;

    // Act - intentionally skip sending any messages

    // Assert - test will panic because we expected 1 message but received 0
}

#[tokio::test(flavor = "multi_thread")]
#[should_panic]
async fn panics_if_the_message_does_not_match_expectation() {
    // Arrange
    let server = MockServer::start().await;
    MockTest::matching::<BlueMessage>()
        .named("panics_if_the_message_does_not_match_expectation")
        .expect(1)
        .mount(&server)
        .await;

    // Act - send a RedMessage, but server expects BlueMessage
    send_udp_packet(server.address(), RedMessage { key: [1, 2] }).await;

    // Assert - test will panic because message type does not match expectation
}

#[tokio::test(flavor = "multi_thread")]
async fn verification_passes_if_expectation_is_satisfied() {
    // Arrange
    let server = MockServer::start().await;
    MockTest::matching::<BlueMessage>()
        .named("verification_passes_if_expectation_is_satisfied")
        .expect(1)
        .mount(&server)
        .await;

    // Act - send exactly one BlueMessage as expected
    send_udp_packet(
        server.address(),
        BlueMessage {
            id: 8,
            kind: 3,
            content: [1, 2, 3, 4],
        },
    )
    .await;

    // Assert - test should pass because expectation was satisfied
}
