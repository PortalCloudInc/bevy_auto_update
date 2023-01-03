use bevy::prelude::*;

use crate::prelude::Updater;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub enum UpdateStatus {
    /// The updater is checking for an update
    CheckingForUpdate,

    /// There is no need to update
    UpToDate,

    /// The update is being downloaded
    Downloading(f32),

    /// The update is being installed
    Installing(f32),

    /// The update has finished installing
    ///
    /// Note: It is a very bad idea to run the game after this state.
    /// Better to restart the game as assets are likely to be out of sync with the code.
    FinishedInstalling,
}

#[derive(Resource)]
pub(crate) struct AutoUpdateState<U: Updater> {
    pub updater: U,
}
