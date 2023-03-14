struct IpV4 {
    host: String,
    addr: String,
}

struct IpV6 {
    host: String,
    addr: (u8, u8, u8),
}

enum IpAddr {
    V4(IpV4),
    V6(IpV6),
}

fn main() {
    let v4 = IpAddr::V4(IpV4 {
        host: String::from("www.baidu.com"),
        addr: String::from("194.56.23.45"),
    });

    if let IpAddr::V4(IpV4 { host, addr }) = v4 {
        println!("v4 host:{}, addr:{}", host, addr);
    }

    let v6 = IpAddr::V6(IpV6 {
        host: String::from("www.baidu.com"),
        addr: (128, 25, 34),
    });

    if let IpAddr::V6(IpV6 { host, addr }) = v6 {
        println!("v6 host:{}, addr:{},{},{}", host, addr.0, addr.1, addr.2);
    }

    // println!("v4 host:{}, addr:{}",&v4.0.host,&v4.0.addr);
    // println!("v6 host:{}, addr:{},{},{}",&v4.v6,&v6.addr.0,&v6.addr.1,&v6.addr.2);
}
