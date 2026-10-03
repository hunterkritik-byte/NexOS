#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TcpState { Closed, Listen, SynReceived, Established, FinWait }

pub struct TcpConnection {
    pub local_port: u16,
    pub remote_port: u16,
    pub state: TcpState,
}

impl TcpConnection {
    pub const fn new(local_port: u16) -> Self {
        Self { local_port, remote_port: 0, state: TcpState::Closed }
    }
}
