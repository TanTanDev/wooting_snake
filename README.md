# wooting_snake
A Snake game compatible with Wooting keyboards only, written with Rust
![](about/snake_wooting_preview.gif)

### Check out the video I made about this project
https://youtu.be/OhhscXz-60g

## Known bugs
If you close the application regulary, the lights could get "stuck".

Fix: unplug / replug the keyboard, OR: starting the game again and,

closing the game through they keyboard menu, works fine.


## Prerequisites
This library uses the crate "wooting-sdk", which requires libclang to be installed, more info on it's github page: https://github.com/davidtwco/rust-wooting-sdk

In order to generate bindings for the SDKs using bindgen, wooting-analog-sdk-sys and wooting-rgb-sdk-sys require libclang. If the submodules for the SDKs are not cloned, then the build scripts will attempt to clone them, in this case, git is required. When building the SDKs, libusb or libudev will be required for the SDKs' hidapi dependency on Linux, these are expected to exist.

Windows
Download and install the official pre-built binary for libclang from the LLVM download page.

## Building
> cargo run

When the prerequisites are met, use cargo as any other project
that's it... I LOVE RUST
