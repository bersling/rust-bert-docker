This was an attempt to wrap the rust-bert project (https://github.com/guillaume-be/rust-bert) in a
rust rocket web server.

However, I found that for my use case the endpoints ended up being a bit too slow to be useful so I switched
to a nlp cloud provider.

I thought instead of deleting this project I'd leave it here since some things can be learned from it and it's a working base:

- [x] To use the rust-bert models the endpoints make use of the `blocking` crate. Otherwise you get the error `Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.`
- [x] When first calling some method, e.g. "zero_shot_classify" or similar, it's possible that stuff first needs to be downloaded. Then the endpoint would take exceptionally long to work. I'm also not sure about how much is loaded in memory, so one would have to take extra precautions that dynamically loaded things don't cause an OOM (out of memory) on the server. Maybe the best approach would be to preload everything required, if you really want to build a web server with it. I don't know how much memory and disk that would end up eating up.
- [x] Even the second calls to something like classify take ~5-8 seconds on a CPU. So it didn't seem feasible.
- [x] There are nlp cloud providers out there. To get started, it might be easier for you too to use one of those.
- [x] The health-check endpoints are functional on Linux (couldn't make it work on Mac M1).

Having said all of that, this project might still serve as a starting or inspiration point for someone. Have fun. :)