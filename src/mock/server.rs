use crate::{MockTest, UdpMockServerState, start_udp_server};
use async_drop::{AsyncDrop, AsyncDropFuture, Dropper};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{net::UdpSocket, sync::RwLock};

pub struct MockServer {
    ///
    state: Arc<RwLock<UdpMockServerState>>,
    /// The address that the OS has assigned to the server
    address: SocketAddr,
    /// A way to shutdown the server when UdpMockServer goes out of scope
    _shutdown_trigger: tokio::sync::watch::Sender<()>,
}

impl MockServer {
    /// Start a new instance of a `MockServer`.
    ///
    /// This server will listen on a random port.
    pub async fn start() -> Dropper<Self> {
        let (shutdown_trigger, shutdown_receiver) = tokio::sync::watch::channel(());
        let (ready_trigger, ready_receiver) = tokio::sync::oneshot::channel();

        let state = Arc::new(RwLock::new(UdpMockServerState::default()));
        let (socket, address) = create_socket_and_address().await;

        start_udp_server(socket, state.clone(), shutdown_receiver, ready_trigger);

        // Wait for server to be ready
        ready_receiver.await.expect("Server could not start up");

        Dropper::new(Self {
            state,
            address,
            _shutdown_trigger: shutdown_trigger,
        })
    }

    /// Gets the address that the server is running on
    pub fn address(&self) -> &SocketAddr {
        &self.address
    }

    /// Registers a test for this server to perform
    pub(crate) async fn register(&self, mock: MockTest) {
        self.state.write().await.mock = Some(mock);
    }
}

///
async fn create_socket_and_address() -> (UdpSocket, SocketAddr) {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .await
        .expect("Failed to bind to an address");
    let address = socket.local_addr().expect("Failed to get server address.");
    (socket, address)
}

/// Verifies the packages received are able to pass the tests that have been registered
impl AsyncDrop for MockServer {
    fn async_drop(&mut self) -> AsyncDropFuture<'_> {
        Box::pin(async {
            tokio::time::sleep(Duration::from_secs(1)).await;

            let state = self.state.read().await;
            let results = state.verify();
            if let Err(e) = results {
                panic!("Verification failed: {}", e)
            }

            Ok(())
        })
    }
}
