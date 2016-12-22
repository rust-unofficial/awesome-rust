# Awesome Rust

A curated list of Rust code and resources, inspired by other awesome lists.

If you want to contribute, please read [this](CONTRIBUTING.md).

## Table of Contents

- [Applications written in Rust](#applications-written-in-rust)
- [Development Tools](#development-tools)
  - [Build system](#build-system)
  - [Debugging](#debugging)
  - [Embedded](#embedded)
  - [FFI](#ffi)
  - [IDEs](#ides)
  - [Profiling](#profiling)
  - [Testing](#testing)
- [Libraries](#libraries)
  - [Astronomy](#astronomy)
  - [Asynchronous](#asynchronous)
  - [Audio](#audio)
  - [Authentication](#authentication)
  - [Bioinformatics](#bioinformatics)
  - [Caching](#caching)
  - [Cloud](#cloud)
  - [Command-line argument parsing](#command-line-argument-parsing)
  - [Command-line interface](#command-line-interface)
  - [Compression](#compression)
  - [Computation](#computation)
  - [Concurrency](#concurrency)
  - [Cryptography](#cryptography)
  - [Database](#database)
  - [Data structures](#data-structures)
  - [Date and time](#date-and-time)
  - [Distributed Systems](#distributed-systems)
  - [Email](#email)
  - [Encoding](#encoding)
  - [Filesystem](#filesystem)
  - [Game development](#game-development)
  - [Geospatial](#geospatial)
  - [Graphics](#graphics)
  - [GUI](#gui)
  - [Image processing](#image-processing)
  - [Logging](#logging)
  - [Machine learning](#machine-learning)
  - [Markup language](#markup-language)
  - [Mobile](#mobile)
  - [Network programming](#network-programming)
  - [Parser](#parser)
  - [Platform specific](#platform-specific)
  - [Template engine](#template-engine)
  - [Text processing](#text-processing)
  - [Virtualization](#virtualization)
  - [Web programming](#web-programming)
- [Resources](#resources)
- [License](#license)


## Applications written in Rust

See also [Friends of Rust](https://www.rust-lang.org/friends.html) (organizations running Rust in production).

* [andschwa/rust-genetic-algorithm](https://github.com/andschwa/rust-genetic-algorithm) — a genetic algorithm for academic benchmark problems [<img src="https://travis-ci.org/andschwa/rust-genetic-algorithm.svg?branch=master">](https://travis-ci.org/andschwa/rust-genetic-algorithm)
* [azerupi/mdBook](https://github.com/azerupi/mdBook) — a command line utility to create books from markdown files [<img src="https://travis-ci.org/azerupi/mdBook.svg?branch=master">](https://travis-ci.org/azerupi/mdBook)
* [bluejekyll/trust-dns](https://github.com/bluejekyll/trust-dns) — a DNS-server [<img src="https://travis-ci.org/bluejekyll/trust-dns.svg?branch=master">](https://travis-ci.org/bluejekyll/trust-dns)
* [ivanceras/curtain](https://github.com/ivanceras/curtain) — a database administration tool for postgresql [<img src="https://api.travis-ci.org/ivanceras/curtain.svg">](https://travis-ci.org/ivanceras/curtain)
* [dlecan/generic-dns-update](https://github.com/dlecan/generic-dns-update) — a tool to update DNS zonefiles with your IP address [<img src="https://travis-ci.org/dlecan/generic-dns-update.svg?branch=master">](https://travis-ci.org/dlecan/generic-dns-update)
* [Factotum](https://github.com/snowplow/factotum) — [A system to programmatically run data pipelines](http://snowplowanalytics.com/blog/2016/04/09/introducing-factotum-data-pipeline-runner/) [<img src="https://travis-ci.org/snowplow/factotum.svg?branch=master">](https://travis-ci.org/snowplow/factotum)
* [Fractalide](https://github.com/fractalide/fractalide) — Simple Rust Microservices
* [imjacobclark/Herd](https://github.com/imjacobclark/Herd) — an experimental HTTP load testing application
* [jedisct1/flowgger](https://github.com/jedisct1/flowgger) — a fast, simple and lightweight data collector
* [kbknapp/docli](https://github.com/kbknapp/docli-rs) — a command line utility for managing DigitalOcean infrastructure [<img src="https://travis-ci.org/kbknapp/docli-rs.svg?branch=master">](https://travis-ci.org/kbknapp/docli-rs)
* [MaidSafe](http://maidsafe.net/) — a decentralized platform.
* [notty](https://github.com/withoutboats/notty) — an experimental project to bring new features to the command-line
* [qmx/limonite](https://github.com/qmx/limonite) — static blog/website generator [<img src="https://travis-ci.org/qmx/limonite.svg?branch=master">](https://travis-ci.org/qmx/limonite)
* [Sandstorm Collections App](https://github.com/sandstorm-io/collections-app)
* [Servo](https://github.com/servo/servo) — a prototype web browser engine
* [Parity](https://github.com/ethcore/parity) — Fast, light, and robust Ethereum implementation [![Build Status](https://travis-ci.org/ethcore/parity.svg?branch=master)](https://travis-ci.org/ethcore/parity)

* **Audio**
  * [indiscipline/zrtstr](https://github.com/indiscipline/zrtstr) — a command line utility for checking if stereo wav files are faux-stereo (i.e. have identical channels) and converting such files to mono. [<img src="https://travis-ci.org/indiscipline/zrtstr.svg?branch=master">](https://travis-ci.org/indiscipline/zrtstr)
* **Database**
  * [pingcap/tikv](https://github.com/pingcap/tikv) — a distributed KV database in Rust [<img src="https://travis-ci.org/pingcap/tikv.svg?branch=master">](https://travis-ci.org/pingcap/tikv)
  * [seppo0010/rsedis](https://github.com/seppo0010/rsedis) — a Redis reimplementation in Rust [<img src="https://travis-ci.org/seppo0010/rsedis.svg?branch=master">](https://travis-ci.org/seppo0010/rsedis)
* **Emulators** [[emulator](https://crates.io/keywords/emulator)]
  * Commodore 64
    * [kondrak/rust64](https://github.com/kondrak/rust64) — [<img src="https://travis-ci.org/kondrak/rust64.svg?branch=master">](https://travis-ci.org/kondrak/rust64)
  * Gameboy
    * [Gekkio/mooneye-gb](https://github.com/Gekkio/mooneye-gb) — [<img src="https://travis-ci.org/Gekkio/mooneye-gb.svg?branch=master">](https://travis-ci.org/Gekkio/mooneye-gb)
    * [mvdnes/rboy](https://github.com/mvdnes/rboy) — [<img src="https://travis-ci.org/mvdnes/rboy.svg?branch=master">](https://travis-ci.org/mvdnes/rboy)
  * NES
    * [iamsix/oxidenes](https://github.com/iamsix/oxidenes) — [<img src="https://travis-ci.org/iamsix/oxidenes.svg?branch=master">](https://travis-ci.org/iamsix/oxidenes)
    * [koute/pinky](https://github.com/koute/pinky) — [<img src="https://travis-ci.org/koute/pinky.svg?branch=master">](https://travis-ci.org/koute/pinky)
    * [pcwalton/sprocketnes](https://github.com/pcwalton/sprocketnes) — [<img src="https://travis-ci.org/pcwalton/sprocketnes.svg?branch=master">](https://travis-ci.org/pcwalton/sprocketnes)
  * Playstation
    * [simias/rustation](https://github.com/simias/rustation) — [<img src="https://travis-ci.org/simias/rustation.svg?branch=master">](https://travis-ci.org/simias/rustation)
  * ZX Spectrum
    * [pacmancoder/rustzx](https://github.com/pacmancoder/rustzx) — [<img src="https://travis-ci.org/pacmancoder/rustzx.svg?branch=master">](https://travis-ci.org/pacmancoder/rustzx)
* **Games**, see also [Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston).
  * [lifthrasiir/angolmois-rust](https://github.com/lifthrasiir/angolmois-rust) — a minimalistic music video game which supports the BMS format [<img src="https://travis-ci.org/lifthrasiir/angolmois-rust.svg?branch=master">](https://travis-ci.org/lifthrasiir/angolmois-rust)
  * [swatteau/sokoban-rs](https://github.com/swatteau/sokoban-rs) — a Sokoban implementation
  * [Zone of Control](https://github.com/ozkriff/zoc) — a turn-based hexagonal strategy game [<img src="https://travis-ci.org/ozkriff/zoc.svg?branch=master">](https://travis-ci.org/ozkriff/zoc)
  * [rhex](https://github.com/dpc/rhex) — hexagonal ascii roguelike
* **Graphics**
  * [ivanceras/svgbobrus](https://github.com/ivanceras/svgbobrus) — converts ASCII diagrams into SVG graphics [<img src="https://api.travis-ci.org/ivanceras/svgbobrus.svg">](https://travis-ci.org/ivanceras/svgbobrus)
  * [RazrFalcon/svgcleaner](https://github.com/RazrFalcon/svgcleaner) — tidies SVG graphics
  * [Twinklebear/tray_rust](https://github.com/Twinklebear/tray_rusts) — a ray tracer [<img src="https://api.travis-ci.org/Twinklebear/tray_rust.svg">](https://travis-ci.org/Twinklebear/tray_rust)
* **Operating systems**, see also [A comparison of operating systems written in Rust](https://github.com/flosse/rust-os-comparison)
  * [redox-os/redox](https://github.com/redox-os/redox) — [<img   src="https://travis-ci.org/redox-os/redox.svg?branch=master">](https://travis-ci.org/redox-os/redox)
  * [thepowersgang/rust_os](https://github.com/thepowersgang/rust_os) — [<img src="https://travis-ci.org/thepowersgang/rust_os.svg?branch=master">](https://travis-ci.org/thepowersgang/rust_os)
* **System tools**
  * [Aaronepower/tokei](https://github.com/Aaronepower/tokei) — counts the lines of code [<img src="https://img.shields.io/travis/Aaronepower/tokei.svg">](https://travis-ci.org/Aaronepower/tokei)
  * [buster/rrun](https://github.com/buster/rrun) — a command launcher for Linux, similar to gmrun  [<img src="https://travis-ci.org/buster/rrun.svg?branch=master">](https://travis-ci.org/buster/rrun)
  * [cristianoliveira/funzzy](https://github.com/cristianoliveira/funzzy) — a configurable filesystem watcher inspired by [entr](http://entrproject.org/) [<img src="https://api.travis-ci.org/cristianoliveira/funzzy.svg?branch=master">](https://travis-ci.org/cristianoliveira/funzzy)
  * [ogham/exa](https://github.com/ogham/exa) — a replacement for 'ls' written in Rust [<img src="https://travis-ci.org/ogham/exa.svg?branch=master">](https://travis-ci.org/ogham/exa)
  * [mmstick/systemd-manager](https://github.com/mmstick/systemd-manager) — a systemd service manager written in Rust using GTK-rs.
  * [mmstick/tv-renamer](https://github.com/mmstick/tv-renamer) — a tv series renaming application with an optional GTK3 frontend. [<img src="https://travis-ci.org/mmstick/tv-renamer.svg?branch=master">](https://travis-ci.org/mmstick/tv-renamer)
  * [uutils/coreutils](https://github.com/uutils/coreutils) — a cross-platform Rust rewrite of the GNU coreutils [<img src="https://travis-ci.org/uutils/coreutils.svg?branch=master">](https://travis-ci.org/uutils/coreutils)
* **Text editors**
  * [gchp/iota](https://github.com/gchp/iota) — a simple text editor [<img src="https://travis-ci.org/gchp/iota.svg?branch=master">](https://travis-ci.org/gchp/iota)
  * [xi-editor](https://github.com/google/xi-editor) — a modern editor with a backend written in Rust.
* **Text processing**
  * [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep) — combines the usability of The Silver Searcher with the raw speed of grep [<img src="https://travis-ci.org/BurntSushi/ripgrep.svg?branch=master">](https://travis-ci.org/BurntSushi/ripgrep)
  * [BurntSushi/xsv](https://github.com/BurntSushi/xsv) — a fast CSV command line tool (slicing, indexing, selecting, searching, sampling, etc.) [<img src="https://travis-ci.org/BurntSushi/xsv.svg?branch=master">](https://travis-ci.org/BurntSushi/xsv)
* **Virtualization**
  * [tailhook/vagga](https://github.com/tailhook/vagga) — a containerization tool without daemons [<img src="https://travis-ci.org/tailhook/vagga.svg?branch=master">](https://travis-ci.org/tailhook/vagga)

## Development tools

* [Clippy](https://github.com/Manishearth/rust-clippy) [[clippy](https://crates.io/crates/clippy)] — Rust lints [<img src="https://travis-ci.org/Manishearth/rust-clippy.svg?branch=master">](https://travis-ci.org/Manishearth/rust-clippy)
  * [Clippy Service](https://clippy.bashy.io/)
* [clog-tool/clog-cli](https://github.com/clog-tool/clog-cli) — generates a changelog from git metadata ([conventional changelog](http://blog.thoughtram.io/announcements/tools/2014/09/18/announcing-clog-a-conventional-changelog-generator-for-the-rest-of-us.html)) [<img src="https://travis-ci.org/clog-tool/clog-cli.svg?branch=master">](https://travis-ci.org/clog-tool/clog-cli)
* [dan-t/rusty-tags](https://github.com/dan-t/rusty-tags) — create ctags/etags for a cargo project and all of its dependencies [<img src="https://travis-ci.org/dan-t/rusty-tags.svg?branch=master">](https://travis-ci.org/dan-t/rusty-tags)
* [frewsxcv/crate-deps](https://github.com/frewsxcv/crate-deps) — generates images of dependency graphs for crates hosted on crates.io
* [git-journal](https://github.com/saschagrunert/git-journal/) — The Git Commit Message and Changelog Generation Framework [<img src="https://travis-ci.org/saschagrunert/git-journal.svg?branch=master">](https://travis-ci.org/saschagrunert/git-journal)
* [killercup/rustfix](https://github.com/killercup/rustfix)  — automatically applies the suggestions made by rustc [<img src="https://travis-ci.org/killercup/rustfix.svg?branch=master">](https://travis-ci.org/killercup/rustfix)
* [Racer](https://github.com/phildawes/racer) — code completion for Rust [<img src="https://travis-ci.org/phildawes/racer.svg?branch=master">](https://travis-ci.org/phildawes/racer)
* [rustfmt](https://github.com/rust-lang-nursery/rustfmt) — a Rust code formatter [<img src="https://travis-ci.org/rust-lang-nursery/rustfmt.svg?branch=master">](https://travis-ci.org/rust-lang-nursery/rustfmt)
* [Rustup](https://github.com/rust-lang-nursery/rustup.rs) — the Rust toolchain installer [<img src="https://travis-ci.org/rust-lang-nursery/rustup.rs.svg?branch=master">](https://travis-ci.org/rust-lang-nursery/rustup.rs)
* [rst](https://github.com/vitiral/rst) — the requirements tracking tool made for developers [<img src="https://travis-ci.org/vitiral/rst.svg?branch=master">](https://travis-ci.org/vitiral/rst)
* [semantic-rs](https://github.com/semantic-rs/semantic-rs) — automatic crate publishing [<img src="https://travis-ci.org/semantic-rs/semantic-rs.svg?branch=master">](https://travis-ci.org/semantic-rs/semantic-rs)

### Build system

* [Cargo](https://crates.io/) — the Rust package manager
  * [rsolomo/cargo-check](https://github.com/rsolomo/cargo-check) [[cargo-check](https://crates.io/crates/cargo-check)] — a wrapper around `cargo rustc -- -Zno-trans` which can be helpful for running a faster compile if you only need correctness checks [<img src="https://travis-ci.org/rsolomo/cargo-check.svg?branch=master">](https://travis-ci.org/rsolomo/cargo-check)
  * [kbknapp/cargo-count](https://github.com/kbknapp/cargo-count) [[cargo-count](https://crates.io/crates/cargo-count)] — lists source code counts and details about cargo projects, including unsafe statistics [<img src="https://travis-ci.org/kbknapp/cargo-count.svg?branch=master">](https://travis-ci.org/kbknapp/cargo-count)
  * [pwoolcoc/cargo-do](https://github.com/pwoolcoc/cargo-do) [[cargo-do](https://crates.io/crates/cargo-do)] — run multiple cargo commands in a row
  * [maxsnew/cargo-dot](https://github.com/maxsnew/cargo-dot) — generate graphs of a Cargo project's dependencies [<img src="https://travis-ci.org/maxsnew/cargo-dot.svg?branch=master">](https://travis-ci.org/maxsnew/cargo-dot)
  * [killercup/cargo-edit](https://github.com/killercup/cargo-edit) [[cargo-edit](https://crates.io/crates/cargo-edit)] — allows you to add and list dependencies by reading/writing to your Cargo.toml file from the command line [<img src="https://travis-ci.org/killercup/cargo-edit.svg?branch=master">](https://travis-ci.org/killercup/cargo-edit)
  * [kbknapp/cargo-graph](https://github.com/kbknapp/cargo-graph) [[cargo-graph](https://crates.io/crates/cargo-graph)] — updated fork of `cargo-dot` with additional features [<img src="https://travis-ci.org/kbknapp/cargo-graph.svg?branch=master">](https://travis-ci.org/kbknapp/cargo-graph)
  * [imp/cargo-info](https://gitlab.com/imp/cargo-info) [[cargo-info](https://crates.io/crates/cargo-info)] — queries crates.io for crates details from command line [<img src="https://travis-ci.org/imp/cargo-info.svg?branch=master">](https://travis-ci.org/imp/cargo-info)
  * [regexident/cargo-modules](https://github.com/regexident/cargo-modules) [[cargo-modules](https://crates.io/crates/cargo-modules)] — A cargo plugin for showing a tree-like overview of a crate's modules. [<img src="https://travis-ci.org/regexident/cargo-modules.svg?branch=master">](https://travis-ci.org/regexident/cargo-modules)
  * [imp/cargo-multi](https://github.com/imp/cargo-multi) [[cargo-multi](https://crates.io/crates/cargo-multi)] — runs specified cargo command on multiple crates [<img src="https://travis-ci.org/imp/cargo-multi.svg?branch=master">](https://travis-ci.org/imp/cargo-multi)
  * [kbknapp/cargo-outdated](https://github.com/kbknapp/cargo-outdated) [[cargo-outdated](https://crates.io/crates/cargo-outdated)] — displays when newer versions of Rust dependencies are available, or out of date [<img src="https://travis-ci.org/kbknapp/cargo-outdated.svg?branch=master">](https://travis-ci.org/kbknapp/cargo-outdated)
  * [sunng87/cargo-release](https://github.com/sunng87/cargo-release) [[cargo-release](https://crates.io/crates/cargo-release)] — tool for releasing git-managed cargo project, build, tag, publish, doc and push
  * [DanielKeep/cargo-script](https://github.com/DanielKeep/cargo-script) [[cargo-script](https://crates.io/crates/cargo-script)] — lets people quickly and easily run Rust "scripts" which can make use of Cargo's package ecosystem
  * [passcod/cargo-watch](https://github.com/passcod/cargo-watch) [[cargo-watch](https://crates.io/crates/cargo-watch)] — utility for cargo to compile projects when sources change [<img src="https://travis-ci.org/passcod/cargo-watch.svg?branch=master">](https://travis-ci.org/passcod/cargo-watch)
  * [BurntSushi/cargo-benchcmp](https://github.com/BurntSushi/cargo-benchcmp) [[cargo-benchcmp](https://crates.io/crates/cargo-benchcmp)] — utility to compare Rust micro-benchmarks
* CMake
  * [SiegeLord/RustCMake](https://github.com/SiegeLord/RustCMake) — an example project showing usage of CMake with Rust [<img src="https://travis-ci.org/SiegeLord/RustCMake.svg?branch=master">](https://travis-ci.org/SiegeLord/RustCMake)


### Debugging

* GDB
  * [rust-gdb](https://github.com/rust-lang/rust/blob/master/src/etc/rust-gdb)
* LLDB
  * [lldb_batchmode.py](https://github.com/rust-lang/rust/blob/master/src/etc/lldb_batchmode.py) — allows to use LLDB in a way similar to GDB's batch mode.


### Embedded

* Cross compiling
  * [japaric/rust-cross](https://github.com/japaric/rust-cross) — everything you need to know about cross compiling Rust programs [<img src="https://travis-ci.org/japaric/rust-cross.svg?branch=master">](https://travis-ci.org/japaric/rust-cross)
  * [japaric/xargo](https://github.com/japaric/xargo) — effortless cross compilation of Rust programs to custom bare-metal targets like ARM Cortex-M [<img src="https://travis-ci.org/japaric/xargo.svg?branch=master">](https://travis-ci.org/japaric/xargo)
* Raspberry Pi
  * [Ogeon/rust-on-raspberry-pi](https://github.com/Ogeon/rust-on-raspberry-pi) — instructions for how to cross compile Rust projects for the Raspberry Pi .


### FFI

See also [Foreign Function Interface](https://doc.rust-lang.org/book/ffi.html),  [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) (a collection of examples of using code written in Rust from other languages) and [FFI examples written in Rust](https://github.com/alexcrichton/rust-ffi-examples).

* C
  * [Sean1708/rusty-cheddar](https://github.com/Sean1708/rusty-cheddar) — generates C header files from Rust source files [<img src="https://travis-ci.org/Sean1708/rusty-cheddar.svg?branch=master">](https://travis-ci.org/Sean1708/rusty-cheddar)
  * [Yamakaky/rust-bindgen](https://github.com/Yamakaky/rust-bindgen) — a Rust bindings generator [<img src="https://travis-ci.org/Yamakaky/rust-bindgen.svg?branch=master">](https://travis-ci.org/Yamakaky/rust-bindgen)
* C++
  * [servo/rust-bindgen](https://github.com/servo/rust-bindgen) —
* Erlang
  * [hansihe/Rustler](https://github.com/hansihe/Rustler) — safe Rust bridge for creating Erlang NIF functions [<img src="https://travis-ci.org/hansihe/Rustler.svg?branch=master">](https://travis-ci.org/hansihe/Rustler)
* Haskell
  * [mgattozzi/curryrs](https://github.com/mgattozzi/curryrs) — Bridge the gap between Haskell and Rust
  * [mgattozzi/haskellrs](https://github.com/mgattozzi/haskellrs) — Rust in Haskell FFI Example
  * [mgattozzi/rushs](https://github.com/mgattozzi/rushs) — Haskell in Rust FFI Example
* Java
  * [drrb/java-rust-example](https://github.com/drrb/java-rust-example) — use Rust from Java [<img src="https://travis-ci.org/drrb/java-rust-example.svg?branch=master">](https://travis-ci.org/drrb/java-rust-example)
  * [GravityScore/RustJNI](https://github.com/GravityScore/RustJNI) — use Java from Rust [<img src="https://travis-ci.org/GravityScore/RustJNI.svg?branch=master">](https://travis-ci.org/GravityScore/RustJNI)
  * [kud1ing/rucaja](https://github.com/kud1ing/rucaja) [[rucaja](https://crates.io/crates/rucaja)] — use Java from Rust [<img src="https://travis-ci.org/kud1ing/rucaja.svg?branch=master">](https://travis-ci.org/kud1ing/rucaja
  * [prevoty/jni-rs](https://github.com/prevoty/jni-rs) [[jni](https://crates.io/crates/jni)] — use Rust from Java [<img src="https://travis-ci.org/prevoty/jni-rs.svg?branch=master">](https://travis-ci.org/prevoty/jni-rs)
  * [sfackler/rust-jni-sys](https://github.com/sfackler/rust-jni-sys) [[jni-sys](https://crates.io/crates/jni-sys)] — Rust definitions corresponding to jni.h [<img src="https://travis-ci.org/sfackler/rust-jni-sys.svg?branch=master">](https://travis-ci.org/sfackler/rust-jni-sys)
* Lua
  * [jcmoyer/rust-lua53](https://github.com/jcmoyer/rust-lua53) — Lua 5.3 bindings for Rust [<img src="https://travis-ci.org/jcmoyer/rust-lua53.svg?branch=master">](https://travis-ci.org/jcmoyer/rust-lua53)
  * [kballard/rust-lua](https://github.com/kballard/rust-lua) — Safe Rust bindings to Lua 5.1 [<img src="https://travis-ci.org/kballard/rust-lua.svg">](https://travis-ci.org/kballard/rust-lua)
  * [tickbh/td_rlua](https://github.com/tickbh/td_rlua) — Zero-cost high-level lua 5.3 wrapper for Rust [<img src="https://api.travis-ci.org/tickbh/td_rlua.svg?branch=master">](https://travis-ci.org/tickbh/td_rlua)
  * [tomaka/hlua](https://github.com/tomaka/hlua) — Rust library to interface with Lua [<img src="https://travis-ci.org/tomaka/hlua.svg?branch=master">](https://travis-ci.org/tomaka/hlua)
* mruby
  * [anima-engine/mrusty](https://github.com/anima-engine/mrusty) — mruby safe bindings for Rust [<img src="https://travis-ci.org/anima-engine/mrusty.svg?branch=master">](https://travis-ci.org/anima-engine/mrusty)
* Node.js
  * [rustbridge/neon](https://github.com/rustbridge/neon) — use Rust from Node.js [<img src="https://travis-ci.org/rustbridge/neon.svg?branch=master">](https://travis-ci.org/rustbridge/neon)
* Objective-C
  * [SSheldon/rust-objc](https://github.com/SSheldon/rust-objc) — Objective-C Runtime bindings and wrapper for Rust
* Python
  * [dgrunwald/rust-cpython](https://github.com/dgrunwald/rust-cpython) — Python bindings [<img src="https://travis-ci.org/dgrunwald/rust-cpython.svg?branch=master">](https://travis-ci.org/dgrunwald/rust-cpython)
  * [lukemetz/rustpy](https://github.com/lukemetz/rustpy) — Python bindings [<img src="https://travis-ci.org/lukemetz/rustpy.svg?branch=master">](https://travis-ci.org/lukemetz/rustpy)
* R
  * [rustr/rustr](https://github.com/rustr/rustr) — use Rust from R, and use R in Rust [<img src="https://travis-ci.org/rustr/rustr.svg?branch=master">](https://travis-ci.org/rustr/rustr)
* Ruby
  * [d-unseductable/ruru](https://github.com/d-unseductable/ruru) — native Ruby extensions written in Rust [<img src="https://travis-ci.org/d-unseductable/ruru.svg?branch=master">](https://travis-ci.org/d-unseductable/ruru)
  * [rustbridge/helix](https://github.com/rustbridge/helix) — write Ruby classes in Rust [<img src="https://travis-ci.org/rustbridge/helix.svg?branch=master">](https://travis-ci.org/rustbridge/helix)


### IDEs

See also [http://areweideyet.com/](http://areweideyet.com/) and [Rust and IDEs](https://www.rust-lang.org/ides.html).

  * [Atom](https://atom.io/)
    * [zargony/atom-language-rust](https://github.com/zargony/atom-language-rust)
  * [Eclipse](https://eclipse.org/)
    * [RustDT](https://github.com/RustDT/RustDT) — [<img src="https://travis-ci.org/RustDT/RustDT.svg?branch=master">](https://travis-ci.org/RustDT/RustDT)
  * [NetBeans](https://netbeans.org/)
    * [drrb/rust-netbeans](https://github.com/drrb/rust-netbeans)
  * [IntelliJ](https://www.jetbrains.com/idea/)
    * [intellij-rust/intellij-rust](https://github.com/intellij-rust/intellij-rust) — [<img src="https://travis-ci.org/intellij-rust/intellij-rust.svg?branch=master">](https://travis-ci.org/intellij-rust/intellij-rust)
    * [intellij-rust/intellij-toml](https://github.com/intellij-rust/intellij-toml) — basic Toml support
    * [JustSid/AfterglowIntelliJ](https://github.com/JustSid/AfterglowIntelliJ) — custom icons
  * [Ride](https://github.com/madeso/ride) — [<img src="https://travis-ci.org/madeso/ride.svg?branch=master">](https://travis-ci.org/madeso/ride)
  * [SolidOak](https://github.com/oakes/SolidOak) — a simple IDE for Rust, based on GTK+ and [Neovim](https://github.com/neovim/neovim)
  * [Vim](http://www.vim.org/) — the ubiquitous text editor
	* [rust.vim](https://github.com/rust-lang/rust.vim) — provides file detection, syntax highlighting, formatting, Syntastic integration, and more.
	* [vim-cargo](https://github.com/timonv/vim-cargo) — command bindings to quickly run cargo stuff from vim.
	* [vim-racer](https://github.com/racer-rust/vim-racer) — allows vim to use [Racer](https://github.com/phildawes/racer) for Rust code completion and navigation.
  * Visual Studio
    * [PistonDevelopers/VisualRust](https://github.com/PistonDevelopers/VisualRust) — a Visual Studio extension for Rust [<img src="https://travis-ci.org/PistonDevelopers/VisualRust.svg?branch=master">](https://travis-ci.org/PistonDevelopers/VisualRust)
  * [Visual Studio Code](https://code.visualstudio.com/)
    * [saviorisdead/RustyCode](https://github.com/saviorisdead/RustyCode)

### Pattern Recognition

* [sfikas/rusteval](https://github.com/sfikas/rusteval) — A tool used to evaluate the output of retrieval algorithms [![Build Status](https://travis-ci.org/sfikas/rusteval.svg?branch=master)](https://travis-ci.org/sfikas/rusteval)

### Profiling

* [ellisonch/rust-stopwatch](https://github.com/ellisonch/rust-stopwatch) — a stopwatch library [<img src="https://travis-ci.org/ellisonch/rust-stopwatch.svg?branch=master">](https://travis-ci.org/ellisonch/rust-stopwatch)
* FlameGraphs
  * [mrhooray/torch](https://github.com/mrhooray/torch) — generates FlameGraphs based on DWARF Debug Info
  * [TyOverby/flame](https://github.com/TyOverby/flame) — [<img src="https://travis-ci.org/TyOverby/flame.svg?branch=master">](https://travis-ci.org/TyOverby/flame)


### Testing

[[testing](https://crates.io/keywords/testing)]

* [BurntSushi/quickcheck](https://github.com/BurntSushi/quickcheck) [[quickcheck](https://crates.io/crates/quickcheck)] — a Rust implementation of [QuickCheck](https://wiki.haskell.org/Introduction_to_QuickCheck1) [<img src="https://travis-ci.org/BurntSushi/quickcheck.svg?branch=master">](https://travis-ci.org/BurntSushi/quickcheck)
* [farcaller/shiny](https://github.com/farcaller/shiny) — a fancy syntax similar to Ruby's Rspec or Objective-C' kiwi [<img src="https://travis-ci.org/farcaller/shiny.svg?branch=master">](https://travis-ci.org/farcaller/shiny)
* [frewsxcv/afl.rs](https://github.com/frewsxcv/afl.rs) — a Rust fuzzer, using [AFL](http://lcamtuf.coredump.cx/afl/) [<img src="https://api.travis-ci.org/frewsxcv/afl.rs.svg?branch=master">](https://travis-ci.org/frewsxcv/afl.rs)
* [reem/stainless](https://github.com/reem/stainless) [[stainless](https://crates.io/crates/stainless)] — Organized, flexible testing framework [<img src="https://travis-ci.org/reem/stainless.svg?branch=master">](https://travis-ci.org/reem/stainless)


## Libraries

### Astronomy

[[astronomy](https://crates.io/keywords/astronomy)]

* [saurvs/astro-rust](https://github.com/saurvs/astro-rust) — astronomy for Rust [<img src="https://travis-ci.org/saurvs/astro-rust.svg?branch=master">](https://travis-ci.org/saurvs/astro-rust)
* [mindriot101/rust-fitsio](https://github.com/mindriot101/rust-fitsio) [[fitsio](https://crates.io/crates/fitsio)] — fits interface library wrapping cfitsio [<img src="https://travis-ci.org/mindriot101/rust-fitsio.svg?branch=master">](https://travis-ci.org/mindriot101/rust-fitsio)
* [flosse/rust-sun](https://github.com/flosse/rust-sun) — A rust port of the JS library suncalc [<img src="https://travis-ci.org/flosse/rust-sun.svg?branch=master">](https://travis-ci.org/flosse/rust-sun)


### Asynchronous

* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) — a coroutine I/O library with a working-stealing scheduler [<img src="https://travis-ci.org/zonyitoo/coio-rs.svg?branch=master">](https://travis-ci.org/zonyitoo/coio-rs)
* [dpc/mioco](https://github.com/dpc/mioco) — Scalable, coroutine-based, asynchronous IO handling library [<img src="https://img.shields.io/travis/dpc/mioco/master.svg?style=flat-square" alt="Travis CI Build Status">](https://travis-ci.org/dpc/mioco)
* [alexcrichton/futures-rs](https://github.com/alexcrichton/futures-rs) — Zero-cost futures in Rust [<img src="https://travis-ci.org/alexcrichton/futures-rs.svg?branch=master" alt="Travis CI Build Status">](https://travis-ci.org/alexcrichton/futures-rs)
* [carllerche/mio](https://github.com/carllerche/mio) — MIO is a lightweight IO library for Rust with a focus on adding as little overhead as possible over the OS abstractions [<img src="https://travis-ci.org/carllerche/mio.svg?branch=master">](https://travis-ci.org/carllerche/mio)


### Audio

[[audio](https://crates.io/keywords/audio)]

* [GuillaumeGomez/rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) — [FMOD](http://www.fmod.org/) bindings [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-fmod.svg?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-fmod)
* [jhasse/ears](https://github.com/jhasse/ears) — a simple library to play Sounds and Musics, on top of OpenAL and libsndfile [<img src="https://travis-ci.org/jhasse/ears.svg?branch=master">](https://travis-ci.org/jhasse/ears)
* [jpernst/openal-rs](https://github.com/jpernst/openal-rs) — [OpenAL 1.1](http://www.openal.org/) bindings [<img src="https://travis-ci.org/jpernst/openal-rs.svg?branch=master">](https://travis-ci.org/jpernst/openal-rs)
* [musitdev/portmidi-rs](https://github.com/musitdev/portmidi-rs) — [PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings [<img src="https://travis-ci.org/musitdev/portmidi-rs.svg?branch=master">](https://travis-ci.org/musitdev/portmidi-rs)
* [RustAudio](https://github.com/RustAudio)
  * [RustAudio/rust-portaudio](https://github.com/RustAudio/rust-portaudio) — [PortAudio](http://www.portaudio.com/) bindings [<img src="https://travis-ci.org/RustAudio/rust-portaudio.svg?branch=master">](https://travis-ci.org/RustAudio/rust-portaudio)


### Authentication

* [keats/rust-jwt](https://github.com/keats/rust-jwt) — [JSON Web Token](https://en.wikipedia.org/wiki/JSON_Web_Token) lib in rust  [![Build Status](https://api.travis-ci.org/Keats/rust-jwt.svg?branch=master)](https://travis-ci.org/Keats/rust-jwt)
* [hngiang/rust-accountkit](https://github.com/hngiang/rust-accountkit) — An implementation [Facebook AccountKit](https://developers.facebook.com/docs/accountkit) for Rust [![Build Status](https://travis-ci.org/hngiang/rust-accountkit.svg?branch=master)](https://travis-ci.org/hngiang/rust-accountkit)

### Bioinformatics

* [Rust-Bio](https://github.com/rust-bio) — bioinformatics libraries in Rust.


### Caching

* [jaysonsantos/bmemcached-rs](https://github.com/jaysonsantos/bmemcached-rs) — Memcached library written in pure rust [<img src="https://travis-ci.org/jaysonsantos/bmemcached-rs.svg?branch=master">](https://travis-ci.org/jaysonsantos/bmemcached-rs)


### Concurrency

* [aturon/crossbeam](https://github.com/aturon/crossbeam) – Support for parallelism and low-level concurrency in Rust [<img src="https://travis-ci.org/aturon/crossbeam.svg?branch=master">](https://travis-ci.org/aturon/crossbeam)
* [nikomatsakis/rayon](https://github.com/nikomatsakis/rayon) – A data parallelism library for Rust [<img src="https://travis-ci.org/nikomatsakis/rayon.svg?branch=master">](https://travis-ci.org/nikomatsakis/rayon)
* [rustcc/coroutine-rs](https://github.com/rustcc/coroutine-rs) – Coroutine Library in Rust [<img src="https://img.shields.io/travis/rustcc/coroutine-rs.svg">](https://travis-ci.org/rustcc/coroutine-rs)
* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) – Coroutine I/O for Rust [<img src="https://travis-ci.org/zonyitoo/coio-rs.svg?branch=master">](https://travis-ci.org/zonyitoo/coio-rs)


### Cloud

* AWS [[aws](https://crates.io/keywords/aws)]
  * [rusoto/rusoto](https://github.com/rusoto/rusoto) — [<img src="https://travis-ci.org/rusoto/rusoto.svg?branch=master">](https://travis-ci.org/rusoto/rusoto)
* DigitalOcean
  * [kbknapp/doapi](https://github.com/kbknapp/doapi-rs) — DigitalOcean v2 API bindings [<img src="https://travis-ci.org/kbknapp/doapi-rs.svg?branch=master">](https://travis-ci.org/kbknapp/doapi-rs)


### Command-line argument parsing

* [docopt/docopt.rs](https://github.com/docopt/docopt.rs) — a Rust implementation of [DocOpt](http://docopt.org) [<img src="https://travis-ci.org/docopt/docopt.rs.svg?branch=master">](https://travis-ci.org/docopt/docopt.rs)
* [kbknapp/clap-rs](https://github.com/kbknapp/clap-rs) — a simple to use, full featured command-line argument parser [<img src="https://travis-ci.org/kbknapp/clap-rs.svg?branch=master">](https://travis-ci.org/kbknapp/clap-rs)


### Command-line interface

* [kkawakam/rustyline](https://github.com/kkawakam/rustyline) — Readline Implementation in Rust [![Build Status](https://travis-ci.org/kkawakam/rustyline.svg?branch=master)](https://travis-ci.org/kkawakam/rustyline)
* [srijs/rust-copperline](https://github.com/srijs/rust-copperline) — pure-Rust Command Line Editing Library


### Compression

* brotli
  * [ende76/brotli-rs](https://github.com/ende76/brotli-rs) — implementation of [Brotli](http://google-opensource.blogspot.co.uk/2015/09/introducing-brotli-new-compression.html) compression
  * [dropbox/rust-brotli](https://github.com/dropbox/rust-brotli) — Brotli decompressor in Rust that optionally avoids the stdlib
* bzip2
  * [alexcrichton/bzip2-rs](https://github.com/alexcrichton/bzip2-rs) — [libbz2](http://www.bzip.org) bindings [<img src="https://travis-ci.org/alexcrichton/bzip2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/bzip2-rs)
* miniz
  * [alexcrichton/flate2-rs](https://github.com/alexcrichton/flate2-rs) — [miniz](https://code.google.com/p/miniz/) bindings [<img src="https://travis-ci.org/alexcrichton/flate2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/flate2-rs)
* snappy
  * [JeffBelgum/rust-snappy](https://github.com/JeffBelgum/rust-snappy) — [snappy](https://github.com/google/snappy) bindings [<img src="https://travis-ci.org/JeffBelgum/rust-snappy.svg?branch=master">](https://travis-ci.org/JeffBelgum/rust-snappy)
* tar
  * [alexcrichton/tar-rs](https://github.com/alexcrichton/tar-rs) — tar archive reading/writing in Rust [<img src="https://travis-ci.org/alexcrichton/tar-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/tar-rs)
* zip
  * [slackito/zip](https://github.com/slackito/zip) — read and write ZIP archives [<img src="https://travis-ci.org/slackito/zip.svg?branch=master">](https://travis-ci.org/slackito/zip)


### Computation

* [BLAS](https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) [[blas](https://crates.io/keywords/blas)]
  * [mikkyang/rust-blas](https://github.com/mikkyang/rust-blas) — BLAS bindings
  * [stainless-steel/blas](https://github.com/stainless-steel/blas) — BLAS bindings [<img src="https://travis-ci.org/stainless-steel/blas.svg?branch=master">](https://travis-ci.org/stainless-steel/blas)
* [GMP](https://gmplib.org/)
  * [thestinger/rust-gmp](https://github.com/thestinger/rust-gmp) — libgmp bindings [<img src="https://travis-ci.org/thestinger/rust-gmp.svg?branch=master">](https://travis-ci.org/thestinger/rust-gmp)
* [GSL](http://www.gnu.org/software/gsl/)
  * [GuillaumeGomez/rust-GSL](https://github.com/GuillaumeGomez) — GSL bindings [<img src="https://travis-ci.org/GuillaumeGomez/rust-GSL.svg?branch=master">](https://travis-ci.org/GuillaumeGomez/rust-GSL)
* [LAPACK](https://en.wikipedia.org/wiki/LAPACK)
  * [stainless-steel/lapack](https://github.com/stainless-steel/lapack) — LAPACK bindings [<img src="https://travis-ci.org/stainless-steel/lapack.svg?branch=master">](https://travis-ci.org/stainless-steel/lapack)
* Parallel
  * [arrayfire/arrayfire-rust](https://github.com/arrayfire/arrayfire-rust) — [Arrayfire](http://arrayfire.com) bindings
  * [autumnai/collenchyma](https://github.com/autumnai/collenchyma) — An extensible, pluggable, backend-agnostic framework for parallel, high-performance computations on CUDA, OpenCL and common host CPU. [<img src="https://travis-ci.org/autumnai/collenchyma.svg?branch=master">](https://travis-ci.org/autumnai/collenchyma)
  * [luqmana/rust-opencl](https://github.com/luqmana/rust-opencl) — [OpenCL](https://www.khronos.org/opencl/) bindings [<img src="https://travis-ci.org/luqmana/rust-opencl.svg?branch=master">](https://travis-ci.org/luqmana/rust-opencl)
* Scirust
  * [indigits/scirust](https://github.com/indigits/scirust) — scientific computing library in Rust [![Build Status](https://travis-ci.org/indigits/scirust.svg?branch=master)](https://travis-ci.org/indigits/scirust)
* Statrs
  * [boxtown/statrs](https://github.com/boxtown/statrs) — Robust statistical computation library in Rust [![Build Status](https://travis-ci.org/boxtown/statrs.svg?branch=master)](https://travis-ci.org/boxtown/statrs)
* Rustimization [[rustimization](https://crates.io/crates/rustimization)]
  * [noshu/rustimization](https://github.com/noshu/rustimization) — A rust optimization library which includes L-BFGS-B and Conjugate Gradient algorithm
* [L-BFGS-B](https://en.wikipedia.org/wiki/Limited-memory_BFGS) [[lbfgsb](http://users.iems.northwestern.edu/~nocedal/lbfgsb.html)]
  * [noshu/lbfgsb-sys](https://github.com/noshu/lbfgsb-sys) — Rust binding of fortran L-BFGS-B subroutine
* [Conjugate Gradient](https://en.wikipedia.org/wiki/Limited-memory_BFGS) [[CG+](http://users.iems.northwestern.edu/~nocedal/CG+.html)]
  * [noshu/cg-sys](https://github.com/noshu/cg-sys) — Rust binding of fortran CG+ subroutine

### Cryptography

[[crypto](https://crates.io/keywords/crypto), [cryptography](https://crates.io/keywords/cryptography)]

* [briansmith/ring](https://github.com/briansmith/ring) — Safe, fast, small crypto using Rust and BoringSSL's cryptography primitives. [<img src="https://travis-ci.org/briansmith/ring.svg?branch=master">](https://travis-ci.org/briansmith/ring)
* [briansmith/webpki](https://github.com/briansmith/webpki) — Web PKI TLS X.509 certificate validation in Rust. [<img src="https://travis-ci.org/briansmith/webpki.svg?branch=master">](https://travis-ci.org/briansmith/webpki)
* [ctz/rustls](https://github.com/ctz/rustls) — a Rust implementation of TLS
* [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) — cryptographic algorithms in Rust [<img src="https://travis-ci.org/DaGenix/rust-crypto.svg?branch=master">](https://travis-ci.org/DaGenix/rust-crypto)
* [dnaq/sodiumoxide](https://github.com/dnaq/sodiumoxide) — [libsodium](https://github.com/jedisct1/libsodium) bindings [<img src="https://travis-ci.org/dnaq/sodiumoxide.svg?branch=master">](https://travis-ci.org/dnaq/sodiumoxide)
* [klutzy/suruga](https://github.com/klutzy/suruga) — a Rust implementation of [TLS 1.2](http://tools.ietf.org/html/rfc5246)
* [libOctavo/octavo](https://github.com/libOctavo/octavo) — Modular hash and crypto library in Rust [<img src="https://api.travis-ci.org/libOctavo/octavo.svg?branch=master">](https://travis-ci.org/libOctavo/octavo)
* [sfackler/rust-native-tls](https://github.com/sfackler/rust-native-tls) — Bindings for native TLS libraries
* [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) — [OpenSSL](https://www.openssl.org/) bindings [<img src="https://travis-ci.org/sfackler/rust-openssl.svg?branch=master">](https://travis-ci.org/sfackler/rust-openssl)
* [sfackler/rust-security-framework](https://github.com/sfackler/rust-security-framework) — Bindings for Security Framework (OSX native crypto)
* [steffengy/schannel-rs](https://github.com/steffengy/schannel-rs) — Bindings for Schannel (Windows native TLS)


### Database

[[database](https://crates.io/keywords/database)]

* [sfackler/r2d2](https://github.com/sfackler/r2d2) — generic connection pool [<img src="https://travis-ci.org/sfackler/r2d2.svg?branch=master">](https://travis-ci.org/sfackler/r2d2)
* NoSQL [[nosql](https://crates.io/keywords/nosql)]
  * [Cassandra](http://cassandra.apache.org) [[cassandra](https://crates.io/keywords/cassandra), [cql](https://crates.io/keywords/cql)]
    * [AlexPikalov/cdrs](https://github.com/AlexPikalov/cdrs) [[cdrs](https://crates.io/crates/cdrs)] — native client written in Rust [<img src="https://travis-ci.org/AlexPikalov/cdrs.svg?branch=master">](https://travis-ci.org/AlexPikalov/cdrs)
    * [tupshin/cassandra-rust](https://github.com/tupshin/cassandra-rs) —  bindings to the C++ client
  * CouchDB [[couchdb](https://crates.io/keywords/couchdb)]
    * [chill-rs/chill](https://github.com/chill-rs/chill) [[couchdb](https://crates.io/crates/chill)] — a Rust client for the CouchDB REST API [<img src="https://travis-ci.org/chill-rs/chill.svg?branch=master">](https://travis-ci.org/chill-rs/chill)
  * Elasticsearch [[elasticsearch](https://crates.io/keywords/elasticsearch)]
    * [benashford/rs-es](https://github.com/benashford/rs-es) [[rs-es](https://crates.io/crates/rs-es)] — a Rust client for the [Elastic](https://www.elastic.co/) REST API [<img src="https://travis-ci.org/benashford/rs-es.svg?branch=master">](https://travis-ci.org/benashford/rs-es)
    * [elastic-rs/elastic-hyper](https://github.com/elastic-rs/elastic-hyper) [[elastic_hyper](https://crates.io/crates/elastic_hyper)] — a lightweight implementation of the Elasticsearch API based on Hyper [<img src="https://travis-ci.org/elastic-rs/elastic-hyper">](https://travis-ci.org/elastic-rs/elastic-hyper)
  * etcd
    * [jimmycuadra/rust-etcd](https://github.com/jimmycuadra/rust-etcd) [[etcd](https://crates.io/crates/etcd)] — A client library for CoreOS's etcd. [<img src="https://travis-ci.org/jimmycuadra/rust-etcd.svg?branch=master">](https://travis-ci.org/jimmycuadra/rust-etcd)
  * ForestDB
    * [vhbit/sherwood](https://github.com/vhbit/sherwood) — [ForestDB](https://github.com/couchbase/forestdb) bindings [<img src="https://travis-ci.org/vhbit/sherwood.svg?branch=master">](https://travis-ci.org/vhbit/sherwood)
  * LevelDB
    * [skade/leveldb](https://github.com/skade/leveldb) — [LevelDB](https://github.com/google/leveldb) bindings [<img src="https://travis-ci.org/skade/leveldb.svg?branch=master">](https://travis-ci.org/skade/leveldb)
  * LMDB [[lmdb](https://crates.io/keywords/lmdb)]
    * [vhbit/lmdb-rs](https://github.com/vhbit/lmdb-rs) [[lmdb-rs](https://crates.io/crates/lmdb-rs)] — [LMDB](http://symas.com/mdb/) bindings [<img src="https://travis-ci.org/vhbit/lmdb-rs.svg?branch=master">](https://travis-ci.org/vhbit/lmdb-rs)
  * MongoDB [[mongodb](https://crates.io/keywords/mongodb)]
    * [mongodb-labs/mongo-rust-driver-prototype](https://github.com/mongodb-labs/mongo-rust-driver-prototype) [[mongodb](https://crates.io/crates/mongodb)] — [MongoDB](https://www.mongodb.org/) bindings [<img src="https://travis-ci.org/mongodb-labs/mongo-rust-driver-prototype.svg">](https://travis-ci.org/mongodb-labs/mongo-rust-driver-prototype)
  * Neo4j [[cypher](https://crates.io/keywords/cypher), [neo4j](https://crates.io/keywords/neo4j)]
  * Redis [[redis](https://crates.io/keywords/redis)]
    * [mitsuhiko/redis-rs](https://github.com/mitsuhiko/redis-rs) — [Redis](http://redis.io) library in Rust [<img src="https://travis-ci.org/mitsuhiko/redis-rs.svg?branch=master">](https://travis-ci.org/mitsuhiko/redis-rs)
  * [RocksDB](http://rocksdb.org/)
    * [spacejam/rust-rocksdb](https://github.com/spacejam/rust-rocksdb) — RocksDB bindings [<img src="https://travis-ci.org/spacejam/rust-rocksdb.svg?branch=master">](https://travis-ci.org/spacejam/rust-rocksdb)
  * [UnQLite](http://unqlite.org)
    * [zitsen/unqlite.rs](https://github.com/zitsen/unqlite.rs) — UnQLite bindings [<img src="https://travis-ci.org/zitsen/unqlite.rs.svg?branch=master">](https://travis-ci.org/zitsen/unqlite.rs)
  * [ZooKeeper](https://zookeeper.apache.org/)
    * [bonifaido/rust-zookeeper](https://github.com/bonifaido/rust-zookeeper) [[zookeeper](https://crates.io/crates/zookeeper)] — A client library for Apache ZooKeeper. [<img src="https://travis-ci.org/bonifaido/rust-zookeeper.svg?branch=master">](https://travis-ci.org/bonifaido/rust-zookeeper)
* SQL [[sql](https://crates.io/keywords/sql)]
  * Microsoft SQL
    * [steffengy/tiberius](https://github.com/steffengy/tiberius) — [<img src="https://travis-ci.org/steffengy/tiberius.svg?branch=master">](https://travis-ci.org/steffengy/tiberius)
  * MySql [[mysql](https://crates.io/keywords/mysql)]
    * [AgilData/mysql-proxy-rs](https://github.com/AgilData/mysql-proxy-rs) — a MySQL Proxy [<img src="https://travis-ci.org/AgilData/mysql-proxy-rs.svg?branch=master">](https://travis-ci.org/AgilData/mysql-proxy-rs)
    * [blackbeam/mysql_async](https://github.com/blackbeam/mysql_async) [[mysql_async](https://crates.io/crates/mysql_async)] — asyncronous Rust Mysql driver based on Tokio. [<img src="https://travis-ci.org/blackbeam/mysql_async.svg?branch=master">](https://travis-ci.org/blackbeam/mysql_async)
    * [blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) [[mysql](https://crates.io/crates/mysql)] — a native MySql client [<img src="https://travis-ci.org/blackbeam/rust-mysql-simple.svg?branch=master">](https://travis-ci.org/blackbeam/rust-mysql-simple)
 * ORM [[orm](https://crates.io/keywords/orm)]
    * [deuterium-orm/deuterium](https://github.com/deuterium-orm/deuterium) — an SQL query builder for Rust  [![Build Status](https://travis-ci.org/deuterium-orm/deuterium.svg?branch=master)](https://travis-ci.org/deuterium-orm/deuterium)
    * [diesel-rs/diesel](https://github.com/diesel-rs/diesel) — an ORM and Query builder for Rust [![Build Status](https://api.travis-ci.org/diesel-rs/diesel.svg)](https://travis-ci.org/diesel-rs/diesel)
    * [ivanceras/rustorm](https://github.com/ivanceras/rustorm) — an ORM for Rust [![Build Status](https://api.travis-ci.org/ivanceras/rustorm.svg)](https://travis-ci.org/ivanceras/rustorm)
  * PostgreSql [[postgres](https://crates.io/keywords/postgres), [postgresql](https://crates.io/keywords/postgresql)]
    * [sfackler/rust-postgres](https://github.com/sfackler/rust-postgres) [[postgres](https://crates.io/crates/postgres)] — a native [PostgreSQL](http://www.postgresql.org) client [<img src="https://travis-ci.org/sfackler/rust-postgres.svg?branch=master">](https://travis-ci.org/sfackler/rust-postgres)
  * Sqlite [[sqlite](https://crates.io/keywords/sqlite)]
    * [dckc/rust-sqlite3](https://github.com/dckc/rust-sqlite3) — [Sqlite3](http://www.sqlite.org/) bindings [<img src="https://travis-ci.org/dckc/rust-sqlite3.svg?branch=master">](https://travis-ci.org/dckc/rust-sqlite3)
    * [jgallagher/rusqlite](https://github.com/jgallagher/rusqlite) — [Sqlite3](http://www.sqlite.org/) bindings [<img src="https://travis-ci.org/jgallagher/rusqlite.svg?branch=master">](https://travis-ci.org/jgallagher/rusqlite)


### Data structures

* [bluss/rust-itertools](https://github.com/bluss/rust-itertools) — [<img src="https://travis-ci.org/bluss/rust-itertools.svg?branch=master">](https://travis-ci.org/bluss/rust-itertools)
* [contain-rs](https://github.com/contain-rs) — Extension of Rust's std::collections
* [fizyk20/generic-array](https://github.com/fizyk20/generic-array) – a hack to allow for arrays sized by typenums [<img src="https://travis-ci.org/fizyk20/generic-array.svg?branch=master">](https://travis-ci.org/fizyk20/generic-array)
* [Nemo157/roaring-rs](https://github.com/Nemo157/roaring-rs) – Roaring Bitmaps in Rust [<img src="https://travis-ci.org/Nemo157/roaring-rs.svg?branch=master">](https://travis-ci.org/Nemo157/roaring-rs)
* [reem/rust-typemap](https://github.com/reem/rust-typemap) — [<img src="https://travis-ci.org/reem/rust-typemap.svg?branch=master">](https://travis-ci.org/reem/rust-typemap)
* [serde-rs/serde](https://github.com/serde-rs/serde) — a framework to generically serialize Rust data structures [<img src="https://api.travis-ci.org/serde-rs/serde.svg?branch=master">](https://travis-ci.org/serde-rs/serde)


### Date and time

[[date](https://crates.io/keywords/date), [time](https://crates.io/keywords/time)]

* [lifthrasiir/rust-chrono](https://github.com/lifthrasiir/rust-chrono) — [<img src="https://travis-ci.org/lifthrasiir/rust-chrono.svg?branch=master">](https://travis-ci.org/lifthrasiir/rust-chrono)
* [rust-lang-deprecated/time](https://github.com/rust-lang-deprecated/time) — [<img src="https://travis-ci.org/rust-lang-deprecated/time.svg?branch=master">](https://travis-ci.org/rust-lang-deprecated/time)


### Distributed Systems

* Apache Kafka
  * [spicavigo/kafka-rust](https://github.com/spicavigo/kafka-rust) — [<img src="https://travis-ci.org/spicavigo/kafka-rust.svg?branch=master">](https://travis-ci.org/spicavigo/kafka-rust)
* Beanstalkd
  * [schickling/rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) — [Beanstalkd](https://github.com/kr/beanstalkd) bindings [<img src="https://travis-ci.org/schickling/rust-beanstalkd.svg?branch=master">](https://travis-ci.org/schickling/rust-beanstalkd)
* HDFS
  * [hyunsik/hdfs-rs](https://github.com/hyunsik/hdfs-rs) — libhdfs bindings [<img src="https://travis-ci.org/hyunsik/hdfs-rs.svg?branch=master">](https://travis-ci.org/hyunsik/hdfs-rs)


### Email

[[email](https://crates.io/keywords/email)]

* [gsquire/sendgrid-rs](https://github.com/gsquire/sendgrid-rs) — unofficial Rust library for SendGrid API [<img src="https://travis-ci.org/gsquire/sendgrid-rs.svg?branch=master">](https://travis-ci.org/gsquire/sendgrid-rs)
* [lettre/lettre](https://github.com/lettre/lettre) — an SMTP-library for Rust [<img src="https://travis-ci.org/lettre/lettre.svg?branch=master">](https://travis-ci.org/lettre/lettre)
* [staktrace/mailparse](https://github.com/staktrace/mailparse) [[mailparse](https://crates.io/crates/mailparse)] — a library for parsing real-world email files [<img src="https://travis-ci.org/staktrace/mailparse.svg?branch=master">](https://travis-ci.org/staktrace/mailparse)

### Encoding

[[encoding](https://crates.io/keywords/encoding)]

* ASN.1
  * [alex/rust-asn1](https://github.com/alex/rust-asn1) — a Rust ASN.1 (DER) serializer [<img src="https://travis-ci.org/alex/rust-asn1.svg?branch=master">](https://travis-ci.org/alex/rust-asn1)
* Bencode
  * [arjantop/rust-bencode](https://github.com/arjantop/rust-bencode) — [Bencode](https://en.wikipedia.org/wiki/Bencode) implementation in Rust [<img src="https://travis-ci.org/arjantop/rust-bencode.svg?branch=master">](https://travis-ci.org/arjantop/rust-bencode)
* Binary
  * [arcnmx/nue](https://github.com/arcnmx/nue) — I/O and binary data encoding for Rust [<img src="https://travis-ci.org/arcnmx/nue.svg?branch=master">](https://travis-ci.org/arcnmx/nue)
  * [TyOverby/bincode](https://github.com/TyOverby/bincode) — a binary encoder/decoder in Rust [<img src="https://travis-ci.org/TyOverby/bincode.svg?branch=master">](https://travis-ci.org/TyOverby/bincode)
* BSON
  * [zonyitoo/bson-rs](https://github.com/zonyitoo/bson-rs) — [<img src="https://travis-ci.org/zonyitoo/bson-rs.svg?branch=master">](https://travis-ci.org/zonyitoo/bson-rs)
* Byte swapping
  * [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder) — Supports big-endian, little-endian and native byte orders [<img src="https://travis-ci.org/BurntSushi/byteorder.svg?branch=master">](https://travis-ci.org/BurntSushi/byteorder)
* Cap'n Proto
  * [dwrensha/capnproto-rust](https://github.com/dwrensha/capnproto-rust) — [<img src="https://travis-ci.org/dwrensha/capnproto-rust.svg?branch=master">](https://travis-ci.org/dwrensha/capnproto-rust)
* CBOR
  * [BurntSushi/rust-cbor](https://github.com/BurntSushi/rust-cbor) — Supports JSON conversion and type-based encoding/decoding [<img src="https://travis-ci.org/BurntSushi/rust-cbor.svg?branch=master">](https://travis-ci.org/BurntSushi/rust-cbor)
* Character Encoding
  * [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding) — [<img src="https://travis-ci.org/lifthrasiir/rust-encoding.svg?branch=master">](https://travis-ci.org/lifthrasiir/rust-encoding)
* CRC
  * [mrhooray/crc-rs](https://github.com/mrhooray/crc-rs) — [<img src="https://travis-ci.org/mrhooray/crc-rs.svg?branch=master">](https://travis-ci.org/mrhooray/crc-rs)
* CSV
  * [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) — [<img src="https://travis-ci.org/BurntSushi/rust-csv.svg?branch=master">](https://travis-ci.org/BurntSushi/rust-csv)
* HTML
  * [servo/html5ever](https://github.com/servo/html5ever) — High-performance browser-grade HTML5 parser [<img src="https://travis-ci.org/servo/html5ever.svg?branch=master">](https://travis-ci.org/servo/html5ever)
  * [veddan/rust-htmlescape](https://github.com/veddan/rust-htmlescape) — encoding/decoding HTML entities [<img src="https://travis-ci.org/veddan/rust-htmlescape.svg?branch=master">](https://travis-ci.org/veddan/rust-htmlescape)
* JSON
  * [serde-rs/json](https://github.com/serde-rs/json) [[serde\_json](https://crates.io/crates/serde_json)] — JSON support for [Serde](https://github.com/serde-rs/serde) framework [<img src="https://travis-ci.org/serde-rs/json.svg?branch=master">](https://travis-ci.org/serde-rs/json)
  * [maciejhirsz/json-rust](https://github.com/maciejhirsz/json-rust) [[json](https://crates.io/crates/json)] — JSON implementation in Rust [<img src="https://travis-ci.org/maciejhirsz/json-rust.svg?branch=master">](https://travis-ci.org/maciejhirsz/json-rust)
* Jsonnet
  * [Qihoo360/rust-jsonnet](https://github.com/Qihoo360/rust-jsonnet) —  [<img src="https://travis-ci.org/Qihoo360/rust-jsonnet.svg?branch=master">](https://travis-ci.org/Qihoo360/rust-jsonnet)
* MsgPack
  * [mneumann/rust-msgpack](https://github.com/mneumann/rust-msgpack) — [<img src="https://travis-ci.org/mneumann/rust-msgpack.svg?branch=master">](https://travis-ci.org/mneumann/rust-msgpack)
  * [3Hren/msgpack-rust](https://github.com/3Hren/msgpack-rust) — a pure Rust low/high level MessagePack implementation [<img src="https://travis-ci.org/3Hren/msgpack-rust.svg?branch=master">](https://travis-ci.org/3Hren/msgpack-rust)
* ProtocolBuffers
  * [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf) — [<img src="https://travis-ci.org/stepancheg/rust-protobuf.svg?branch=master">](https://travis-ci.org/stepancheg/rust-protobuf)
* RON (Rusty Object Notation)
  * [kvark/ron](https://github.com/kvark/ron) — [<img src="https://travis-ci.org/kvark/ron.svg?branch=master">](https://travis-ci.org/kvark/ron)
* Tnetstring
  * [erickt/rust-tnetstring](https://github.com/erickt/rust-tnetstring) — [<img src="https://travis-ci.org/erickt/rust-tnetstring.svg?branch=master">](https://travis-ci.org/erickt/rust-tnetstring)
* TOML
  * [alexcrichton/toml-rs](https://github.com/alexcrichton/toml-rs) — [<img src="https://travis-ci.org/alexcrichton/toml-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/toml-rs)
* XML
  * [Florob/RustyXML](https://github.com/Florob/RustyXML) — an XML parser written in Rust [<img src="https://travis-ci.org/Florob/RustyXML.svg?branch=master">](https://travis-ci.org/Florob/RustyXML)
  * [shepmaster/sxd-document](https://github.com/shepmaster/sxd-document) — An XML library in Rust [<img src="https://travis-ci.org/shepmaster/sxd-document.svg?branch=master">](https://travis-ci.org/shepmaster/sxd-document)
  * [shepmaster/sxd-xpath](https://github.com/shepmaster/sxd-xpath) — An XPath library in Rust [<img src="https://travis-ci.org/shepmaster/sxd-xpath.svg?branch=master">](https://travis-ci.org/shepmaster/sxd-xpath)
  * [netvl/xml-rs](https://github.com/netvl/xml-rs) — a streaming XML library [<img src="https://travis-ci.org/netvl/xml-rs.svg?branch=master">](https://travis-ci.org/netvl/xml-rs)
* YAML
  * [chyh1990/yaml-rust](https://github.com/chyh1990/yaml-rust) — The missing YAML 1.2 implementation for Rust. [<img src="https://travis-ci.org/chyh1990/yaml-rust.svg?branch=master">](https://travis-ci.org/chyh1990/yaml-rust)
  * [dtolnay/serde-yaml](https://github.com/dtolnay/serde-yaml) [[serde\_yaml](https://crates.io/crates/serde_yaml)] — YAML support for [Serde](https://github.com/serde-rs/serde) framework [<img src="https://travis-ci.org/dtolnay/serde-yaml.svg?branch=master">](https://travis-ci.org/dtolnay/serde-yaml)
  * [kimhyunkang/libyaml-rust](https://github.com/kimhyunkang/libyaml-rust) — [libyaml](http://pyyaml.org/wiki/LibYAML) bindings [<img src="https://travis-ci.org/kimhyunkang/libyaml-rust.svg?branch=master">](https://travis-ci.org/kimhyunkang/libyaml-rust)

### Filesystem

[[filesystem](https://crates.io/keywords/filesystem)]

* Temporary Files
  * [rust-lang-nursery/tempdir](https://github.com/rust-lang-nursery/tempdir) — temporary directory library [<img src="https://travis-ci.org/rust-lang-nursery/tempdir.svg?branch=master">](https://travis-ci.org/rust-lang-nursery/tempdir)
  * [Stebalien/tempfile](https://github.com/Stebalien/tempfile) — temporary file library [<img src="https://travis-ci.org/Stebalien/tempfile.svg?branch=master">](https://travis-ci.org/Stebalien/tempfile)
  * [Stebalien/xattr](https://github.com/Stebalien/xattr) [[xattr](https://crates.io/crates/xattr)] — list and manipulate unix extended file attributes [<img src="https://travis-ci.org/Stebalien/xattr.svg?branch=master">](https://travis-ci.org/Stebalien/xattr)

### Game development

* Allegro
  * [SiegeLord/RustAllegro](https://github.com/SiegeLord/RustAllegro) — [Allegro 5](http://liballeg.org/) bindings [<img src="https://travis-ci.org/SiegeLord/RustAllegro.svg?branch=master">](https://travis-ci.org/SiegeLord/RustAllegro)
* Challonge
  * [vityafx/challonge-rs](https://github.com/vityafx/challonge-rs) [[challonge](https://crates.io/crates/challonge)] — Client library for the Challonge REST API. Helps to organize tournaments. [<img src="https://travis-ci.org/vityafx/challonge-rs.svg?branch=master">](https://travis-ci.org/vityafx/challonge-rs)
* Corange
  * [lucidscape/corange-rs](https://github.com/lucidscape/corange-rs) — [Corange](https://github.com/orangeduck/Corange) bindings
* Entity-Component Systems (ECS)
  * [slide-rs/specs](https://github.com/slide-rs/specs) — Specs Parallel ECS [<img src="https://travis-ci.org/slide-rs/specs.svg">](httpsL//github.com/travis-ci.org/slide-rs/specs)
* Game Engines
  * [Amethyst](https://www.amethyst.rs) — Data-oriented game engine [<img src="https://travis-ci.org/amethyst/amethyst.svg?branch=master">](https://travis-ci.org/amethyst/amethyst)
  * [Piston](http://www.piston.rs) — [<img src="https://travis-ci.org/PistonDevelopers/piston.svg?branch=master">](https://travis-ci.org/PistonDevelopers/piston)
* SDL [[sdl](https://crates.io/keywords/sdl)]
  * [brson/rust-sdl](https://github.com/brson/rust-sdl) — [SDL1](http://www.libsdl.org/) bindings [<img src="https://travis-ci.org/brson/rust-sdl.svg?branch=master">](https://travis-ci.org/brson/rust-sdl)
  * [AngryLawyer/rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) — [SDL2](http://www.libsdl.org/) bindings [<img src="https://travis-ci.org/AngryLawyer/rust-sdl2.svg?branch=master">](https://travis-ci.org/AngryLawyer/rust-sdl2)
* SFML
  * [jeremyletang/rust-sfml](https://github.com/jeremyletang/rust-sfml) — [SFML](http://www.sfml-dev.org/) bindings [<img src="https://travis-ci.org/jeremyletang/rust-sfml.svg?branch=master">](https://travis-ci.org/jeremyletang/rust-sfml)
* Voxlap
  * [bbodi/rust-voxlap](https://github.com/bbodi/rust-voxlap) — [Voxlap](http://advsys.net/ken/voxlap.htm) bindings

### Geospatial

[[geo](https://crates.io/keywords/geo), [gis](https://crates.io/keywords/gis)]

* [Georust](https://github.com/georust) — geospatial tools and libraries written in Rust


### Graphics

[[graphics](https://crates.io.keywords/graphics)]

* [gfx-rs/gfx](https://github.com/gfx-rs/gfx) — A high-performance, bindless graphics API for Rust. [<img src="https://img.shields.io/travis/gfx-rs/gfx/master.svg">](https://travis-ci.org/gfx-rs/gfx)
* Font
  * [dylanede/rusttype](https://github.com/dylanede/rusttype) — a pure Rust alternative to libraries like FreeType [<img src="https://img.shields.io/travis/dylanede/rusttype/master.svg">](https://travis-ci.org/dylanede/rusttype)
* OpenGL [[opengl](https://crates.io/keywords/opengl)]
  * [brendanzab/gl-rs](https://github.com/brendanzab/gl-rs) — [<img src="https://travis-ci.org/brendanzab/gl-rs.svg?branch=master">](https://travis-ci.org/brendanzab/gl-rs)
  * [PistonDevelopers/glfw-rs](https://github.com/PistonDevelopers/glfw-rs) — [<img src="https://travis-ci.org/PistonDevelopers/glfw-rs.svg?branch=master">](https://travis-ci.org/PistonDevelopers/glfw-rs)
  * [tomaka/glium](https://github.com/tomaka/glium) — safe OpenGL wrapper for the Rust language. [<img src="https://travis-ci.org/tomaka/glium.svg?branch=master">](https://travis-ci.org/tomaka/glium)
  * [tomaka/glutin](https://github.com/tomaka/glutin) — Rust alternative to [GLFW](http://www.glfw.org/) [<img src="https://travis-ci.org/tomaka/glutin.svg?branch=master">](https://travis-ci.org/tomaka/glutin)
* PDF
  * [kaj/rust-pdf](https://github.com/kaj/rust-pdf) — [<img src="https://travis-ci.org/kaj/rust-pdf.svg?branch=master">](https://travis-ci.org/kaj/rust-pdf)
* [Vulkan](https://www.khronos.org/vulkan/) [[vulkan](https://crates.io/keywords/vulkan)]
  * [tomaka/vulkano](https://github.com/tomaka/vulkano) [[vulkano](https://crates.io/crates/vulkano)] — [<img src="https://travis-ci.org/tomaka/vulkano.svg?branch=master">](https://travis-ci.org/tomaka/vulkano)


### GUI

[[gui](https://crates.io/keywords/gui)]

* [PistonDevelopers/conrod](https://github.com/PistonDevelopers/conrod/) — An easy-to-use, immediate-mode, 2D GUI library written entirely in Rust [<img src="https://travis-ci.org/PistonDevelopers/conrod.svg?branch=master">](https://travis-ci.org/PistonDevelopers/conrod)
* Cocoa
  * [kylewlacy/sorbet-cocoa](https://github.com/kylewlacy/sorbet-cocoa) — [<img src="https://travis-ci.org/kylewlacy/sorbet-cocoa.svg?branch=master">](https://travis-ci.org/kylewlacy/sorbet-cocoa)
  * [servo/cocoa-rs](https://github.com/servo/cocoa-rs)
* [IUP](http://webserver2.tecgraf.puc-rio.br/iup/)
  * [dcampbell24/iup-rust](https://github.com/dcampbell24/iup-rust) — IUP bindings [<img src="https://travis-ci.org/dcampbell24/iup-rust.svg?branch=master">](https://travis-ci.org/dcampbell24/iup-rust)
  * [Kiss-ui](https://github.com/KISS-UI/kiss-ui) — a simple UI framework built on IUP [![Build Status](https://travis-ci.org/cybergeek94/kiss-ui.svg?branch=master)](https://travis-ci.org/cybergeek94/kiss-ui)
* [GTK+](http://www.gtk.org) [[gtk](https://crates.io/keywords/gtk)]
  * [gtk-rs/gtk](https://github.com/gtk-rs/gtk) — GTK+ bindings [<img src="https://travis-ci.org/gtk-rs/gtk.svg?branch=master">](https://travis-ci.org/gtk-rs/gtk)
* [ImGui](https://github.com/ocornut/imgui)
  * [imgui-rs](https://github.com/Gekkio/imgui-rs) — Rust bindings for ImGui [<img src="https://travis-ci.org/Gekkio/imgui-rs.svg?branch=master">](https://travis-ci.org/Gekkio/imgui-rs)
* [libui](https://github.com/andlabs/libui)
  * [pcwalton/libui-rs](https://github.com/pcwalton/libui-rs) — libui bindings [<img src="https://travis-ci.org/pcwalton/libui-rs.svg?branch=master">](https://travis-ci.org/pcwalton/libui-rs)
* [ncurses](http://www.gnu.org/software/ncurses/) [[ncurses](https://crates.io/keywords/ncurses)]
  * [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) — ncurses bindings [<img src="https://travis-ci.org/jeaye/ncurses-rs.svg?branch=master">](https://travis-ci.org/jeaye/ncurses-rs)
* [saurvs/nfd-rs](https://github.com/saurvs/nfd-rs) — Open native UI file dialogs in Linux, OS X and Windows
* [Qt](http://doc.qt.io)
  * [cyndis/qmlrs](https://github.com/cyndis/qmlrs) — QtQuick bindings [<img src="https://travis-ci.org/cyndis/qmlrs.svg?branch=master">](https://travis-ci.org/cyndis/qmlrs)
  * [kitech/qt.rs](https://github.com/kitech/qt.rs) — Qt5 bindings [<img src="https://travis-ci.org/kitech/qt.rs.svg?branch=master">](https://travis-ci.org/kitech/qt.rs)
  * [rust-qt](https://github.com/rust-qt) —
  * [White-Oak/qml-rust] (https://github.com/White-Oak/qml-rust) — QML bindings. [<img src="https://travis-ci.org/White-Oak/qml-rust.svg?branch=master">](https://travis-ci.org/White-Oak/qml-rust)
* [Sciter](http://sciter.com/)
  * [pravic/rust-sciter](https://github.com/pravic/rust-sciter) — Sciter bindings [<img src="https://ci.appveyor.com/api/projects/status/github/pravic/rust-sciter?svg=true">](https://ci.appveyor.com/project/pravic/rust-sciter)
* [Termbox](https://github.com/nsf/termbox)
  * [gchp/rustbox](https://github.com/gchp/rustbox) — a Rust implementation of Termbox [<img src="https://travis-ci.org/gchp/rustbox.svg?branch=master">](https://travis-ci.org/gchp/rustbox)


### Image processing

* [chyh1990/imageproc](https://github.com/chyh1990/imageproc) — An advanced image processing library for Rust. [![Build Status](https://travis-ci.org/chyh1990/imageproc.svg?branch=master)](https://travis-ci.org/chyh1990/imageproc)
* [cybergeek94/img-hash](https://github.com/cybergeek94/img_hash) — Perceptual image hashing and comparison for equality and similarity.
* [PistonDevelopers/image](https://github.com/PistonDevelopers/image) — Basic imaging processing functions and methods for converting to and from image formats [<img src="https://travis-ci.org/PistonDevelopers/image.svg?branch=master">](https://travis-ci.org/PistonDevelopers/image)


### Logging

[[log](https://crates.io/keywords/log)]

* [rust-lang-nursery/log](https://github.com/rust-lang-nursery/log) — Logging implementation for Rust [![Build Status](https://travis-ci.org/rust-lang-nursery/log.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/log)
* [slog-rs/slog](https://github.com/slog-rs/slog) — Structured, composable logging for Rust [![Build Status](https://travis-ci.org/slog-rs/slog.svg?branch=master)](https://travis-ci.org/slog-rs/slog)
* [sfackler/log4rs](https://github.com/sfackler/log4rs) — highly configurable logging framework modeled after Java's Logback and log4j libraries [![Build Status](https://travis-ci.org/sfackler/log4rs.svg?branch=master)](https://travis-ci.org/sfackler/log4rs)


### Machine learning

[[machine learning](https://crates.io/keywords/machine-learning)]

See also [About Rust’s Machine Learning Community](https://medium.com/@autumn_eng/about-rust-s-machine-learning-community-4cda5ec8a790#.hvkp56j3f).

* [AtheMathmo/rusty-machine](https://github.com/AtheMathmo/rusty-machine) — Machine learning library for Rust [![Build Status](https://travis-ci.org/AtheMathmo/rusty-machine.svg?branch=master)](https://travis-ci.org/AtheMathmo/rusty-machine)
* [autumnai/leaf](https://github.com/autumnai/leaf) — Open Machine Intelligence framework. [![Build Status](https://travis-ci.org/autumnai/leaf.svg?branch=master)](https://travis-ci.org/autumnai/leaf)
* [tensorflow/rust](https://github.com/tensorflow/rust) — Rust language bindings for TensorFlow. [![Build Status](https://travis-ci.org/tensorflow/rust.svg?branch=master)](https://travis-ci.org/tensorflow/rust)
* [maciejkula/rustlearn](https://github.com/maciejkula/rustlearn) — Machine learning crate for Rust. [![Circle CI](https://circleci.com/gh/maciejkula/rustlearn.svg?style=svg)](https://circleci.com/gh/maciejkula/rustlearn)


### Markup language

* CommonMark
  * [google/pulldown-cmark](https://github.com/google/pulldown-cmark) — [CommonMark](http://commonmark.org/) parser in Rust


### Mobile

[Geal/rust_on_mobile](https://github.com/Geal/rust_on_mobile)

* Android
  * [tomaka/android-rs-glue](https://github.com/tomaka/android-rs-glue) — glue between Rust and Android [<img src="https://travis-ci.org/tomaka/android-rs-glue.svg?branch=master">](https://travis-ci.org/tomaka/android-rs-glue)
* iOS
  * [TimNN/cargo-lipo](https://github.com/TimNN/cargo-lipo) — a cargo lipo subcommand which automatically creates a universal library for use with your iOS application. [<img src="https://travis-ci.org/TimNN/cargo-lipo.svg?branch=master">](https://travis-ci.org/TimNN/cargo-lipo)
  * [vhbit/ObjCrust](https://github.com/vhbit/ObjCrust) — using Rust to create an iOS static library [<img src="https://travis-ci.org/vhbit/ObjCrust.svg?branch=master">](https://travis-ci.org/vhbit/ObjCrust)
* Pebble
  * [andars/pebble.rs](https://github.com/andars/pebble.rs) — a crate that allows Rust to be used to develop Pebble applications.


### Network programming

* FTP
  * [mattnenterprise/rust-ftp](https://github.com/mattnenterprise/rust-ftp) — an [FTP](https://en.wikipedia.org/wiki/File_Transfer_Protocol) client for Rust [<img src="https://travis-ci.org/mattnenterprise/rust-ftp.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-ftp)
* Low level
  * [libpnet/libpnet](https://github.com/libpnet/libpnet) — a cross-platform, low level networking [<img src="https://travis-ci.org/libpnet/libpnet.svg?branch=master">](https://travis-ci.org/libpnet/libpnet)
  * [tokio-rs/tokio](https://github.com/tokio-rs/tokio) — a network application framework for rapid development and highly scalable production deployments of clients and servers.
  * [dylanmckay/protocol](https://github.com/dylanmckay/protocol) — Custom TCP/UDP protocol definitions
* NanoMsg
  * [thehydroimpulse/nanomsg.rs](https://github.com/thehydroimpulse/nanomsg.rs) — [nanomsg](http://nanomsg.org/) bindings [<img src="https://travis-ci.org/thehydroimpulse/nanomsg.rs.svg?branch=master">](https://travis-ci.org/thehydroimpulse/nanomsg.rs)
* NNTP
  * [mattnenterprise/rust-nntp](https://github.com/mattnenterprise/rust-nntp) — an [NNTP](https://en.wikipedia.org/wiki/Network_News_Transfer_Protocol) client for Rust [<img src="https://travis-ci.org/mattnenterprise/rust-nntp.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-nntp)
* POP3
  * [mattnenterprise/rust-pop3](https://github.com/mattnenterprise/rust-pop3) — a [POP3](https://en.wikipedia.org/wiki/Post_Office_Protocol) client for Rust [<img src="https://travis-ci.org/mattnenterprise/rust-pop3.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-pop3)
* SSH
  * [alexcrichton/ssh2-rs](https://github.com/alexcrichton/ssh2-rs) — [libssh2](http://www.libssh2.org/) bindings [<img src="https://travis-ci.org/alexcrichton/ssh2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/ssh2-rs)
  * [Thrussh](http://pijul.org/thrussh/) — an SSH library written from scratch in Rust, backed by [libsodium](https://download.libsodium.org/doc/)
* Stomp
  * [zslayton/stomp-rs](https://github.com/zslayton/stomp-rs) — a [STOMP 1.2](http://stomp.github.io/stomp-specification-1.2.html) client implementation in Rust [<img src="https://travis-ci.org/zslayton/stomp-rs.svg?branch=master">](https://travis-ci.org/zslayton/stomp-rs)
* uTP
  * [meqif/rust-utp](https://github.com/meqif/rust-utp) — a [uTP](http://www.bittorrent.org/beps/bep_0029.html) (Micro Transport Protocol) library for Rust. [<img src="https://travis-ci.org/meqif/rust-utp.svg?branch=master">](https://travis-ci.org/meqif/rust-utp)
* ZeroMQ
  * [erickt/rust-zmq](https://github.com/erickt/rust-zmq) — [ZeroMQ](http://zeromq.org/) bindings [<img src="https://travis-ci.org/erickt/rust-zmq.svg?branch=master">](https://travis-ci.org/erickt/rust-zmq)


### Parser

  * [dragostis/pest](https://github.com/dragostis/pest) — Elegant, efficient grammars [![Build Status](https://travis-ci.org/dragostis/pest.svg?branch=master)]
(https://travis-ci.org/dragostis/pest)
  * [Geal/nom](https://github.com/Geal/nom) — parser combinator library [<img src="https://travis-ci.org/Geal/nom.svg?branch=master">](https://travis-ci.org/Geal/nom)
  * [ivanceras/inquerest](https://github.com/ivanceras/inquerest) — an URL parameter parser for rest filter inquiry [![Build Status](https://travis-ci.org/ivanceras/inquerest.svg?branch=master)](https://travis-ci.org/ivanceras/inquerest)
  * [kevinmehall/rust-peg](https://github.com/kevinmehall/rust-peg) — Parsing Expression Grammar (PEG) parser generator
  * [m4rw3r/chomp](https://github.com/m4rw3r/chomp) – A fast monadic-style parser combinator [<img src="https://travis-ci.org/m4rw3r/chomp.svg?branch=master">](https://travis-ci.org/m4rw3r/chomp)
  * [Marwes/combine](https://github.com/Marwes/combine) — parser combinator library [<img src="https://travis-ci.org/Marwes/combine.svg?branch=master">](https://travis-ci.org/Marwes/combine)
  * [nikomatsakis/lalrpop](https://github.com/nikomatsakis/lalrpop) — LR(1) parser generator for Rust [![Build status](https://travis-ci.org/nikomatsakis/lalrpop.svg?branch=master)](https://travis-ci.org/nikomatsakis/lalrpop)
  * [ptal/oak](https://github.com/ptal/oak) — a typed PEG parser generator (compiler plugin)
  * [rustless/queryst](https://github.com/rustless/queryst) — a query string parsing library for Rust inspired by https://github.com/ljharb/qs [![Build Status](https://travis-ci.org/rustless/queryst.svg?branch=master)](https://travis-ci.org/rustless/queryst)


### Platform specific

* Linux
  * [hannobraun/inotify-rs](https://github.com/hannobraun/inotify-rs) — [inotify](https://en.wikipedia.org/wiki/Inotify) bindings [<img src="https://travis-ci.org/hannobraun/inotify-rs.svg?branch=master">](https://travis-ci.org/hannobraun/inotify-rs)
* Unix-like
  * [nix-rust/nix](https://github.com/nix-rust/nix) — Unix-like API bindings [<img src="https://travis-ci.org/nix-rust/nix.svg?branch=master">](https://travis-ci.org/nix-rust/nix)
  * [zargony/rust-fuse](https://github.com/zargony/rust-fuse) — [FUSE](https://github.com/libfuse/libfuse) bindings <img src="https://travis-ci.org/zargony/rust-fuse.svg?branch=master">
* Windows
  * [retep998/winapi-rs](https://github.com/retep998/winapi-rs) — Windows API bindings [<img src="https://travis-ci.org/retep998/winapi-rs.svg?branch=master">](https://travis-ci.org/retep998/winapi-rs)


### Template engine

* Handlebars
  * [sunng87/handlebars-rust](https://github.com/sunng87/handlebars-rust) — Handlebars template engine with inheritance, custom helper support. [<img src="https://travis-ci.org/sunng87/handlebars-rust.svg?branch=master">](https://travis-ci.org/sunng87/handlebars-rust)
* HTML
  * [lfairy/maud](https://github.com/lfairy/maud) — compile-time HTML templates [<img src="https://travis-ci.org/lfairy/maud.svg?branch=master">](https://travis-ci.org/lfairy/maud)
  * [Stebalien/horrorshow-rs](https://github.com/Stebalien/horrorshow-rs) — compile-time HTML templates [<img src="https://travis-ci.org/Stebalien/horrorshow-rs.svg?branch=master">](https://travis-ci.org/Stebalien/horrorshow-rs)
  * [kaj/ructe](https://github.com/kaj/ructe) — HTML template system for Rust [<img src="https://travis-ci.org/kaj/ructe.svg?branch=master">](https://travis-ci.org/kaj/ructe)
  * [Keats/tera](https://github.com/Keats/tera) — template engine based on Jinja2 and the Django template language. [<img src="https://travis-ci.org/Keats/tera.svg?branch=master">](https://travis-ci.org/Keats/tera)
* Mustache
  * [rustache/rustache](https://github.com/rustache/rustache) — [<img src="https://travis-ci.org/rustache/rustache.svg?branch=master">](https://travis-ci.org/rustache/rustache)
* [tailhook/marafet](https://github.com/tailhook/marafet) — Compiler for Jade-like template language to cito.js-based virtual dom


### Text processing

* [BurntSushi/suffix](https://github.com/BurntSushi/suffix) — Linear time suffix array construction (with Unicode support) [<img src="https://travis-ci.org/BurntSushi/suffix.svg?branch=master">](https://travis-ci.org/BurntSushi/suffix)
* [BurntSushi/tabwriter](https://github.com/BurntSushi/tabwriter) — Elastic tab stops (i.e., text column alignment) [<img src="https://travis-ci.org/BurntSushi/tabwriter.svg?branch=master">](https://travis-ci.org/BurntSushi/tabwriter)
* [pwoolcoc/ngrams](https://github.com/pwoolcoc/ngrams) — Construct [n-grams](https://en.wikipedia.org/wiki/N-gram) from arbitrary iterators [<img src="https://travis-ci.org/pwoolcoc/ngrams.svg?branch=master">](https://travis-ci.org/pwoolcoc/ngrams)
* [rust-lang-nursery/regex](https://github.com/rust-lang-nursery/regex) — Regular expressions (RE2 style) [<img src="https://travis-ci.org/rust-lang-nursery/regex.svg?branch=master">](https://travis-ci.org/rust-lang-nursery/regex)


### Virtualization

* [beneills/quantum](https://github.com/beneills/quantum) — Advanced Rust quantum computer simulator [<img src="https://travis-ci.org/beneills/quantum.svg?branch=master">](https://travis-ci.org/beneills/quantum)
* [ekse/unicorn-rs](https://github.com/ekse/unicorn-rs) — Rust bindings for the unicorn CPU emulator
* [saurvs/hypervisor-rs](https://github.com/saurvs/hypervisor-rs) — Hardware-accelerated virtualization on OS X


### Web programming

See also [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison).


* Deployment
  * Docker
    * [emk/rust-musl-builder](https://github.com/emk/rust-musl-builder) — Docker images for compiling static Rust binaries using musl-libc and musl-gcc, with static versions of useful C libraries
  * Google App Engine
    * [DenisKolodin/rust-app-engine](https://github.com/DenisKolodin/rust-app-engine) — App Engine Rust boilerplate
  * Heroku
    * [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust) — a buildpack for Rust applications on Heroku
* HTTP Client
  * [alexcrichton/curl-rust](https://github.com/alexcrichton/curl-rust) — [libcurl](http://curl.haxx.se/libcurl/) bindings [<img src="https://travis-ci.org/alexcrichton/curl-rust.svg?branch=master">](https://travis-ci.org/alexcrichton/curl-rust)
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [<img src="https://travis-ci.org/hyperium/hyper.svg?branch=master">](https://travis-ci.org/hyperium/hyper)
  * [seanmonstar/reqwest](https://github.com/seanmonstar/reqwest) — an ergonomic HTTP Client for Rust. [<img src="https://travis-ci.org/seanmonstar/reqwest.svg?branch=master">](https://travis-ci.org/seanmonstar/reqwest)
* HTTP Server
  * [fengsp/pencil](https://github.com/fengsp/pencil) — [<img src="https://travis-ci.org/fengsp/pencil.svg?branch=master">](https://travis-ci.org/fengsp/pencil)
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [<img src="https://travis-ci.org/hyperium/hyper.svg?branch=master">](https://travis-ci.org/hyperium/hyper)
  * [Iron](https://github.com/iron/iron) — a middleware-based server framework [<img src="https://travis-ci.org/iron/iron.svg?branch=master">](https://travis-ci.org/iron/iron)
    * [sunng87/handlebars-iron](https://github.com/sunng87/handlebars-iron) — [Handlebars-rust](https://github.com/sunng87/handlebars-rust) as an Iron web framework middleware. [<img src="https://travis-ci.org/sunng87/handlebars-iron.svg?branch=master">](https://travis-ci.org/sunng87/handlebars-iron)
  * [Nickel](https://github.com/nickel-org/nickel.rs/) — inspired by [Express](http://expressjs.com/) [<img src="https://travis-ci.org/nickel-org/nickel.rs.svg?branch=master">](https://travis-ci.org/nickel-org/nickel.rs)
  * [Ogeon/rustful](https://github.com/Ogeon/rustful) — a RESTful web framework for Rust  [<img src="https://travis-ci.org/Ogeon/rustful.svg?branch=master">](https://travis-ci.org/Ogeon/rustful)
  * [Rustless](https://github.com/rustless/rustless) — a REST-like API micro-framework inspired by [Grape](https://github.com/ruby-grape/grape) and [Hyper](https://github.com/hyperium/hyper) [<img src="https://travis-ci.org/rustless/rustless.svg?branch=master">](https://travis-ci.org/rustless/rustless)
  * [tiny-http](https://github.com/frewsxcv/tiny-http) — Low level HTTP server library [<img src="https://travis-ci.org/frewsxcv/tiny-http.svg?branch=master">](https://travis-ci.org/frewsxcv/tiny-http)
  * [tomaka/rouille](https://github.com/tomaka/rouille) — Web framework in Rust[<img src="https://travis-ci.org/tomaka/rouille.svg?branch=master">](https://travis-ci.org/tomaka/rouille)
  * [sappworks/sapper](https://github.com/sappworks/sapper) — A lightweight web framework built on async hyper, implemented in Rust language. [<img src="https://travis-ci.org/sappworks/sapper.svg?branch=master">](https://travis-ci.org/tomaka/rouille)
* [WebSocket](https://datatracker.ietf.org/doc/rfc6455/)
  * [cyderize/rust-websocket](https://github.com/cyderize/rust-websocket) — a framework for dealing with WebSocket connections (both clients and servers) [<img src="https://travis-ci.org/cyderize/rust-websocket.svg?branch=master">](https://travis-ci.org/cyderize/rust-websocket)
  * [vityafx/urlshortener-rs](https://github.com/vityafx/urlshortener-rs) [[urlshortener](https://crates.io/crates/urlshortener)] — A very simple urlshortener library for Rust. [<img src="https://travis-ci.org/vityafx/urlshortener-rs.svg?branch=master">](https://travis-ci.org/vityafx/urlshortener-rs)
  * [housleyjk/ws-rs](https://github.com/housleyjk/ws-rs) — lightweight, event-driven WebSockets for Rust [<img src="https://travis-ci.org/housleyjk/ws-rs.svg?branch=stable">](https://travis-ci.org/housleyjk/ws-rs)


## Resources

* Benchmarks
  * [TeXitoi/benchmarksgame-rs](https://github.com/TeXitoi/benchmarksgame-rs) — Rust implementations for the [The Computer Language Benchmarks Game](http://benchmarksgame.alioth.debian.org/) [<img src="https://travis-ci.org/TeXitoi/benchmarksgame-rs.svg?branch=master">](https://travis-ci.org/TeXitoi/benchmarksgame-rs)
* Decks & Presentations
  * [Learning systems programming with Rust](https://speakerdeck.com/jvns/learning-systems-programming-with-rust) — Presented by [Julia Evans](https://twitter.com/@b0rk) @ Rustconf 2016.
* Learning
  * [exercism.io](http://exercism.io/languages/rust) — programming exercises that help you learn new concepts in Rust.
  * [Learning Rust With Entirely Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/) — in-depth exploration of Rust's memory management rules, through implementing a few different types of list structures.
  * [Rust by Example](http://rustbyexample.com/)
  * [rust-learning](https://github.com/ctjhoa/rust-learning) — a collection of useful resources to learn Rust
  * [Rustlings](https://github.com/carols10cents/rustlings) — small exercises to get you used to reading and writing Rust code
* Podcasts
  * [New Rustacean](http://www.newrustacean.com) — a podcast about learning Rust
  * [Rusty Radio](http://rustyrad.io) — covering the rust ecosystem
* [RustCamp 2015 Talks](http://confreaks.tv/events/rustcamp2015)
* [Rust Design Patterns](https://github.com/nrc/patterns)
* [Rust Guidelines](http://aturon.github.io/)
* [RustBooks](https://github.com/sger/RustBooks) — list of RustBooks



## License

[![CC0](http://i.creativecommons.org/p/zero/1.0/88x31.png)](http://creativecommons.org/publicdomain/zero/1.0/)
