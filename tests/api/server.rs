use udp_wiremock::MockServer;

#[tokio::test]
async fn start_multiple_different_servers() {
    // Arrange
    let mock_server_one = MockServer::start().await;
    let mock_server_two = MockServer::start().await;

    // Act - no-op

    // Assert
    assert!(mock_server_one.address() != mock_server_two.address());
}
