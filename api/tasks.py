from celery import Celery
from yt_dlp import YoutubeDL

celery = Celery(
    "tasks",
    broker="redis://localhost:6379/0",
    backend="redis://localhost:6379/0"
)
ydl = YoutubeDL()


@celery.task
def download_task(video_url):
    ydl.download(video_url)
    return f"[download] task started: {video_url}"


@celery.task
def info_task(video_url):
    info = ydl.extract_info(video_url, download=False)
    return info
