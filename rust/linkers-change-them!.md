# Rust Linkers - Change them! 

The default linker in Rust (GCC) is slow, a small project can take 12+seconds to link to the correct packages, this is slow and get slower the more you use it. 

Here you can use the LLVM linked "LLD" which will speed up your compile times by multiple increments.

> "LLD is a linker from the LLVM project that is a drop-in replacement for system linkers and runs much faster than them. [..] When you link a large program on a multicore machine, you can expect that LLD runs more than twice as fast as the GNU gold linker. Your mileage may vary, though."

For users on Linux add this to your [.cargo/config.toml](https://doc.rust-lang.org/cargo/reference/config.html):
```
[target.x86_64-unknown-linux-gnu]
rustflags = [
    "-C", "link-arg=-fuse-ld=lld",
]
```

## Performance

This is a link time comparison on a 2-socket 20-core 40-thread Xeon E5-2680 2.80 GHz machine with an SSD drive. We ran gold and lld with or without multi-threading support. To disable multi-threading, we added -no-threads to the command lines.
| Program | Output size | GNU ld | GNU gold w/o threads | GNU gold w/threads | lld w/o threads | lld w/threads |
| --- | --- | --- | --- | --- | --- | --- |
| ffmpeg dbg | 92 MiB | 1.72s | 1.16s | 1.01s | 0.60s | 0.35s |
| mysqld dbg | 154 MiB | 8.50s | 2.96s | 2.68s | 1.06s | 0.68s |
| clang dbg | 1.67 GiB | 104.03s | 34.18s | 23.49s | 14.82s | 5.28s |
| chromium dbg | 1.14 GiB | 209.05s [1] | 64.70s | 60.82s | 27.60s | 16.70s |

## Potential Errors 

When running cargo build and the following error occurs:
```
  = note: collect2: fatal error: cannot find ‘ld’
          compilation terminated.
```
You choose linkers not by overriding the ld command but choosing to run ld.bfd, ld.gold, ld.lld, etc.

It's the bit after the dot in ld.xyz that you specify in -fuse-ld=xyz. So just make sure that /usr/bin/ld.lld points to your linker and should be good. Something like

```ln -s /usr/bin/ld.lld-* /usr/bin/ld.lld```
[(source)](https://users.rust-lang.org/t/cannot-find-ld-when-using-lld/47420)
