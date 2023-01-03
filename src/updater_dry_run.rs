use std::sync::{Once, RwLock};

use semver::{Version, VersionReq};

use crate::prelude::{UpdateStatus, Updater};

static STATUS: RwLock<UpdateStatus> = RwLock::new(UpdateStatus::CheckingForUpdate);

/// Pretends there is always an update and fakes the process.
///
/// Useful for testing your update splash screen.
#[derive(Default, Clone)]
pub struct DryRun {
    pub is_fake_update_available: bool,
}

impl Updater for DryRun {
    fn start(&self, _current_version: Version, _requirement: VersionReq) {
        static THREAD: Once = Once::new();

        if !self.is_fake_update_available {
            *STATUS.write().unwrap() = UpdateStatus::UpToDate;
            return;
        }

        THREAD.call_once(|| {
            std::thread::spawn(|| {
                *STATUS.write().unwrap() = UpdateStatus::CheckingForUpdate;
                std::thread::sleep(std::time::Duration::from_secs_f32(2.0));

                for i in 0..=100 {
                    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
                    *STATUS.write().unwrap() = UpdateStatus::Downloading(i as f32);
                }

                for i in 0..=100 {
                    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
                    *STATUS.write().unwrap() = UpdateStatus::Installing(i as f32);
                }

                *STATUS.write().unwrap() = UpdateStatus::FinishedInstalling;
            });
        });
    }

    fn status(&self) -> UpdateStatus {
        *STATUS.read().unwrap()
    }
}
