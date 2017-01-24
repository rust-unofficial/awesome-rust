Awesome Rust
============

A curated list of Rust code and resources, inspired by other awesome lists. The goal is to have only projects that are mostly stable and useful to users.

If you want to contribute, please read [this](CONTRIBUTING.md) .

-   [Awesome Rust](#awesome-rust)
    -   [Applications written in Rust](#applications-written-in-rust)
        -   [Games](#games)
        -   [Operating systems](#operating-systems)
        -   [System tools](#system-tools)
    -   [Development Tools](#development-tools)
        -   [Debugging](#debugging)
        -   [Embedded](#embedded)
        -   [FFI](#ffi)
        -   [IDEs](#ides)
        -   [Profiling](#profiling)
        -   [Testing](#testing)
    -   [Libraries](#libraries)
        -   [Astronomy](#astronomy)
        -   [Asynchronous](#asynchronous)
        -   [Audio](#audio)
        -   [Authentication](#authentication)
        -   [Bioinformatics](#bioinformatics)
        -   [Build system](#build-system)
        -   [Cloud](#cloud)
        -   [Command-line argument parsing](#command-line-argument-parsing)
        -   [Command-line interface](#command-line-interface)
        -   [Compression](#compression)
        -   [Computation](#computation)
        -   [Concurrency](#concurrency)
        -   [Cryptography](#cryptography)
        -   [Database](#database)
        -   [Data structures](#data-structures)
        -   [Date and time](#date-and-time)
        -   [Distributed Systems](#distributed-systems)
        -   [Email](#email)
        -   [Encoding](#encoding)
        -   [Game development](#game-development)
        -   [Games](#games)
        -   [Geospatial](#geospatial)
        -   [GUI](#gui)
        -   [Image processing](#image-processing)
        -   [Machine learning](#machine-learning)
        -   [Markup language](#markup-language)
        -   [Mobile](#mobile)
        -   [Network programming](#network-programming)
        -   [Parser](#parser)
        -   [Platform specific](#platform-specific)
        -   [Template engine](#template-engine)
        -   [Text processing](#text-processing)
        -   [Virtualization](#virtualization)
        -   [Web programming](#web-programming)
    -   [Resources](#resources)
    -   [License](#license)

Applications written in Rust
----------------------------

-   [azerupi/mdBook](https://github.com/azerupi/mdBook) <span> | ★ 154, pushed 6 days ago | </span> — a command line utility to create books from markdown files [![](https://travis-ci.org/azerupi/mdBook.svg?branch=master)](https://travis-ci.org/azerupi/mdBook)
-   [bluejekyll/trust-dns](https://github.com/bluejekyll/trust-dns) <span> | ★ 92, pushed 11 days ago | </span> — a DNS-server [![](https://travis-ci.org/bluejekyll/trust-dns.svg?branch=master)](https://travis-ci.org/bluejekyll/trust-dns)
-   [BurntSushi/xsv](https://github.com/BurntSushi/xsv) <span> | ★ 613, pushed 89 days ago | </span> — a fast CSV command line tool (slicing, indexing, selecting, searching, sampling, etc.) [![](https://travis-ci.org/BurntSushi/xsv.svg?branch=master)](https://travis-ci.org/BurntSushi/xsv)
-   Emulators
    -   [simias/rustation](https://github.com/simias/rustation) <span> | ★ 231, pushed 33 days ago | </span> — a Playstation emulator [![](https://travis-ci.org/simias/rustation.svg?branch=master)](https://travis-ci.org/simias/rustation)
-   [gchp/iota](https://github.com/gchp/iota) <span> | ★ 803, pushed 2 days ago | </span> — a simple text editor [![](https://travis-ci.org/gchp/iota.svg?branch=master)](https://travis-ci.org/gchp/iota)
-   [dlecan/generic-dns-update](https://github.com/dlecan/generic-dns-update) <span> | ★ 1, pushed 5 days ago | </span> — a tool to update DNS zonefiles with your IP address [![](https://travis-ci.org/dlecan/generic-dns-update.svg?branch=master)](https://travis-ci.org/dlecan/generic-dns-update)
-   [Factotum](https://github.com/snowplow/factotum) <span> | ★ 20, pushed 0 days ago | </span> - [A system to programmatically run data pipelines](http://snowplowanalytics.com/blog/2016/04/09/introducing-factotum-data-pipeline-runner/) [![](https://travis-ci.org/snowplow/factotum.svg?branch=master)](https://travis-ci.org/snowplow/factotum)
-   [Fractalide](https://github.com/fractalide/fractalide) <span> | ★ 24, pushed 29 days ago | </span> — Flow-based Programming environment.
-   [imjacobclark/Herd](https://github.com/imjacobclark/Herd) <span> | ★ 52, pushed 186 days ago | </span> — an experimental HTTP load testing application
-   [jedisct1/flowgger](https://github.com/jedisct1/flowgger) <span> | ★ 102, pushed 12 days ago | </span> — a fast, simple and lightweight data collector
-   [kbknapp/docli](https://github.com/kbknapp/docli-rs) <span> | ★ 21, pushed 40 days ago | </span> — a command line utility for managing DigitalOcean infrastructure [![](https://travis-ci.org/kbknapp/docli-rs.svg?branch=master)](https://travis-ci.org/kbknapp/docli-rs)
-   [MaidSafe](http://maidsafe.net/) — a decentralized platform.
-   [qmx/limonite](https://github.com/qmx/limonite) <span> | ★ 7, pushed 72 days ago | </span> — static blog/website generator [![](https://travis-ci.org/qmx/limonite.svg?branch=master)](https://travis-ci.org/qmx/limonite)
-   [seppo0010/rsedis](https://github.com/seppo0010/rsedis) <span> | ★ 950, pushed 178 days ago | </span> — a Redis reimplementation [![](https://travis-ci.org/seppo0010/rsedis.svg?branch=master)](https://travis-ci.org/seppo0010/rsedis)
-   [Servo](https://github.com/servo/servo) <span> | ★ 6549, pushed 0 days ago | </span> — a prototype web browser engine
-   Virtualization
    -   [tailhook/vagga](https://github.com/tailhook/vagga) <span> | ★ 772, pushed 0 days ago | </span> — a containerization tool without daemons [![](https://travis-ci.org/tailhook/vagga.svg?branch=master)](https://travis-ci.org/tailhook/vagga)
-   [cristianoliveira/funzzy](https://github.com/cristianoliveira/funzzy) <span> | ★ 16, pushed 3 days ago | </span> — a configurable watcher inspired in [entr](http://entrproject.org/) [![](https://api.travis-ci.org/cristianoliveira/funzzy.svg?branch=master)](https://travis-ci.org/cristianoliveira/funzzy)

### Games

See also [Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston) .

-   [lifthrasiir/angolmois-rust](https://github.com/lifthrasiir/angolmois-rust) <span> | ★ 63, pushed 275 days ago | </span> — a minimalistic music video game which supports the BMS format [![](https://travis-ci.org/lifthrasiir/angolmois-rust.svg?branch=master)](https://travis-ci.org/lifthrasiir/angolmois-rust)
-   [swatteau/sokoban-rs](https://github.com/swatteau/sokoban-rs) <span> | ★ 55, pushed 2 days ago | </span> — a Sokoban implementation
-   [Zone of Control](https://github.com/ozkriff/zoc) <span> | ★ 48, pushed 27 days ago | </span> — a turn-based hexagonal strategy game [![](https://travis-ci.org/ozkriff/zoc.svg?branch=master)](https://travis-ci.org/ozkriff/zoc)
-   [rhex](https://github.com/dpc/rhex) <span> | ★ 44, pushed 6 days ago | </span> — hexagonal ascii roguelike

### Operating systems

[A comparison of operating systems written in Rust](https://github.com/flosse/rust-os-comparison)

-   [redox-os/redox](https://github.com/redox-os/redox) <span> | ★ 3926, pushed 0 days ago | </span> — [![](https://travis-ci.org/redox-os/redox.svg?branch=master)](https://travis-ci.org/redox-os/redox)
-   [thepowersgang/rust\_os](https://github.com/thepowersgang/rust_os) <span> | ★ 152, pushed 0 days ago | </span> — [![](https://travis-ci.org/thepowersgang/rust_os.svg?branch=master)](https://travis-ci.org/thepowersgang/rust_os)

### System tools

-   [Aaronepower/tokei](https://github.com/Aaronepower/tokei) <span> | ★ 50, pushed 20 days ago | </span> — counts the lines of code [![](https://img.shields.io/travis/Aaronepower/tokei.svg)](https://travis-ci.org/Aaronepower/tokei)
-   [buster/rrun](https://github.com/buster/rrun) <span> | ★ 27, pushed 17 days ago | </span> — a command launcher for Linux, similar to gmrun [![](https://travis-ci.org/buster/rrun.svg?branch=master)](https://travis-ci.org/buster/rrun)
-   [ogham/exa](https://github.com/ogham/exa) <span> | ★ 669, pushed 0 days ago | </span> — a replacement for 'ls' written in Rust [![](https://travis-ci.org/ogham/exa.svg?branch=master)](https://travis-ci.org/ogham/exa)
-   [mmstick/systemd-manager](https://github.com/mmstick/systemd-manager) <span> | ★ 15, pushed 5 days ago | </span> -- a systemd service manager written in Rust using GTK-rs.
-   [uutils/coreutils](https://github.com/uutils/coreutils) <span> | ★ 2790, pushed 2 days ago | </span> — a cross-platform Rust rewrite of the GNU coreutils [![](https://travis-ci.org/uutils/coreutils.svg?branch=master)](https://travis-ci.org/uutils/coreutils)

Development tools
-----------------

-   [Clippy](https://github.com/Manishearth/rust-clippy) <span> | ★ 604, pushed 0 days ago | </span> — Rust lints [![](https://travis-ci.org/Manishearth/rust-clippy.svg?branch=master)](https://travis-ci.org/Manishearth/rust-clippy)
-   [clog-tool/clog-cli](https://github.com/clog-tool/clog-cli) <span> | ★ 192, pushed 15 days ago | </span> — generates a changelog from git metadata ( [conventional changelog](http://blog.thoughtram.io/announcements/tools/2014/09/18/announcing-clog-a-conventional-changelog-generator-for-the-rest-of-us.html) ) [![](https://travis-ci.org/clog-tool/clog-cli.svg?branch=master)](https://travis-ci.org/clog-tool/clog-cli)
-   [dan-t/rusty-tags](https://github.com/dan-t/rusty-tags) <span> | ★ 72, pushed 4 days ago | </span> — create ctags/etags for a cargo project and all of its dependencies [![](https://travis-ci.org/dan-t/rusty-tags.svg?branch=master)](https://travis-ci.org/dan-t/rusty-tags)
-   [frewsxcv/crate-deps](https://github.com/frewsxcv/crate-deps) <span> | ★ 12, pushed 170 days ago | </span> — generates images of dependency graphs for crates hosted on crates.io
-   Multirust — manage multiple Rust installations
    -   [brson/multirust](https://github.com/brson/multirust) <span> | ★ 671, pushed 5 days ago | </span> — the original multirust as shell scripts [![](https://travis-ci.org/brson/multirust.svg?branch=master)](https://travis-ci.org/brson/multirust)
    -   [Diggsey/multirust-rs](https://github.com/Diggsey/multirust-rs) — multirust reimplementation in Rust [![](https://travis-ci.org/Diggsey/multirust-rs.svg?branch=master)](https://travis-ci.org/Diggsey/multirust-rs)
-   [Racer](https://github.com/phildawes/racer) <span> | ★ 1470, pushed 5 days ago | </span> — code completion for Rust [![](https://travis-ci.org/phildawes/racer.svg?branch=master)](https://travis-ci.org/phildawes/racer)
-   [rustfmt](https://github.com/rust-lang-nursery/rustfmt) <span> | ★ 682, pushed 0 days ago | </span> — a Rust code formatter [![](https://travis-ci.org/rust-lang-nursery/rustfmt.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/rustfmt)

### Debugging

-   GDB
    -   [rust-gdb](https://github.com/rust-lang/rust/blob/master/src/etc/rust-gdb)
-   LLDB
    -   [lldb\_batchmode.py](https://github.com/rust-lang/rust/blob/master/src/etc/lldb_batchmode.py) — allows to use LLDB in a way similar to GDB's batch mode.

### Embedded

-   Cross compiling
    -   [japaric/rust-cross](https://github.com/japaric/rust-cross) <span> | ★ 223, pushed 0 days ago | </span> — everything you need to know about cross compiling Rust programs [![](https://travis-ci.org/japaric/rust-cross.svg?branch=master)](https://travis-ci.org/japaric/rust-cross)
-   Raspberry Pi
    -   [Ogeon/rust-on-raspberry-pi](https://github.com/Ogeon/rust-on-raspberry-pi) <span> | ★ 154, pushed 4 days ago | </span> — instructions for how to cross compile Rust projects for the Raspberry Pi .

### FFI

See also [Foreign Function Interface](https://doc.rust-lang.org/book/ffi.html) , [Rust Inside Other Languages](https://doc.rust-lang.org/book/rust-inside-other-languages.html) and [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) (a collection of examples of using code written in Rust from other languages).

-   C
    -   [crabtw/rust-bindgen](https://github.com/crabtw/rust-bindgen) <span> | ★ 487, pushed 3 days ago | </span> — a Rust bindings generator [![](https://travis-ci.org/crabtw/rust-bindgen.svg?branch=master)](https://travis-ci.org/crabtw/rust-bindgen)
    -   [Sean1708/rusty-cheddar](https://github.com/Sean1708/rusty-cheddar) <span> | ★ 84, pushed 46 days ago | </span> — generates C header files from Rust source files [![](https://travis-ci.org/Sean1708/rusty-cheddar.svg?branch=master)](https://travis-ci.org/Sean1708/rusty-cheddar)
-   Erlang
    -   [hansihe/Rustler](https://github.com/hansihe/Rustler) <span> | ★ 171, pushed 2 days ago | </span> - Safe Rust bridge for creating Erlang NIF functions [![](https://travis-ci.org/hansihe/Rustler.svg?branch=master)](https://travis-ci.org/hansihe/Rustler)
-   Java
    -   [drrb/java-rust-example](https://github.com/drrb/java-rust-example) <span> | ★ 117, pushed 326 days ago | </span> — use Rust from Java [![](https://travis-ci.org/drrb/java-rust-example.svg?branch=master)](https://travis-ci.org/drrb/java-rust-example)
-   mruby
    -   [anima-engine/mrusty](https://github.com/anima-engine/mrusty) <span> | ★ 115, pushed 0 days ago | </span> - mruby safe bindings for Rust [![](https://travis-ci.org/anima-engine/mrusty.svg?branch=master)](https://travis-ci.org/anima-engine/mrusty)
-   Node.js
    -   [rustbridge/neon](https://github.com/rustbridge/neon) <span> | ★ 833, pushed 14 days ago | </span> — use Rust from Node.js [![](https://travis-ci.org/rustbridge/neon.svg?branch=master)](https://travis-ci.org/rustbridge/neon)
-   Objective-C
    -   [SSheldon/rust-objc](https://github.com/SSheldon/rust-objc) <span> | ★ 67, pushed 19 days ago | </span> — Objective-C Runtime bindings and wrapper for Rust
-   Python
    -   [dgrunwald/rust-cpython](https://github.com/dgrunwald/rust-cpython) <span> | ★ 166, pushed 2 days ago | </span> — Python bindings [![](https://travis-ci.org/dgrunwald/rust-cpython.svg?branch=master)](https://travis-ci.org/dgrunwald/rust-cpython)
    -   [lukemetz/rustpy](https://github.com/lukemetz/rustpy) <span> | ★ 55, pushed 30 days ago | </span> — Python bindings [![](https://travis-ci.org/lukemetz/rustpy.svg?branch=master)](https://travis-ci.org/lukemetz/rustpy)
-   R
    -   [rustr/rustr](https://github.com/rustr/rustr) <span> | ★ 20, pushed 4 days ago | </span> — use Rust from R, and use R in Rust [![](https://travis-ci.org/rustr/rustr.svg?branch=master)](https://travis-ci.org/rustr/rustr)

### IDEs

See also <http://areweideyet.com/> and [Rust and IDEs](https://www.rust-lang.org/ides.html) .

-   [intellij-rust](https://github.com/intellij-rust/intellij-rust) <span> | ★ 546, pushed 0 days ago | </span> — an [IntelliJ](https://www.jetbrains.com/idea/) -based IDE for Rust [![](https://travis-ci.org/intellij-rust/intellij-rust.svg?branch=master)](https://travis-ci.org/intellij-rust/intellij-rust)
-   [PistonDevelopers/VisualRust](https://github.com/PistonDevelopers/VisualRust) <span> | ★ 396, pushed 23 days ago | </span> — a Visual Studio extension for Rust [![](https://travis-ci.org/PistonDevelopers/VisualRust.svg?branch=master)](https://travis-ci.org/PistonDevelopers/VisualRust)
-   [Ride](https://github.com/madeso/ride) <span> | ★ 98, pushed 112 days ago | </span> — [![](https://travis-ci.org/madeso/ride.svg?branch=master)](https://travis-ci.org/madeso/ride)
-   [RustDT](https://github.com/RustDT/RustDT) <span> | ★ 246, pushed 0 days ago | </span> — an [Eclipse](https://eclipse.org/) -based IDE for Rust [![](https://travis-ci.org/RustDT/RustDT.svg?branch=master)](https://travis-ci.org/RustDT/RustDT)
-   [SolidOak](https://github.com/oakes/SolidOak) <span> | ★ 641, pushed 31 days ago | </span> — a simple IDE for Rust, based on GTK+ and [Neovim](https://github.com/neovim/neovim)
-   [Visual Studio Code](https://code.visualstudio.com/)

### Profiling

-   [ellisonch/rust-stopwatch](https://github.com/ellisonch/rust-stopwatch) <span> | ★ 17, pushed 292 days ago | </span> — a stopwatch library [![](https://travis-ci.org/ellisonch/rust-stopwatch.svg?branch=master)](https://travis-ci.org/ellisonch/rust-stopwatch)
-   [mrhooray/torch](https://github.com/mrhooray/torch) <span> | ★ 64, pushed 236 days ago | </span> — generates FlameGraphs based on DWARF Debug Info

### Testing

-   [BurntSushi/quickcheck](https://github.com/BurntSushi/quickcheck) <span> | ★ 318, pushed 38 days ago | </span> — a Rust implementation of [QuickCheck](https://wiki.haskell.org/Introduction_to_QuickCheck1) [![](https://travis-ci.org/BurntSushi/quickcheck.svg?branch=master)](https://travis-ci.org/BurntSushi/quickcheck)
-   [farcaller/shiny](https://github.com/farcaller/shiny) <span> | ★ 71, pushed 212 days ago | </span> — a fancy syntax similar to Ruby's Rspec or Objective-C' kiwi [![](https://travis-ci.org/farcaller/shiny.svg?branch=master)](https://travis-ci.org/farcaller/shiny)
-   [frewsxcv/afl.rs](https://github.com/frewsxcv/afl.rs) <span> | ★ 192, pushed 11 days ago | </span> — a Rust fuzzer, using [AFL](http://lcamtuf.coredump.cx/afl/) [![](https://api.travis-ci.org/frewsxcv/afl.rs.svg?branch=master)](https://travis-ci.org/frewsxcv/afl.rs)

Libraries
---------

### Astronomy

-   [saurvs/astro-rust](https://github.com/saurvs/astro-rust) <span> | ★ 18, pushed 10 days ago | </span> — astronomy for Rust [![](https://travis-ci.org/saurvs/astro-rust.svg?branch=master)](https://travis-ci.org/saurvs/astro-rust)

### Asynchronous

-   [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) <span> | ★ 206, pushed 5 days ago | </span> — a coroutine I/O library with a working-stealing scheduler [![](https://travis-ci.org/zonyitoo/coio-rs.svg?branch=master)](https://travis-ci.org/zonyitoo/coio-rs)
-   [thehydroimpulse/tangle](https://github.com/thehydroimpulse/tangle) <span> | ★ 6, pushed 22 days ago | </span> — a scala-inspired futures library [![](https://travis-ci.org/thehydroimpulse/tangle.svg?branch=master)](https://travis-ci.org/thehydroimpulse/tangle)

### Audio

-   [GuillaumeGomez/rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) <span> | ★ 8, pushed 78 days ago | </span> — [FMOD](http://www.fmod.org/) bindings [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-fmod.svg?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-fmod)
-   [RustAudio/rust-portaudio](https://github.com/RustAudio/rust-portaudio) <span> | ★ 70, pushed 6 days ago | </span> — [PortAudio](http://www.portaudio.com/) bindings [![](https://travis-ci.org/RustAudio/rust-portaudio.svg?branch=master)](https://travis-ci.org/RustAudio/rust-portaudio)
-   [jhasse/ears](https://github.com/jhasse/ears) <span> | ★ 5, pushed 54 days ago | </span> — a simple library to play Sounds and Musics, on top of OpenAL and libsndfile [![](https://travis-ci.org/jhasse/ears.svg?branch=master)](https://travis-ci.org/jhasse/ears)
-   [jpernst/openal-rs](https://github.com/jpernst/openal-rs) <span> | ★ 13, pushed 79 days ago | </span> — [OpenAL 1.1](http://www.openal.org/) bindings [![](https://travis-ci.org/jpernst/openal-rs.svg?branch=master)](https://travis-ci.org/jpernst/openal-rs)
-   [RustAudio](https://github.com/RustAudio)
-   [musitdev/portmidi-rs](https://github.com/musitdev/portmidi-rs) <span> | ★ 24, pushed 24 days ago | </span> — [PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings [![](https://travis-ci.org/musitdev/portmidi-rs.svg?branch=master)](https://travis-ci.org/musitdev/portmidi-rs)

### Authentication

-   [keats/rust-jwt](https://github.com/keats/rust-jwt) <span> | ★ 17, pushed 4 days ago | </span> — [JSON Web Token](https://en.wikipedia.org/wiki/JSON_Web_Token) lib in rust [![Build Status](https://api.travis-ci.org/Keats/rust-jwt.svg?branch=master)](https://travis-ci.org/Keats/rust-jwt)

### Bioinformatics

-   [Rust-Bio](https://github.com/rust-bio) — bioinformatics libraries in Rust.

### Build system

-   [Cargo](https://crates.io/) — the Rust package manager
    -   [rsolomo/cargo-check](https://github.com/rsolomo/cargo-check) <span> | ★ 61, pushed 100 days ago | </span> — a wrapper around `      cargo rustc -- -Zno-trans     ` which can be helpful for running a faster compile if you only need correctness checks [![](https://travis-ci.org/rsolomo/cargo-check.svg?branch=master)](https://travis-ci.org/rsolomo/cargo-check)
    -   [kbknapp/cargo-count](https://github.com/kbknapp/cargo-count) <span> | ★ 42, pushed 19 days ago | </span> — lists source code counts and details about cargo projects, including unsafe statistics [![](https://travis-ci.org/kbknapp/cargo-count.svg?branch=master)](https://travis-ci.org/kbknapp/cargo-count)
    -   [pwoolcoc/cargo-do](https://github.com/pwoolcoc/cargo-do) <span> | ★ 7, pushed 34 days ago | </span> — run multiple cargo commands in a row
    -   [maxsnew/cargo-dot](https://github.com/maxsnew/cargo-dot) <span> | ★ 34, pushed 152 days ago | </span> — generate graphs of a Cargo project's dependencies [![](https://travis-ci.org/maxsnew/cargo-dot.svg?branch=master)](https://travis-ci.org/maxsnew/cargo-dot)
    -   [killercup/cargo-edit](https://github.com/killercup/cargo-edit) <span> | ★ 99, pushed 10 days ago | </span> — allows you to add and list dependencies by reading/writing to your Cargo.toml file from the command line [![](https://travis-ci.org/killercup/cargo-edit.svg?branch=master)](https://travis-ci.org/killercup/cargo-edit)
    -   [kbknapp/cargo-graph](https://github.com/kbknapp/cargo-graph) <span> | ★ 22, pushed 27 days ago | </span> — updated fork of `      cargo-dot     ` with additional features [![](https://travis-ci.org/kbknapp/cargo-graph.svg?branch=master)](https://travis-ci.org/kbknapp/cargo-graph)
    -   [kbknapp/cargo-outdated](https://github.com/kbknapp/cargo-outdated) <span> | ★ 39, pushed 28 days ago | </span> — displays when newer versions of Rust dependencies are available, or out of date [![](https://travis-ci.org/kbknapp/cargo-outdated.svg?branch=master)](https://travis-ci.org/kbknapp/cargo-outdated)
    -   [imp/cargo-multi](https://github.com/imp/cargo-multi) <span> | ★ 0, pushed 11 days ago | </span> \[ [cargo-multi](https://crates.io/crates/cargo-multi/) \] — runs specified cargo command on multiple crates [![](https://travis-ci.org/imp/cargo-multi.svg?branch=master)](https://travis-ci.org/imp/cargo-multi)
    -   [DanielKeep/cargo-script](https://github.com/DanielKeep/cargo-script) <span> | ★ 62, pushed 6 days ago | </span> — lets people quickly and easily run Rust "scripts" which can make use of Cargo's package ecosystem
    -   [passcod/cargo-watch](https://github.com/passcod/cargo-watch) <span> | ★ 107, pushed 80 days ago | </span> — utility for cargo to compile projects when sources change [![](https://travis-ci.org/passcod/cargo-watch.svg?branch=master)](https://travis-ci.org/passcod/cargo-watch)
-   CMake
    -   [SiegeLord/RustCMake](https://github.com/SiegeLord/RustCMake) <span> | ★ 45, pushed 178 days ago | </span> — an example project showing usage of CMake with Rust [![](https://travis-ci.org/SiegeLord/RustCMake.svg?branch=master)](https://travis-ci.org/SiegeLord/RustCMake)

### Concurrency

-   [aturon/crossbeam](https://github.com/aturon/crossbeam) <span> | ★ 383, pushed 1 days ago | </span> – Support for parallelism and low-level concurrency in Rust [![](https://travis-ci.org/aturon/crossbeam.svg?branch=master)](https://travis-ci.org/aturon/crossbeam)
-   [nikomatsakis/rayon](https://github.com/nikomatsakis/rayon) <span> | ★ 345, pushed 1 days ago | </span> – A data parallelism library for Rust [![](https://travis-ci.org/nikomatsakis/rayon.svg?branch=master)](https://travis-ci.org/nikomatsakis/rayon)

### Cloud

-   AWS
    -   [rusoto/rusoto](https://github.com/rusoto/rusoto) <span> | ★ 95, pushed 0 days ago | </span> — [![](https://travis-ci.org/rusoto/rusoto.svg?branch=master)](https://travis-ci.org/rusoto/rusoto)
    -   [thommay/aws-rs](https://github.com/thommay/aws-rs) <span> | ★ 4, pushed 273 days ago | </span> — [![](https://travis-ci.org/thommay/aws-rs.svg?branch=master)](https://travis-ci.org/thommay/aws-rs)
-   DigitalOcean
    -   [kbknapp/doapi](https://github.com/kbknapp/doapi-rs) <span> | ★ 12, pushed 11 days ago | </span> — DigitalOcean v2 API bindings [![](https://travis-ci.org/kbknapp/doapi-rs.svg?branch=master)](https://travis-ci.org/kbknapp/doapi-rs)

### Command-line argument parsing

-   [docopt/docopt.rs](https://github.com/docopt/docopt.rs) <span> | ★ 342, pushed 12 days ago | </span> — a Rust implementation of [DocOpt](http://docopt.org) [![](https://travis-ci.org/docopt/docopt.rs.svg?branch=master)](https://travis-ci.org/docopt/docopt.rs)
-   [kbknapp/clap-rs](https://github.com/kbknapp/clap-rs) <span> | ★ 444, pushed 0 days ago | </span> — a simple to use, full featured command-line argument parser [![](https://travis-ci.org/kbknapp/clap-rs.svg?branch=master)](https://travis-ci.org/kbknapp/clap-rs)

### Command-line interface

-   [kkawakam/rustyline](https://github.com/kkawakam/rustyline) <span> | ★ 8, pushed 1 days ago | </span> - Readline Implementation in Rust [![Build Status](https://travis-ci.org/kkawakam/rustyline.svg?branch=master)](https://travis-ci.org/kkawakam/rustyline)
-   [srijs/rust-copperline](https://github.com/srijs/rust-copperline) <span> | ★ 10, pushed 45 days ago | </span> - Pure-Rust Command Line Editing Library

### Compression

-   [alexcrichton/bzip2-rs](https://github.com/alexcrichton/bzip2-rs) <span> | ★ 6, pushed 0 days ago | </span> — [libbz2](http://www.bzip.org) bindings [![](https://travis-ci.org/alexcrichton/bzip2-rs.svg?branch=master)](https://travis-ci.org/alexcrichton/bzip2-rs)
-   [alexcrichton/flate2-rs](https://github.com/alexcrichton/flate2-rs) <span> | ★ 45, pushed 0 days ago | </span> — [miniz](https://code.google.com/p/miniz/) bindings [![](https://travis-ci.org/alexcrichton/flate2-rs.svg?branch=master)](https://travis-ci.org/alexcrichton/flate2-rs)
-   [alexcrichton/tar-rs](https://github.com/alexcrichton/tar-rs) <span> | ★ 32, pushed 0 days ago | </span> — tar archive reading/writing in Rust [![](https://travis-ci.org/alexcrichton/tar-rs.svg?branch=master)](https://travis-ci.org/alexcrichton/tar-rs)
-   [ende76/brotli-rs](https://github.com/ende76/brotli-rs) <span> | ★ 29, pushed 3 days ago | </span> — implementation of [Brotli](http://google-opensource.blogspot.co.uk/2015/09/introducing-brotli-new-compression.html) compression
-   [JeffBelgum/rust-snappy](https://github.com/JeffBelgum/rust-snappy) <span> | ★ 4, pushed 72 days ago | </span> — [snappy](https://github.com/google/snappy) bindings [![](https://travis-ci.org/JeffBelgum/rust-snappy.svg?branch=master)](https://travis-ci.org/JeffBelgum/rust-snappy)
-   [slackito/zip](https://github.com/slackito/zip) <span> | ★ 9, pushed 396 days ago | </span> — read and write ZIP archives [![](https://travis-ci.org/slackito/zip.svg?branch=master)](https://travis-ci.org/slackito/zip)

### Computation

-   BLAS \[ [blas](https://crates.io/keywords/blas) \]
    -   [mikkyang/rust-blas](https://github.com/mikkyang/rust-blas) <span> | ★ 42, pushed 44 days ago | </span> — [BLAS](https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) bindings
    -   [stainless-steel/blas](https://github.com/stainless-steel/blas) <span> | ★ 9, pushed 0 days ago | </span> — [BLAS](https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) bindings [![](https://travis-ci.org/stainless-steel/blas.svg?branch=master)](https://travis-ci.org/stainless-steel/blas)
-   GMP
    -   [thestinger/rust-gmp](https://github.com/thestinger/rust-gmp) <span> | ★ 27, pushed 326 days ago | </span> — [libgmp](https://gmplib.org/) bindings [![](https://travis-ci.org/thestinger/rust-gmp.svg?branch=master)](https://travis-ci.org/thestinger/rust-gmp)
-   GSL
    -   [GuillaumeGomez/rust-GSL](https://github.com/GuillaumeGomez) — [GSL](http://www.gnu.org/software/gsl/) bindings [![](https://travis-ci.org/GuillaumeGomez/rust-GSL.svg?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-GSL)
-   LAPACK
    -   [stainless-steel/lapack](https://github.com/stainless-steel/lapack) <span> | ★ 9, pushed 0 days ago | </span> — [LAPACK](https://en.wikipedia.org/wiki/LAPACK) bindings [![](https://travis-ci.org/stainless-steel/lapack.svg?branch=master)](https://travis-ci.org/stainless-steel/lapack)
-   OpenCL
    -   [arrayfire/arrayfire-rust](https://github.com/arrayfire/arrayfire-rust) <span> | ★ 72, pushed 0 days ago | </span> — [Arrayfire](http://arrayfire.com) bindings
    -   [luqmana/rust-opencl](https://github.com/luqmana/rust-opencl) <span> | ★ 108, pushed 43 days ago | </span> — [OpenCL](https://www.khronos.org/opencl/) bindings [![](https://travis-ci.org/luqmana/rust-opencl.svg?branch=master)](https://travis-ci.org/luqmana/rust-opencl)
-   Scirust
    -   [indigits/scirust](https://github.com/indigits/scirust) <span> | ★ 55, pushed 213 days ago | </span> — scientific computing library in Rust [![Build Status](https://travis-ci.org/indigits/scirust.svg?branch=master)](https://travis-ci.org/indigits/scirust)

### Cryptography

\[ [crypto](https://crates.io/keywords/crypto) , [cryptography](https://crates.io/keywords/cryptography) \]

-   [briansmith/ring](https://github.com/briansmith/ring) <span> | ★ 129, pushed 0 days ago | </span> — Safe, fast, small crypto using Rust and BoringSSL's cryptography primitives. [![](https://travis-ci.org/briansmith/ring.svg?branch=master)](https://travis-ci.org/briansmith/ring)
-   [briansmith/webpki](https://github.com/briansmith/webpki) <span> | ★ 37, pushed 52 days ago | </span> — Web PKI TLS X.509 certificate validation in Rust. [![](https://travis-ci.org/briansmith/webpki.svg?branch=master)](https://travis-ci.org/briansmith/webpki)
-   [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) <span> | ★ 382, pushed 0 days ago | </span> — cryptographic algorithms in Rust [![](https://travis-ci.org/DaGenix/rust-crypto.svg?branch=master)](https://travis-ci.org/DaGenix/rust-crypto)
-   [dnaq/sodiumoxide](https://github.com/dnaq/sodiumoxide) <span> | ★ 144, pushed 15 days ago | </span> — [libsodium](https://github.com/jedisct1/libsodium) bindings [![](https://travis-ci.org/dnaq/sodiumoxide.svg?branch=master)](https://travis-ci.org/dnaq/sodiumoxide)
-   [klutzy/suruga](https://github.com/klutzy/suruga) <span> | ★ 118, pushed 92 days ago | </span> — a Rust implementation of [TLS 1.2](http://tools.ietf.org/html/rfc5246)
-   [libOctavo/octavo](https://github.com/libOctavo/octavo) <span> | ★ 97, pushed 0 days ago | </span> — Modular hash and crypto library in Rust [![](https://api.travis-ci.org/libOctavo/octavo.svg?branch=master)](https://travis-ci.org/libOctavo/octavo)
-   [seb-m/common.rs](https://github.com/seb-m/common.rs) <span> | ★ 1, pushed 481 days ago | </span> — Common Rust crypto utilities [![](https://travis-ci.org/seb-m/common.rs.svg?branch=master)](https://travis-ci.org/seb-m/common.rs)
-   [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) <span> | ★ 164, pushed 0 days ago | </span> — [OpenSSL](https://www.openssl.org/) bindings [![](https://travis-ci.org/sfackler/rust-openssl.svg?branch=master)](https://travis-ci.org/sfackler/rust-openssl)

### Database

\[ [database](https://crates.io/keywords/database) \]

-   [pingcap/tikv](https://github.com/pingcap/tikv) <span> | ★ 550, pushed 0 days ago | </span> — TiKV is a distributed KV database powered by Rust [![](https://travis-ci.org/pingcap/tikv.svg?branch=master)](https://travis-ci.org/pingcap/tikv)
-   [sfackler/r2d2](https://github.com/sfackler/r2d2) <span> | ★ 76, pushed 5 days ago | </span> — generic connection pool [![](https://travis-ci.org/sfackler/r2d2.svg?branch=master)](https://travis-ci.org/sfackler/r2d2)
-   NoSQL \[ [nosql](https://crates.io/keywords/nosql) \]
    -   Cassandra \[ [cassandra](https://crates.io/keywords/cassandra) , [cql](https://crates.io/keywords/cql) \]
        -   [tupshin/cassandra-rust](https://github.com/tupshin/cassandra-rs) <span> | ★ 23, pushed 36 days ago | </span> — [Cassandra](http://cassandra.apache.org) bindings
    -   CouchDB \[ [couchdb](https://crates.io/keywords/couchdb) \]
        -   [couchdb-rs/couchdb](https://github.com/couchdb-rs/couchdb) <span> | ★ 6, pushed 25 days ago | </span> \[ [couchdb](https://crates.io/crates/couchdb/) \] — a Rust client for the CouchDB REST API [![](https://travis-ci.org/couchdb-rs/couchdb.svg?branch=master)](https://travis-ci.org/couchdb-rs/couchdb)
    -   Elasticsearch \[ [elasticsearch](https://crates.io/keywords/elasticsearch) \]
        -   [benashford/rs-es](https://github.com/benashford/rs-es) <span> | ★ 67, pushed 8 days ago | </span> \[ [rs-es](https://crates.io/crates/rs-es/) \] — a Rust client for the [Elastic](https://www.elastic.co/) REST API [![](https://travis-ci.org/benashford/rs-es.svg?branch=master)](https://travis-ci.org/benashford/rs-es)
    -   etcd
        -   [jimmycuadra/rust-etcd](https://github.com/jimmycuadra/rust-etcd) <span> | ★ 42, pushed 5 days ago | </span> \[ [etcd](https://crates.io/crates/etcd/) \] — A client library for CoreOS's etcd. [![](https://travis-ci.org/jimmycuadra/rust-etcd.svg?branch=master)](https://travis-ci.org/jimmycuadra/rust-etcd)
    -   ForestDB
        -   [vhbit/sherwood](https://github.com/vhbit/sherwood) <span> | ★ 0, pushed 300 days ago | </span> — [ForestDB](https://github.com/couchbase/forestdb) bindings [![](https://travis-ci.org/vhbit/sherwood.svg?branch=master)](https://travis-ci.org/vhbit/sherwood)
    -   LMDB \[ [lmdb](https://crates.io/keywords/lmdb) \]
        -   [vhbit/lmdb-rs](https://github.com/vhbit/lmdb-rs) <span> | ★ 43, pushed 48 days ago | </span> \[ [lmdb-rs](https://crates.io/crates/lmdb-rs/) \] — [LMDB](http://symas.com/mdb/) bindings [![](https://travis-ci.org/vhbit/lmdb-rs.svg?branch=master)](https://travis-ci.org/vhbit/lmdb-rs)
    -   MongoDB \[ [mongodb](https://crates.io/keywords/mongodb) \]
        -   [mongodb-labs/mongo-rust-driver-prototype](https://github.com/mongodb-labs/mongo-rust-driver-prototype) <span> | ★ 99, pushed 1 days ago | </span> \[ [mongodb](https://crates.io/crates/mongodb/) \] — [MongoDB](https://www.mongodb.org/) bindings [![](https://travis-ci.org/mongodb-labs/mongo-rust-driver-prototype.svg)](https://travis-ci.org/mongodb-labs/mongo-rust-driver-prototype)
    -   Neo4j \[ [cypher](https://crates.io/keywords/cypher) , [neo4j](https://crates.io/keywords/neo4j) \]
    -   Redis \[ [redis](https://crates.io/keywords/redis) \]
        -   [mitsuhiko/redis-rs](https://github.com/mitsuhiko/redis-rs) <span> | ★ 414, pushed 7 days ago | </span> — [Redis](http://redis.io) library in Rust [![](https://travis-ci.org/mitsuhiko/redis-rs.svg?branch=master)](https://travis-ci.org/mitsuhiko/redis-rs)
-   SQL \[ [sql](https://crates.io/keywords/sql) \]
    -   MySql \[ [mysql](https://crates.io/keywords/mysql) \]
        -   [blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) <span> | ★ 90, pushed 12 days ago | </span> \[ [mysql](https://crates.io/crates/mysql) \] — a native MySql client [![](https://travis-ci.org/blackbeam/rust-mysql-simple.svg?branch=master)](https://travis-ci.org/blackbeam/rust-mysql-simple)
    -   ORM \[ [orm](https://crates.io/keywords/orm) \]
        -   [deuterium-orm/deuterium-orm](https://github.com/deuterium-orm/deuterium) <span> | ★ 155, pushed 322 days ago | </span> — an SQL query builder for Rust [![Build Status](https://travis-ci.org/deuterium-orm/deuterium.svg?branch=master)](https://travis-ci.org/deuterium-orm/deuterium)
        -   [ivanceras/rustorm](https://github.com/ivanceras/rustorm) <span> | ★ 122, pushed 3 days ago | </span> — an ORM for Rust [![Build Status](https://api.travis-ci.org/ivanceras/rustorm.svg)](https://travis-ci.org/ivanceras/rustorm)
        -   [phonkee/treasure](https://github.com/phonkee/treasure) <span> | ★ 42, pushed 255 days ago | </span> — an ORM for Rust
        -   [sgrif/diesel](https://github.com/sgrif/diesel) — an ORM and Query builder for Rust [![Build Status](https://api.travis-ci.org/sgrif/diesel.svg)](https://travis-ci.org/sgrif/diesel)
    -   PostgreSql \[ [postgres](https://crates.io/keywords/postgres) , [postgresql](https://crates.io/keywords/postgresql) \]
        -   [sfackler/rust-postgres](https://github.com/sfackler/rust-postgres) <span> | ★ 522, pushed 1 days ago | </span> \[ [postgres](https://crates.io/crates/postgres/) \] — a native [PostgreSQL](http://www.postgresql.org) client [![](https://travis-ci.org/sfackler/rust-postgres.svg?branch=master)](https://travis-ci.org/sfackler/rust-postgres)
    -   Sqlite \[ [sqlite](https://crates.io/keywords/sqlite) \]
        -   [dckc/rust-sqlite3](https://github.com/dckc/rust-sqlite3) <span> | ★ 40, pushed 1 days ago | </span> — [Sqlite3](http://www.sqlite.org/) bindings [![](https://travis-ci.org/dckc/rust-sqlite3.svg?branch=master)](https://travis-ci.org/dckc/rust-sqlite3)
        -   [jgallagher/rusqlite](https://github.com/jgallagher/rusqlite) <span> | ★ 84, pushed 4 days ago | </span> — [Sqlite3](http://www.sqlite.org/) bindings [![](https://travis-ci.org/jgallagher/rusqlite.svg?branch=master)](https://travis-ci.org/jgallagher/rusqlite)
        -   [linuxfood/rustsqlite](https://github.com/linuxfood/rustsqlite) <span> | ★ 78, pushed 380 days ago | </span> — [Sqlite3](http://www.sqlite.org/) bindings [![](https://travis-ci.org/linuxfood/rustsqlite.svg?branch=master)](https://travis-ci.org/linuxfood/rustsqlite)

### Data structures

-   [bluss/rust-itertools](https://github.com/bluss/rust-itertools) <span> | ★ 135, pushed 3 days ago | </span> — [![](https://travis-ci.org/bluss/rust-itertools.svg?branch=master)](https://travis-ci.org/bluss/rust-itertools)
-   [fizyk20/generic-array](https://github.com/fizyk20/generic-array) <span> | ★ 9, pushed 31 days ago | </span> – a hack to allow for arrays sized by typenums [![](https://travis-ci.org/fizyk20/generic-array.svg?branch=master)](https://travis-ci.org/fizyk20/generic-array)
-   [Nemo157/roaring-rs](https://github.com/Nemo157/roaring-rs) <span> | ★ 27, pushed 9 days ago | </span> – Roaring Bitmaps in Rust [![](https://travis-ci.org/Nemo157/roaring-rs.svg?branch=master)](https://travis-ci.org/Nemo157/roaring-rs)
-   [reem/rust-typemap](https://github.com/reem/rust-typemap) <span> | ★ 45, pushed 61 days ago | </span> — [![](https://travis-ci.org/reem/rust-typemap.svg?branch=master)](https://travis-ci.org/reem/rust-typemap)
-   [serde-rs/serde](https://github.com/serde-rs/serde) <span> | ★ 358, pushed 0 days ago | </span> — a framework to generically serialize Rust data structures [![](https://api.travis-ci.org/serde-rs/serde.svg?branch=master)](https://travis-ci.org/serde-rs/serde)

### Date and time

\[ [date](https://crates.io/keywords/date) , [time](https://crates.io/keywords/time) \]

-   [lifthrasiir/rust-chrono](https://github.com/lifthrasiir/rust-chrono) <span> | ★ 160, pushed 22 days ago | </span> — [![](https://travis-ci.org/lifthrasiir/rust-chrono.svg?branch=master)](https://travis-ci.org/lifthrasiir/rust-chrono)
-   [rust-lang-deprecated/time](https://github.com/rust-lang-deprecated/time) <span> | ★ 65, pushed 0 days ago | </span> — [![](https://travis-ci.org/rust-lang-deprecated/time.svg?branch=master)](https://travis-ci.org/rust-lang-deprecated/time)

### Distributed Systems

-   Apache Kafka
    -   [spicavigo/kafka-rust](https://github.com/spicavigo/kafka-rust) <span> | ★ 94, pushed 1 days ago | </span> — [![](https://travis-ci.org/spicavigo/kafka-rust.svg?branch=master)](https://travis-ci.org/spicavigo/kafka-rust)
-   Beanstalkd
    -   [schickling/rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) <span> | ★ 11, pushed 100 days ago | </span> — [Beanstalkd](https://github.com/kr/beanstalkd) bindings [![](https://travis-ci.org/schickling/rust-beanstalkd.svg?branch=master)](https://travis-ci.org/schickling/rust-beanstalkd)
-   HDFS
    -   [hyunsik/hdfs-rs](https://github.com/hyunsik/hdfs-rs) <span> | ★ 14, pushed 195 days ago | </span> — libhdfs bindings [![](https://travis-ci.org/hyunsik/hdfs-rs.svg?branch=master)](https://travis-ci.org/hyunsik/hdfs-rs)

### Email

\[ [email](https://crates.io/keywords/email) \]

-   [gsquire/sendgrid-rs](https://github.com/gsquire/sendgrid-rs) <span> | ★ 4, pushed 1 days ago | </span> — unofficial Rust library for SendGrid API [![](https://travis-ci.org/gsquire/sendgrid-rs.svg?branch=master)](https://travis-ci.org/gsquire/sendgrid-rs)
-   [lettre/lettre](https://github.com/lettre/lettre) <span> | ★ 69, pushed 5 days ago | </span> — an SMTP-library for Rust [![](https://travis-ci.org/lettre/lettre.svg?branch=master)](https://travis-ci.org/lettre/lettre)

### Encoding

\[ [encoding](https://crates.io/keywords/encoding) \]

-   ASN.1
    -   [alex/rust-asn1](https://github.com/alex/rust-asn1) <span> | ★ 18, pushed 16 days ago | </span> — a Rust ASN.1 (DER) serializer [![](https://travis-ci.org/alex/rust-asn1.svg?branch=master)](https://travis-ci.org/alex/rust-asn1)
-   Bencode
    -   [arjantop/rust-bencode](https://github.com/arjantop/rust-bencode) <span> | ★ 19, pushed 165 days ago | </span> — [Bencode](https://en.wikipedia.org/wiki/Bencode) implementation in Rust [![](https://travis-ci.org/arjantop/rust-bencode.svg?branch=master)](https://travis-ci.org/arjantop/rust-bencode)
-   Binary
    -   [arcnmx/nue](https://github.com/arcnmx/nue) <span> | ★ 21, pushed 247 days ago | </span> — I/O and binary data encoding for Rust [![](https://travis-ci.org/arcnmx/nue.svg?branch=master)](https://travis-ci.org/arcnmx/nue)
    -   [TyOverby/bincode](https://github.com/TyOverby/bincode) <span> | ★ 107, pushed 4 days ago | </span> — a binary encoder/decoder in Rust [![](https://travis-ci.org/TyOverby/bincode.svg?branch=master)](https://travis-ci.org/TyOverby/bincode)
-   Byte swapping
    -   [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder) <span> | ★ 85, pushed 18 days ago | </span> — Supports big-endian, little-endian and native byte orders [![](https://travis-ci.org/BurntSushi/byteorder.svg?branch=master)](https://travis-ci.org/BurntSushi/byteorder)
-   Cap'n Proto
    -   [dwrensha/capnproto-rust](https://github.com/dwrensha/capnproto-rust) <span> | ★ 291, pushed 77 days ago | </span> — [![](https://travis-ci.org/dwrensha/capnproto-rust.svg?branch=master)](https://travis-ci.org/dwrensha/capnproto-rust)
-   CBOR
    -   [BurntSushi/rust-cbor](https://github.com/BurntSushi/rust-cbor) <span> | ★ 37, pushed 226 days ago | </span> — Supports JSON conversion and type-based encoding/decoding [![](https://travis-ci.org/BurntSushi/rust-cbor.svg?branch=master)](https://travis-ci.org/BurntSushi/rust-cbor)
-   Character Encoding
    -   [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding) <span> | ★ 95, pushed 57 days ago | </span> — [![](https://travis-ci.org/lifthrasiir/rust-encoding.svg?branch=master)](https://travis-ci.org/lifthrasiir/rust-encoding)
-   CRC
    -   [mrhooray/crc-rs](https://github.com/mrhooray/crc-rs) <span> | ★ 10, pushed 245 days ago | </span> — [![](https://travis-ci.org/mrhooray/crc-rs.svg?branch=master)](https://travis-ci.org/mrhooray/crc-rs)
-   CSV
    -   [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) <span> | ★ 167, pushed 23 days ago | </span> — [![](https://travis-ci.org/BurntSushi/rust-csv.svg?branch=master)](https://travis-ci.org/BurntSushi/rust-csv)
-   HTML
    -   [servo/html5ever](https://github.com/servo/html5ever) <span> | ★ 292, pushed 4 days ago | </span> — High-performance browser-grade HTML5 parser [![](https://travis-ci.org/servo/html5ever.svg?branch=master)](https://travis-ci.org/servo/html5ever)
-   JSON
    -   [serde-rs/json](https://github.com/serde-rs/json) <span> | ★ 45, pushed 0 days ago | </span> \[ [serde\_json](https://crates.io/crates/serde_json/) \] — JSON support for [Serde](https://github.com/serde-rs/serde) framework [![](https://travis-ci.org/serde-rs/json.svg?branch=master)](https://travis-ci.org/serde-rs/json)
-   Jsonnet
    -   [Qihoo360/rust-jsonnet](https://github.com/Qihoo360/rust-jsonnet) <span> | ★ 16, pushed 224 days ago | </span> — [![](https://travis-ci.org/Qihoo360/rust-jsonnet.svg?branch=master)](https://travis-ci.org/Qihoo360/rust-jsonnet)
-   MsgPack
    -   [mneumann/rust-msgpack](https://github.com/mneumann/rust-msgpack) <span> | ★ 95, pushed 4 days ago | </span> — [![](https://travis-ci.org/mneumann/rust-msgpack.svg?branch=master)](https://travis-ci.org/mneumann/rust-msgpack)
    -   [3Hren/msgpack-rust](https://github.com/3Hren/msgpack-rust) <span> | ★ 70, pushed 18 days ago | </span> — a pure Rust low/high level MessagePack implementation [![](https://travis-ci.org/3Hren/msgpack-rust.svg?branch=master)](https://travis-ci.org/3Hren/msgpack-rust)
-   ProtocolBuffers
    -   [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf) <span> | ★ 238, pushed 2 days ago | </span> — [![](https://travis-ci.org/stepancheg/rust-protobuf.svg?branch=master)](https://travis-ci.org/stepancheg/rust-protobuf)
-   RON (Rusty Object Notation)
    -   [kvark/ron](https://github.com/kvark/ron) <span> | ★ 61, pushed 348 days ago | </span> — [![](https://travis-ci.org/kvark/ron.svg?branch=master)](https://travis-ci.org/kvark/ron)
-   Tnetstring
    -   [erickt/rust-tnetstring](https://github.com/erickt/rust-tnetstring) <span> | ★ 13, pushed 526 days ago | </span> — [![](https://travis-ci.org/erickt/rust-tnetstring.svg?branch=master)](https://travis-ci.org/erickt/rust-tnetstring)
-   TOML
    -   [alexcrichton/toml-rs](https://github.com/alexcrichton/toml-rs) <span> | ★ 152, pushed 0 days ago | </span> — [![](https://travis-ci.org/alexcrichton/toml-rs.svg?branch=master)](https://travis-ci.org/alexcrichton/toml-rs)
-   XML
    -   [Florob/RustyXML](https://github.com/Florob/RustyXML) <span> | ★ 53, pushed 49 days ago | </span> — an XML parser written in Rust [![](https://travis-ci.org/Florob/RustyXML.svg?branch=master)](https://travis-ci.org/Florob/RustyXML)
    -   [shepmaster/sxd-document](https://github.com/shepmaster/sxd-document) <span> | ★ 49, pushed 0 days ago | </span> — An XML library in Rust [![](https://travis-ci.org/shepmaster/sxd-document.svg?branch=master)](https://travis-ci.org/shepmaster/sxd-document)
    -   [shepmaster/sxd-xpath](https://github.com/shepmaster/sxd-xpath) <span> | ★ 19, pushed 0 days ago | </span> — An XPath library in Rust [![](https://travis-ci.org/shepmaster/sxd-xpath.svg?branch=master)](https://travis-ci.org/shepmaster/sxd-xpath)
    -   [netvl/xml-rs](https://github.com/netvl/xml-rs) <span> | ★ 98, pushed 35 days ago | </span> — a streaming XML library [![](https://travis-ci.org/netvl/xml-rs.svg?branch=master)](https://travis-ci.org/netvl/xml-rs)
-   YAML
    -   [chyh1990/yaml-rust](https://github.com/chyh1990/yaml-rust) <span> | ★ 77, pushed 6 days ago | </span> — The missing YAML 1.2 implementation for Rust. [![](https://travis-ci.org/chyh1990/yaml-rust.svg?branch=master)](https://travis-ci.org/chyh1990/yaml-rust)
    -   [dtolnay/serde-yaml](https://github.com/dtolnay/serde-yaml) <span> | ★ 6, pushed 28 days ago | </span> \[ [serde\_yaml](https://crates.io/crates/serde_yaml/) \] — YAML support for [Serde](https://github.com/serde-rs/serde) framework [![](https://travis-ci.org/dtolnay/serde-yaml.svg?branch=master)](https://travis-ci.org/dtolnay/serde-yaml)
    -   [kimhyunkang/libyaml-rust](https://github.com/kimhyunkang/libyaml-rust) <span> | ★ 14, pushed 175 days ago | </span> — [libyaml](http://pyyaml.org/wiki/LibYAML) bindings [![](https://travis-ci.org/kimhyunkang/libyaml-rust.svg?branch=master)](https://travis-ci.org/kimhyunkang/libyaml-rust)

### Game development

-   Allegro
    -   [SiegeLord/RustAllegro](https://github.com/SiegeLord/RustAllegro) <span> | ★ 27, pushed 0 days ago | </span> — [Allegro 5](http://liballeg.org/) bindings [![](https://travis-ci.org/SiegeLord/RustAllegro.svg?branch=master)](https://travis-ci.org/SiegeLord/RustAllegro)
-   Amethyst
    -   [ebkalderon/amethyst](https://github.com/ebkalderon/amethyst) <span> | ★ 4, pushed 9 days ago | </span> — data-oriented game engine [![](https://travis-ci.org/ebkalderon/amethyst.svg?branch=master)](https://travis-ci.org/ebkalderon/amethyst)
-   Corange
    -   [lucidscape/corange-rs](https://github.com/lucidscape/corange-rs) <span> | ★ 18, pushed 121 days ago | </span> — [Corange](https://github.com/orangeduck/Corange) bindings
-   Piston
    -   [Piston](http://www.piston.rs) — [![](https://travis-ci.org/PistonDevelopers/piston.svg?branch=master)](https://travis-ci.org/PistonDevelopers/piston)
-   SDL \[ [sdl](https://crates.io/keywords/sdl) \]
    -   [AngryLawyer/rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) <span> | ★ 348, pushed 6 days ago | </span> — [SDL2](http://www.libsdl.org/) bindings [![](https://travis-ci.org/AngryLawyer/rust-sdl2.svg?branch=master)](https://travis-ci.org/AngryLawyer/rust-sdl2)
    -   [brson/rust-sdl](https://github.com/brson/rust-sdl) <span> | ★ 159, pushed 322 days ago | </span> — [SDL1](http://www.libsdl.org/) bindings [![](https://travis-ci.org/brson/rust-sdl.svg?branch=master)](https://travis-ci.org/brson/rust-sdl)
-   SFML
    -   [jeremyletang/rust-sfml](https://github.com/jeremyletang/rust-sfml) <span> | ★ 191, pushed 3 days ago | </span> — [SFML](http://www.sfml-dev.org/) bindings [![](https://travis-ci.org/jeremyletang/rust-sfml.svg?branch=master)](https://travis-ci.org/jeremyletang/rust-sfml)
-   Voxlap
    -   [bbodi/rust-voxlap](https://github.com/bbodi/rust-voxlap) <span> | ★ 9, pushed 543 days ago | </span> — [Voxlap](http://advsys.net/ken/voxlap.htm) bindings

### Geospatial

\[ [geo](https://crates.io/keywords/geo) , [gis](https://crates.io/keywords/gis) \]

-   [Georust](https://github.com/georust) — geospatial tools and libraries written in Rust

### GUI

\[ [gui](https://crates.io/keywords/gui) \]

-   [PistonDevelopers/conrod](https://github.com/PistonDevelopers/conrod/) — An easy-to-use, immediate-mode, 2D GUI library written entirely in Rust [![](https://travis-ci.org/PistonDevelopers/conrod.svg?branch=master)](https://travis-ci.org/PistonDevelopers/conrod)
-   [pravic/rust-sciter](https://github.com/pravic/rust-sciter) <span> | ★ 10, pushed 2 days ago | </span> — Rust bindings for the Sciter, embeddable HTML/CSS/script engine (cross-platform desktop GUI toolkit). [![](https://ci.appveyor.com/api/projects/status/github/pravic/rust-sciter?svg=true)](https://ci.appveyor.com/project/pravic/rust-sciter)
-   Cocoa
    -   [servo/rust-cocoa](https://github.com/servo/cocoa-rs) <span> | ★ 182, pushed 19 days ago | </span>
-   IUP
    -   [dcampbell24/iup-rust](https://github.com/dcampbell24/iup-rust) <span> | ★ 16, pushed 173 days ago | </span> — [IUP](http://webserver2.tecgraf.puc-rio.br/iup/) bindings [![](https://travis-ci.org/dcampbell24/iup-rust.svg?branch=master)](https://travis-ci.org/dcampbell24/iup-rust)
    -   [Kiss-ui](https://github.com/KISS-UI/kiss-ui) <span> | ★ 243, pushed 136 days ago | </span> — a simple UI framework built on IUP [![Build Status](https://travis-ci.org/cybergeek94/kiss-ui.svg?branch=master)](https://travis-ci.org/cybergeek94/kiss-ui)
-   GTK+ \[ [gtk](https://crates.io/keywords/gtk) \]
    -   [gtk-rs/gtk](https://github.com/gtk-rs/gtk) <span> | ★ 264, pushed 0 days ago | </span> — [GTK+](http://www.gtk.org) bindings [![](https://travis-ci.org/gtk-rs/gtk.svg?branch=master)](https://travis-ci.org/gtk-rs/gtk)
-   ncurses \[ [ncurses](https://crates.io/keywords/ncurses) \]
    -   [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) <span> | ★ 161, pushed 16 days ago | </span> — [ncurses](http://www.gnu.org/software/ncurses/) bindings [![](https://travis-ci.org/jeaye/ncurses-rs.svg?branch=master)](https://travis-ci.org/jeaye/ncurses-rs)
-   OpenGL \[ [opengl](https://crates.io/keywords/opengl) \]
    -   [bjz/gl-rs](https://github.com/bjz/gl-rs) <span> | ★ 215, pushed 27 days ago | </span> — [![](https://travis-ci.org/bjz/gl-rs.svg?branch=master)](https://travis-ci.org/bjz/gl-rs)
    -   [bjz/glfw-rs](https://github.com/PistonDevelopers/glfw-rs) <span> | ★ 213, pushed 2 days ago | </span> — [![](https://travis-ci.org/bjz/glfw-rs.svg?branch=master)](https://travis-ci.org/bjz/glfw-rs)
    -   [tomaka/glium](https://github.com/tomaka/glium) <span> | ★ 768, pushed 2 days ago | </span> — safe OpenGL wrapper for the Rust language. [![](https://travis-ci.org/tomaka/glium.svg?branch=master)](https://travis-ci.org/tomaka/glium)
    -   [tomaka/glutin](https://github.com/tomaka/glutin) <span> | ★ 418, pushed 2 days ago | </span> — Rust alternative to [GLFW](http://www.glfw.org/) [![](https://travis-ci.org/tomaka/glutin.svg?branch=master)](https://travis-ci.org/tomaka/glutin)
-   Qt
    -   [cyndis/qmlrs](https://github.com/cyndis/qmlrs) <span> | ★ 229, pushed 16 days ago | </span> — [QtQuick](http://doc.qt.io) bindings [![](https://travis-ci.org/cyndis/qmlrs.svg?branch=master)](https://travis-ci.org/cyndis/qmlrs)
-   Termbox
    -   [gchp/rustbox](https://github.com/gchp/rustbox) <span> | ★ 172, pushed 2 days ago | </span> — a Rust implementation of [termbox](https://github.com/nsf/termbox) [![](https://travis-ci.org/gchp/rustbox.svg?branch=master)](https://travis-ci.org/gchp/rustbox)

### Image processing

-   [chyh1990/imageproc](https://github.com/chyh1990/imageproc) <span> | ★ 32, pushed 89 days ago | </span> — An advanced image processing library for Rust. [![Build Status](https://travis-ci.org/chyh1990/imageproc.svg?branch=master)](https://travis-ci.org/chyh1990/imageproc)
-   [cybergeek94/img-hash](https://github.com/cybergeek94/img_hash) <span> | ★ 31, pushed 177 days ago | </span> — Perceptual image hashing and comparison for equality and similarity.
-   [PistonDevelopers/image](https://github.com/PistonDevelopers/image) <span> | ★ 293, pushed 0 days ago | </span> — Basic imaging processing functions and methods for converting to and from image formats [![](https://travis-ci.org/PistonDevelopers/image.svg?branch=master)](https://travis-ci.org/PistonDevelopers/image)

### Machine learning

-   [autumnai/leaf](https://github.com/autumnai/leaf) <span> | ★ 3499, pushed 0 days ago | </span> — Open Machine Intelligence framework. [![Build Status](https://travis-ci.org/autumnai/leaf.svg?branch=master)](https://travis-ci.org/autumnai/leaf)
-   [maciejkula/rustlearn](https://github.com/maciejkula/rustlearn) <span> | ★ 141, pushed 50 days ago | </span> — Machine learning crate for Rust. [![Circle CI](https://circleci.com/gh/maciejkula/rustlearn.svg?style=svg)](https://circleci.com/gh/maciejkula/rustlearn)

### Markup language

-   CommonMark
    -   [google/pulldown-cmark](https://github.com/google/pulldown-cmark) <span> | ★ 186, pushed 8 days ago | </span> — [CommonMark](http://commonmark.org/) parser in Rust

### Mobile

-   Android
    -   [tomaka/android-rs-glue](https://github.com/tomaka/android-rs-glue) <span> | ★ 228, pushed 0 days ago | </span> — glue between Rust and Android [![](https://travis-ci.org/tomaka/android-rs-glue.svg?branch=master)](https://travis-ci.org/tomaka/android-rs-glue)
-   iOS
    -   [TimNN/cargo-lipo](https://github.com/TimNN/cargo-lipo) <span> | ★ 7, pushed 22 days ago | </span> — a cargo lipo subcommand which automatically creates a universal library for use with your iOS application. [![](https://travis-ci.org/TimNN/cargo-lipo.svg?branch=master)](https://travis-ci.org/TimNN/cargo-lipo)
    -   [vhbit/ObjCrust](https://github.com/vhbit/ObjCrust) <span> | ★ 35, pushed 291 days ago | </span> — using Rust to create an iOS static library [![](https://travis-ci.org/vhbit/ObjCrust.svg?branch=master)](https://travis-ci.org/vhbit/ObjCrust)
-   Pebble
    -   [andars/pebble.rs](https://github.com/andars/pebble.rs) <span> | ★ 27, pushed 154 days ago | </span> — a crate that allows Rust to be used to develop Pebble applications.

### Network programming

-   FTP
    -   [mattnenterprise/rust-ftp](https://github.com/mattnenterprise/rust-ftp) <span> | ★ 10, pushed 6 days ago | </span> — an [FTP](https://en.wikipedia.org/wiki/File_Transfer_Protocol) client for Rust [![](https://travis-ci.org/mattnenterprise/rust-ftp.svg?branch=master)](https://travis-ci.org/mattnenterprise/rust-ftp)
-   Low level
    -   [libpnet/libpnet](https://github.com/libpnet/libpnet) <span> | ★ 276, pushed 1 days ago | </span> — a cross-platform, low level networking [![](https://travis-ci.org/libpnet/libpnet.svg?branch=master)](https://travis-ci.org/libpnet/libpnet)
-   NanoMsg
    -   [thehydroimpulse/nanomsg.rs](https://github.com/thehydroimpulse/nanomsg.rs) <span> | ★ 155, pushed 43 days ago | </span> — [nanomsg](http://nanomsg.org/) bindings [![](https://travis-ci.org/thehydroimpulse/nanomsg.rs.svg?branch=master)](https://travis-ci.org/thehydroimpulse/nanomsg.rs)
-   NNTP
    -   [mattnenterprise/rust-nntp](https://github.com/mattnenterprise/rust-nntp) <span> | ★ 3, pushed 100 days ago | </span> — an [NNTP](https://en.wikipedia.org/wiki/Network_News_Transfer_Protocol) client for Rust [![](https://travis-ci.org/mattnenterprise/rust-nntp.svg?branch=master)](https://travis-ci.org/mattnenterprise/rust-nntp)
-   POP3
    -   [mattnenterprise/rust-pop3](https://github.com/mattnenterprise/rust-pop3) <span> | ★ 3, pushed 319 days ago | </span> — a [POP3](https://en.wikipedia.org/wiki/Post_Office_Protocol) client for Rust [![](https://travis-ci.org/mattnenterprise/rust-pop3.svg?branch=master)](https://travis-ci.org/mattnenterprise/rust-pop3)
-   SSH
    -   [alexcrichton/ssh2-rs](https://github.com/alexcrichton/ssh2-rs) <span> | ★ 44, pushed 0 days ago | </span> — [libssh2](http://www.libssh2.org/) bindings [![](https://travis-ci.org/alexcrichton/ssh2-rs.svg?branch=master)](https://travis-ci.org/alexcrichton/ssh2-rs)
-   Stomp
    -   [zslayton/stomp-rs](https://github.com/zslayton/stomp-rs) <span> | ★ 34, pushed 56 days ago | </span> — a [STOMP 1.2](http://stomp.github.io/stomp-specification-1.2.html) client implementation in Rust [![](https://travis-ci.org/zslayton/stomp-rs.svg?branch=master)](https://travis-ci.org/zslayton/stomp-rs)
-   uTP
    -   [meqif/rust-utp](https://github.com/meqif/rust-utp) <span> | ★ 31, pushed 104 days ago | </span> — a [uTP](http://www.bittorrent.org/beps/bep_0029.html) (Micro Transport Protocol) library for Rust. [![](https://travis-ci.org/meqif/rust-utp.svg?branch=master)](https://travis-ci.org/meqif/rust-utp)
-   ZeroMQ
    -   [erickt/rust-zmq](https://github.com/erickt/rust-zmq) <span> | ★ 166, pushed 32 days ago | </span> — [ZeroMQ](http://zeromq.org/) bindings [![](https://travis-ci.org/erickt/rust-zmq.svg?branch=master)](https://travis-ci.org/erickt/rust-zmq)

### Parser

-   [Geal/nom](https://github.com/Geal/nom) <span> | ★ 762, pushed 7 days ago | </span> — parser combinator library [![](https://travis-ci.org/Geal/nom.svg?branch=master)](https://travis-ci.org/Geal/nom)
-   [ivanceras/inquerest](https://github.com/ivanceras/inquerest) <span> | ★ 14, pushed 10 days ago | </span> — an URL parameter parser for rest filter inquiry [![Build Status](https://travis-ci.org/ivanceras/inquerest.svg?branch=master)](https://travis-ci.org/ivanceras/inquerest)
-   [kevinmehall/rust-peg](https://github.com/kevinmehall/rust-peg) <span> | ★ 257, pushed 18 days ago | </span> — Parsing Expression Grammar (PEG) parser generator
-   [m4rw3r/chomp](https://github.com/m4rw3r/chomp) <span> | ★ 98, pushed 19 days ago | </span> – A fast monadic-style parser combinator [![](https://travis-ci.org/m4rw3r/chomp.svg?branch=master)](https://travis-ci.org/m4rw3r/chomp)
-   [Marwes/combine](https://github.com/Marwes/combine) <span> | ★ 137, pushed 3 days ago | </span> — parser combinator library [![](https://travis-ci.org/Marwes/combine.svg?branch=master)](https://travis-ci.org/Marwes/combine)
-   [nikomatsakis/lalrpop](https://github.com/nikomatsakis/lalrpop) <span> | ★ 217, pushed 0 days ago | </span> - LR(1) parser generator for Rust [![Build status](https://travis-ci.org/nikomatsakis/lalrpop.svg?branch=master)](https://travis-ci.org/nikomatsakis/lalrpop)
-   [ptal/oak](https://github.com/ptal/oak) <span> | ★ 76, pushed 2 days ago | </span> — a typed PEG parser generator (compiler plugin)
-   [rustless/queryst](https://github.com/rustless/queryst) <span> | ★ 18, pushed 147 days ago | </span> — a query string parsing library for Rust inspired by <https://github.com/ljharb/qs> [![Build Status](https://travis-ci.org/rustless/queryst.svg?branch=master)](https://travis-ci.org/rustless/queryst)

### Platform specific

-   Linux
    -   [hannobraun/inotify-rs](https://github.com/hannobraun/inotify-rs) <span> | ★ 35, pushed 75 days ago | </span> — [inotify](https://en.wikipedia.org/wiki/Inotify) bindings [![](https://travis-ci.org/hannobraun/inotify-rs.svg?branch=master)](https://travis-ci.org/hannobraun/inotify-rs)
-   Unix-like
    -   [carllerche/nix-rust](https://github.com/carllerche/nix-rust) — Unix-like API bindings [![](https://travis-ci.org/carllerche/nix-rust.svg?branch=master)](https://travis-ci.org/carllerche/nix-rust)
    -   [zargony/rust-fuse](https://github.com/zargony/rust-fuse) <span> | ★ 126, pushed 22 days ago | </span> — [FUSE](https://github.com/libfuse/libfuse) bindings ![](https://travis-ci.org/zargony/rust-fuse.svg?branch=master)
-   Windows
    -   [retep998/winapi-rs](https://github.com/retep998/winapi-rs) <span> | ★ 155, pushed 3 days ago | </span> — Windows API bindings [![](https://travis-ci.org/retep998/winapi-rs.svg?branch=master)](https://travis-ci.org/retep998/winapi-rs)

### Template engine

-   Handlebars
    -   [sunng87/handlebars-rust](https://github.com/sunng87/handlebars-rust) <span> | ★ 102, pushed 25 days ago | </span> — Handlebars template engine with inheritance, custom helper support. [![](https://travis-ci.org/sunng87/handlebars-rust.svg?branch=master)](https://travis-ci.org/sunng87/handlebars-rust)
-   HTML
    -   [lfairy/maud](https://github.com/lfairy/maud) <span> | ★ 105, pushed 29 days ago | </span> — compile-time HTML templates [![](https://travis-ci.org/lfairy/maud.svg?branch=master)](https://travis-ci.org/lfairy/maud)
    -   [Stebalien/horrorshow-rs](https://github.com/Stebalien/horrorshow-rs) <span> | ★ 35, pushed 24 days ago | </span> — compile-time HTML templates [![](https://travis-ci.org/Stebalien/horrorshow-rs.svg?branch=master)](https://travis-ci.org/Stebalien/horrorshow-rs)
-   Mustache
    -   [rustache/rustache](https://github.com/rustache/rustache) <span> | ★ 128, pushed 191 days ago | </span> — [![](https://travis-ci.org/rustache/rustache.svg?branch=master)](https://travis-ci.org/rustache/rustache)
-   [tailhook/marafet](https://github.com/tailhook/marafet) — Compiler for Jade-like template language to cito.js-based virtual dom

### Text processing

-   [BurntSushi/suffix](https://github.com/BurntSushi/suffix) <span> | ★ 31, pushed 234 days ago | </span> — Linear time suffix array construction (with Unicode support) [![](https://travis-ci.org/BurntSushi/suffix.svg?branch=master)](https://travis-ci.org/BurntSushi/suffix)
-   [BurntSushi/tabwriter](https://github.com/BurntSushi/tabwriter) <span> | ★ 26, pushed 94 days ago | </span> — Elastic tab stops (i.e., text column alignment) [![](https://travis-ci.org/BurntSushi/tabwriter.svg?branch=master)](https://travis-ci.org/BurntSushi/tabwriter)
-   [pwoolcoc/ngrams](https://github.com/pwoolcoc/ngrams) <span> | ★ 1, pushed 53 days ago | </span> — Construct [n-grams](https://en.wikipedia.org/wiki/N-gram) from arbitrary iterators [![](https://travis-ci.org/pwoolcoc/ngrams.svg?branch=master)](https://travis-ci.org/pwoolcoc/ngrams)
-   [rust-lang-nursery/regex](https://github.com/rust-lang-nursery/regex) <span> | ★ 171, pushed 4 days ago | </span> — Regular expressions (RE2 style) [![](https://travis-ci.org/rust-lang-nursery/regex.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/regex)

### Virtualization

-   [saurvs/hypervisor-rs](https://github.com/saurvs/hypervisor-rs) <span> | ★ 1, pushed 16 days ago | </span> — Hardware-accelerated virtualization on OS X

### Web programming

See also [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison) .

-   HTTP Client
    -   [carllerche/curl-rust](https://github.com/carllerche/curl-rust) <span> | ★ 190, pushed 20 days ago | </span> — [libcurl](http://curl.haxx.se/libcurl/) bindings [![](https://travis-ci.org/carllerche/curl-rust.svg?branch=master)](https://travis-ci.org/carllerche/curl-rust)
    -   [hyperium/hyper](https://github.com/hyperium/hyper) <span> | ★ 1591, pushed 0 days ago | </span> — an HTTP implementation [![](https://travis-ci.org/hyperium/hyper.svg?branch=master)](https://travis-ci.org/hyperium/hyper)
    -   [vhbit/curl-rs](https://github.com/vhbit/curl-rs) <span> | ★ 1, pushed 600 days ago | </span> — [libcurl](http://curl.haxx.se/libcurl/) bindings [![](https://travis-ci.org/vhbit/curl-rs.svg?branch=master)](https://travis-ci.org/vhbit/curl-rs)
-   HTTP Server
    -   [fengsp/pencil](https://github.com/fengsp/pencil) <span> | ★ 583, pushed 15 days ago | </span> — [![](https://travis-ci.org/fengsp/pencil.svg?branch=master)](https://travis-ci.org/fengsp/pencil)
    -   [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [![](https://travis-ci.org/hyperium/hyper.svg?branch=master)](https://travis-ci.org/hyperium/hyper)
    -   [Iron](https://github.com/iron/iron) <span> | ★ 2792, pushed 6 days ago | </span> — a middleware-based server framework [![](https://travis-ci.org/iron/iron.svg?branch=master)](https://travis-ci.org/iron/iron)
        -   [sunng87/handlebars-iron](https://github.com/sunng87/handlebars-iron) <span> | ★ 67, pushed 7 days ago | </span> — [Handlebars-rust](https://github.com/sunng87/handlebars-rust) as an Iron web framework middleware. [![](https://travis-ci.org/sunng87/handlebars-iron.svg?branch=master)](https://travis-ci.org/sunng87/handlebars-iron)
    -   [Nickel](https://github.com/nickel-org/nickel.rs/) — inspired by [Express](http://expressjs.com/) [![](https://travis-ci.org/nickel-org/nickel.rs.svg?branch=master)](https://travis-ci.org/nickel-org/nickel.rs)
    -   [Ogeon/rustful](https://github.com/Ogeon/rustful) <span> | ★ 729, pushed 6 days ago | </span> — a RESTful web framework for Rust [![](https://travis-ci.org/Ogeon/rustful.svg?branch=master)](https://travis-ci.org/Ogeon/rustful)
    -   [Rustless](https://github.com/rustless/rustless) <span> | ★ 237, pushed 16 days ago | </span> — a REST-like API micro-framework inspired by [Grape](https://github.com/ruby-grape/grape) and [Hyper](https://github.com/hyperium/hyper) [![](https://travis-ci.org/rustless/rustless.svg?branch=master)](https://travis-ci.org/rustless/rustless)
    -   [tiny-http](https://github.com/frewsxcv/tiny-http) <span> | ★ 171, pushed 140 days ago | </span> — Low level HTTP server library [![](https://travis-ci.org/frewsxcv/tiny-http.svg?branch=master)](https://travis-ci.org/frewsxcv/tiny-http)
-   [WebSocket](https://datatracker.ietf.org/doc/rfc6455/)
    -   [cyderize/rust-websocket](https://github.com/cyderize/rust-websocket) <span> | ★ 190, pushed 8 days ago | </span> — a framework for dealing with WebSocket connections (both clients and servers) [![](https://travis-ci.org/cyderize/rust-websocket.svg?branch=master)](https://travis-ci.org/cyderize/rust-websocket)
    -   [housleyjk/ws-rs](https://github.com/housleyjk/ws-rs) <span> | ★ 122, pushed 6 days ago | </span> — lightweight, event-driven WebSockets for Rust [![](https://travis-ci.org/housleyjk/ws-rs.svg?branch=stable)](https://travis-ci.org/housleyjk/ws-rs)

Resources
---------

-   Benchmarks
    -   [TeXitoi/benchmarksgame-rs](https://github.com/TeXitoi/benchmarksgame-rs) <span> | ★ 13, pushed 0 days ago | </span> — Rust implementations for the [The Computer Language Benchmarks Game](http://benchmarksgame.alioth.debian.org/) [![](https://travis-ci.org/TeXitoi/benchmarksgame-rs.svg?branch=master)](https://travis-ci.org/TeXitoi/benchmarksgame-rs)
-   Podcasts
    -   [New Rustacean](http://www.newrustacean.com) — a podcast about learning Rust
    -   [Rusty Radio](http://rustyrad.io) — covering the rust ecosystem
-   [Rust by Example](http://rustbyexample.com/)
-   [RustCamp 2015 Talks](http://confreaks.tv/events/rustcamp2015)
-   [Rust Design Patterns](https://github.com/nrc/patterns) <span> | ★ 376, pushed 105 days ago | </span>
-   [Rust Guidelines](http://aturon.github.io/)
-   [rust-learning](https://github.com/ctjhoa/rust-learning) <span> | ★ 579, pushed 1 days ago | </span> — a collection of useful resources to learn Rust
-   [Rustlings](https://github.com/carols10cents/rustlings) <span> | ★ 1139, pushed 0 days ago | </span> — small exercises to get you used to reading and writing Rust code

License
-------

[![CC0](http://i.creativecommons.org/p/zero/1.0/88x31.png)](http://creativecommons.org/publicdomain/zero/1.0/)
