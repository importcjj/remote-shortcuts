use std::net::IpAddr;

pub fn get_lan_ip() -> Option<IpAddr> {
    get_if_addrs::get_if_addrs().ok().and_then(|addrs| {
        addrs
            .iter()
            .filter(|i| !i.is_loopback())
            .next()
            .map(|i| i.ip())
    })
}
