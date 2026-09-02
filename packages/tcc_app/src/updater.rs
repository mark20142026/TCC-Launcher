//! Self-updater.
//!
//! Checks a JSON manifest published with the latest GitHub release, compares
//! versions, downloads the new installer for this platform, verifies its
//! SHA-256 checksum, and runs it. CI regenerates the manifest on every release
//! (see the release job in `.github/workflows/ci.yml`), so shipping a new
//! version needs no code changes.

use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::constants::APP_VERSION;

/// The manifest lives next to the installers as a release asset of the latest
/// release, so its URL is stable across versions. The same file is what
/// `backend/api/latest.js` proxies for the web side.
pub const UPDATE_MANIFEST_URL: &str =
    "https://github.com/pnbx/TCC-Launcher/releases/latest/download/latest.json";

/// Information about an available update.
#[derive(Debug, Clone)]
pub struct UpdateInfo {
    pub version: String,
    pub notes: Option<String>,
    pub url: String,
    pub sha256: Option<String>,
}

/// Fetches the update manifest and returns the pending update, if any.
pub async fn check_for_updates() -> anyhow::Result<Option<UpdateInfo>> {
    let net = tcc_net::RequestClient::new();
    let manifest: Value = net
        .http()
        .get(UPDATE_MANIFEST_URL)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let version = manifest
        .get("version")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("update manifest is missing `version`"))?
        .to_string();

    if !is_newer(&version, APP_VERSION) {
        return Ok(None);
    }

    // Accept both key styles: our GitHub manifest uses polyio's short form
    // ("windows-x64") while the backend `releases` table stores the long
    // form ("windows-x86_64").
    let key = platform_key();
    let platform = manifest
        .get("platforms")
        .and_then(|p| {
            p.get(&key).or_else(|| {
                p.get(&key.replace("-x64", "-x86_64"))
            })
        })
        .ok_or_else(|| anyhow::anyhow!("no update published for platform `{key}`"))?;

    let url = platform
        .get("url")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow::anyhow!("manifest entry is missing `url`"))?
        .to_string();
    let sha256 = platform
        .get("sha256")
        .and_then(Value::as_str)
        .map(str::to_string);

    Ok(Some(UpdateInfo {
        version,
        notes: manifest.get("notes").and_then(Value::as_str).map(str::to_string),
        url,
        sha256,
    }))
}

/// Downloads the update and verifies its checksum.
pub async fn download_update(info: &UpdateInfo) -> anyhow::Result<PathBuf> {
    let net = tcc_net::RequestClient::new();
    let bytes = net
        .http()
        .get(&info.url)
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;

    if let Some(expected) = &info.sha256 {
        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let got: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        if !got.eq_ignore_ascii_case(expected) {
            anyhow::bail!("checksum mismatch: expected {expected}, got {got}");
        }
    }

    let dir = polyio::get_cache_dir().unwrap_or_else(std::env::temp_dir);
    std::fs::create_dir_all(&dir)?;
    let name = info
        .url
        .rsplit('/')
        .next()
        .unwrap_or("tcc-update-installer.exe");
    let path = dir.join(name);
    let mut file = std::fs::File::create(&path)?;
    file.write_all(&bytes)?;
    Ok(path)
}

/// Runs the downloaded installer and exits so it can replace the app files.
pub fn install_update(installer: &PathBuf) -> anyhow::Result<()> {
    Command::new(installer).arg("/S").spawn()?;
    std::process::exit(0);
}

/// Background check that runs on a detached thread at startup. Installs an
/// update when one is available; stays silent otherwise.
pub fn auto_check_background() {
    let Ok(rt) = tokio::runtime::Runtime::new() else {
        return;
    };
    let Ok(Some(info)) = rt.block_on(check_for_updates()) else {
        return;
    };
    let _ = rt.block_on(apply_update(&info));
}

/// Manual check from Settings: always talks back via a message dialog.
pub fn check_and_install_interactive() {
    let Ok(rt) = tokio::runtime::Runtime::new() else {
        info_dialog("Could not create an async runtime for the update check.");
        return;
    };

    match rt.block_on(check_for_updates()) {
        Ok(Some(info)) => {
            if let Err(e) = rt.block_on(apply_update(&info)) {
                info_dialog(&format!("Update to {} failed:\n{e:#}", info.version));
            }
        }
        Ok(None) => info_dialog(&format!(
            "You are already on the latest version ({APP_VERSION})."
        )),
        Err(e) => info_dialog(&format!("Update check failed:\n{e:#}")),
    }
}

async fn apply_update(info: &UpdateInfo) -> anyhow::Result<()> {
    info_dialog(&format!(
        "Downloading update to {}...\nThe launcher will restart when it finishes.",
        info.version
    ));
    let path = download_update(info).await?;
    install_update(&path)
}

fn info_dialog(text: &str) {
    rfd::MessageDialog::new()
        .set_title("TCC Launcher")
        .set_level(rfd::MessageLevel::Info)
        .set_buttons(rfd::MessageButtons::Ok)
        .set_description(text)
        .show();
}

fn platform_key() -> String {
    format!("{}-{}", polyio::get_os(), polyio::get_arch())
}

fn version_tuple(version: &str) -> (u64, u64, u64) {
    let core = version.trim().trim_start_matches('v');
    let core = core.split(['-', '+']).next().unwrap_or("");
    let mut parts = core.split('.');
    let mut next = || parts.next().and_then(|p| p.parse::<u64>().ok()).unwrap_or(0);
    (next(), next(), next())
}

fn is_newer(candidate: &str, current: &str) -> bool {
    version_tuple(candidate) > version_tuple(current)
}
