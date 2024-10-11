use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use zbus::zvariant::{DeserializeDict, Type};
use zbus::{connection, interface, Connection, DBusError};

#[tokio::main(flavor = "current_thread")]
async fn main() -> zbus::Result<()> {
    let _server = start_server(Arc::new(Mutex::new(HashMap::<u32, Notification>::new()))).await?;
    loop {
    }
}

struct Notifications {
    notification_map: Arc<Mutex<HashMap<u32, Notification>>>,
}

#[interface(name = "org.freedesktop.Notifications")]
impl Notifications {
    async fn get_capabilities(&self) -> Box<[String]> {
        Box::new([
            "action-icons".to_string(), "actions".to_string(),
            "body".to_string(), "body-hyperlinks".to_string(), "body-markup".to_string(),
            "icon-multi".to_string(), "icon-static".to_string(),
            "persistence".to_string(), "sound".to_string()
        ])
    }

    async fn get_server_information(&self) -> (String, String, String, String) {
        (
            "fnc".to_string(),
            "femmeromantic".to_string(),
            "0.1.0".to_string(),
            "1.2".to_string(),
        )
    }

    async fn notify(&mut self, _notification: Notification) -> u32 { 1 }

    async fn close_notification(&mut self, _id: u32) -> Result<(), Error> { Ok(()) }

    async fn notification_closed(&mut self, _id: u32, _reason: u32) {}

    async fn action_invoked(&mut self, _id: u32, _action_key: String) {}

    async fn activation_token(&mut self, _id: u32, _activation_token: u32) {}
}

pub(crate) async fn start_server(
    notification_map: Arc<Mutex<HashMap<u32, Notification>>>,
) -> zbus::Result<Connection> {
    let notification = Notifications {
        notification_map: notification_map.clone(),
    };
    let connection = connection::Builder::session()?
        .name("org.freedesktop.Notifications")?
        .serve_at("/org/freedesktop/Notifications", notification)?
        .build()
        .await?;
    Ok(connection)
}

#[derive(Deserialize, Type, Debug)]
struct Notification {
    app_name: String,
    replaces_id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Box<[String]>,
    hints: Hint,
    expire_timeout: i32,
}

#[derive(DeserializeDict, Type, Debug)]
#[zvariant(signature = "a{sv}")]
#[allow(unused)]
struct Hint {
    name: Option<String>,
    variant: Option<Variant>,
}

#[derive(DeserializeDict, Debug, Type)]
#[zvariant(signature = "v")]
#[allow(unused)]
struct Variant {
    boolean: Option<bool>,
    string: Option<Box<String>>,
    bytes: Option<Box<[u8]>>,
    int32: Option<i32>,
    byte: Option<u8>,
}

#[derive(Debug, DBusError)]
enum Error {
    #[zbus(error)]
    ZBus(zbus::Error),
}