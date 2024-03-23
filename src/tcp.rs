
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

    fn wrap_in_ipv4(self, new_tcp_header: etherparse::TcpHeader, ipv4_header: etherparse::Ipv4Header){

        etherparse::Ipv4Header::new(
            ipv4_header.destination, 
            ipv4_header.source,
            64, 
            new_tcp_header.header_len_u16(),
            etherparse::IpTrafficClass::Tcp,
        )
    }

    ///The packets contain the respective header and payload (u8).
    pub fn on_packet(&mut self, ipv4_packet: (etherparse::Ipv4Header, &[u8]), tcp_packet: (etherparse::TcpHeader, &[u8])){
        // follow the state diagram in RFC 793 (https://datatracker.ietf.org/doc/html/rfc793#section-3.3)
        match *self.state {
            TcpState::Closed => return, 
            TcpState::Listen => {
                if !tcp_packet.0.syn {
                    //got unexpected syn 
                    //*self.state = TcpState::Closed; 
                    return;
                }

                // create a new tcp header 
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcp_packet.0.destination_port, 
                    tcp_packet.0.source_port, 
                    sequence_number: 0, 
                    acknowledgment_number: tcp_packet.0.sequence_number + 1,
                ); 


                syn_ack.syn = true; 
                syn_ack.ack = true;

                //syn_ack.to_bytes()

                new_ipv4_header = wrap_in_ipv4(syn_ack, ipv4_packet.0)
            }
        }
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
        //TcpState::Closed// Default state
        TcpState::Listen
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
