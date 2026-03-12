use crate::helpers::{GreenMessage, send_message};
use udp_wiremock::{MockServer, MockTest};

/// This struct and `GreenMessage` will serialize to identical bytes
///
/// They are indistinguishable on the wire
#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct YellowMessage {
    pub baz: [u8; 7],
    pub qux: [u8; 2],
}

/// Since `Yellow` and `Green` messages have identical field types/sizes,
/// both deserialize successfully as either type.
///
/// `udp-wiremock` will not able able to tell them apart
#[tokio::test(flavor = "multi_thread")]
async fn limitation_of_udp_wiremock_will_make_this_passfix_this() {
    // Arrange
    let server = MockServer::start().await;
    MockTest::matching::<GreenMessage>()
        .named("verification_passes_if_at_least_one_expectation_is_satisfied")
        .expect(1)
        .mount(&server)
        .await;

    // Act
    send_message(
        server.address(),
        YellowMessage {
            baz: [1, 2, 3, 4, 5, 6, 7],
            qux: [1, 2],
        },
    )
    .await;

    // Assert - test "will" pass because expectation was satisfied by a limitation of this crate
}
