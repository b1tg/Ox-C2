pub fn gen_uuid(ip: &str, mac: &str) -> String {
    let ip_mac = format!("{}{}", ip, mac);
    let digest = md5::compute(ip_mac.as_bytes());
    return format!("{:x}", digest);
}
