/*
 * Copyright © 2025 Valve Corporation
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

use ashpd::{
    AppID, PortalError, WindowIdentifierType,
    backend::{
        Result,
        request::RequestImpl,
        screenshot::{ColorOptions, ScreenshotImpl, ScreenshotOptions},
    },
    desktop::{Color, HandleToken, screenshot::Screenshot as ScreenshotResponse},
    zbus::DBusError,
};
use async_trait::async_trait;
use futures_util::StreamExt;
use inotify::{EventMask, Inotify, WatchMask};

#[derive(Default)]
pub struct Screenshot;

#[async_trait]
impl RequestImpl for Screenshot {
    async fn close(&self, _token: HandleToken) {}
}

fn log_error(error: PortalError) -> Result<ScreenshotResponse> {
    log::error!("{}", error.description().unwrap_or(error.name().as_str()));
    Err(error)
}

#[async_trait]
impl ScreenshotImpl for Screenshot {
    async fn screenshot(
        &self,
        token: HandleToken,
        app_id: Option<AppID>,
        _window_identifier: Option<WindowIdentifierType>,
        _options: ScreenshotOptions,
    ) -> Result<ScreenshotResponse> {
        if app_id.is_some() {
            log::info!(
                "Screenshot requested by {} with token {}",
                app_id.unwrap(),
                token
            );
        } else {
            log::info!("Screenshot requested with token {}", token);
        }
        let path = xdg_user::pictures().unwrap_or(None);
        let mut path = match path {
            Some(p) => std::path::PathBuf::from(p),
            None => {
                return log_error(PortalError::Failed(format!(
                    "No XDG pictures directory to save screenshot to"
                )));
            }
        };
        path.push(format!(
            "Screenshot_{}.png",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ));
        let url = match url::Url::from_file_path(path.as_path()) {
            Ok(url) => url,
            _ => {
                return log_error(PortalError::Failed(format!(
                    "Invalid file path: {}",
                    path.display()
                )));
            }
        };
        // gamescope processes the screenshot command asynchronously, so
        // we need to wait for the file to be written to disk
        let inotify = match Inotify::init() {
            Ok(inotify) => {
                if inotify
                    .watches()
                    .add(&path.parent().unwrap(), WatchMask::CLOSE_WRITE)
                    .is_ok()
                {
                    Some(inotify)
                } else {
                    log::warn!("Failed to add inotify file watch");
                    None
                }
            }
            _ => {
                log::warn!("Failed to initialize inotify");
                None
            }
        };

        if std::process::Command::new("gamescopectl")
            .arg("screenshot")
            .arg(path.as_path())
            .status()
            .is_ok_and(|s| s.success())
        {
            match inotify {
                Some(inotify) => {
                    let mut buffer = [0; 1024];
                    match inotify.into_event_stream(&mut buffer) {
                        Ok(mut stream) => {
                            while let Some(event_or_error) = stream.next().await {
                                match event_or_error {
                                    Ok(event) => {
                                        if event.mask == EventMask::CLOSE_WRITE
                                            && event.name
                                                == Some(path.file_name().unwrap().to_os_string())
                                        {
                                            log::info!("Screenshot saved to {}", path.display());
                                            return Ok(ScreenshotResponse::new(url));
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => log::warn!("Failed to stream inotify events"),
                    }
                }
                None => (),
            }
            log::info!("Screenshot requested, pending saving at {}", path.display());
            return Ok(ScreenshotResponse::new(url));
        }
        log_error(PortalError::Failed(format!("Failed to take screenshot")))
    }

    async fn pick_color(
        &self,
        _token: HandleToken,
        _app_id: Option<AppID>,
        _window_identifier: Option<WindowIdentifierType>,
        _options: ColorOptions,
    ) -> Result<Color> {
        Err(PortalError::NotFound(format!(
            "PickColor method is not implemented"
        )))
    }
}
