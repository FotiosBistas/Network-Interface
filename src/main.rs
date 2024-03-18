use std::process::Command; 
use std::io::{self, ErrorKind};

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
    }

    Ok(())
}

//ipv4 and ipv6
const SUPPORTED_PROTOCOLS: &[&str] = &["0800", "86DD"];


fn main() -> Result<(), io::Error >{
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
        let nbytes = iface.recv(&mut buffer).unwrap();
        // network interface receives on big endian
        let flags = u16::from_be_bytes([buffer[0], buffer[1]]);
        let proto = u16::from_be_bytes([buffer[2], buffer[3]]);
        //  buffer too small, header only data
        assert!(nbytes >= 4);
        eprintln!("Flags: {:x?}", flags);
        eprintln!("Proto: {:x?}", proto);
        eprintln!("Len of bytes: {:?}", nbytes);
        eprintln!("Bytes/packet received: {:x?}", &buffer[4..nbytes]);

        let proto_bytes = proto.to_be_bytes();
        let proto_hex_encoded = hex::encode(proto_bytes);
        let proto_slice = proto_hex_encoded.as_str();
        
        //println!("{:?}", proto_slice);

        if !SUPPORTED_PROTOCOLS.contains(&proto_slice){
            eprintln!("Continuing caught unsupported protocol");
            continue; 
        }
        
        match proto_slice {
            "0800" => {
                match etherparse::Ipv4Header::from_slice(&buffer[4..nbytes]){
                    Ok(header) => {
                        eprintln!("Flags: {:x?}", flags);
                        eprintln!("Proto: {:x?}", proto);
                        eprintln!("Len of bytes: {:?}", nbytes);
                        eprintln!("Packet received: {:?}", header);
                    }
                    Err(e) => {eprintln!("Error: {:?} while parsing packet ignoring", e)}
                }
            }
            "86DD" => todo!(),
            _ => todo!()
        }
        
    }
}


