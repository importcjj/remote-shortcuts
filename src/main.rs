use auto_array::auto_array;
use keys::KeyCode;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use winapi::shared::minwindef::DWORD;
use winapi::um::winuser::keybd_event;

pub mod keys;
mod server;
mod utils;

const KEYEVENTF_KEYDOWN: DWORD = 0x0000;
const KEYEVENTF_KEYUP: DWORD = 0x0002;

fn keydown(kc: KeyCode) {
    unsafe {
        keybd_event(kc, 0, KEYEVENTF_KEYDOWN, 0);
    }
}

fn keyup(kc: KeyCode) {
    unsafe {
        keybd_event(kc, 0, KEYEVENTF_KEYUP, 0);
    }
}

fn keypress(kc: KeyCode) {
    keydown(kc);
    keyup(kc);
}

fn hot_key(kcs: &[KeyCode], interval: Duration) {
    for kc in kcs {
        keydown(*kc);
        sleep(interval);
    }

    for kc in kcs.iter().rev() {
        keyup(*kc)
    }
}

macro_rules! hot_key {
    ($dur:expr, [$first:tt $($rest: tt)*]) => {{
        keydown($first);
        $(
            sleep($dur);
            keydown($rest);
        )*
        hot_key!([$($rest)*] $first);
    }};


    ([$first:tt $($rest: tt)*] $($reversed:tt)+) => {
        hot_key!([$($rest)*] $first $($reversed)+)
    };

    ([] $($rest: tt)*) => {
        $(keyup($rest);)*
    }
}

fn main() -> io::Result<()> {
    let ip = utils::get_lan_ip().expect("LAN ip");
    let port: u16 = 8080;

    let page_addr = format!("http://{}:{}/index.html", ip, port);
    let serve_addr = format!("0.0.0.0:{}", port);
    qr2term::print_qr(page_addr).unwrap();

    let server = server::new_server();
    smol::block_on(async { server.listen(&serve_addr).await })?;

    // // // hot_key!(Duration::from_secs(1), [KEY_WIN KEY_R])
    Ok(())
}
