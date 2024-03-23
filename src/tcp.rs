
/// transmission control block for TCP
#[derive(Clone, Copy, Debug)]
pub struct TCB {
     
    pub identifier: IdentifyingTCB, 

    // Sequence and acknowledgment numbers
    pub sequence_number: u32,
    pub acknowledgment_number: u32,

    // Window size
    pub window_size: u16,

    // TCP state (e.g., LISTEN, SYN_SENT, ESTABLISHED, etc.)
    pub state: TcpState,
}

impl Default for TCB {

    fn default() -> TCB{
        TCB::new()
    }
}

impl TCB{

    // Default constructor with dummy values
    pub fn new() -> Self {
        Self {
            identifier: IdentifyingTCB::default(),
            sequence_number: 0,
            acknowledgment_number: 0,
            window_size: 0,
            state: TcpState::default(),
        }
    }

    // Constructor with specified values
    pub fn with_values(identifier: IdentifyingTCB, sequence_number: u32, acknowledgment_number: u32, window_size: u16, state: TcpState) -> Self {
        Self {
            identifier,
            sequence_number,
            acknowledgment_number,
            window_size,
            state,
        }
    }

    ///The packets contain the respective header and payload (u8).
    pub fn on_packet(&mut self, ipv4_packet: (etherparse::Ipv4Header, &[u8]), tcp_packet: (etherparse::TcpHeader, &[u8])){

    }
}

/// fields required for identifying a TCB
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IdentifyingTCB{
    // Local and remote addresses and ports
    pub local_address: [u8; 4],
    pub remote_address: [u8; 4],
    pub local_port: u16,
    pub remote_port: u16,
}

impl Default for IdentifyingTCB {
    fn default() -> IdentifyingTCB {
        IdentifyingTCB {
            local_address: [0, 0, 0, 0],  // Dummy IP
            remote_address: [0, 0, 0, 0], // Dummy IP
            local_port: 0,                // Dummy port
            remote_port: 0,               // Dummy port
        }
    }
}

impl IdentifyingTCB {


    pub fn new(local_address: [u8;4], remote_address:[u8;4], local_port:u16, remote_port:u16) -> IdentifyingTCB {
        IdentifyingTCB{local_address: local_address, remote_address: remote_address, local_port: local_port, remote_port: remote_port}
    }

    // thank you https://youtu.be/5rb0vvJ7NCY for the insiration to check this out
    pub fn pack_tcb(&self) -> u128{
        // the TCB struct is 96 bits in size but there is no type for 96bits
        // next one is u128

        //IP addresses are big endian
        let local_addr = u32::from_be_bytes(self.local_address) as u128;
        let remote_addr = u32::from_be_bytes(self.remote_address) as u128;
        let local_port = self.local_port as u128;
        let remote_port = self.remote_port as u128;

        //bitwise or 
        // leave the most significand 32 bits as zeros
        (local_addr << 64) | (remote_addr << 32) | (remote_port << 16) | (local_port)
    }

    pub fn unpack_tcb(packed: u128) -> Self {
        let local_addr = ((packed >> 64) & 0xFFFFFFFF) as u32;
        let remote_addr = ((packed >> 32) & 0xFFFFFFFF) as u32;
        let remote_port = ((packed >> 16) & 0xFFFF) as u16;
        let local_port = (packed & 0xFFFF) as u16;

        IdentifyingTCB {
            local_address: local_addr.to_be_bytes(),
            remote_address: remote_addr.to_be_bytes(),
            local_port: local_port,
            remote_port: remote_port,
        }
    }
}


#[derive(Clone, Copy, Debug)]
pub enum TcpState {
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

impl Default for TcpState {
    fn default() -> TcpState {
        TcpState::Established// Default state
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn pack_unpack_tcb_test() {
       let original_tcb = IdentifyingTCB {
            local_address: [192, 168, 1, 1],
            remote_address: [10, 0, 0, 1],
            local_port: 12345,
            remote_port: 54321,
        };

        let packed = original_tcb.pack_tcb();
        let unpacked_tcb = IdentifyingTCB::unpack_tcb(packed);

        println!("Original TCB: {:?}", original_tcb);
        println!("Unpacked TCB: {:?}", unpacked_tcb);

        assert_eq!(original_tcb, unpacked_tcb);
    }
}
