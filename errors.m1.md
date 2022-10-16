```
" "-ltorch" "-lc10" "-liconv" "-lSystem" "-lresolv" "-lc" "-lm" "-liconv" "-L" "/opt/homebrew/Cellar/rust/1.64.0/lib/rustlib/aarch64-apple-darwin/lib" "-o" "/mypath/rust-bert-server/target/debug/deps/rust_bert_server-e2ac65bcaafc0ee0" "-Wl,-dead_strip" "-nodefaultlibs"
  = note: ld: warning: ignoring file /mypath/rust-bert-server/libtorch/lib/libtorch.dylib, building for macOS-arm64 but attempting to link with file built for macOS-x86_64
          ld: warning: ignoring file /mypath/rust-bert-server/libtorch/lib/libc10.dylib, building for macOS-arm64 but attempting to link with file built for macOS-x86_64
          ld: warning: ignoring file /mypath/rust-bert-server/libtorch/lib/libtorch_cpu.dylib, building for macOS-arm64 but attempting to link with file built for macOS-x86_64
          Undefined symbols for architecture arm64:
            "c10::IValue::reportToTensorTypeError() const", referenced from:
                c10::IValue::toTensor() && in libtorch_sys-1e341a4af5f9299f.rlib(torch_api.o)
```

How to fix this?