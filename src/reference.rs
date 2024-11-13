use std::{
    collections::{HashMap, HashSet},
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use tracing::{error, info, warn};

pub const REF_STORE_FILENAME: &str = "refstore.ron";
pub const REF_FOLDER: &str = "refs";
#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct RefStore {
    pub references: HashSet<Reference>,
    #[serde(skip)]
    pub ref_data: HashMap<PathBuf, image::RgbaImage>,
}
impl RefStore {
    pub fn sync_with_local_filesystem(&mut self) {
        let Some(project_dirs) = directories_next::ProjectDirs::from("com", "refline", "refline")
        else {
            return;
        };
        let mut ref_folder = project_dirs.data_dir().to_path_buf();
        ref_folder.push(REF_FOLDER);

        // Extend the reference store with all new images that were found
        self.references
            .extend(recursive_get_imgs(&ref_folder, true));
    }
    pub fn try_load() -> Option<RefStore> {
        let project_dirs = directories_next::ProjectDirs::from("com", "refline", "refline")?;
        let mut ref_store_path = project_dirs.data_dir().to_path_buf();
        ref_store_path.push(REF_STORE_FILENAME);
        let file = match fs::File::open(&ref_store_path) {
            Ok(file) => file,
            Err(e) => {
                warn!("Can not find storage file for references at {ref_store_path:?} with error {e:?}");
                return None;
            }
        };
        let des: RefStore = match ron::de::from_reader(file) {
            Ok(res) => res,
            Err(e) => {
                error!("Could not deserialize storage file for references at {ref_store_path:?} with error {e:?}");
                return None;
            }
        };
        Some(des)
    }
    pub fn push_folders(&mut self, folders: &[impl AsRef<Path>], is_sfw: bool) {
        let Some(project_path) = directories_next::ProjectDirs::from("com", "refline", "refline")
        else {
            error!("Could not read project directories");
            return;
        };
        let mut ref_dir = project_path.data_dir().to_path_buf();
        ref_dir.push(REF_FOLDER);
        if !ref_dir.exists() {
            if let Err(e) = fs::DirBuilder::new().recursive(true).create(&ref_dir) {
                error!("Tried to create ref folder, but failed with: {e:?}");
            } else {
                info!("Created ref folder");
            }
        }
        for folder in folders {
            self.push_folder(folder.as_ref(), is_sfw, ref_dir.clone());
        }
    }
    fn push_folder(
        &mut self,
        folder_to_add: &Path,
        is_sfw: bool,
        mut ref_line_ref_folder: PathBuf,
    ) {
        info!("Add symbolic link to {folder_to_add:?}");
        if ref_line_ref_folder
            .read_dir()
            .unwrap()
            .flatten()
            .filter_map(|folder| fs::read_link(folder.path()).ok())
            .any(|folder| folder == folder_to_add)
        {
            tracing::warn!(
                "tried to add folder {ref_line_ref_folder:?} but it was already linked. Ignoring request"
            );
            return;
        }
        ref_line_ref_folder.push(folder_to_add.file_name().unwrap());
        while ref_line_ref_folder.exists() {
            let Some(filename) = ref_line_ref_folder.file_name().unwrap().to_str() else {
                tracing::error!(
                    "Could not add folder {folder_to_add:?}, because it doesn't contain valid unicode"
                );
                return;
            };

            let new_filename = filename
                .rsplit_once("_")
                .and_then(|(f, last_part)| last_part.parse::<u32>().ok().map(|l| (f, l)))
                .map(|(f, l)| (f, (l + 1).to_string()))
                .map(|(f, l)| format!("{f}_{l}"))
                .unwrap_or(format!("{filename}_1"));
            ref_line_ref_folder.pop();
            ref_line_ref_folder.push(new_filename);
        }
        if let Err(e) = symlink::symlink_dir(folder_to_add, &ref_line_ref_folder) {
            error!("Failed to add symlink with src: {folder_to_add:?} dst: {ref_line_ref_folder:?} error: {e}");
        };
        warn!("reference folder loaded at {ref_line_ref_folder:?}");
        for mut img in recursive_get_imgs(&folder_to_add, false) {
            img.is_sfw = is_sfw;
            self.references.replace(img);
        }
    }
}

#[derive(Eq, Hash, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Reference {
    pub path: PathBuf,
    pub is_sfw: bool,
}
impl PartialEq for Reference {
    fn eq(&self, other: &Self) -> bool {
        self.path.eq(&other.path)
    }
}
fn recursive_get_imgs(root: &Path, follow_root_links: bool) -> impl Iterator<Item = Reference> {
    walkdir::WalkDir::new(root)
        .follow_root_links(follow_root_links)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.into_path())
        .filter(|p| p.is_file())
        .filter(|p| {
            p.extension()
                .map(|ext| {
                    ext == OsStr::new("jpg")
                        || ext == OsStr::new("png")
                        || ext == OsStr::new("webp")
                })
                .unwrap_or_default()
        })
        .map(|image_path| Reference {
            path: image_path,
            is_sfw: true,
        })
}
