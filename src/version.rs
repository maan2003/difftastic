use std::fmt;

use lazy_static::lazy_static;

pub struct CommitInfo {
    pub short_commit_hash: &'static str,
    pub commit_date: &'static str,
}

pub struct VersionInfo {
    pub version: &'static str,
    pub commit_info: Option<CommitInfo>,
    pub rustc_version: Option<&'static str>,
}

impl fmt::Display for VersionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.version)?;

        match (&self.commit_info, self.rustc_version) {
            (Some(commit_info), Some(rustc_version)) => write!(
                f,
                " ({} {}, built with rustc {})",
                commit_info.short_commit_hash, commit_info.commit_date, rustc_version
            )?,
            (Some(commit_info), None) => write!(
                f,
                " ({} {})",
                commit_info.short_commit_hash, commit_info.commit_date
            )?,
            (None, Some(rustc_version)) => write!(f, " (built with rustc {})", rustc_version)?,
            (None, None) => {}
        }

        Ok(())
    }
}

lazy_static! {
    pub static ref VERSION: String = version().to_string();
}

pub const fn version() -> VersionInfo {
    let version = env!("CARGO_PKG_VERSION");
    let commit_info = match (
        option_env!("DFT_COMMIT_SHORT_HASH"),
        option_env!("DFT_COMMIT_DATE"),
    ) {
        (Some(short_commit_hash), Some(commit_date)) => Some(CommitInfo {
            short_commit_hash,
            commit_date,
        }),
        _ => None,
    };

    let rustc_version = option_env!("DFT_RUSTC_VERSION");

    VersionInfo {
        version,
        commit_info,
        rustc_version,
    }
}
