# Develop

```shell
# start redis
docker-compose up -d
# start celery
# Note: by default celery starts a number of worker processes equal to the number of CPU cores
# this can be changed by passing --concurrency=<n> to the worker
# @see: https://docs.celeryq.dev/en/latest/userguide/workers.html#concurrency
cd api
celery -A tasks worker --loglevel=INFO # --concurrency=2
# start fastapi
cd api
uvicorn main:app --reload
# start frontend
cd app
npm run dev

```