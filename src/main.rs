use std::process::Command; 
use std::io::{self, ErrorKind};
use std::collections::HashMap;

mod tcp;


/// Run a shell command. Return an error if it fails in any way.
fn cmd(cmd: &str, args: &[&str]) -> io::Result<()> {
    let mut child = Command::new(cmd)
        .args(args)
        .spawn()
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to execute {}: {}", cmd, e)))?;

    let ecode = child.wait()
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to wait on {}: {}", cmd, e)))?;

    if !ecode.success() {
        return Err(io::Error::new(
            ErrorKind::Other,
            format!("Command {} failed with exit status: {}", cmd, ecode),
        ));
    };

    Ok(())
}

// these are hexadecimal
// for convinience reasons here the hexadecimal format is a plain string
// without the x
//ipv4 and ipv6
const ETH_SUPPORTED_PROTOCOLS: &[&str] = &["0800", "86DD"];

const IP_SUPPORTED_PROTOCOLS: &[&str] = &["UDP", "TCP"];



fn main() -> Result<(), io::Error >{
    let mut connections: HashMap<u128, tcp::TCB> = Default::default();
    //packet types received can be found here: https://docs.kernel.org/networking/tuntap.html
    let iface = tun_tap::Iface::new("Itun", tun_tap::Mode::Tun).expect("Failed to create interface");
    eprintln!("IFACE: {:?}", iface);
    let name = iface.name();
    eprintln!("NAME: {:?}", name);

    cmd(
        "sudo", 
        &["ip", "addr", "add", "dev", iface.name(), "192.168.1.136/24"]
    )?;

    cmd(
        "sudo", 
        &["ip", "link", "set", "up", "dev", iface.name()]
    )?;

    loop {
        // Flags 2 bytes 
        // Proto 2 bytes 
        // raw protocol 
        // allocate 1500 + 4 bytes for the header
        let mut buffer = vec![0; 1504];
        let eth_nbytes = iface.recv(&mut buffer).unwrap();
        // network interface receives on big endian
        let eth_flags = u16::from_be_bytes([buffer[0], buffer[1]]);
        let eth_proto = u16::from_be_bytes([buffer[2], buffer[3]]);
        //  buffer too small, header only data
        assert!(eth_nbytes >= 4);
        eprintln!("Flags: {:x?}", eth_flags);
        eprintln!("Proto: {:x?}", eth_proto);
        eprintln!("Len of bytes: {:?}", eth_nbytes);
        eprintln!("Bytes/packet received: {:x?}", &buffer[4..eth_nbytes]);

        let proto_bytes = eth_proto.to_be_bytes();
        let proto_hex_encoded = hex::encode(proto_bytes);
        let proto_slice = proto_hex_encoded.as_str();
        
        //println!("{:?}", proto_slice);

        if !ETH_SUPPORTED_PROTOCOLS.contains(&proto_slice){
            eprintln!("Unsupported ETH protocol {:x?}. Continuing...", eth_proto);
            continue; 
        }

        match proto_slice {
            "0800" => {
                let ipv4_packet = match etherparse::Ipv4Header::from_slice(&buffer[4..eth_nbytes]) {
                    Ok(ipv4_packet) => ipv4_packet,
                    Err(e) => {
                        eprintln!("Error: {:?} while parsing ETHERNET packet. Ignoring...", e);
                        continue;
                    }
                };

                let destination_address = ipv4_packet.0.destination;
                let source_address = ipv4_packet.0.source;
                let protocol = ipv4_packet.0.protocol;
                let total_len = ipv4_packet.0.total_len;

                eprintln!("Source: {:?} -> Destination: {:?} with protocol: {:?} and payload length: {:?}", source_address, destination_address, protocol, total_len);

                let protocol_keyword = match protocol.keyword_str() {
                    Some(keyword) => keyword,
                    None => {
                        eprintln!("Failed to get protocol keyword string. Continuing...");
                        continue;
                    }
                };

                if !IP_SUPPORTED_PROTOCOLS.contains(&protocol_keyword) {
                    eprintln!("Unsupported IP protocol {}. Continuing...", protocol_keyword);
                    continue;
                }

                let ipv4_payload = ipv4_packet.1;

                match protocol_keyword {
                    "TCP" => {

                        let tcp_packet = match etherparse::TcpHeader::from_slice(ipv4_payload) {
                            Ok(tcp_packet) => { 
                                eprintln!("Successfully parsed TCP packet.");
                                tcp_packet 
                            }
                            Err(e) => {
                                eprintln!("Error while parsing TCP packet. Ignoring...");
                                continue; 
                            }

                        };

                        let source_port = tcp_packet.0.source_port;
                        let destination_port = tcp_packet.0.destination_port;
                        eprintln!("Source port: {:?} -> Destination Port: {:?}. Received TCP packet", source_port, destination_port);

                        let tcb_identifier = tcp::IdentifyingTCB::new(
                            destination_address,
                            source_address,
                            destination_port,
                            source_port,
                        );

                        connections.entry(tcb_identifier.pack_tcb()).or_default().on_packet(ipv4_packet, tcp_packet);

                    }
                    "UDP" => {
                        // Handle UDP case
                        todo!()
                    }
                    _ => {
                        eprintln!("Unsupported protocol. Continuing...");
                    }
                }
            }
            "86DD" => {
                // Handle IPv6 case
                todo!()
            }
            _ => {
                // Handle other cases
                todo!()
            }
        }   
    }
}

