use std::{
    env, process,
    time::{SystemTime, UNIX_EPOCH},
};

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
    let mut args = env::args().skip(1);
    let mut unique = false;
    if let Some(arg) = args.next() {
        if args.next().is_some() {
            eprintln!("Too many arguments");
            process::exit(1);
        }
        match arg.as_str() {
            "--unique" => unique = true,
            "--help" => {
                eprintln!(
                    "Options:\n\t--unique\tKeep only one global per interface (highest version)"
                );
                process::exit(0);
            }
            _ => {
                eprintln!("Unknown argument: {arg}");
                process::exit(1);
            }
        }
    }

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

    registry.globals.sort_by(|g1, g2| {
        g1.interface
            .cmp(&g2.interface)
            .then(g2.version.cmp(&g1.version))
    });

    if unique {
        let mut prev = String::new();
        registry.globals.retain(|Global { interface, .. }| {
            if interface == &prev {
                false
            } else {
                prev = interface.clone();
                true
            }
        });
    }

    println!("{}", serde_json::to_string_pretty(&registry).unwrap());
}
