use anyhow::Result;
use apalis::prelude::*;
use futures_util::FutureExt;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyString};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub video_url: String,
}

#[pyclass]
struct Callback {
    // callback_function is called from Python
    #[allow(dead_code)]
    callback_fn: Box<dyn Fn(Python<'_>, &PyDict) -> PyResult<()> + Send>,
}

#[pymethods]
impl Callback {
    fn __call__(&self, py: Python<'_>, py_dict: &PyDict) -> PyResult<()> {
        (self.callback_fn)(py, py_dict)
    }
}

fn make_callback() -> Box<Callback> {
    Box::new(Callback {
        callback_fn: Box::new(move |py, py_dict| progress_hook(py, py_dict)),
    })
}

/// Callback for progress reporting by YoutubeDL
#[pyfunction]
#[pyo3(signature = (info, /))]
fn progress_hook(py: Python<'_>, info: &PyDict) -> PyResult<()> {
    // PY: if "info_dict" in info:
    let info_dict = info.get_item("info_dict");
    if info_dict.is_some() {
        let info_dict = info_dict.unwrap().downcast::<PyDict>()?;
        // PY: if "__real_download" in info['info_dict']:
        let real_download = info_dict
            .get_item("__real_download")
            .unwrap()
            .downcast::<PyBool>()?;
        if real_download.is_true() {
            // PY: if "_percent_str" in info:
            let _percent_str = info.get_item("_percent_str");
            if _percent_str.is_some() {
                let percent_str = _percent_str.unwrap().downcast::<PyString>()?.to_string();
                log::info!("Callback: progress_hook {}", percent_str);

                let pym = PyModule::from_code(
                    py,
                    r#"
import http.client

def py_callback(*args, **kwargs):
    host = "localhost"
    conn = http.client.HTTPConnection(host, 8000)
    conn.request("GET", "/", headers={"Host": host})
                    "#,
                    "",
                    "",
                );
                let fun: Py<PyAny> = pym?.getattr("py_callback")?.into();
                fun.call(py, (), None)?;
                log::info!("Callback: progress sent");
            }
        }
    }
    Ok(())
}

impl Job for Video {
    const NAME: &'static str = "yt_dlw::Video";
}
pub async fn download(job: Video, _ctx: JobContext) {
    log::info!("Attempting to download url");
    // pyo3::prepare_freethreaded_python();
    let _: Result<()> = Python::with_gil(|py| {
        // PY: from yt_dlp import YoutubeDL
        let mod_yt_dlp = PyModule::import(py, "yt_dlp")?;
        // PY: self._ydl = YoutubeDL()
        let ydl = mod_yt_dlp.getattr("YoutubeDL")?.call0()?;

        let mod_yt_dlw = PyModule::from_code(
            py,
            fs::read_to_string("yt_dlw.py").unwrap().as_str(),
            "yt_dlw.py",
            "yt_dlw",
        )?;
        let yt_dlw_progress_hook = mod_yt_dlw.getattr("progress_hook")?;

        // PY: self.ydl.add_progress_hook(self.progress_hook)
        // PY: self.ydl.add_postprocessor_hook(self.progress_hook)
        let ydl_add_progress_hook = ydl.getattr("add_progress_hook")?;
        let ydl_add_postprocessor_hook = ydl.getattr("add_postprocessor_hook")?;
        // let cb1 = make_callback();
        // let cb2 = make_callback();
        // let _ = ydl_add_progress_hook.call1((cb1.into_py(py),))?;
        // let _ = ydl_add_postprocessor_hook.call1((cb2.into_py(py),))?;
        let _ = ydl_add_progress_hook.call1((yt_dlw_progress_hook,))?;
        let _ = ydl_add_postprocessor_hook.call1((yt_dlw_progress_hook,))?;

        // PY: self.ydl.download(video_url)
        let ydl_download = ydl.getattr("download")?;
        ydl_download.call1((vec![job.video_url],))?;

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
