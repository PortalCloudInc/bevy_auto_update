use std::{str::FromStr, sync::RwLock};

use bevy::prelude::*;
use restson::Response;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::prelude::{UpdateStatus, Updater};

static STATUS: RwLock<UpdateStatus> = RwLock::new(UpdateStatus::CheckingForUpdate);

struct GitHubParams {
    username: String,
    repo: String,
}

/// Does not require the game to be open source, but does require a repo with public releases.
#[derive(Clone)]
pub struct GitHub {
    username: String,
    repo_name: String,
}

impl Default for GitHub {
    fn default() -> Self {
        Self {
            username: String::new(),
            repo_name: String::new(),
        }
    }
}

impl GitHub {
    fn __start(&self, current_version: Version, requirement: VersionReq) -> Option<UpdateStatus> {
        let client = restson::RestClient::builder()
            .blocking("https://api.github.com")
            .unwrap();
        let params = GitHubParams {
            username: self.username.clone(),
            repo: self.repo_name.clone(),
        };
        let response = client.get(&params).ok()?;

        let mut releases: Vec<GitHubRelease> = (response as Response<Vec<GitHubReleaseRaw>>)
            .into_inner()
            .into_iter()
            // No drafts or pre-releases
            .filter(|r: &GitHubReleaseRaw| r.prerelease == false)
            .filter(|r: &GitHubReleaseRaw| r.draft == false)
            // Convert to GitHubRelease
            .filter_map(|r: GitHubReleaseRaw| r.try_into().ok())
            // Must be newer than current version
            .filter(|r: &GitHubRelease| r.version > current_version)
            // Must match version requirement
            .filter(|r: &GitHubRelease| requirement.matches(&r.version))
            .collect();

        releases.sort_by(|a, b| b.version.cmp(&a.version));

        let latest_release = releases
            .into_iter()
            .filter(|release| {
                release
                    .assets
                    .iter()
                    .any(|asset| asset.name == "manifest.json")
            })
            .next()?;

        info!("Found release: {}", latest_release.name);

        Some(UpdateStatus::Downloading(0.0))
    }
}

impl Updater for GitHub {
    fn start(&self, current_version: Version, requirement: VersionReq) {
        match self.__start(current_version, requirement) {
            Some(status) => *STATUS.write().unwrap() = status,
            None => *STATUS.write().unwrap() = UpdateStatus::UpToDate,
        }
    }

    fn status(&self) -> UpdateStatus {
        *STATUS.read().unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GitHubReleaseRaw {
    pub url: String,
    pub tag_name: String,
    pub assets: Vec<Asset>,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
}

struct GitHubRelease {
    pub name: String,
    pub version: Version,
    pub assets: Vec<Asset>,
}

impl TryFrom<GitHubReleaseRaw> for GitHubRelease {
    type Error = <Version as FromStr>::Err;
    fn try_from(release: GitHubReleaseRaw) -> Result<Self, Self::Error> {
        Ok(Self {
            name: release.name,
            version: release.tag_name.parse()?,
            assets: release.assets,
        })
    }
}

// for getting the latest one
impl<'a> restson::RestPath<&'a GitHubParams> for GitHubReleaseRaw {
    fn get_path(repo: &'a GitHubParams) -> Result<String, restson::Error> {
        Ok(format!(
            "repos/{}/{}/releases/latest",
            repo.username, repo.repo
        ))
    }
}

// for getting all of them
impl<'a> restson::RestPath<&'a GitHubParams> for Vec<GitHubReleaseRaw> {
    fn get_path(repo: &'a GitHubParams) -> Result<String, restson::Error> {
        Ok(format!("repos/{}/{}/releases", repo.username, repo.repo))
    }
}
