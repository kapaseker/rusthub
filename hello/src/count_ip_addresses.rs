pub fn ips_between(start: &str, end: &str) -> u32 {
    ip_count(end) - ip_count(start)
}

pub fn ip_count(ip: &str) -> u32 {
    ip.split('.').enumerate().map(|(i, s)| s.to_string().parse::<u32>().unwrap() * 256u32.pow(3 - i as u32)).sum()
}

mod test {
    use crate::count_ip_addresses::ip_count;

    #[test]
    fn case_1() {
        println!("{}",ip_count("10.0.0.0"));
        println!("{}",ip_count("10.0.1.0"));
    }
}