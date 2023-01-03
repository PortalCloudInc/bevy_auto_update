use semver::{Version, VersionReq};

use crate::resources::UpdateStatus;

pub trait Updater: Default + Clone + Send + Sync + 'static {
    fn start(&self, current_version: Version, requirement: VersionReq);

    fn status(&self) -> UpdateStatus;
}
