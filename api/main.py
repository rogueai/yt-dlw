import socketio
import uvicorn
from fastapi import FastAPI
from pydantic import BaseModel

from tasks import download_task, info_task

app = FastAPI()

# @see: https://github.com/miguelgrinberg/python-engineio/issues/142#issuecomment-545807047
# we probably want to send update events on a lower rate
# Payload.max_decode_packets = 50
sio = socketio.AsyncServer(async_mode='asgi', cors_allowed_origins='*')
asgi = socketio.ASGIApp(sio)
app.mount("/ws", asgi)


class Video(BaseModel):
    url: str


@sio.on('download_progress')
async def download_progress(sid, data):
    """
    Celery tasks send download progress information via WebSocket
    Each event is broadcasted to all subscribers, except to the caller sid

    :param sid:
    :param data:
    :return:
    """
    await sio.emit("progress", data, skip_sid=sid)
    print(f"Download progress: {sid} {data}")


@app.post("/download/")
async def download(video: Video):
    download_task.delay(video.url)
    return {"message": "Video download queued"}


@app.get("/info/")
async def download(url: str):
    res = info_task.delay(url)
    info = res.get()
    return info


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
