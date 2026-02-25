use crate::{Mock, VerificationError};
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tokio::net::UdpSocket;

pub struct Packet {
    pub body: Vec<u8>,
}

#[derive(Default)]
struct UdpMockServerState {
    mock: Option<Mock>,
    received_packets: Vec<Packet>,
}

pub struct UdpMockServer {
    state: Arc<RwLock<UdpMockServerState>>,
    /// The address that the OS has assigned to the server
    server_address: SocketAddr,
    /// A way to shutdown the server when UdpMockServer goes out of scope
    _shutdown_trigger: tokio::sync::watch::Sender<()>,
}

impl UdpMockServer {
    /// Start a new instance of a `MockServer` listening on a random port.
    pub async fn start() -> Self {
        let (shutdown_trigger, shutdown_receiver) = tokio::sync::watch::channel(());

        let state = Arc::new(RwLock::new(UdpMockServerState::default()));
        let server_state = state.clone();
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .expect("Failed to bind to an address");
        let server_address = socket.local_addr().expect("Failed to get server address.");

        // We could send and receive a response from the server to determine when it has been initialized.
        // This will allow us to return from this function and be certain that our server is ready for work.
        // However, this would compliacte things.
        let (ready_tx, ready_rx) = tokio::sync::oneshot::channel();

        std::thread::spawn(move || {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Cannot build local tokio runtime");

            runtime.block_on(run_server(
                socket,
                server_state,
                shutdown_receiver,
                ready_tx,
            ));
        });

        // Wait for server to be ready
        ready_rx.await.ok();

        Self {
            state,
            server_address,
            _shutdown_trigger: shutdown_trigger,
        }
    }

    pub fn address(&self) -> &SocketAddr {
        &self.server_address
    }

    pub fn register(&self, mock: Mock) {
        self.state.write().unwrap().mock = Some(mock);
    }

    pub fn verify(&self) -> Result<(), VerificationError> {
        let state = self.state.read().unwrap();
        if let Some(mock) = &state.mock {
            mock.verify(&state.received_packets)?;
        }
        Ok(())
    }
}

impl Drop for UdpMockServer {
    // Clean up when the `UdpMockServer` instance goes out of scope.
    fn drop(&mut self) {
        self.verify().expect("msg")
        // The sender half of the channel, `shutdown_trigger`, gets dropped here
        // Triggering the graceful shutdown of the server itself.
    }
}

async fn run_server(
    socket: UdpSocket,
    server_state: Arc<RwLock<UdpMockServerState>>,
    mut shutdown_signal: tokio::sync::watch::Receiver<()>,
    ready_tx: tokio::sync::oneshot::Sender<()>,
) {
    log::info!("server is running");
    ready_tx.send(()).ok();

    let mut buf = [0u8; 2048];
    loop {
        tokio::select! {
            _ = shutdown_signal.changed() => {
                log::info!("Mock server shutting down");
                break;
            },
            res = socket.recv_from(&mut buf) => {
                match res {
                    Ok((len, _addr)) => {
                        let mut state = server_state.write().unwrap();
                        state.received_packets.push(Packet { body: buf[..len].to_vec() });
                    },
                    Err(_e) => {
                        // This will trigger whenever we try to shutdown the server
                    }
                }
            }
        };
    }
}
