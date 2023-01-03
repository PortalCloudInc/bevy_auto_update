pub mod plugin;
pub mod resources;
pub(crate) mod systems;
pub mod updater;
pub mod updater_dry_run;
pub mod updater_github;

pub mod prelude {
    pub use crate::{
        plugin::AutoUpdatePlugin, resources::UpdateStatus, updater::Updater, updater_github::GitHub,
    };
}
