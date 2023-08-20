# import requests
#
#
# def progress_hook(info: dict):
#     requests.post("http://localhost:8000/internal", json=info)

from websockets.sync.client import connect
from yt_dlp import YoutubeDL
import json

websocket = None


def progress_hook(info: dict):
    global websocket
    if websocket is None:
        websocket = connect("ws://localhost:8000/ws")
    # requests.post("http://localhost:8000/internal", json=info)
    websocket.send(json.dumps(info))


def download(video_url: str):
    ydl = YoutubeDL()
    ydl.add_progress_hook(progress_hook)
    ydl.add_postprocessor_hook(progress_hook)
    ydl.download(video_url)
