use postcard::to_allocvec;
use std::net::SocketAddr;
use udp_wiremock::{Mock, UdpMockServer};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Message {
    id: u8,
    kind: u16,
    content: [u8; 4],
}

fn send_udp_packet(dest: &SocketAddr, msg: &[u8]) {
    use std::net::UdpSocket;
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.send_to(msg, dest).unwrap();
}

#[tokio::test]
async fn start_multiple_different_servers() {
    // Arrange
    let mock_server_one = UdpMockServer::start().await;
    let mock_server_two = UdpMockServer::start().await;

    // Act
    // no-op

    // Assert
    assert!(mock_server_one.address() != mock_server_two.address());
}

#[tokio::test]
#[should_panic]
async fn panics_if_the_expectation_is_not_satisfied() {
    // Arrange
    let mock_server = UdpMockServer::start().await;
    Mock::matching::<Message>()
        .named("panics_if_the_expectation_is_not_satisfied")
        .expect(1)
        .mount(&mock_server);

    // Act - no input the server

    // Assert
    // UdpMockServer will panic when dropped
}

#[tokio::test]
async fn passes_if_expectation_is_satisfied() {
    // Arrange
    let mock_server = UdpMockServer::start().await;
    Mock::matching::<Message>()
        .named("passes_if_expectation_is_satisfied")
        .expect(1)
        .mount(&mock_server);

    // Act - send input the server
    send_udp_packet(
        mock_server.address(),
        &to_allocvec(&Message {
            id: 8,
            kind: 3,
            content: [1, 2, 3, 4],
        })
        .unwrap(),
    );

    // FIXME: Need to figure out a way to verify after the server is done processing 
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Assert
    // UdpMockServer will not panic since expectation is met
}
