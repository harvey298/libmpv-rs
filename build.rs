// Copyright (C) 2016  ParadoxSpiral
//
// This file is part of mpv-rs.
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA

#[cfg(feature = "build_libmpv")]
use std::env;

// #[cfg(all(feature = "build_libmpv", not(target_os = "windows")))]
use std::process::Command;

#[cfg(not(feature = "build_libmpv"))]
fn main() {}

#[cfg(all(feature = "build_libmpv", target_os = "windows"))]
fn main() {
    #[cfg(feature = "grab_libmpv")] get_libmpv();

    let source = env::var("MPV_SOURCE").expect("env var `MPV_SOURCE` not set");

    if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "64" {
        println!("cargo:rustc-link-search={}/64/", source);
    } else {
        println!("cargo:rustc-link-search={}/32/", source);
    }
}

#[cfg(all(feature = "build_libmpv", not(target_os = "windows")))]
fn main() {
    #[cfg(feature = "grab_libmpv")] get_libmpv();

    let source = env::var("MPV_SOURCE").expect("env var `MPV_SOURCE` not set");
    let num_threads = env::var("NUM_JOBS").unwrap();

    // `target` (in cfg) doesn't really mean target. It means target(host) of build script,
    // which is a bit confusing because it means the actual `--target` everywhere else.
    #[cfg(target_pointer_width = "64")]
    {
        if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "32" {
            panic!("Cross-compiling to different arch not yet supported");
        }
    }
    #[cfg(target_pointer_width = "32")]
    {
        if env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap() == "64" {
            panic!("Cross-compiling to different arch not yet supported");
        }
    }

    // The mpv build script interprets the TARGET env var, which is set by cargo to e.g.
    // x86_64-unknown-linux-gnu, thus the script can't find the compiler.
    // TODO: When Cross-compiling to different archs is implemented, this has to be handled.
    env::remove_var("TARGET");

    let cmd = format!(
        "cd {} && echo \"--enable-libmpv-shared\" > {0}/mpv_options \
         && {0}/build -j{}",
        source, num_threads
    );

    Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .spawn()
        .expect("mpv-build build failed")
        .wait()
        .expect("mpv-build build failed");

    println!("cargo:rustc-link-search={}/mpv/build/", source);
}


fn get_libmpv() {
    #[cfg(not(target_os = "windows"))]
    let url = "https://github.com/mpv-player/mpv-build.git";

    #[cfg(target_os = "windows")]
    let url = "https://github.com/mpv-player/mpv";

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let mpv_src = format!("{out_dir}/mpv_src/");

    Command::new("git")
        .arg("clone")
        .arg(&url)
        .arg(&mpv_src)
        .spawn()
        .expect("mpv-build build failed")
        .wait()
        .expect("mpv-build build failed");

    std::env::set_var("MPV_SOURCE", mpv_src);

    #[cfg(not(target_os = "windows"))]
    {
        let parent = mpv_src;
        let ffmpeg_master = format!("{parent}use-ffmpeg-master");
        let libplacebo_master = format!("{parent}use-libplacebo-master");
        let mpv_master = format!("{parent}use-mpv-master");
        let build = format!("{parent}build");
        
        // Build libmpv
        Command::new("printf").arg("\"%s\n\"").arg("-Dlibmpv=true").arg(">").arg("mpv_options").spawn().wait().expect("mpv-build build failed");

        Command::new(ffmpeg_master).spawn().unwrap().wait().expect("mpv-build build failed");
        Command::new(libplacebo_master).spawn().unwrap().wait().expect("mpv-build build failed");
        Command::new(mpv_master).spawn().unwrap().wait().expect("mpv-build build failed");
        Command::new(build).spawn().unwrap().wait().expect("mpv-build build failed");
    }
}
