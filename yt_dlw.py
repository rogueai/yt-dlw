import requests


def progress_hook(info: dict):
    requests.post("http://localhost:8000/internal", json=info)
