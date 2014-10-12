# Awesome Rust

A curated list of awesome Rust code and resources. Inspired by the other [awesome lists](https://github.com/bayandin/awesome-awesomeness).

Only projects that are stable and useful to users are added. Projects that do not compile with Rust-nightly for a longer time are moved to `UNSTABLE.md`.

- [Awesome Rust](#awesome-rust)
  - [Code](#code)
    - [Audio](#audio)
    - [Build system](#build-system)
    - [Command-line argument parsing](#command-line-argument-parsing)
    - [Compression](#compression)
    - [Computation](#computation)
    - [Cryptography](#cryptography)
    - [Database](#database)
    - [Encoding](#encoding)
    - [Game development](#game-development)
    - [Games](#games)
    - [GUI](#gui)
    - [Image processing](#image-processing)
    - [Network programming](#network-programming)
    - [Template engine](#template-engine)
    - [Testing](#testing)
    - [Web programming](#web-programming)
  - [Resources](#resources)

## Code


### Audio

* [bjz/openal-rs](https://github.com/bjz/openal-rs/) — [OpenAL 1.1](http://www.openal.org/) bindings
* [JeremyLetang/ears](https://github.com/JeremyLetang/ears) — a simple library to play Sounds and Musics, on top of OpenAL and libsndfile [<img src="https://travis-ci.org/jeremyletang/ears.svg?branch=master">](https://travis-ci.org/JeremyLetang/ears)
* [JeremyLetang/rust-portaudio](https://github.com/JeremyLetang/rust-portaudio) — [PortAudio](http://www.portaudio.com/) bindings [<img src="https://travis-ci.org/jeremyletang/rust-portaudio.svg?branch=master">](https://travis-ci.org/JeremyLetang/rust-portaudio)
* [musitdev/rust-portmidi](https://github.com/musitdev/rust-portmidi) — [PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings [<img src="https://travis-ci.org/musitdev/rust-portmidi.svg?branch=master">](https://travis-ci.org/musitdev/rust-portmidi)

### Build system

* [Cargo](http://crates.io) — the Rust package manager
* CMake
  * [SiegeLord/RustCMake](https://github.com/SiegeLord/RustCMake) — an example project showing usage of CMake with Rust [<img src="https://travis-ci.org/SiegeLord/RustCMake.svg?branch=master">](https://travis-ci.org/SiegeLord/RustCMake)
* Make
  * [PistonDevelopers/rust-empty](https://github.com/PistonDevelopers/rust-empty) — a Makefile to get started with Rust

### Command-line argument parsing

* [docopt/docopt.rs](https://github.com/docopt/docopt.rs) — a Rust implementation of [DocOpt](http://docopt.org) [<img src="https://travis-ci.org/docopt/docopt.rs.svg?branch=master">](https://travis-ci.org/docopt/docopt.rs)

### Compression

* [lifthrasiir/rust-zip](https://github.com/lifthrasiir/rust-zip) — read and write ZIP archives [<img src="https://travis-ci.org/lifthrasiir/rust-zip.svg?branch=master">](https://travis-ci.org/lifthrasiir/rust-zip)

### Computation

* [eholk/rust-opencl](https://github.com/eholk/rust-opencl) — [OpenCL](https://www.khronos.org/opencl/) bindings [<img src="https://travis-ci.org/eholk/rust-opencl.svg?branch=master">](https://travis-ci.org/eholk/rust-opencl)

### Cryptography

* [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) — cryptographic algorithms in Rust [<img src="https://travis-ci.org/DaGenix/rust-crypto.svg?branch=master">](https://travis-ci.org/DaGenix/rust-crypto)
* [dnaq/sodiumoxide](https://github.com/dnaq/sodiumoxide) — [libsodium](https://github.com/jedisct1/libsodium) bindings
* [klutzy/suruga](https://github.com/klutzy/suruga) — a Rust implementation of [TLS 1.2](http://tools.ietf.org/html/rfc5246)
* [seb-m/common.rs](https://github.com/klutzy/suruga) — Common Rust crypto utilities
* [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) — OpenSSL bindings [<img src="https://travis-ci.org/sfackler/rust-openssl.svg?branch=master">](https://travis-ci.org/sfackler/rust-openssl)

### Database

* SQL
  * MySql
    * [blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) — a native MySql client [<img src="https://travis-ci.org/blackbeam/rust-mysql-simple.svg?branch=master">](https://travis-ci.org/blackbeam/rust-mysql-simple)
  * PostgreSql
    * [sfackler/rust-postgres](https://github.com/sfackler/rust-postgres) — a native [PostgreSQL](http://www.postgresql.org) client [<img src="https://travis-ci.org/sfackler/rust-postgres.svg?branch=master">](https://travis-ci.org/sfackler/rust-postgres)
  * Sqlite
    * [linuxfood/rustsqlite](https://github.com/linuxfood/rustsqlite) — [Sqlite3](http://www.sqlite.org/) bindings

### Encoding

* Cap'n Proto
  * [dwrensha/capnproto-rust](https://github.com/dwrensha/capnproto-rust) — [<img src="https://travis-ci.org/dwrensha/capnproto-rust.svg?branch=master">](https://travis-ci.org/dwrensha/capnproto-rust)
* Character Encoding
  * [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding) — [<img src="https://travis-ci.org/lifthrasiir/rust-encoding.svg?branch=master">](https://travis-ci.org/lifthrasiir/rust-encoding)
* CSV
  * [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) — [<img src="https://api.travis-ci.org/BurntSushi/rust-csv.svg?branch=master">](https://travis-ci.org/BurntSushi/rust-csv)
* MsgPck
  * [mneumann/rust-msgpack](https://github.com/mneumann/rust-msgpack) — [<img src="https://travis-ci.org/mneumann/rust-msgpack.svg?branch=master">](https://travis-ci.org/mneumann/rust-msgpack)
* ProtocolBuffers
  * [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf) — [<img src="https://travis-ci.org/stepancheg/rust-protobuf.svg?branch=master">](https://travis-ci.org/stepancheg/rust-protobuf)
* TOML
  * [alexcrichton/toml-rs](https://github.com/alexcrichton/toml-rs)
* Tnetstring
* XML
  * [Florob/RustyXML](https://github.com/Florob/RustyXML) — an XML parser written in Rust [<img src="https://travis-ci.org/Florob/RustyXML.svg?branch=master">](https://travis-ci.org/Florob/RustyXM)
  * [netvl/rust-xml](https://github.com/netvl/rust-xml) — a streaming XML library [<img src="https://travis-ci.org/netvl/rust-xml.svg?branch=master">](https://travis-ci.org/netvl/rust-xml)
  * [Ygg01/xml-air](https://github.com/Ygg01/xml-air) — A hybrid pull, DOM parser written in pure Rust [<img src="https://travis-ci.org/Ygg01/xml-air.svg?branch=master">](https://travis-ci.org/Ygg01/xml-air)

### Game development

* [PistonDevelopers/piston](https://github.com/pistondevelopers/piston) — [<img src="https://travis-ci.org/PistonDevelopers/piston.svg?branch=master">](https://travis-ci.org/PistonDevelopers/piston)

### Games

* [lifthrasiir/angolmois-rust](https://github.com/lifthrasiir/angolmois-rust) — a minimalistic music video game which supports the BMS format [<img src="https://travis-ci.org/lifthrasiir/angolmois-rust.svg?branch=master">](https://travis-ci.org/lifthrasiir/angolmois-rust)

### GUI

* Cocoa
  * [mozilla-servo/rust-cocoa](https://github.com/mozilla-servo/rust-cocoa)
* Gtk+
  * [JeremyLetang/rgtk](https://github.com/JeremyLetang/rgtk) — [Gtk+](http://www.gtk.org) bindings [<img src="https://travis-ci.org/jeremyletang/rgtk.svg?branch=master">](https://travis-ci.org/jeremyletang/rgtk)
* ncurses
  * [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) — [<img src="https://travis-ci.org/jeaye/ncurses-rs.svg?branch=master">](https://travis-ci.org/jeaye/ncurses-rs)
* OpenGL
* SDL
  * [AngryLawyer/rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) — [SDL2](http://www.libsdl.org/) bindings [<img src="https://travis-ci.org/AngryLawyer/rust-sdl2.svg?branch=master">](https://travis-ci.org/AngryLawyer/rust-sdl2)
  * [brson/rust-sdl](https://github.com/brson/rust-sdl) — [SDL1](http://www.libsdl.org/) bindings [<img src="https://travis-ci.org/brson/rust-sdl.svg?branch=master">](https://travis-ci.org/brson/rust-sdl)
* SFML
  * [jeremyletang/rust-sfml](https://github.com/JeremyLetang/rust-sfml) — [SFML](http://www.sfml-dev.org/) bindings [<img src="https://travis-ci.org/jeremyletang/rust-sfml.svg?branch=master">](https://travis-ci.org/jeremyLetang/rust-sfml)
* Termbox
  * [gchp/rustbox](https://github.com/gchp/rustbox) — a Rust implementation of [termbox](http://github.com/nsf/termbox)
* wxWidgets

### Image processing

* [PistonDevelopers/image](https://github.com/PistonDevelopers/image) — Basic imaging processing functions and methods for converting to and from image formats [<img src="https://travis-ci.org/PistonDevelopers/image.svg?branch=master">](https://travis-ci.org/PistonDevelopers/image)

### Network programming

* ZeroMQ
  * [erickt/rust-zmq](https://github.com/erickt/rust-zmq) — [ZeroMQ](http://zeromq.org) bindings [<img src="https://travis-ci.org/erickt/rust-zmq.svg?branch=master">](https://travis-ci.org/erickt/rust-zmq)

### Template engine

* Mustache
  * [rustache/rustache](https://github.com/rustache/rustache) — [<img src="https://travis-ci.org/rustache/rustache.svg?branch=master">](https://travis-ci.org/rustache/rustache)

### Testing

* [BurntSushi/quickcheck](https://github.com/BurntSushi/quickcheck) — a Rust implementation of [QuickCheck](http://www.haskell.org/haskellwiki/Introduction_to_QuickCheck1) [<img src="https://travis-ci.org/BurntSushi/quickcheck.svg?branch=master">](https://travis-ci.org/BurntSushi/quickcheck)
* [farcaller/shiny](https://github.com/farcaller/shiny) — a fancy syntax similar to ruby's rspec or Objective-C's kiwi [<img src="https://travis-ci.org/farcaller/shiny.svg?branch=master">](https://travis-ci.org/farcaller/shiny)

### Web programming

See also [http://arewewebyet.com/](http://arewewebyet.com/)

* Core
  * [chris-morgan/rust-http](https://github.com/chris-morgan/rust-http) — will be replaced by [Teepee](http://teepee.rs/) [<img src="https://travis-ci.org/chris-morgan/rust-http.svg?branch=master">](https://travis-ci.org/chris-morgan/rust-http)
  * [hyperium/hyper](https://github.com/hyperium/hyper) — [<img src="https://travis-ci.org/hyperium/hyper.svg?branch=master">](https://travis-ci.org/hyperium/hyper)
* Client
  * [carllerche/curl-rust](https://github.com/carllerche/curl-rust) — [libcurl](http://curl.haxx.se/libcurl/) bindings
  * [vhbit/curl-rs](https://github.com/vhbit/curl-rs) — [libcurl](http://curl.haxx.se/libcurl/) bindings
* Server
  * [erickt/rust-mongrel2](https://github.com/erickt/rust-mongrel2) — [Mongrel2](http://mongrel2.org) bindings [<img src="https://travis-ci.org/erickt/rust-mongrel2.svg?branch=master">](https://travis-ci.org/erickt/rust-mongrel2)
  * [Iron](http://ironframework.io/) — inspired by [Express](http://expressjs.com/) [<img src="https://travis-ci.org/iron/iron.svg?branch=master">](https://travis-ci.org/iron/iron)
  * [Nickel](http://nickel.rs/) — inspired by [Express](http://expressjs.com/) [<img src="https://travis-ci.org/nickel-org/nickel.rs.svg?branch=master">](https://travis-ci.org/nickel-org/nickel.rs)

## Resources

* [Rust by Example](http://rustbyexample.com/)
* [Rust CI](http://www.rust-ci.org) — a [Travis CI](https://travis-ci.com) dashboard for Rust projects
* [Rust Guidelines](http://aturon.github.io)