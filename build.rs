use std::path::PathBuf;

fn main() {
    if !en_data_available() {
        #[cfg(feature = "download-data")]
        download::en_download_data();

        #[cfg(not(feature = "download-data"))]
        panic!(
            "Necessary data for language 'en' not found. You can manually add it (see README on GitHub), or enable the 'download-data' feature to automatically download it."
        );
    }
}

/// Check if all artifacts for 'en' are available. If at least one is missing, this returns false.
fn en_data_available() -> bool {
    let data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/en/data");
    let data_files = ["us_gold.json", "us_silver.json", "model.fst"];

    data_files
        .iter()
        .all(|&fname| data_path.join(fname).exists())
}

#[cfg(feature = "download-data")]
mod download {
    use anyhow::{Context, Result};
    use std::path::PathBuf;
    use uuid::Uuid;

    /// Download artifacts for en from GitHub releases
    pub fn en_download_data() {
        let download_url =
            "https://github.com/lastleon/phonemoro/releases/download/v0.1.0/release.zip";
        let data_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/en/data");

        // Download and unzip to temporary directory
        let tmp_dir = std::env::temp_dir().join(Uuid::new_v4().to_string());

        let downloaded_file =
            download(download_url).expect("Downloading data from GitHub release page failed.");

        zip_extract::extract(std::io::Cursor::new(downloaded_file), &tmp_dir, false)
            .expect("Unzipping release.zip failed.");

        // Move files to correct location
        let tmp_en_data_dir = tmp_dir.join("en");
        for entry in tmp_en_data_dir
            .read_dir()
            .expect("Reading the contents of the unzipped directory failed.")
        {
            let fpath = entry.expect("Reading a in the unzipped directory failed.");
            std::fs::rename(fpath.path(), data_path.join(fpath.file_name()))
                .expect("Moving file from unzipped directory to data directory failed.");
        }
    }

    /// Download file from url to memory
    fn download<S: AsRef<str>>(url: S) -> Result<Vec<u8>> {
        let mut resp = ureq::get(url.as_ref())
            .call()
            .with_context(|| "Sending download request failed.")?;

        if resp.status() != ureq::http::StatusCode::OK {
            anyhow::bail!("Request failed with status: {}", resp.status());
        }

        // Note: Limit raised to 50 MiB (only ~21 MB should be necessary)
        Ok(resp
            .body_mut()
            .with_config()
            .limit(50 * 1024 * 1024)
            .read_to_vec()?)
    }
}
