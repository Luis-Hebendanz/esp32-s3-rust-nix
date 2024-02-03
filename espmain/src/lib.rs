pub fn mac_to_string(mac: &[u8]) -> String {
    let mut mac_str = String::new();
    for i in 0..mac.len() {
        mac_str.push_str(&format!("{:02x}", mac[i]));
        if i < mac.len() - 1 {
            mac_str.push(':');
        }
    }
    mac_str
}
