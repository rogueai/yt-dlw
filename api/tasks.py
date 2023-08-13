import socketio
from celery import Celery
from celery import Task
from yt_dlp import YoutubeDL

import config


class YtDlpTask(Task):
    """
    Task handling yt-dlp operations.
    Question arises whether there should be a yt-dlp per task, or a global one. It all depends on how yt-dlp manages
    concurrent operations. For now it seems safer to rely on Celery for concurrency, and assume yt-dlp has none baked
    in.
    """
    _ydl = None
    _sio = None

    def __init__(self):
        self._ydl = YoutubeDL()
        self.ydl.add_progress_hook(self.progress_hook)
        self.ydl.add_postprocessor_hook(self.progress_hook)
        self._sio = socketio.Client(reconnection_attempts=10, reconnection_delay=3, reconnection_delay_max=30)

    def progress_hook(self, info: dict):
        if "info_dict" in info:
            if "__real_download" in info['info_dict']:
                if info['info_dict']['__real_download'] is True:
                    message = {
                        "status": info['status']
                    }
                    if "_percent_str" in info:
                        message["progress"] = info['_percent_str']
                    self.sio.emit("progress", message, namespace="/server")

    @property
    def ydl(self):
        return self._ydl

    @property
    def sio(self):
        if not self._sio.connected:
            self._sio.connect(config.API_URL, socketio_path="/ws/socket.io", namespaces="/server")
        return self._sio


celery = Celery(
    "tasks",
    broker=config.CELERY_BROKER,
    backend=config.CELERY_BACKEND,
)
celery.conf.update(task_routes={
    'tasks.download_task': {
        'queue': 'downloads',
    },
    'tasks.info_task': {
        'queue': 'info',
    },
})


@celery.task(base=YtDlpTask, bind=True)
def download_task(self: YtDlpTask, ydl_home, video_url):
    self.ydl.params["paths"] = {
        'home': ydl_home
    }
    self.ydl.download(video_url)
    return f"[download] task started: {video_url}"


@celery.task(base=YtDlpTask, bind=True)
def info_task(self: YtDlpTask, video_url):
    info = self.ydl.extract_info(video_url, download=False)
    return info
