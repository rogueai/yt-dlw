from fastapi import BackgroundTasks, FastAPI

from tasks import celery, download_task

app = FastAPI()


@app.get("/")
def read_root():
    return {"Hello": "World"}


# URL = 'https://www.youtube.com/watch?v=BaW_jenozKc'
@app.get("/download/{video_id}")
async def download(video_id: str, background_tasks: BackgroundTasks):
    # background_tasks.add_task(download_task, video_id)
    download_task.delay(video_id)
    return {"message": "Video download queued"}
