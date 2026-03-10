use serde::de::DeserializeOwned;

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
