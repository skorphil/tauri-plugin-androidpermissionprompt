use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
pub mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Androidpermissionprompt;
#[cfg(mobile)]
use mobile::Androidpermissionprompt;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the androidpermissionprompt APIs.
pub trait AndroidpermissionpromptExt<R: Runtime> {
  fn androidpermissionprompt(&self) -> &Androidpermissionprompt<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AndroidpermissionpromptExt<R> for T {
  fn androidpermissionprompt(&self) -> &Androidpermissionprompt<R> {
    self.state::<Androidpermissionprompt<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("androidpermissionprompt")
    .invoke_handler(tauri::generate_handler![
        commands::ping,
        #[cfg(mobile)]
        commands::check_permissions,
        #[cfg(mobile)]
        commands::request_permissions,
    ])
    .setup(|app, api| {
      #[cfg(mobile)]
      let androidpermissionprompt = mobile::init(app, api)?;
      #[cfg(desktop)]
      let androidpermissionprompt = desktop::init(app, api)?;
      app.manage(androidpermissionprompt);
      Ok(())
    })
    .build()
}
