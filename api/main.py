from typing import Union

from fastapi import BackgroundTasks, FastAPI

from yt_dlp import YoutubeDL

app = FastAPI()

ydl = YoutubeDL()


def download_task(video_id):
    ydl.download('https://www.youtube.com/watch?v=' + video_id)


@app.get("/")
def read_root():
    return {"Hello": "World"}


# URL = 'https://www.youtube.com/watch?v=BaW_jenozKc'
@app.get("/download/{video_id}")
async def download(video_id: str, background_tasks: BackgroundTasks):
    background_tasks.add_task(download_task, video_id)
    return {"message": "Video download queued"}
