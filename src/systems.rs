use bevy::prelude::*;

use crate::{
    prelude::{UpdateStatus, Updater},
    resources::AutoUpdateState,
};

pub(crate) fn update_status<U: Updater>(
    updater: Res<AutoUpdateState<U>>,
    mut status: ResMut<UpdateStatus>,
) {
    *status = updater.updater.status();
}
