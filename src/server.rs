use crate::keys;
use crate::keys::KEYBD_MAPPING;
use crate::KeyCode;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tide::{Request, Response, Server, StatusCode};

#[derive(Deserialize)]
struct Command {
    shortcuts: Vec<String>,
    hotkey_mode: bool,
}

#[derive(Serialize)]
struct Relay<'a> {
    status: u8,
    message: &'a str,
}

pub fn new_server() -> Server<()> {
    let mut app = tide::new();

    app.at("/submit").all(|mut req: Request<()>| async move {
        let command: Command = req.body_json().await?;
        if command.hotkey_mode {
            let keycodes: Vec<KeyCode> = command
                .shortcuts
                .iter()
                .map(|name| *KEYBD_MAPPING.get(name.as_str()).unwrap())
                .collect();
            crate::hot_key(&keycodes, Duration::from_millis(100))
        } else {
            command
                .shortcuts
                .iter()
                .for_each(|name| match KEYBD_MAPPING.get(name.as_str()) {
                    Some(kc) => crate::keypress(*kc),
                    None => println!("unknown key {}", name),
                });
        }

        let r = Relay {
            status: 0,
            message: "success",
        };
        Ok(Response::new(StatusCode::Ok).body_json(&r)?)
    });

    app.at("/").serve_dir("src/templates/").unwrap();

    app
}
