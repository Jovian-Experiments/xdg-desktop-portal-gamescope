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

use crate::gamescope_pipewire::get_gamescope_pipewire_node_id;

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
        session_token: HandleToken,
        _app_id: Option<AppID>,
        options: SelectSourcesOptions,
    ) -> Result<SelectSourcesResponse> {
        // TODO: actually select the sources
        log::info!(
            "ScreenCast sources selection for session {session_token}: {:?}",
            options
        );
        Ok(SelectSourcesResponse {})
    }

    async fn start_cast(
        &self,
        session_token: HandleToken,
        _app_id: Option<AppID>,
        _window_identifier: Option<WindowIdentifierType>,
        _options: StartCastOptions,
    ) -> Result<Streams> {
        match get_gamescope_pipewire_node_id() {
            Ok(node_id) => {
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
            }
            Err(msg) => {
                let errormsg = format!("gamescope stream not available: {msg}");
                log::error!("{}", errormsg.as_str());
                Err(PortalError::Failed(errormsg))
            }
        }
    }
}

#[async_trait]
impl SessionImpl for Screencast {
    async fn session_closed(&self, session_token: HandleToken) -> Result<()> {
        log::info!("ScreenCast session {session_token} closed");
        Ok(())
    }
}
