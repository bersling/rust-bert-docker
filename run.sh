export LIBTORCH=/Users/bersling/IT-Projects/rust-bert-docker/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
export ROCKET_PORT=8001
cargo run
