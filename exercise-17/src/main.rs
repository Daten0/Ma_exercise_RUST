// My Code
fn main() {
    let traffic_packets = [
        "IP: 192.168.1.50  | ROUTE: /api/v1/data  | REQ_COUNT: 04",
        "IP: 10.0.0.12     | ROUTE: /api/v1/admin | REQ_COUNT: 15",
        "IP: 172.16.254.1  | ROUTE: /api/v1/users | REQ_COUNT: 02",
        "IP: 192.168.1.99  | ROUTE: /api/v1/admin | REQ_COUNT: 07",
    ];

    let mut admin_route_attempts = 0;

    for packet in traffic_packets {

        println!("[LOG] {}", extract_and_format_ip(packet));

        if is_admin_route(packet) {
            if packet.contains("REQ_COUNT: 15") > packet.contains("REQ_COUNT: 10") {
                println!("[SECURITY ALERT] Admin access requested! Status: [BLOCK] (Rate Limit Exceeded)");
            } else {
                println!("[SECURITY ALERT] Admin access requested! Status: [ALLOW] (Within Normal Limits)");
            }
            admin_route_attempts += 1;
        }
        
        
    }

    println!("Total Admin Route Access Requests Monitored: {}", admin_route_attempts);

    

    
}

fn is_admin_route(packet: &str) -> bool {
    if packet.contains("/api/v1/admin") {
        true
    } else {
        false
    }
}

fn extract_and_format_ip(packet: &str) -> String {

    let packets = packet[3..=18].trim();

    let mut ip_format = packets.to_string();

    ip_format.insert_str(0, "CLIENT_IP: ");

    ip_format

}