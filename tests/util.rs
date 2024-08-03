use std::path::{Path, PathBuf};

const TMP_BASE_DIR: &str = "/tmp";

pub struct TempCacheDir(PathBuf);

impl TempCacheDir {
    pub fn new() -> anyhow::Result<Self> {
        let id = uuid::Uuid::now_v7().to_string();
        let path = PathBuf::from(TMP_BASE_DIR).join(id);
        std::fs::create_dir(&path)?;
        Ok(Self(path))
    }
}

impl AsRef<Path> for TempCacheDir {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl Drop for TempCacheDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0.join("cxd.cache"));
        std::fs::remove_dir(&self.0).expect("Unique cache dir not present");
    }
}
