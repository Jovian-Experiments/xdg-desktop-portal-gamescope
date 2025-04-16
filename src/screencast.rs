use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
    time::Duration,
};

use ashpd::{
    AppID, PortalError, WindowIdentifierType,
    backend::{
        Result,
        request::RequestImpl,
        screencast::{
            CreateSessionOptions, ScreencastImpl, SelectSourcesOptions, SelectSourcesResponse,
            StartCastOptions,
        },
        session::CreateSessionResponse,
    },
    desktop::{
        HandleToken,
        screencast::{CursorMode, SourceType, Stream, Streams as StartCastResponse},
    },
    enumflags2::BitFlags,
    zbus::zvariant::OwnedObjectPath,
};
use async_trait::async_trait;
use pipewire::{context::Context, main_loop::MainLoop};

struct Terminate;

fn find_pipewire_node_in_thread(
    sender: mpsc::Sender<u32>,
    receiver: pipewire::channel::Receiver<Terminate>,
    node_name: String,
) {
    pipewire::init();
    let mainloop = MainLoop::new(None).expect("Failed to create pipewire main loop");

    let _receiver = receiver.attach(mainloop.loop_(), {
        let mainloop = mainloop.clone();
        move |_| mainloop.quit()
    });

    let context = Context::new(&mainloop).expect("Failed to create pipewire context");
    let core = context
        .connect(None)
        .expect("Failed to connect to pipewire context");
    let registry = core
        .get_registry()
        .expect("Failed to get pipewire registry");

    let name = String::from(node_name);
    let _listener = registry
        .add_listener_local()
        .global(move |global| {
            if let Some(props) = global.props {
                if let Some(node_name) = props.get("node.name") {
                    if node_name == name {
                        let _ = sender.send(global.id);
                    }
                }
            }
        })
        .register();

    mainloop.run();
}

fn find_pipewire_node(node_name: &str) -> std::result::Result<u32, mpsc::RecvTimeoutError> {
    let (main_sender, main_receiver) = mpsc::channel();
    let (pw_sender, pw_receiver) = pipewire::channel::channel();

    let name = String::from(node_name);
    let pw_thread =
        thread::spawn(move || find_pipewire_node_in_thread(main_sender, pw_receiver, name));
    let result = main_receiver.recv_timeout(Duration::from_secs(1));
    let _ = pw_sender.send(Terminate);
    let _ = pw_thread.join();
    result
}

#[derive(Default)]
pub struct Screencast {
    sessions: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl RequestImpl for Screencast {
    async fn close(&self, _token: HandleToken) {}
}

#[async_trait]
impl ScreencastImpl for Screencast {
    fn available_source_types(&self) -> BitFlags<SourceType> {
        SourceType::Monitor | SourceType::Window
    }

    fn available_cursor_mode(&self) -> BitFlags<CursorMode> {
        CursorMode::Hidden | CursorMode::Embedded | CursorMode::Metadata
    }

    async fn create_session(
        &self,
        _handle: OwnedObjectPath,
        session_handle: OwnedObjectPath,
        _app_id: Option<AppID>,
        _options: CreateSessionOptions,
    ) -> Result<CreateSessionResponse> {
        let session = session_handle.to_string();
        let mut sessions = self.sessions.lock().unwrap();
        if sessions.contains(&session) {
            let errormsg = format!("A session with handle `{session}` already exists");
            log::error!("{}", errormsg.as_str());
            return Err(PortalError::Exist(errormsg));
        }
        sessions.push(session.clone());
        log::info!("ScreenCast session created: {session}");
        Ok(CreateSessionResponse::new(session))
    }

    async fn session_closed(&self, session_handle: OwnedObjectPath) -> Result<()> {
        let session = session_handle.to_string();
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(index) = sessions.iter().position(|x| *x == session) {
            sessions.swap_remove(index);
            log::info!("ScreenCast session closed: {session}");
            Ok(())
        } else {
            let errormsg = format!("Unknown session: `{session}`");
            log::error!("{}", errormsg.as_str());
            Err(PortalError::NotFound(errormsg))
        }
    }

    async fn select_sources(
        &self,
        session_handle: OwnedObjectPath,
        _app_id: Option<AppID>,
        _options: SelectSourcesOptions,
    ) -> Result<SelectSourcesResponse> {
        let session = session_handle.to_string();
        let sessions = self.sessions.lock().unwrap();
        if !sessions.contains(&session) {
            let errormsg = format!("Unknown session: `{session}`");
            log::error!("{}", errormsg.as_str());
            return Err(PortalError::NotFound(errormsg));
        }
        // TODO: actually select the sources
        Ok(SelectSourcesResponse {})
    }

    async fn start_cast(
        &self,
        session_handle: OwnedObjectPath,
        _app_id: Option<AppID>,
        _window_identifier: Option<WindowIdentifierType>,
        _options: StartCastOptions,
    ) -> Result<StartCastResponse> {
        let session = session_handle.to_string();
        let sessions = self.sessions.lock().unwrap();
        if !sessions.contains(&session) {
            let errormsg = format!("Unknown session: `{session}`");
            log::error!("{}", errormsg.as_str());
            return Err(PortalError::NotFound(errormsg));
        }
        if let Ok(node_id) = find_pipewire_node("gamescope") {
            log::info!("ScreenCast starting with pipewire node {node_id}");
            let mut streams = vec![];
            streams.push(Stream::new(
                node_id,
                None,
                None,
                Some(SourceType::Monitor),
                None,
            ));
            Ok(StartCastResponse::new(streams, None))
        } else {
            let errormsg = format!("gamescope stream not available");
            log::error!("{}", errormsg.as_str());
            Err(PortalError::Failed(errormsg))
        }
    }
}
