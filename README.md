# Awesome Rust

A curated list of awesome Rust code and resources. Inspired by the other [awesome lists](https://github.com/bayandin/awesome-awesomeness).

Only projects that are more than a prototype should be added. Projects that do not compile with Rust-nightly for a long time have to be removed.

- [Awesome Rust](#awesome-rust)
  - [Code](#code)
    - [Audio](#audio)
    - [Compression](#compression)
    - [Computation](#computation)
    - [Cryptography](#cryptography)
    - [Encoding](#encoding)
    - [GUI](#gui)
    - [Web Programming](#web-programming)
  - [Resources](#resources)


## Code

### Audio

* [bjz/openal-rs](https://github.com/bjz/openal-rs/) — [OpenAL 1.1](http://www.openal.org/) bindings
* [JeremyLetang/ears](https://github.com/JeremyLetang/ears) — a simple library to play Sounds and Musics, on top of OpenAL and libsndfile [<img src="https://travis-ci.org/jeremyletang/ears.png?branch=master">](https://travis-ci.org/JeremyLetang/ears)
* [JeremyLetang/rust-portaudio](https://github.com/JeremyLetang/rust-portaudio) — [PortAudio](http://www.portaudio.com/) bindings [<img src="https://travis-ci.org/jeremyletang/rust-portaudio.png?branch=master">](https://travis-ci.org/JeremyLetang/rust-portaudio)
* [musitdev/rust-portmidi](https://github.com/musitdev/rust-portmidi) — [PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings [<img src="https://travis-ci.org/musitdev/rust-portmidi.png?branch=master">](https://travis-ci.org/musitdev/rust-portmidi)

### Compression

* [lifthrasiir/rust-zip](https://github.com/lifthrasiir/rust-zip) — read and write ZIP archives [<img src="https://travis-ci.org/lifthrasiir/rust-zip.png?branch=master">](https://travis-ci.org/lifthrasiir/rust-zip)

### Computation
* [eholk/rust-opencl](https://github.com/eholk/rust-opencl) — [OpenCL](https://www.khronos.org/opencl/) bindings [<img src="https://travis-ci.org/eholk/rust-opencl.png?branch=master">](https://travis-ci.org/eholk/rust-opencl)

### Cryptography

* [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) — cryptographic algorithms in Rust [<img src="https://travis-ci.org/DaGenix/rust-crypto.png?branch=master">](https://travis-ci.org/DaGenix/rust-crypto)
* [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) — OpenSSL bindings [<img src="https://travis-ci.org/sfackler/rust-openssl.png?branch=master">](https://travis-ci.org/sfackler/rust-openssl)

### Encoding

* Cap'n Proto
    * [dwrensha/capnproto-rust](https://github.com/dwrensha/capnproto-rust) — [<img src="https://travis-ci.org/dwrensha/capnproto-rust.png?branch=master">](https://travis-ci.org/dwrensha/capnproto-rust)
* Character Encoding
    * [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding) — [<img src="https://travis-ci.org/lifthrasiir/rust-encoding.png?branch=rust-0.9-pre">](https://travis-ci.org/lifthrasiir/rust-encoding)
* CSV
  * [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) — [<img src="https://api.travis-ci.org/BurntSushi/rust-csv.png">](https://travis-ci.org/BurntSushi/rust-csv)
  * [Geal/rust-csv](https://github.com/Geal/rust-csv) — [<img src="https://travis-ci.org/Geal/rust-csv.png?branch=master">](https://travis-ci.org/Geal/rust-csv)
* MsgPck
  * [mneumann/rust-msgpack](https://github.com/mneumann/rust-msgpack) — [<img src="https://travis-ci.org/mneumann/rust-msgpack.png?branch=master">](https://travis-ci.org/mneumann/rust-msgpack)
* ProtocolBuffers
  * [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf) — [<img src="https://travis-ci.org/stepancheg/rust-protobuf.png?branch=master">](https://travis-ci.org/stepancheg/rust-protobuf)
  * [tildeio/buffoon](https://github.com/tildeio/buffoon)
* TOML
  * [alexcrichton/toml-rs](https://github.com/alexcrichton/toml-rs) — 
* Tnetstring
  * [erickt/rust-tnetstring](https://github.com/erickt/rust-tnetstring) — [<img src="https://travis-ci.org/erickt/rust-tnetstring.png?branch=master">](https://travis-ci.org/erickt/rust-tnetstring)
* XML
   * [bjz/sax-rs](https://github.com/bjz/sax-rs) — bindings to libxml2's SAX parser [<img src="https://travis-ci.org/bjz/sax-rs.png?branch=master">](https://travis-ci.org/bjz/sax-rs)
   * [DanielFath/xml-parser](https://github.com/DanielFath/xml-parser) — A hybrid pull, DOM parser written in pure Rust [![Build Status](https://travis-ci.org/DanielFath/xml-parser.png?branch=master)](https://travis-ci.org/DanielFath/xml-parser)
   * [Florob/RustyXML](https://github.com/Florob/RustyXML) — an XML parser written in Rust
   * [netvl/rust-xml](https://github.com/netvl/rust-xml) — a streaming XML library

### GUI

* Cocoa
  * [mozilla-servo/rust-cocoa](https://github.com/mozilla-servo/rust-cocoa) — 
* Gtk
  * [JeremyLetang/rgtk](https://github.com/JeremyLetang/rgtk) — [<img src="https://travis-ci.org/jeremyletang/rgtk.png?branch=master">](https://travis-ci.org/jeremyletang/rgtk)
* XML
* ncurses
  * [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) — [![Build Status](https://travis-ci.org/jeaye/ncurses-rs.png?branch=master)](https://travis-ci.org/jeaye/ncurses-rs.png)
* Termbox
  * [gchp/rustbox](https://github.com/gchp/rustbox) — 

### Web Programming

See also http://arewewebyet.com/

  * core
    * [chris-morgan/rust-http](https://github.com/chris-morgan/rust-http) — will be replaced by [Teepee](http://teepee.rs/). [<img src="https://travis-ci.org/chris-morgan/rust-http.png?branch=master">](https://travis-ci.org/chris-morgan/rust-http)
  * client
    * [carllerche/curl-rust](https://github.com/carllerche/curl-rust) — [libcurl](http://curl.haxx.se/libcurl/) bindings
  * server
    * [erickt/rust-mongrel2](https://github.com/erickt/rust-mongrel2) — [Mongrel2](http://mongrel2.org) bindings [<img src="https://travis-ci.org/erickt/rust-mongrel2.png?branch=master">](https://travis-ci.org/erickt/rust-mongrel2)
    * [Iron](http://ironframework.io/) — inspired by [Express](http://expressjs.com/). [<img src="https://travis-ci.org/iron/iron.png?branch=master">](https://travis-ci.org/iron/iron)
    * [Nickel](http://nickel.rs/) — inspired by [Express](http://expressjs.com/).

## Resources
  * [Rust by Example](http://rustbyexample.com/)
