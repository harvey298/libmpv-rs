# libmpv-rs
A libmpv abstraction written in rust that's easy to use and provides the ability to read next to all video and audio codecs.

# Dependencies
Rust version >= 1.30. Libmpv version 1.101 (mpv version 0.29.1) is the minimum required version.
ninja and meson are required on all targets, Ninja can be replaced with other similar tools.

For ease of building, you can use the `build_libmpv` feature that is used to link against. Especially useful to cross compile to windows. The `MPV_SOURCE` environment variable needs to be set to a directory containing the mpv source you want to build against. For windows targets this is expected to be already built, with a directory named `MPV_SOURCE/64` or `/32` containing [build artifacts](https://mpv.srsfckn.biz/) for 64-bit and 32-bit targets respectively. On unix this is expected to be a copy of the mpv-build repo or you can enable `grab_libmpv` which will grab the latest version of mpv/libmpv and build it.

# Examples
To run an example, execute `cargo run [--release] --example x -- test-data/speech_12kbps_mb.wav`, where x is any of:
* `events`: event enumeration
* `protocol`: implementation of custom `filereader://` protocol that… reads files

# Creating `libmpv.so.2`
This is for Unix targets only!
`https://github.com/mpv-player/mpv-build`
`cd mpv-build`
`./use-ffmpeg-master`
`./use-libplacebo-master`
`./use-mpv-master`
`./build`
`sudo ./install`

# Contributing
All pull requests/issues welcome.

# Known Issues

## Missing `mpv.lib`
run `lib /def:mpv-1.dll /out:mpv.lib` in the Developer Powershell on windows
I no longer have the link to where I downloaded `mpv-1.dll` from source forge, so I cannot send you to the proper download, I will leave theses files in `windows`

## No sound output on Linux/MPV error -14
Install `libasound2-dev` or any package which provides development files for your audio system

## Cannot find libmpv at runtime
Set `LD_LIBARY_PATH` to where you stored `libmpv.so.2`
when using `grab_libmpv` on Linux it will build mpv from source (including `libmpv`), you can find the completed libary/binary/development files in `OUT_DIR` where you can do `sudo ./install` to install mpv, libmpv and the development files.