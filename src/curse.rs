use std::path::{Path, PathBuf};

#[cfg(feature="serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature="serde")]
use std::fs::read_to_string;
#[cfg(feature="serde")]
use serde_json::Error;

// Modpacks

#[cfg_attr(feature="serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
#[derive(Debug, Clone)]
pub struct CursePackFiles {
    project_id: usize,
    file_id: usize,
}

impl CursePackFiles {
    pub fn get_project_id(&self) -> usize {
        self.project_id
    }

    pub fn get_file_id(&self) -> usize {
        self.file_id
    }
}

#[cfg_attr(feature="serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct CursePack {
    pub name: String,
    pub author: String,
    files: Vec<CursePackFiles>,
}

impl CursePack {
    pub fn get_files(&self) -> &Vec<CursePackFiles> {
        &self.files
    }
}

#[cfg(feature="serde")]
fn deserializ_pack(path: &str) -> Result<CursePack, Error> {
    let aux = read_to_string(path).unwrap();
    serde_json::from_str(&aux)
}

#[cfg(feature="serde")]
pub fn load_curse_pack(pack_path: &str) -> Option<CursePack> {
    match read_to_string(pack_path) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("Error reading the pack \n\n{error}");
            return None;
        }
    };

    match deserializ_pack(pack_path) {
        Ok(e) => Some(e),
        Err(error) => {
            eprintln!("Error deserializing the pack \n\n{error}");
            None
        }
    }
}

// Mods

#[cfg_attr(feature="serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
/// This struct only contains data about the mod logo.
pub struct Logo {
    pub id: usize,
    pub mod_id: usize,
    pub thumbnail_url: String,
    pub url: String,
}

#[cfg_attr(feature="serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
/// This struct contains the data about the specific file of a mod
pub struct CurseFile {
    pub id: usize,
    pub game_id: Option<usize>,
    pub mod_id: usize,
    pub display_name: String,
    pub file_name: PathBuf,
    pub download_url: Option<String>,
    pub file_length: usize,
    pub game_versions: Vec<String>,
}

impl CurseFile {
    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_game_id(&self) -> usize {
        self.game_id
            .unwrap_or_default()
    }

    pub fn get_mod_id(&self) -> usize {
        self.mod_id
    }

    pub fn get_game_versions(&self) -> &[String] {
        &self.game_versions
    }

    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }

    pub fn get_file_name(&self) -> &Path {
        &self.file_name
    }

    pub fn get_download_url(&self) -> &str {
        self.download_url
            .as_ref()
            .map_or("", |s| s)
    }
}

#[cfg_attr(feature="serde", derive(Deserialize, Serialize))]
#[derive(Debug, Clone)]
pub struct FingerPrintInfo {
    pub id: usize,
    pub file: CurseFile,
}

/// This struct contains the data about the request of a fingerprint
/// requests are like
/// ```json
/// "data": {
///     exactMatches: [
///         CurseFile
///     ]
/// }
/// ```
#[cfg_attr(feature="serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
pub struct CurseFingerPrint {
    exact_matches: Vec<FingerPrintInfo>,
}

impl CurseFingerPrint {
    pub fn get_file(&self) -> &CurseFile {
        &self.exact_matches[0].file
    }
}

/// This struct contains the data about a single version of a mod
#[cfg_attr(feature="serde", derive(Deserialize, Serialize), serde(rename_all = "camelCase"))]
#[derive(Clone, Debug)]
pub struct CurseVersion {
    pub id: usize,
    pub game_id: usize,
    pub name: String,
    pub slug: String,
    pub download_count: usize,
    pub latest_files: Vec<CurseFile>,
}

/// This struct contains the data about the multiple versions of a mod
#[cfg_attr(feature="serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug)]
pub struct CurseVersions {
    pub data: Vec<CurseVersion>,
}

/// Because the standard response from Curse API is:
/// "data": {
///     * fields of other struct *
/// }
/// We need this struct.
#[cfg(feature="serde")]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurseResponse<T: Serialize> {
    pub data: T,
}
