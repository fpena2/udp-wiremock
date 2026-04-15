use crate::{
    MockServer,
    matcher::{DeserializableMatcher, PacketMatcher},
};
use serde::de::DeserializeOwned;

pub struct MockTest {
    pub name: Option<String>,
    pub matchers: Vec<Box<dyn PacketMatcher>>,
    pub n_expect_packets: usize,
}

impl MockTest {
    pub fn matching<T: DeserializeOwned + Send + Sync + 'static>() -> Self {
        Self {
            name: None,
            matchers: vec![Box::new(DeserializableMatcher::<T>::new())],
            n_expect_packets: 1,
        }
    }

    pub fn or<T: DeserializeOwned + Send + Sync + 'static>(mut self) -> Self {
        // FIXME: warn if T has the same struct as any other T in here
        self.matchers
            .push(Box::new(DeserializableMatcher::<T>::new()));
        self
    }

    pub fn named<N: Into<String>>(mut self, mock_name: N) -> Self {
        self.name = Some(mock_name.into());
        self
    }

    pub fn expect(mut self, count: usize) -> Self {
        self.n_expect_packets = count;
        self
    }

    pub async fn mount(self, server: &MockServer) {
        server.register(self).await;
    }
}
