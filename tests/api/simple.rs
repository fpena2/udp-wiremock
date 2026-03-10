use crate::helpers::{BlueMessage, send_message};
use udp_wiremock::{MockServer, MockTest};

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
    send_message(
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
