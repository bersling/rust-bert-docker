docker run -d --rm -p 8000:8000 --name rust-bert rust-bert-server

# You can make health checks when it's up like this:
# curl localhost:8000/api/health-check
# curl localhost:8000/api/health-check-zero-shot
