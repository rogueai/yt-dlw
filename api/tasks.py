from celery import Celery
from yt_dlp import YoutubeDL

celery = Celery(
    "tasks",
    broker="redis://localhost:6379/0",
    backend="redis://localhost:6379/0"
)
ydl = YoutubeDL()


@celery.task
def download_task(video_id):
    ydl.download('https://www.youtube.com/watch?v=' + video_id)
    return f"Task started for video: {video_id}"
