export LIBTORCH=/Users/bersling/IT-Projects/rust-bert-server/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
export ROCKET_PORT=8000
cargo run
