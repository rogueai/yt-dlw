from fastapi import FastAPI
from pydantic import BaseModel

from tasks import download_task, info_task

app = FastAPI()


class Video(BaseModel):
    url: str


@app.post("/download/")
async def download(video: Video):
    # background_tasks.add_task(download_task, video_id)
    download_task.delay(video.url)
    return {"message": "Video download queued"}


@app.get("/info/")
async def download(url: str):
    # background_tasks.add_task(download_task, video_id)
    res = info_task.delay(url)
    info = res.get()
    return info
