This was an attempt to wrap the rust-bert project (https://github.com/guillaume-be/rust-bert) in a
rust rocket web server.

However, I found that for my use case the endpoints ended up being a too slow to be useful so I switched
to a nlp cloud provider. Example: The zero_shot_classify with default settings takes:
- 2 minutes on the first endpoint call (models are downloaded)
- 21 seconds on the second endpoint call (don't know why this is still slower than the third call)
- 6 seconds on the next calls.

I thought instead of deleting this project I'd leave it here since some things can be learned from it and it's a working base:

- [x] To use the rust-bert models the endpoints make use of the `blocking` crate. Otherwise you get the error `Cannot drop a runtime in a context where blocking is not allowed. This happens when a runtime is dropped from within an asynchronous context.`
- [x] When first calling some method, e.g. "zero_shot_classify", it's possible that stuff first needs to be downloaded. Then the endpoint would take exceptionally long to respond (can take over 2 minutes). I'm also not sure about how much is loaded in memory and how much to disk, so one would have to take extra precautions that dynamically loaded things don't cause an OOM (out of memory) on the server. Maybe the best approach would be to preload everything required, if you really want to build a web server with it. I don't know how much memory and disk that would end up eating up. Maybe it would also be good to make the web server really microservice like and have a dedicated microservice for every endpoint, such that if one goes OOM the others don't crash. Then the question is, could this maybe even be realized with something like AWS lambda?
- [x] Even the second calls to something like classify (with default config, bart MNLI) take ~5-8 seconds on a CPU. So it didn't seem feasible for my use case.
- [x] There are nlp cloud providers out there. To get started, it might be easier for you too to use one of those.
- [x] The health-check endpoints are functional on Linux (couldn't make it work on Mac M1).
- [x] The docker image takes quite long to build. Maybe something like rust chef could help to speed up consecutive builds by leveraging docker layering.

Having said all of that, this project might still serve as a starting or inspiration point for someone. Have fun. :)
