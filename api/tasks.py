import socketio
from celery import Celery
from yt_dlp import YoutubeDL

celery = Celery(
    "tasks",
    broker="redis://localhost:6379/0",
    backend="redis://localhost:6379/0"
)
ydl = YoutubeDL()

sio = socketio.Client(reconnection_attempts=10, reconnection_delay=3, reconnection_delay_max=30)


def progress_hook(info):
    check_connection()
    sio.emit("download_progress", {
        "status": info['status'],
        "progress": info['_percent_str'],
    })


ydl.add_progress_hook(progress_hook)
# ydl.add_postprocessor_hook(progress_hook)


def check_connection():
    if not sio.connected:
        sio.connect(
            "http://localhost:8000/", socketio_path="/ws/socket.io"
        )


@celery.task(bind=True)
def download_task(self, video_url):
    ydl.download(video_url)
    return f"[download] task started: {video_url}"


@celery.task
def info_task(video_url):
    info = ydl.extract_info(video_url, download=False)
    return info
