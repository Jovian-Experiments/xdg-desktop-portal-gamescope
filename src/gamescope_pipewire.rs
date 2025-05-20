use std::{env, os::unix::net::UnixStream, path::PathBuf};
use wayland_client::{ConnectError, Connection, Dispatch, QueueHandle, protocol::wl_registry};

// Generate rust code for the custom gamescope pipewire protocol.
// The protocol definition in data/ was copied from
// https://github.com/ValveSoftware/gamescope/blob/master/protocol/gamescope-pipewire.xml
mod protocol {
    use wayland_client;
    pub mod __interfaces {
        wayland_scanner::generate_interfaces!("./data/gamescope-pipewire.xml");
    }
    use self::__interfaces::*;
    wayland_scanner::generate_client_code!("./data/gamescope-pipewire.xml");
}

use protocol::gamescope_pipewire;

#[derive(Default)]
struct State {
    gamescope_pipewire_global_found: bool,
    node_id: Option<u32>,
}

impl Dispatch<wl_registry::WlRegistry, ()> for State {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<State>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            if interface == "gamescope_pipewire" {
                let _ = registry.bind::<gamescope_pipewire::GamescopePipewire, _, _>(
                    name,
                    version,
                    qh,
                    (),
                );
                state.gamescope_pipewire_global_found = true;
            }
        }
    }
}

impl Dispatch<gamescope_pipewire::GamescopePipewire, ()> for State {
    fn event(
        state: &mut Self,
        _: &gamescope_pipewire::GamescopePipewire,
        event: gamescope_pipewire::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<State>,
    ) {
        let gamescope_pipewire::Event::StreamNode { node_id } = event;
        state.node_id = Some(node_id);
    }
}

fn get_gamescope_wayland_socket() -> Result<UnixStream, ConnectError> {
    let socket_name = env::var_os("GAMESCOPE_WAYLAND_DISPLAY")
        .or(Some("gamescope-0".into())) // default wayland server display name
        .map(Into::<PathBuf>::into)
        .ok_or(ConnectError::NoCompositor)?;
    let mut socket_path = env::var_os("XDG_RUNTIME_DIR")
        .map(Into::<PathBuf>::into)
        .ok_or(ConnectError::NoCompositor)?;
    if !socket_path.is_absolute() {
        return Err(ConnectError::NoCompositor);
    }
    socket_path.push(socket_name);
    UnixStream::connect(socket_path).map_err(|_| ConnectError::NoCompositor)
}

pub(crate) fn get_gamescope_pipewire_node_id() -> Result<u32, String> {
    if let Ok(socket) = get_gamescope_wayland_socket() {
        if let Ok(connection) = Connection::from_socket(socket) {
            let display = connection.display();
            let mut event_queue = connection.new_event_queue();
            let qh = event_queue.handle();
            let _registry = display.get_registry(&qh, ());
            let mut state = State::default();

            // First roundtrip to get all the advertised globals
            // (among which we expect the gamescope pipewire interface)
            if event_queue.roundtrip(&mut state).is_err() {
                return Err(format!("Wayland protocol dispatch error"));
            }
            if !state.gamescope_pipewire_global_found {
                return Err(format!("gamescope pipewire global object not found"));
            }

            // Second roundtrip to process all the events sent by the gamescope pipewire object
            if event_queue.roundtrip(&mut state).is_err() {
                return Err(format!("Wayland protocol dispatch error"));
            }
            return state
                .node_id
                .ok_or(format!("gamescope pipewire node ID not advertised"));
        }
    }
    Err(format!("failed to connect to wayland socket"))
}
