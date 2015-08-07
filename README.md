# Awesome Rust

A curated list of awesome Rust code and resources, inspired by the other [awesome lists](https://github.com/bayandin/awesome-awesomeness).

The goal is to have only projects that are mostly stable and useful to users.

- [Awesome Rust](#awesome-rust)
  - [Applications written in Rust](#applications-written-in-rust)
    - [Games](#games)
  - [Libraries](#libraries)
    - [Audio](#audio)
    - [Build system](#build-system)
    - [Command-line argument parsing](#command-line-argument-parsing)
    - [Compression](#compression)
    - [Computation](#computation)
    - [Cryptography](#cryptography)
    - [Database](#database)
    - [Data structures](#data-structures)
    - [Date and time](#date-and-time)
    - [Encoding](#encoding)
    - [Filesystem](#filesystem)
    - [Game development](#game-development)
    - [Games](#games)
    - [GUI](#gui)
    - [Image processing](#image-processing)
    - [Markup language](#markup-language)
    - [Mobile](#mobile)
    - [Network programming](#network-programming)
    - [Parser](#parser)
    - [Platform specific](#platform-specific)
    - [Template engine](#template-engine)
    - [Testing](#testing)
    - [Text processing](#text-processing)
    - [Web programming](#web-programming)
  - [Resources](#resources)
  - [Development Tools](#development-tools)
    - [IDEs](#ides)
  - [License](#license)

## Applications written in Rust

* [BurntSushi/xsv](https://github.com/BurntSushi/xsv) — A fast CSV command line tool (slicing, indexing, selecting, searching, sampling, etc.) [<img src="https://travis-ci.org/BurntSushi/xsv.svg?branch=master">](https://travis-ci.org/BurntSushi/xsv)
* [buster/rrun](https://github.com/buster/rrun) — A command launcher for Linux, similar to gmrun  [<img src="https://travis-ci.org/buster/rrun.svg?branch=master">](https://travis-ci.org/buster/rrun)
* [gchp/iota](https://github.com/gchp/iota) — a simple text editor written in Rust [<img src="https://travis-ci.org/gchp/iota.svg?branch=master">](https://travis-ci.org/gchp/iota)
* [imjacobclark/Herd](https://github.com/imjacobclark/Herd) — an experimental HTTP load testing application written in Rust
* [kbknapp/docli](https://github.com/kbknapp/docli-rs) — a command line utility for managing DigitalOcean infrastructure [<img src="https://travis-ci.org/kbknapp/docli-rs.svg?branch=master">](https://travis-ci.org/kbknapp/docli-rs)
* [MaidSafe](http://maidsafe.net/) — a decentralized platform.
* [seppo0010/rsedis](https://github.com/seppo0010/rsedis) — Redis reimplementation in Rust. [<img src="https://travis-ci.org/seppo0010/rsedis.svg?branch=master">](https://travis-ci.org/seppo0010/rsedis)
* [Servo](https://github.com/servo/servo) — a prototype web browser engine written in Rust
* [thoughtram/clog](https://github.com/thoughtram/clog) — a [conventional changelog](http://blog.thoughtram.io/announcements/tools/2014/09/18/announcing-clog-a-conventional-changelog-generator-for-the-rest-of-us.html) port (generates a changelog from git metadata) [<img src="https://travis-ci.org/thoughtram/clog.svg?branch=master">](https://travis-ci.org/thoughtram/clog)
* [uutils/coreutils](https://github.com/uutils/coreutils) — cross-platform Rust rewrite of the GNU coreutils [<img src="https://travis-ci.org/uutils/coreutils.svg?branch=master">](https://travis-ci.org/uutils/coreutils)

### Games

* [Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston)
* [lifthrasiir/angolmois-rust](https://github.com/lifthrasiir/angolmois-rust) — a minimalistic music video game which supports the BMS format [<img src="https://travis-ci.org/lifthrasiir/angolmois-rust.svg?branch=master">](https://travis-ci.org/lifthrasiir/angolmois-rust)


## Libraries

### Audio

* [GuillaumeGomez/rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) — [FMOD](http://www.fmod.org/) bindings [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-fmod.png?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-fmod)
* [JeremyLetang/ears](https://github.com/JeremyLetang/ears) — a simple library to play Sounds and Musics, on top of OpenAL and libsndfile [<img src="https://travis-ci.org/jeremyletang/ears.svg?branch=master">](https://travis-ci.org/jeremyletang/ears)
* [JeremyLetang/rust-portaudio](https://github.com/JeremyLetang/rust-portaudio) — [PortAudio](http://www.portaudio.com/) bindings [<img src="https://travis-ci.org/jeremyletang/rust-portaudio.svg?branch=master">](https://travis-ci.org/jeremyletang/rust-portaudio)
* [jpernst/openal-rs](https://github.com/jpernst/openal-rs) — [OpenAL 1.1](http://www.openal.org/) bindings [<img src="https://travis-ci.org/jpernst/openal-rs.svg?branch=master">](https://travis-ci.org/jpernst/openal-rs)
* [RustAudio](https://github.com/RustAudio)
* [samdoshi/portmidi-rs](https://github.com/samdoshi/portmidi-rs) — [PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings [<img src="https://travis-ci.org/samdoshi/portmidi-rs.svg?branch=master">](https://travis-ci.org/samdoshi/portmidi-rs)

### Build system

* [Cargo](http://crates.io/) — the Rust package manager
* CMake
  * [SiegeLord/RustCMake](https://github.com/SiegeLord/RustCMake) — an example project showing usage of CMake with Rust [<img src="https://travis-ci.org/SiegeLord/RustCMake.svg?branch=master">](https://travis-ci.org/SiegeLord/RustCMake)

### Command-line argument parsing

* [docopt/docopt.rs](https://github.com/docopt/docopt.rs) — a Rust implementation of [DocOpt](http://docopt.org) [<img src="https://travis-ci.org/docopt/docopt.rs.svg?branch=master">](https://travis-ci.org/docopt/docopt.rs)
* [kbknapp/clap-rs](https://github.com/kbknapp/clap-rs) — a simple to use, full featured command-line argument parser [<img src="https://travis-ci.org/kbknapp/clap-rs.svg?branch=master">](https://travis-ci.org/kbknapp/clap-rs)

### Compression

* [alexcrichton/bzip2-rs](https://github.com/alexcrichton/bzip2-rs) — [libbz2](http://www.bzip.org) bindings [<img src="https://travis-ci.org/alexcrichton/bzip2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/bzip2-rs)
* [alexcrichton/flate2-rs](https://github.com/alexcrichton/flate2-rs) — [miniz](https://code.google.com/p/miniz/) bindings [<img src="https://travis-ci.org/alexcrichton/flate2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/flate2-rs)
* [alexcrichton/tar-rs](https://github.com/alexcrichton/tar-rs) — tar archive reading/writing in Rust [<img src="https://travis-ci.org/alexcrichton/tar-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/tar-rs)
* [slackito/zip](https://github.com/slackito/zip) — read and write ZIP archives [<img src="https://travis-ci.org/slackito/zip.svg?branch=master">](https://travis-ci.org/slackito/zip)

### Computation

* BLAS
  * [mikkyang/rust-blas](https://github.com/mikkyang/rust-blas) — [BLAS](http://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) bindings
  * [stainless-steel/blas](https://github.com/stainless-steel/blas) — [BLAS](http://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) bindings [<img src="https://travis-ci.org/stainless-steel/blas.svg?branch=master">](https://travis-ci.org/stainless-steel/blas)
* GMP
  * [thestinger/rust-gmp](https://github.com/thestinger/rust-gmp) — [libgmp](https://gmplib.org/) bindings [<img src="https://travis-ci.org/thestinger/rust-gmp.svg?branch=master">](https://travis-ci.org/thestinger/rust-gmp)
* GSL
  * [GuillaumeGomez/rust-GSL](https://github.com/GuillaumeGomez) — [GSL](http://www.gnu.org/software/gsl/) bindings [<img src="https://travis-ci.org/GuillaumeGomez/rust-GSL.svg?branch=master">](https://travis-ci.org/GuillaumeGomez/rust-GSL)
* LAPACK
  * [stainless-steel/lapack](https://github.com/stainless-steel/lapack) — [LAPACK](http://en.wikipedia.org/wiki/LAPACK) bindings [<img src="https://travis-ci.org/stainless-steel/lapack.svg?branch=master">](https://travis-ci.org/stainless-steel/lapack)
* OpenCL
  * [luqmana/rust-opencl](https://github.com/luqmana/rust-opencl) — [OpenCL](https://www.khronos.org/opencl/) bindings [<img src="https://travis-ci.org/luqmana/rust-opencl.svg?branch=master">](https://travis-ci.org/luqmana/rust-opencl)

### Cryptography

* [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) — cryptographic algorithms in Rust [<img src="https://travis-ci.org/DaGenix/rust-crypto.svg?branch=master">](https://travis-ci.org/DaGenix/rust-crypto)
* [dnaq/sodiumoxide](https://github.com/dnaq/sodiumoxide) — [libsodium](https://github.com/jedisct1/libsodium) bindings [<img src="https://travis-ci.org/dnaq/sodiumoxide.svg?branch=master">](https://travis-ci.org/dnaq/sodiumoxide)
* [klutzy/suruga](https://github.com/klutzy/suruga) — a Rust implementation of [TLS 1.2](http://tools.ietf.org/html/rfc5246)
* [seb-m/common.rs](https://github.com/seb-m/common.rs) — Common Rust crypto utilities [<img src="https://travis-ci.org/seb-m/common.rs.svg?branch=master">](https://travis-ci.org/seb-m/common.rs)
* [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) — [OpenSSL](https://www.openssl.org/) bindings [<img src="https://travis-ci.org/sfackler/rust-openssl.svg?branch=master">](https://travis-ci.org/sfackler/rust-openssl)


### Database

* NoSQL
  * Cassandra
    * [tupshin/cassandra-rust](https://github.com/tupshin/cassandra-rust) — [Cassandra](http://cassandra.apache.org) bindings
  * Elasticsearch
    * [benashford/rs-es] (https://github.com/benashford/rs-es) — A Rust client for the [Elastic] (https://www.elastic.co/) REST API [<img src="https://travis-ci.org/benashford/rs-es.svg?branch=master">](https://travis-ci.org/benashford/rs-es)
  * ForestDB
    * [vhbit/sherwood](https://github.com/vhbit/sherwood) — [ForestDB](https://github.com/couchbase/forestdb) bindings [<img src="https://travis-ci.org/vhbit/sherwood.svg?branch=master">](https://travis-ci.org/vhbit/sherwood)
  * LMDB
    * [vhbit/lmdb-rs](https://github.com/vhbit/lmdb-rs) — [LMDB](http://symas.com/mdb/) bindings [<img src="https://travis-ci.org/vhbit/lmdb-rs.svg?branch=master">](https://travis-ci.org/vhbit/lmdb-rs)
  * Redis
    * [mitsuhiko/redis-rs](https://github.com/mitsuhiko/redis-rs) — [Redis](http://redis.io) library in Rust [<img src="https://travis-ci.org/mitsuhiko/redis-rs.svg?branch=master">](https://travis-ci.org/mitsuhiko/redis-rs)
* SQL
  * MySql
    * [blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) — a native MySql client [<img src="https://travis-ci.org/blackbeam/rust-mysql-simple.svg?branch=master">](https://travis-ci.org/blackbeam/rust-mysql-simple)
  * PostgreSql
    * [sfackler/rust-postgres](https://github.com/sfackler/rust-postgres) — a native [PostgreSQL](http://www.postgresql.org) client [<img src="https://travis-ci.org/sfackler/rust-postgres.svg?branch=master">](https://travis-ci.org/sfackler/rust-postgres)
  * Sqlite
    * [dckc/rust-sqlite3](https://github.com/dckc/rust-sqlite3) — [Sqlite3](http://www.sqlite.org/) bindings [<img src="https://travis-ci.org/dckc/rust-sqlite3.svg?branch=master">](https://travis-ci.org/dckc/rust-sqlite3)
    * [jgallagher/rusqlite](https://github.com/jgallagher/rusqlite) — [Sqlite3](http://www.sqlite.org/) bindings [<img src="https://travis-ci.org/jgallagher/rusqlite.svg?branch=master">](https://travis-ci.org/jgallagher/rusqlite)
    * [linuxfood/rustsqlite](https://github.com/linuxfood/rustsqlite) — [Sqlite3](http://www.sqlite.org/) bindings [<img src="https://travis-ci.org/linuxfood/rustsqlite.svg?branch=master">](https://travis-ci.org/linuxfood/rustsqlite)

### Date and time

* [lifthrasiir/rust-chrono](https://github.com/lifthrasiir/rust-chrono) — [<img src="https://travis-ci.org/lifthrasiir/rust-chrono.svg?branch=master">](https://travis-ci.org/lifthrasiir/rust-chrono)
* [rust-lang/time](https://github.com/rust-lang/time) — [<img src="https://travis-ci.org/rust-lang/time.svg?branch=master">](https://travis-ci.org/rust-lang/time)

### Data structures

* [bluss/rust-itertools](https://github.com/bluss/rust-itertools) — [<img src="https://travis-ci.org/bluss/rust-itertools.svg?branch=master">](https://travis-ci.org/bluss/rust-itertools)
* [reem/rust-typemap](https://github.com/reem/rust-typemap) — [<img src="https://travis-ci.org/reem/rust-typemap.svg?branch=master">](https://travis-ci.org/reem/rust-typemap)
* [serde-rs/serde](https://github.com/serde-rs/serde) — a framework to generically serialize Rust data structures [<img src="https://api.travis-ci.org/serde-rs/serde.svg?branch=master">](https://travis-ci.org/serde-rs/serde)

### Encoding

* ASN.1
  * [alex/rust-asn1](https://github.com/alex/rust-asn1) — a Rust ASN.1 (DER) serializer [<img src="https://travis-ci.org/alex/rust-asn1.svg?branch=master">](https://travis-ci.org/alex/rust-asn1)
* Bencode
  * [arjantop/rust-bencode](https://github.com/arjantop/rust-bencode) — [Bencode](http://en.wikipedia.org/wiki/Bencode) implementation in Rust [<img src="https://travis-ci.org/arjantop/rust-bencode.svg?branch=master">](https://travis-ci.org/arjantop/rust-bencode)
* Binary
  * [arcnmx/nue](https://github.com/arcnmx/nue) — I/O and binary data encoding for Rust [<img src="https://travis-ci.org/arcnmx/nue.svg?branch=master">](https://travis-ci.org/arcnmx/nue)
  * [TyOverby/bincode](https://github.com/TyOverby/bincode) — a binary encoder/decoder in Rust [<img src="https://travis-ci.org/TyOverby/bincode.svg?branch=master">](https://travis-ci.org/TyOverby/bincode)
* Byte swapping
  * [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder) — Supports big-endian, little-endian and native byte orders [<img src="https://travis-ci.org/BurntSushi/byteorder.svg?branch=master">](https://travis-ci.org/BurntSushi/byteorder)
* Cap'n Proto
  * [dwrensha/capnproto-rust](https://github.com/dwrensha/capnproto-rust) — [<img src="https://travis-ci.org/dwrensha/capnproto-rust.svg?branch=master">](https://travis-ci.org/dwrensha/capnproto-rust)
* CBOR
  * [BurntSushi/rust-cbor](https://github.com/BurntSushi/rust-cbor) — Supports JSON conversion and type-based encoding/decoding [<img src="https://travis-ci.org/BurntSushi/rust-cbor.svg?branch=master">](https://travis-ci.org/BurntSushi/rust-cbor)
* Character Encoding
  * [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding) — [<img src="https://travis-ci.org/lifthrasiir/rust-encoding.svg?branch=master">](https://travis-ci.org/lifthrasiir/rust-encoding)
* CSV
  * [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) — [<img src="https://travis-ci.org/BurntSushi/rust-csv.svg?branch=master">](https://travis-ci.org/BurntSushi/rust-csv)
* HTML
  * [servo/html5ever](https://github.com/servo/html5ever) — High-performance browser-grade HTML5 parser [<img src="https://travis-ci.org/servo/html5ever.svg?branch=master">](https://travis-ci.org/servo/html5ever)
* MsgPack
  * [mneumann/rust-msgpack](https://github.com/mneumann/rust-msgpack) — [<img src="https://travis-ci.org/mneumann/rust-msgpack.svg?branch=master">](https://travis-ci.org/mneumann/rust-msgpack)
  * [3Hren/msgpack-rust](https://github.com/3Hren/msgpack-rust) — a pure Rust low/high level MessagePack implementation [<img src="https://travis-ci.org/3Hren/msgpack-rust.svg?branch=master">](https://travis-ci.org/3Hren/msgpack-rust)
* ProtocolBuffers
  * [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf) — [<img src="https://travis-ci.org/stepancheg/rust-protobuf.svg?branch=master">](https://travis-ci.org/stepancheg/rust-protobuf)
* Tnetstring
  * [erickt/rust-tnetstring](https://github.com/erickt/rust-tnetstring) — [<img src="https://travis-ci.org/erickt/rust-tnetstring.svg?branch=master">](https://travis-ci.org/erickt/rust-tnetstring)
* TOML
  * [alexcrichton/toml-rs](https://github.com/alexcrichton/toml-rs) — [<img src="https://travis-ci.org/alexcrichton/toml-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/toml-rs)
* XML
  * [Florob/RustyXML](https://github.com/Florob/RustyXML) — an XML parser written in Rust [<img src="https://travis-ci.org/Florob/RustyXML.svg?branch=master">](https://travis-ci.org/Florob/RustyXML)
  * [netvl/xml-rs](https://github.com/netvl/xml-rs) — a streaming XML library [<img src="https://travis-ci.org/netvl/xml-rs.svg?branch=master">](https://travis-ci.org/netvl/xml-rs)
* YAML
  * [chyh1990/yaml-rust](https://github.com/chyh1990/yaml-rust) — The missing YAML 1.2 implementation for Rust. [<img src="https://travis-ci.org/chyh1990/yaml-rust.svg?branch=master">](https://travis-ci.org/chyh1990/yaml-rust)
  * [kimhyunkang/libyaml-rust](https://github.com/kimhyunkang/libyaml-rust) — [libyaml](http://pyyaml.org/wiki/LibYAML) bindings [<img src="https://travis-ci.org/kimhyunkang/libyaml-rust.svg?branch=master">](https://travis-ci.org/kimhyunkang/libyaml-rust)

### Filesystem

* FUSE
  * [zargony/rust-fuse](https://github.com/zargony/rust-fuse) — [FUSE](http://fuse.sourceforge.net/) bindings <img src="https://travis-ci.org/zargony/rust-fuse.svg?branch=master">

### Game development

* [bbodi/rust-voxlap](https://github.com/bbodi/rust-voxlap) — [Voxlap](http://advsys.net/ken/voxlap.htm) bindings
* [PistonDevelopers/piston](https://github.com/PistonDevelopers/piston) — [<img src="https://travis-ci.org/PistonDevelopers/piston.svg?branch=master">](https://travis-ci.org/PistonDevelopers/piston)
* [SiegeLord/RustAllegro](https://github.com/SiegeLord/RustAllegro) — [Allegro 5](http://liballeg.org/) bindings [<img src="https://travis-ci.org/SiegeLord/RustAllegro.svg?branch=master">](https://travis-ci.org/SiegeLord/RustAllegro)

### GUI

* Cocoa
  * [servo/rust-cocoa](https://github.com/servo/cocoa-rs)
* IUP
  * [dcampbell24/iup-rust](https://github.com/dcampbell24/iup-rust) — [IUP](http://webserver2.tecgraf.puc-rio.br/iup/) bindings [<img src="https://travis-ci.org/dcampbell24/iup-rust.svg?branch=master">](https://travis-ci.org/dcampbell24/iup-rust)
* GTK+
  * [rust-gnome/gtk](https://github.com/rust-gnome/gtk) — [GTK+](http://www.gtk.org) bindings [<img src="https://travis-ci.org/rust-gnome/gtk.svg?branch=master">](https://travis-ci.org/rust-gnome/gtk)
* ncurses
  * [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) — [ncurses](http://www.gnu.org/software/ncurses/) bindings [<img src="https://travis-ci.org/jeaye/ncurses-rs.svg?branch=master">](https://travis-ci.org/jeaye/ncurses-rs)
* OpenGL
  * [bjz/gl-rs](https://github.com/bjz/gl-rs) — [<img src="https://travis-ci.org/bjz/gl-rs.svg?branch=master">](https://travis-ci.org/bjz/gl-rs)
  * [bjz/glfw-rs](https://github.com/bjz/glfw-rs) — [<img src="https://travis-ci.org/bjz/glfw-rs.svg?branch=master">](https://travis-ci.org/bjz/glfw-rs)
  * [servo/rust-glut](https://github.com/servo/rust-glut)
  * [tomaka/glutin](https://github.com/tomaka/glutin) — Rust alternative to [GLFW](http://www.glfw.org/) [<img src="https://travis-ci.org/tomaka/glutin.svg?branch=master">](https://travis-ci.org/tomaka/glutin)
* Qt
  * [cyndis/qmlrs](https://github.com/cyndis/qmlrs) — [QtQuick](http://doc.qt.io) bindings [<img src="https://travis-ci.org/cyndis/qmlrs.svg?branch=master">](https://travis-ci.org/cyndis/qmlrs)
* SDL
  * [AngryLawyer/rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) — [SDL2](http://www.libsdl.org/) bindings [<img src="https://travis-ci.org/AngryLawyer/rust-sdl2.svg?branch=master">](https://travis-ci.org/AngryLawyer/rust-sdl2)
  * [brson/rust-sdl](https://github.com/brson/rust-sdl) — [SDL1](http://www.libsdl.org/) bindings [<img src="https://travis-ci.org/brson/rust-sdl.svg?branch=master">](https://travis-ci.org/brson/rust-sdl)
  * [PistonDevelopers/conrod](https://github.com/PistonDevelopers/conrod/) — An easy-to-use, immediate-mode, 2D GUI library written entirely in Rust [<img src="https://travis-ci.org/PistonDevelopers/conrod.svg?branch=master">](https://travis-ci.org/PistonDevelopers/conrod)
* SFML
  * [jeremyletang/rust-sfml](https://github.com/jeremyletang/rust-sfml) — [SFML](http://www.sfml-dev.org/) bindings [<img src="https://travis-ci.org/jeremyletang/rust-sfml.svg?branch=master">](https://travis-ci.org/jeremyletang/rust-sfml)
* Termbox
  * [gchp/rustbox](https://github.com/gchp/rustbox) — a Rust implementation of [termbox](http://github.com/nsf/termbox) [<img src="https://travis-ci.org/gchp/rustbox.svg?branch=master">](https://travis-ci.org/gchp/rustbox)


### Image processing

* [chyh1990/imageproc](https://github.com/chyh1990/imageproc) — An advanced image processing library for Rust. [![Build Status](https://travis-ci.org/chyh1990/imageproc.svg?branch=master)](https://travis-ci.org/chyh1990/imageproc)
* [cybergeek94/img-hash](https://github.com/cybergeek94/img-hash) — Perceptual image hashing and comparison for equality and similarity. [![Build Status](https://travis-ci.org/cybergeek94/img-hash.svg?branch=master)](https://travis-ci.org/cybergeek94/img-hash)
* [PistonDevelopers/image](https://github.com/PistonDevelopers/image) — Basic imaging processing functions and methods for converting to and from image formats [<img src="https://travis-ci.org/PistonDevelopers/image.svg?branch=master">](https://travis-ci.org/PistonDevelopers/image)


### Markup language

* CommonMark
  * [google/pulldown-cmark](https://github.com/google/pulldown-cmark) — [CommonMark](http://commonmark.org/) parser in Rust

### Mobile

* [tomaka/android-rs-glue](https://github.com/tomaka/android-rs-glue) — glue between Rust and Android [<img src="https://travis-ci.org/tomaka/android-rs-glue.svg?branch=master">](https://travis-ci.org/tomaka/android-rs-glue)
* [vhbit/ObjCrust](https://github.com/vhbit/ObjCrust) — using Rust to create an iOS static library [<img src="https://travis-ci.org/vhbit/ObjCrust.svg?branch=master">](https://travis-ci.org/vhbit/ObjCrust)

### Network programming

* Beanstalkd
  * [schickling/rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) — [Beanstalkd](https://github.com/kr/beanstalkd) bindings [<img src="https://travis-ci.org/schickling/rust-beanstalkd.svg?branch=master">](https://travis-ci.org/schickling/rust-beanstalkd)
* FTP
  * [mattnenterprise/rust-ftp](https://github.com/mattnenterprise/rust-ftp) — An [FTP](http://en.wikipedia.org/wiki/File_Transfer_Protocol) client for Rust [<img src="https://travis-ci.org/mattnenterprise/rust-ftp.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-ftp)
* Low level
  * [libpnet/libpnet](https://github.com/libpnet/libpnet) — Cross-platform, low level networking [<img src="https://travis-ci.org/libpnet/libpnet.svg?branch=master">](https://travis-ci.org/libpnet/libpnet)
* NanoMsg
  * [thehydroimpulse/nanomsg.rs](https://github.com/thehydroimpulse/nanomsg.rs) — a modern messaging library that is the successor to ZeroMQ [<img src="https://travis-ci.org/thehydroimpulse/nanomsg.rs.svg?branch=master">](https://travis-ci.org/thehydroimpulse/nanomsg.rs)
* NNTP
  * [mattnenterprise/rust-nntp](https://github.com/mattnenterprise/rust-nntp) — A [NNTP](http://en.wikipedia.org/wiki/Network_News_Transfer_Protocol) client for Rust [<img src="https://travis-ci.org/mattnenterprise/rust-nntp.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-nntp)
* POP3
  * [mattnenterprise/rust-pop3](https://github.com/mattnenterprise/rust-pop3) — A [POP3](http://en.wikipedia.org/wiki/Post_Office_Protocol) client for Rust [<img src="https://travis-ci.org/mattnenterprise/rust-pop3.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-pop3)
* SSH
  * [alexcrichton/ssh2-rs](https://github.com/alexcrichton/ssh2-rs) — [libssh2](http://www.libssh2.org/) bindings [<img src="https://travis-ci.org/alexcrichton/ssh2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/ssh2-rs)
* Stomp
  * [zslayton/stomp-rs](https://github.com/zslayton/stomp-rs) — [STOMP 1.2](http://stomp.github.io/stomp-specification-1.2.html) client implementation in Rust [<img src="https://travis-ci.org/zslayton/stomp-rs.svg?branch=master">](https://travis-ci.org/zslayton/stomp-rs)
* uTP
  * [meqif/rust-utp](https://github.com/meqif/rust-utp) — A [uTP](http://www.bittorrent.org/beps/bep_0029.html) (Micro Transport Protocol) library for Rust. [<img src="https://travis-ci.org/meqif/rust-utp.svg?branch=master">](https://travis-ci.org/meqif/rust-utp)
* ZeroMQ
  * [erickt/rust-zmq](https://github.com/erickt/rust-zmq) — [ZeroMQ](http://zeromq.org/) bindings [<img src="https://travis-ci.org/erickt/rust-zmq.svg?branch=master">](https://travis-ci.org/erickt/rust-zmq)


### Parser

  * [Geal/nom](https://github.com/Geal/nom) — parser combinator library [<img src="https://travis-ci.org/Geal/nom.svg?branch=master">](https://travis-ci.org/Geal/nom)
  * [kevinmehall/rust-peg](https://github.com/kevinmehall/rust-peg) — Parsing Expression Grammar (PEG) parser generator
  * [Marwes/combine](https://github.com/Marwes/combine) — parser combinator library [<img src="https://travis-ci.org/Marwes/combine.svg?branch=master">](https://travis-ci.org/Marwes/combine)


### Platform specific

* DigitalOcean
  * [kbknapp/doapi](https://github.com/kbknapp/doapi-rs) — DigitalOcean v2 API bindings [<img src="https://travis-ci.org/kbknapp/doapi-rs.svg?branch=master">](https://travis-ci.org/kbknapp/doapi-rs)
* Linux
  * [carllerche/nix-rust](https://github.com/carllerche/nix-rust) — Linux API bindings [<img src="https://travis-ci.org/carllerche/nix-rust.svg?branch=master">](https://travis-ci.org/carllerche/nix-rust)
* Windows
  * [retep998/winapi-rs](https://github.com/retep998/winapi-rs) — Windows API bindings [<img src="https://travis-ci.org/retep998/winapi-rs.svg?branch=master">](https://travis-ci.org/retep998/winapi-rs)

### Template engine

* HTML
  * [lfairy/maud](https://github.com/lfairy/maud) — compile-time HTML templates [<img src="https://travis-ci.org/lfairy/maud.svg?branch=master">](https://travis-ci.org/lfairy/maud)
  * [Stebalien/horrorshow-rs](https://github.com/Stebalien/horrorshow-rs) — compile-time HTML templates [<img src="https://travis-ci.org/Stebalien/horrorshow-rs.svg?branch=master">](https://travis-ci.org/Stebalien/horrorshow-rs)
* Mustache
  * [rustache/rustache](https://github.com/rustache/rustache) — [<img src="https://travis-ci.org/rustache/rustache.svg?branch=master">](https://travis-ci.org/rustache/rustache)
* [sunng87/handlebars-iron](https://github.com/sunng87/handlebars-iron) — [Handlebars-rust](https://github.com/sunng87/handlebars-rust) as an Iron web framework middleware. [<img src="https://travis-ci.org/sunng87/handlebars-iron.svg?branch=master">](https://travis-ci.org/sunng87/handlebars-iron)
* [sunng87/handlebars-rust](https://github.com/sunng87/handlebars-rust) — Handlebars template engine with inheritance, custom helper support. [<img src="https://travis-ci.org/sunng87/handlebars-rust.svg?branch=master">](https://travis-ci.org/sunng87/handlebars-rust)

### Testing

* [BurntSushi/quickcheck](https://github.com/BurntSushi/quickcheck) — a Rust implementation of [QuickCheck](http://www.haskell.org/haskellwiki/Introduction_to_QuickCheck1) [<img src="https://travis-ci.org/BurntSushi/quickcheck.svg?branch=master">](https://travis-ci.org/BurntSushi/quickcheck)
* [farcaller/shiny](https://github.com/farcaller/shiny) — a fancy syntax similar to Ruby's Rspec or Objective-C' kiwi [<img src="https://travis-ci.org/farcaller/shiny.svg?branch=master">](https://travis-ci.org/farcaller/shiny)

### Text processing

* [BurntSushi/suffix](https://github.com/BurntSushi/suffix) — Linear time suffix array construction (with Unicode support) [<img src="https://travis-ci.org/BurntSushi/suffix.svg?branch=master">](https://travis-ci.org/BurntSushi/suffix)
* [BurntSushi/tabwriter](https://github.com/BurntSushi/suffix) — Elastic tab stops (i.e., text column alignment) [<img src="https://travis-ci.org/BurntSushi/tabwriter.svg?branch=master">](https://travis-ci.org/BurntSushi/tabwriter)
* [rust-lang/regex](https://github.com/rust-lang/regex) — Regular expressions (RE2 style) [<img src="https://travis-ci.org/rust-lang/regex.svg?branch=master">](https://travis-ci.org/rust-lang/regex)

### Web programming

See also [http://arewewebyet.com/](http://arewewebyet.com/)

* Client
  * [carllerche/curl-rust](https://github.com/carllerche/curl-rust) — [libcurl](http://curl.haxx.se/libcurl/) bindings [<img src="https://travis-ci.org/carllerche/curl-rust.svg?branch=master">](https://travis-ci.org/carllerche/curl-rust)
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [<img src="https://travis-ci.org/hyperium/hyper.svg?branch=master">](https://travis-ci.org/hyperium/hyper)
  * [vhbit/curl-rs](https://github.com/vhbit/curl-rs) — [libcurl](http://curl.haxx.se/libcurl/) bindings [<img src="https://travis-ci.org/vhbit/curl-rs.svg?branch=master">](https://travis-ci.org/vhbit/curl-rs)
* Server
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [<img src="https://travis-ci.org/hyperium/hyper.svg?branch=master">](https://travis-ci.org/hyperium/hyper)
  * [Iron](http://ironframework.io/) — a middleware-based server framework [<img src="https://travis-ci.org/iron/iron.svg?branch=master">](https://travis-ci.org/iron/iron)
  * [Nickel](http://nickel.rs/) — inspired by [Express](http://expressjs.com/) [<img src="https://travis-ci.org/nickel-org/nickel.rs.svg?branch=master">](https://travis-ci.org/nickel-org/nickel.rs)
  * [Ogeon/rustful](https://github.com/Ogeon/rustful) — a RESTful web framework for Rust  [<img src="https://travis-ci.org/Ogeon/rustful.svg?branch=master">](https://travis-ci.org/Ogeon/rustful)
  * [Rustless](http://rustless.org/) — a REST-like API micro-framework inspired by [Grape](https://github.com/intridea/grape) and [Hyper](https://github.com/hyperium/hyper) [<img src="https://travis-ci.org/rustless/rustless.svg?branch=master">](https://travis-ci.org/rustless/rustless)
  * [tiny-http](https://github.com/frewsxcv/tiny-http) — Low level HTTP server library [<img src="https://travis-ci.org/frewsxcv/tiny-http.svg?branch=master">](https://travis-ci.org/frewsxcv/tiny-http)

## Resources

* [Rust by Example](http://rustbyexample.com/)
* [Rust CI](http://www.rust-ci.org/) — a [Travis CI](https://travis-ci.com) dashboard for Rust projects
* [Rust Guidelines](http://aturon.github.io/)
* [rust-learning](https://github.com/ctjhoa/rust-learning) — a collection of useful resources to learn Rust


## Development tools

* [brson/multirust](https://github.com/brson/multirust) — manage multiple Rust installations [<img src="https://travis-ci.org/brson/multirust.svg?branch=master">](https://travis-ci.org/brson/multirust)
* [crabtw/rust-bindgen](https://github.com/crabtw/rust-bindgen) — a Rust bindings generator [<img src="https://travis-ci.org/crabtw/rust-bindgen.svg?branch=master">](https://travis-ci.org/crabtw/rust-bindgen)
* [dan-t/rusty-tags](https://github.com/dan-t/rusty-tags) — create ctags/etags for a cargo project and all of its dependencies [<img src="https://travis-ci.org/dan-t/rusty-tags.svg?branch=master">](https://travis-ci.org/dan-t/rusty-tags)
* [ellisonch/rust-stopwatch](https://github.com/ellisonch/rust-stopwatch) — a stopwatch library for Rust. Used to time things. [<img src="https://travis-ci.org/ellisonch/rust-stopwatch.svg?branch=master">](https://travis-ci.org/ellisonch/rust-stopwatch)
* [frewsxcv/crate-deps](https://github.com/frewsxcv/crate-deps) — generates images of dependency graphs for crates hosted on crates.io
* [kmcallister/afl.rs](https://github.com/kmcallister/afl.rs) — a Rust fuzzer, using [AFL](http://lcamtuf.coredump.cx/afl/) [<img src="https://travis-ci.org/kmcallister/afl.rs.svg?branch=master">](https://travis-ci.org/kmcallister/afl.rs)
* [Manishearth/rust-clippy](https://github.com/Manishearth/rust-clippy) — a collection of lints that catch newbie mistakes and oversights during the build [<img src="https://travis-ci.org/Manishearth/rust-clippy.svg?branch=master">](https://travis-ci.org/Manishearth/rust-clippy)
* [nrc/rustfmt](https://github.com/nrc/rustfmt) — a Rust code formatter [<img src="https://travis-ci.org/nrc/rustfmt.svg?branch=master">](https://travis-ci.org/nrc/rustfmt)
* [phildawes/racer](https://github.com/phildawes/racer) — code completion for Rust [<img src="https://travis-ci.org/phildawes/racer.svg?branch=master">](https://travis-ci.org/phildawes/racer)

### IDEs

  * [PistonDevelopers/VisualRust](https://github.com/PistonDevelopers/VisualRust) — a Visual Studio extension for Rust [<img src="https://travis-ci.org/PistonDevelopers/VisualRust.svg?branch=master">](https://travis-ci.org/PistonDevelopers/VisualRust)
  * [Ride](https://github.com/madeso/ride) — [<img src="https://travis-ci.org/madeso/ride.svg?branch=master">](https://travis-ci.org/madeso/ride)
  * [RustDT](https://github.com/RustDT/RustDT) — an [Eclipse](https://eclipse.org/)-based IDE for Rust [<img src="https://travis-ci.org/RustDT/RustDT.svg?branch=master">](https://travis-ci.org/RustDT/RustDT)
  * [SolidOak](https://github.com/oakes/SolidOak) — a simple IDE for Rust, based on GTK+ and [Neovim](https://github.com/neovim/neovim) [<img src="https://travis-ci.org/oakes/SolidOak.svg?branch=master">](https://travis-ci.org/oakes/SolidOak)
  * [Vektah/idea-rust](https://github.com/Vektah/idea-rust) — an [IntelliJ](https://www.jetbrains.com/idea/)-based IDE for Rust [<img src="https://travis-ci.org/Vektah/idea-rust.svg?branch=master">](https://travis-ci.org/Vektah/idea-rust)
  * [Visual Studio Code](https://code.visualstudio.com/)


## License

[![CC0](http://i.creativecommons.org/p/zero/1.0/88x31.png)](http://creativecommons.org/publicdomain/zero/1.0/)
