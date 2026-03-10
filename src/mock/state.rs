use crate::{MockTest, VerificationError};

type Packet = Vec<u8>;

#[derive(Default)]
pub(crate) struct UdpMockServerState {
    pub mock: Option<MockTest>,
    pub received_packets: Vec<Packet>,
}

impl UdpMockServerState {
    pub(crate) fn verify(&self) -> Result<(), VerificationError> {
        if let Some(mock) = &self.mock {
            if self.received_packets.len() != mock.n_expect_packets {
                return Err(VerificationError::InvalidPacketsReceived {
                    expected: mock.n_expect_packets,
                    found: self.received_packets.len(),
                });
            }

            for packet in &self.received_packets {
                if let Some(failing_match) = mock.matchers.iter().find(|m| !m.matches(&packet)) {
                    return Err(VerificationError::InvalidPacketType(
                        std::any::type_name_of_val(failing_match).to_string(),
                    ));
                }
            }
        }

        Ok(())
    }
}
