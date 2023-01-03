pub use bevy::prelude::*;
use semver::{Version, VersionReq};

use crate::{
    prelude::Updater,
    resources::{AutoUpdateState, UpdateStatus},
    systems::update_status,
};

/// A plugin that automatically updates the game to the latest version.
///
/// # Example
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_autoupdate::{AutoUpdatePlugin, GitHub};
///
/// fn main() {
///     App::new()
///         .add_plugin(AutoUpdatePlugin {
///             updater: GitHub {
///                 username: "DenverCoder9".to_string(),
///                 repo_name: "cool_bevy_game".to_string(),
///             },
///             ..Default::default()
///         })
///         .run();
/// ```
pub struct AutoUpdatePlugin<U: Updater> {
    /// Current version of the game
    pub current_version: Version,

    /// Defaults to wildcard
    pub version_requirement: VersionReq,

    /// Must be spicified for something to happen
    pub updater: U,
}

impl<U: Updater> Default for AutoUpdatePlugin<U> {
    fn default() -> Self {
        Self {
            current_version: Version::new(0, 0, 0),
            version_requirement: VersionReq::STAR,
            updater: U::default(),
        }
    }
}

impl<U: Updater> Plugin for AutoUpdatePlugin<U> {
    fn build(&self, app: &mut App) {
        app.insert_resource(UpdateStatus::CheckingForUpdate);
        let updater = self.updater.clone();
        updater.start(
            self.current_version.clone(),
            self.version_requirement.clone(),
        );
        app.insert_resource(AutoUpdateState { updater })
            .add_system(update_status::<U>);
    }
}
