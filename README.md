# yt-dlw

Lightweight web interface for yt-dlp.

It's not _that_ lightweight, but the projects tries to keep hardware constraints to the bare minimum,
while keeping a certain degree of functionality.

Based on initial rough estimates, on idle we're looking at something like this for RAM usage:
```
- celery worker: 90MB
- redis:         20MB
- api server:   100MB
- frontend:      90MB
---------------------
- total:        300MB
```

### Develop

```shell
# start redis
docker-compose up -d
# start celery
# Note: by default celery starts a number of worker processes equal to the number of CPU cores
# this can be changed by passing --concurrency=<n> to the worker
# @see: https://docs.celeryq.dev/en/latest/userguide/workers.html#concurrency
cd api
celery -A tasks worker --loglevel=INFO --queues=downloads,info # --concurrency=2
# start fastapi
cd api
uvicorn main:app --reload
# start frontend
cd app
npm run dev

```

If you want more control over what's happening in celery, you can install flower:

```shell
pip install flower
# start celery with flower 
celery -A tasks flower --loglevel=INFO --queues=downloads,info
```

Flower web interface can then be accessed at http://localhost:5555 and used to monitor queues,
tasks, etc.

### TODO

- [ ] Logging
- [ ] Auth
- [ ] Rootless docker images

### Done
- [x] Docker image and compose sample