docker stop rust-bert || true
docker run -d --rm -p 8000:8000 --env RUST_BACKTRACE=1 --name rust-bert bersling/rust-bert-cpu
sleep 5
curl localhost:8000/api/health-check
curl localhost:8000/api/health-check-zero-shot
docker system prune --force
