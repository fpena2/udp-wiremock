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
                let hits = mock
                    .matchers
                    .iter()
                    .filter_map(|m| m.matches(&packet).then_some(m.type_name()));

                if hits.count() == 0 {
                    return Err(VerificationError::InvalidPacketType(format!(
                        "{:?}",
                        packet
                    )));
                }
            }
        }

        Ok(())
    }
}
