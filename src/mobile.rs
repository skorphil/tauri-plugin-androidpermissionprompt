use serde::de::DeserializeOwned;
use tauri::{
  plugin::{PluginApi, PluginHandle},
  AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_androidpermissionprompt);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
  _app: &AppHandle<R>,
  api: PluginApi<R, C>,
) -> crate::Result<Androidpermissionprompt<R>> {
  #[cfg(target_os = "android")]
  let handle = api.register_android_plugin("com.plugin.androidpermissionprompt", "ExamplePlugin")?;
  #[cfg(target_os = "ios")]
  let handle = api.register_ios_plugin(init_plugin_androidpermissionprompt)?;
  Ok(Androidpermissionprompt(handle))
}

/// Access to the androidpermissionprompt APIs.
pub struct Androidpermissionprompt<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Androidpermissionprompt<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    self
      .0
      .run_mobile_plugin("ping", payload)
      .map_err(Into::into)
  }
  pub fn check_permissions(&self) -> crate::Result<PermissionStatus> {
    self.0
        .run_mobile_plugin("checkPermissions", ())
        .map_err(Into::into)
}

    pub fn request_permissions(
        &self,
        permissions: Option<Vec<PermissionType>>,
    ) -> crate::Result<PermissionStatus> {
        self.0
            .run_mobile_plugin(
                "requestPermissions",
                serde_json::json!({ "permissions": permissions }), // use of undeclared crate or module `serde_json`
            )
            .map_err(Into::into)
    }
}
