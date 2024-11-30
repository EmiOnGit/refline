use std::fs;

use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{error, warn};

pub fn try_load<T: DeserializeOwned>(file_name: &str) -> Option<T> {
    let project_dirs = directories_next::ProjectDirs::from("", "", "refline")?;
    let mut ref_store_path = project_dirs.data_dir().to_path_buf();
    ref_store_path.push(file_name);
    let file = match fs::File::open(&ref_store_path) {
        Ok(file) => file,
        Err(e) => {
            warn!(
                "Can not find storage file for references at {ref_store_path:?} with error {e:?}"
            );
            return None;
        }
    };
    let des: T = match ron::de::from_reader(file) {
        Ok(res) => res,
        Err(e) => {
            error!("Could not deserialize storage file for references at {ref_store_path:?} with error {e:?}");
            return None;
        }
    };
    Some(des)
}
/// Returns None if saving was not successfully
/// Some(()) if the ref store was saved.
pub fn save_to_disk<T: Serialize>(t: &T, path: &str) -> Option<()> {
    let project_dirs = directories_next::ProjectDirs::from("", "", "refline")?;
    let mut ref_store_path = project_dirs.data_dir().to_path_buf();
    ref_store_path.push(path);

    let file = fs::File::create(ref_store_path).ok()?;
    to_writer_pretty(file, t, PrettyConfig::new()).ok()?;
    tracing::info!("saved ref store to disk");
    Some(())
}
