import socketio
from celery.app.control import Inspect
from fastapi import FastAPI
from pydantic import BaseModel
from pydantic_settings import BaseSettings

from tasks import download_task, info_task, celery


class Settings(BaseSettings):
    ydl_home: str = "completed"


class Video(BaseModel):
    url: str


settings = Settings()
app = FastAPI()

# @see: https://github.com/miguelgrinberg/python-engineio/issues/142#issuecomment-545807047
# we probably want to send update events on a lower rate
# Payload.max_decode_packets = 50
sio = socketio.AsyncServer(async_mode='asgi', cors_allowed_origins='*', namespaces=["/client", "/server"])
asgi = socketio.ASGIApp(sio)
app.mount("/ws", asgi)


@sio.on(event='progress', namespace="/server")
async def progress(sid, data):
    """
    Celery tasks send download progress information via WebSocket
    Each event is broadcasted to all subscribers, except to the caller sid

    Excluding the caller sid is somewhat an extra safety measure, as server and client events are on different channels:
    tasks send events on the /server channel whereas the event is emitted to all clients subscribing to the /client
    channel.

    :param sid:
    :param data:
    :return:
    """
    await sio.emit("progress", data, namespace="/client", skip_sid=sid)
    print(f"Download progress: {sid} {data}")


@app.post("/download/")
async def download(video: Video):

    download_task.delay(settings.ydl_home, video.url)
    return {"message": "Video download queued"}


@app.get("/status/")
async def status():
    i: Inspect = celery.control.inspect()
    active = i.active()
    reserved = i.reserved()
    return {"message": "OK"}


@app.get("/info/")
async def download(url: str):
    res = info_task.delay(url)
    info = res.get()
    return info
