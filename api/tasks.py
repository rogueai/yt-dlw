from celery import Celery
from yt_dlp import YoutubeDL

celery = Celery(
    "tasks",
    broker="redis://localhost:6379/0",
    backend="redis://localhost:6379/0"
)
ydl = YoutubeDL()


def progress_hook(s):
    print(s)


@celery.task
def download_task(video_url):
    ydl.add_progress_hook(progress_hook)
    ydl.add_postprocessor_hook(progress_hook)
    ydl.download(video_url)
    return f"[download] task started: {video_url}"


@celery.task
def info_task(video_url):
    info = ydl.extract_info(video_url, download=False)
    return info
