use crate::helpers::{BlueMessage, RedMessage, send_message};
use udp_wiremock::{MockServer, MockTest};

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
    send_message(server.address(), RedMessage { key: [1, 2] }).await;

    // Assert - test will panic because message type does not match expectation
}
