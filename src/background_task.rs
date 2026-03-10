use crate::UdpMockServerState;
use std::sync::Arc;
use tokio::{net::UdpSocket, sync::RwLock};

pub(crate) fn start_udp_server(
    socket: UdpSocket,
    state: Arc<RwLock<UdpMockServerState>>,
    mut shutdown_signal: tokio::sync::watch::Receiver<()>,
    ready_signal: tokio::sync::oneshot::Sender<()>,
) {
    let server_future = async move {
        let mut buf = [0u8; 2048];

        // Signal that the server has started up
        ready_signal.send(()).ok();

        loop {
            tokio::select! {
                _ = shutdown_signal.changed() => {
                    break;
                },
                res = socket.recv_from(&mut buf) => {
                    match res {
                        Ok((len, _addr)) => {
                            let mut state = state.write().await;
                            state.received_packets.push(buf[..len].to_vec());
                        },
                        Err(_e) => {
                            // This will trigger whenever we try
                            // to shutdown the server
                        }
                    }
                }
            };
        }
    };

    tokio::spawn(server_future);
}
