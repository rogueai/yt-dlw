use anyhow::Result;
use apalis::prelude::*;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub video_url: String,
}

impl Job for Video {
    const NAME: &'static str = "yt_dlw::Video";
}
pub async fn download(job: Video, _ctx: JobContext) {
    log::info!("Attempting to download url");
    // pyo3::prepare_freethreaded_python();
    let _: Result<()> = Python::with_gil(|py| {
        // PY: from yt_dlp import YoutubeDL
        let mod_yt_dlp = PyModule::import(py, "yt_dlw")?;
        // PY: self._ydl = YoutubeDL()
        let _ = mod_yt_dlp.getattr("download")?.call1((vec![job.video_url],))?;

        Ok(())
    });
}

pub async fn info() {
    log::info!("Requesting info");
}

#[derive(Debug)]
pub enum YtDlpError {
    NoStorage,
    SomeError(&'static str),
}

impl std::fmt::Display for YtDlpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
