use std::{
    collections::HashMap,
    ffi::OsStr,
    path::{Path, PathBuf},
};

use crate::io;

pub const REF_STORE_FILENAME: &str = "refstore.ron";
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct RefStore {
    pub source_folders: Vec<SourceFolder>,
    #[serde(skip)]
    /// Deserialized images.
    /// Keys are `Self::references`
    pub ref_data: HashMap<PathBuf, image::RgbaImage>,
}
impl RefStore {
    pub fn reference_count(&self, sfw_filter: bool) -> usize {
        self.source_folders
            .iter()
            .filter(|source| if sfw_filter { source.is_sfw } else { true })
            .map(|source| source.children.len())
            .sum()
    }
    pub fn get_reference(&self, index: usize, sfw_filter: bool) -> Option<&Reference> {
        let mut index = index;
        for source in &self.source_folders {
            if sfw_filter && !source.is_sfw {
                continue;
            }
            if index < source.children.len() {
                return Some(&source.children[index]);
            }
            index = index - source.children.len();
        }
        None
    }
    pub fn sync_with_source_folders(&mut self) {
        for source in &mut self.source_folders {
            source.children = recursive_get_imgs(&source.path, source.is_sfw).collect();
        }
    }
    pub fn try_load() -> Option<RefStore> {
        io::try_load(REF_STORE_FILENAME)
    }
    pub fn save_to_disk(&self) -> Option<()> {
        io::save_to_disk(self, REF_STORE_FILENAME)
    }
    pub fn push_folders(&mut self, folders: &[impl AsRef<Path>], is_sfw: bool) {
        for folder in folders {
            self.push_folder(folder.as_ref(), is_sfw);
        }
        self.save_to_disk();
    }
    fn push_folder(&mut self, folder_to_add: &Path, is_sfw: bool) {
        let mut source = SourceFolder {
            path: folder_to_add.to_path_buf(),
            is_sfw,
            children: Vec::new(),
        };
        if !self.source_folders.contains(&source) {
            source.children = recursive_get_imgs(&source.path, false).collect();
            self.source_folders.push(source);
            tracing::info!("Pushed source folder {folder_to_add:?} successfully ");
        }
    }
}

#[derive(Eq, Hash, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Reference {
    pub path: PathBuf,
}
impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}
fn recursive_get_imgs(
    source_path: &Path,
    follow_root_links: bool,
) -> impl Iterator<Item = Reference> {
    walkdir::WalkDir::new(source_path)
        .follow_root_links(follow_root_links)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file())
        .filter(|p| {
            p.extension()
                .map(|ext| ext.to_ascii_lowercase())
                .map(|ext| {
                    ext == OsStr::new("jpg")
                        || ext == OsStr::new("png")
                        || ext == OsStr::new("webp")
                })
                .unwrap_or_default()
        })
        .map(move |image_path| Reference { path: image_path })
}
#[derive(Eq, Hash, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SourceFolder {
    pub path: PathBuf,
    pub is_sfw: bool,
    #[serde(skip)]
    pub children: Vec<Reference>,
}
impl PartialEq for SourceFolder {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}
