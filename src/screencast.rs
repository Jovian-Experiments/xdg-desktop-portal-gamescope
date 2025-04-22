use std::{sync::mpsc, thread, time::Duration};

use ashpd::{
    AppID, PortalError, WindowIdentifierType,
    backend::{
        Result,
        request::RequestImpl,
        screencast::{
            CreateSessionOptions, ScreencastImpl, SelectSourcesOptions, SelectSourcesResponse,
            StartCastOptions,
        },
        session::{CreateSessionResponse, SessionImpl},
    },
    desktop::{
        HandleToken,
        screencast::{CursorMode, SourceType, StreamBuilder, Streams, StreamsBuilder},
    },
    enumflags2::BitFlags,
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
pub struct Screencast {}

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
        _token: HandleToken,
        session_token: HandleToken,
        _app_id: Option<AppID>,
        _options: CreateSessionOptions,
    ) -> Result<CreateSessionResponse> {
        log::info!("ScreenCast session created: {session_token}");
        Ok(CreateSessionResponse::new(session_token))
    }

    async fn select_sources(
        &self,
        _session_token: HandleToken,
        _app_id: Option<AppID>,
        _options: SelectSourcesOptions,
    ) -> Result<SelectSourcesResponse> {
        // TODO: actually select the sources
        Ok(SelectSourcesResponse {})
    }

    async fn start_cast(
        &self,
        session_token: HandleToken,
        _app_id: Option<AppID>,
        _window_identifier: Option<WindowIdentifierType>,
        _options: StartCastOptions,
    ) -> Result<Streams> {
        if let Ok(node_id) = find_pipewire_node("gamescope") {
            log::info!(
                "ScreenCast for session {session_token} starting with pipewire node {node_id}"
            );
            let mut streams = vec![];
            streams.push(
                StreamBuilder::new(node_id)
                    .source_type(SourceType::Monitor)
                    .build(),
            );
            Ok(StreamsBuilder::new(streams).build())
        } else {
            let errormsg = format!("gamescope stream not available");
            log::error!("{}", errormsg.as_str());
            Err(PortalError::Failed(errormsg))
        }
    }
}

#[async_trait]
impl SessionImpl for Screencast {
    async fn session_closed(&self, _session_token: HandleToken) -> Result<()> {
        Ok(())
    }
}
