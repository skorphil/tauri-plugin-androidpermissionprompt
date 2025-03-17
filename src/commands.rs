use tauri::{AppHandle, command, Runtime};

use crate::models::*;
use crate::Result;
use crate::AndroidpermissionpromptExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.androidpermissionprompt().ping(payload)
}

#[cfg(mobile)]
// use tauri::{AppHandle, Runtime};
#[cfg(mobile)]
use crate::models::*;
#[cfg(mobile)]
// use crate::AndroidpermissionpromptExt;
#[cfg(mobile)]
// use crate::Result;

#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub(crate) async fn check_permissions<R: Runtime>(app: AppHandle<R>) -> Result<PermissionStatus> {
    app.androidpermissionprompt().check_permissions()
}

#[cfg(mobile)]
#[tauri::command]
#[specta::specta]
pub(crate) async fn request_permissions<R: Runtime>(
    app: AppHandle<R>,
    permissions: Option<Vec<PermissionType>>,
) -> Result<PermissionStatus> {
    app.androidpermissionprompt().request_permissions(permissions)
}

#[cfg(mobile)]
#[tauri::command]
pub fn request_read_audio_permission() -> bool {
    use crate::GLOBAL_APP_HANDLE; // no `GLOBAL_APP_HANDLE` in the root
    use tauri::plugin::PermissionState;
    use tauri_plugin_androidpermissionprompt::models::PermissionType; // failed to resolve: use of undeclared crate or module `tauri_plugin_androidpermissionprompt
    use tauri_plugin_androidpermissionprompt::YourAppExt; // failed to resolve: use of undeclared crate or module `tauri_plugin_androidpermissionprompt

    let app = GLOBAL_APP_HANDLE
        .get()
        .expect("Failed to get GLOBAL_APP_HANDLE");
    let res = app
        .androidpermissionprompt()
        .check_permissions()
        .expect("Failed to request read audio permission");
    if res.audio != PermissionState::Granted {
        return app
            .androidpermissionprompt()
            .request_permissions(Some(vec![PermissionType::Audio]))
            .unwrap()
            .audio
            == PermissionState::Granted;
    }
    true
}
