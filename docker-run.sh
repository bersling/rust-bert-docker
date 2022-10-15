export RUST_BACKTRACE=1
docker stop rust-bert || true
docker run -d --rm -p 8000:8000 --name rust-bert bersling/rust-bert-cpu
curl localhost:8000/api/health-check
curl localhost:8000/api/health-check-zero-shot
