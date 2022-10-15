docker run -d --rm -p 8000:8000 --name rust-bert bersling/rust-bert-cpu
curl localhost:8000/api/health-check
