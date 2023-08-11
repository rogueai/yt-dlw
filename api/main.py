import tasks
import uvicorn
from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from pydantic import BaseModel

from tasks import download_task, info_task

app = FastAPI()

# with tasks.celery.connection() as connection:
#     recv = tasks.celery.events.Receiver(connection, handlers={
#         '*': print("Celery event"),
#     })
#     recv.capture(limit=None, timeout=None, wakeup=True)


class ConnectionManager:
    def __init__(self):
        self.active_connections: list[WebSocket] = []

    async def connect(self, ws: WebSocket):
        await ws.accept()
        self.active_connections.append(ws)

    def disconnect(self, ws: WebSocket):
        self.active_connections.remove(ws)

    async def send_message(self, message: str, ws: WebSocket):
        await ws.send_text(message)

    async def broadcast(self, message: str):
        for connection in self.active_connections:
            await connection.send_text(message)


manager = ConnectionManager()


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


# TODO: investigate Socket.IO https://github.com/tiangolo/fastapi/issues/129#issuecomment-714636723
@app.websocket("/ws")
async def websocket(ws: WebSocket):
    await manager.connect(ws)
    try:
        while True:
            data = await ws.receive_text()
            await manager.send_message(f"You wrote: {data}", ws)
            await manager.broadcast(f"Client says: {data}")
    except WebSocketDisconnect:
        manager.disconnect(ws)
        await manager.broadcast(f"Client left the chat")


if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=8000)
