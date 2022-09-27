use std::net::ToSocketAddrs;
//use std::{net::TcpStream, os::windows::prelude::AsRawSocket};

// use mac_address::get_mac_address;
pub fn gen_uuid(ip: &str, mac: &str) -> String {
    let ip_mac = format!("{}{}", ip, mac);
    let digest = md5::compute(ip_mac.as_bytes());
    return format!("{:x}", digest);
}

// pub fn get_mac()  {
//     // let interfaces = datalink::interfaces();
//     // dbg!(interfaces);
//     match get_mac_address() {
//         Ok(Some(ma)) => {
//             println!("M1AC addr = {}", ma);
//             println!("bytes = {:?}", ma.bytes());
//         }
//         Ok(None) => println!("No MAC address found."),
//         Err(e) => println!("{:?}", e),
//     }
// }

// // fn get_conn_local_ip<A: ToSocketAddrs>(addr: A) -> Option<> {
// //     let mut stream = TcpStream::connect(addr).map_or(default, f);
// //     let local_addr = stream.local_addr().unwrap().ip();
// // }

// #[cfg(test)]
// mod tests {
//     use std::{net::TcpStream, os::windows::prelude::AsRawSocket};

//     use super::get_mac;

//     #[test]
//     fn test_get_mac() {
//         get_mac();
//         let mut stream = TcpStream::connect("39.156.69.79:80").unwrap();
//         let local_addr = stream.local_addr().unwrap().ip();
//         dbg!(local_addr);
//     }
// }
