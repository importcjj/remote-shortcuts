use qrcode::QrCode;
use std::iter;
use std::net::IpAddr;
use qrcode::render::unicode;

pub fn get_lan_ip() -> Option<IpAddr> {
    get_if_addrs::get_if_addrs().ok().and_then(|addrs| {
        addrs
            .iter()
            .filter(|i| !i.is_loopback())
            .next()
            .map(|i| i.ip())
    })
}

pub fn print_qr(text: &str) {
    let code = QrCode::new(text).unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", image);
}
