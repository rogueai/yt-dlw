import os

CELERY_BROKER = os.getenv("CELERY_BROKER", "redis://localhost:6379/0")
CELERY_BACKEND = os.getenv("CELERY_BACKEND", "redis://localhost:6379/0")
API_URL = os.getenv("API_URL", "http://localhost:8000/")
