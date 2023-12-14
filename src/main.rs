use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;
use wayland_client::{protocol::wl_registry, Connection, Dispatch, QueueHandle};

#[derive(Serialize)]
struct Global {
    interface: String,
    version: u32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Registry {
    generation_timestamp: u64,
    globals: Vec<Global>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for Registry {
    fn event(
        state: &mut Self,
        _: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        if let wl_registry::Event::Global {
            interface, version, ..
        } = event
        {
            state.globals.push(Global { interface, version });
        }
    }
}

fn main() {
    let conn = Connection::connect_to_env().unwrap();
    let display = conn.display();

    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();

    let _registry = display.get_registry(&qh, ());

    let generation_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let mut registry = Registry {
        generation_timestamp,
        globals: vec![],
    };
    event_queue.roundtrip(&mut registry).unwrap();

    println!("{}", serde_json::to_string_pretty(&registry).unwrap());
}
