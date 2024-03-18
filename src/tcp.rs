/// transmission control block for TCP
struct TCB {
    // Local and remote addresses and ports
    local_address: [u8; 4],
    remote_address: [u8; 4],
    local_port: u16,
    remote_port: u16,

    // Sequence and acknowledgment numbers
    sequence_number: u32,
    acknowledgment_number: u32,

    // Window size
    window_size: u16,

    // TCP state (e.g., LISTEN, SYN_SENT, ESTABLISHED, etc.)
    state: TcpState,
}

enum TcpState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}
