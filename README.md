This was an attempt to wrap the rust-bert project (https://github.com/guillaume-be/rust-bert) in a
rust rocket web server.

However, I found that for my use case the endpoints ended up being a bit too slow to be useful so I switched
to a nlp cloud provider.

I thought instead of deleting this project I'd leave it here since some things can be learned from it and it's a working base:

- [x] To use the rust-bert models the endpoints make use of the `blocking` crate. Otherwise you get the error