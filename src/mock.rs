use crate::{Packet, UdpMockServer, VerificationError};
use serde::de::DeserializeOwned;
// use std::num::NonZeroUsize;

pub trait PacketMatcher: Send + Sync {
    fn matches(&self, packet: &[u8]) -> bool;
}

pub struct DeserializableMatcher<T: DeserializeOwned> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned> DeserializableMatcher<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: DeserializeOwned> PacketMatcher for DeserializableMatcher<T>
where
    T: DeserializeOwned + Send + Sync,
{
    fn matches(&self, packet: &[u8]) -> bool {
        postcard::from_bytes::<T>(packet).is_ok()
    }
}

pub struct Mock {
    pub name: Option<String>,
    pub matchers: Vec<Box<dyn PacketMatcher>>,
    pub n_expect_packets: usize,
}

impl Mock {
    pub fn matching<T: DeserializeOwned + Send + Sync + 'static>() -> Self {
        Self {
            name: None,
            matchers: vec![Box::new(DeserializableMatcher::<T>::new())],
            n_expect_packets: 1,
        }
    }

    pub fn named<N: Into<String>>(mut self, mock_name: N) -> Self {
        self.name = Some(mock_name.into());
        self
    }

    pub fn expect(mut self, count: usize) -> Self {
        self.n_expect_packets = count;
        self
    }

    pub fn verify(&self, packets: &[Packet]) -> Result<(), VerificationError> {
        if packets.len() != self.n_expect_packets {
            return Err(VerificationError::InvalidPacketsReceived {
                expected: self.n_expect_packets,
                found: packets.len(),
            });
        }

        for packet in packets {
            for matcher in &self.matchers {
                if !matcher.matches(&packet.body) {
                    return Err(VerificationError::InvalidPacketType(
                        std::any::type_name_of_val(&matcher).to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn mount(self, server: &UdpMockServer) {
        server.register(self);
    }
}
