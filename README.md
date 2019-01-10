# Awesome Rust [<img src="https://api.travis-ci.org/rust-unofficial/awesome-rust.svg?branch=master">](https://travis-ci.org/rust-unofficial/awesome-rust)

A curated list of Rust code and resources.

If you want to contribute, please read [this](CONTRIBUTING.md).

## Table of contents

- [Applications](#applications)
  - [Audio](#audio)
  - [Cryptocurrencies](#cryptocurrencies)
  - [Database](#database)
  - [Emulators](#emulators)
  - [Games](#games)
  - [Graphics](#graphics)
  - [Industrial automation](#industrial-automation)
  - [Operating systems](#operating-systems)
  - [Productivity](#productivity)
  - [Security tools](#security-tools)
  - [System tools](#system-tools)
  - [Text editors](#text-editors)
  - [Text processing](#text-processing)
  - [Utilities](#utilities)
  - [Video](#video)
  - [Virtualization](#virtualization)
  - [Window managers](#window-managers)
  - [Web](#web)
  - [Web Servers](#web-servers)
- [Development tools](#development-tools)
  - [Build system](#build-system)
  - [Debugging](#debugging)
  - [Deployment](#deployment)
  - [Embedded](#embedded)
  - [FFI](#ffi)
  - [IDEs](#ides)
  - [Pattern recognition](#pattern-recognition)
  - [Profiling](#profiling)
  - [Static analysis](#static-analysis)
  - [Testing](#testing)
  - [Transpiling](#transpiling)
- [Libraries](#libraries)
  - [Artificial Intelligence](#artificial-intelligence)
      - [Genetic algorithms](#genetic-algorithms)
      - [Machine learning](#machine-learning)
  - [Astronomy](#astronomy)
  - [Asynchronous](#asynchronous)
  - [Audio](#audio-1)
  - [Authentication](#authentication)
  - [Automotive](#automotive)
  - [Bioinformatics](#bioinformatics)
  - [Caching](#caching)
  - [Cloud](#cloud)
  - [Command-line](#command-line)
  - [Compression](#compression)
  - [Computation](#computation)
  - [Concurrency](#concurrency)
  - [Configuration](#configuration)
  - [Cryptography](#cryptography)
  - [Database](#database-1)
  - [Data processing](#data-processing)
  - [Data structures](#data-structures)
  - [Data visualization](#data-visualization)
  - [Date and time](#date-and-time)
  - [Distributed systems](#distributed-systems)
  - [Email](#email)
  - [Encoding](#encoding)
  - [Filesystem](#filesystem)
  - [Game development](#game-development)
  - [Geospatial](#geospatial)
  - [Graphics](#graphics-1)
  - [Graph processing](#graph-processing)
  - [GUI](#gui)
  - [Image processing](#image-processing)
  - [Industrial automation](#industrial-automation)
  - [Language specification](#language-specification)
  - [Logging](#logging)
  - [Macro](#macro)
  - [Markup language](#markup-language)
  - [Mobile](#mobile)
  - [Network programming](#network-programming)
  - [Parser](#parser)
  - [Platform specific](#platform-specific)
  - [Scripting](#scripting)
  - [Template engine](#template-engine)
  - [Text processing](#text-processing-1)
  - [Text search](#text-search)
  - [Virtualization](#virtualization-1)
  - [Web programming](#web-programming)
  - [Peripherals](#peripherals)
- [Resources](#resources)
- [License](#license)

## Applications

See also [Rust — Production](https://www.rust-lang.org/production)(organizations running Rust in production).

* [andschwa/rust-genetic-algorithm](https://github.com/andschwa/rust-genetic-algorithm) — a genetic algorithm for academic benchmark problems [<img src="https://api.travis-ci.org/andschwa/rust-genetic-algorithm.svg?branch=master">](https://travis-ci.org/andschwa/rust-genetic-algorithm)
* [azerupi/mdBook](https://github.com/rust-lang-nursery/mdBook) — a command line utility to create books from markdown files [<img src="https://api.travis-ci.org/azerupi/mdBook.svg?branch=master">](https://travis-ci.org/azerupi/mdBook)
* [bluejekyll/trust-dns](https://github.com/bluejekyll/trust-dns) — a DNS-server [<img src="https://api.travis-ci.org/bluejekyll/trust-dns.svg?branch=master">](https://travis-ci.org/bluejekyll/trust-dns)
* [ivanceras/diwata](https://github.com/ivanceras/diwata) — a database administration tool for postgresql [<img src="https://api.travis-ci.org/ivanceras/diwata.svg">](https://travis-ci.org/ivanceras/diwata)
* [linkerd/linkerd2-proxy](https://github.com/linkerd/linkerd2-proxy) — Ultralight service mesh for Kubernetes.
* [danielrangel/rsign](https://bitbucket.org/danielrangel/rsign) [[rsign](https://crates.io/crates/rsign)] — A simple command-line tool used to generate/sign/verify digital signatures designed to be compatible with Minisign  [![Codeship Status for danielrangel/rsign](https://app.codeship.com/projects/60b28d80-7645-0135-4402-1639b58199d0/status?branch=master)](https://app.codeship.com/projects/244452)
* [darrint/device-blocker](https://github.com/darrint/device-blocker) — Limit screen time to children's various mobile devices by blocking internet access on the family Wifi router.
* [dlecan/generic-dns-update](https://github.com/dlecan/generic-dns-update) — a tool to update DNS zonefiles with your IP address [<img src="https://api.travis-ci.org/dlecan/generic-dns-update.svg?branch=master">](https://travis-ci.org/dlecan/generic-dns-update)
* [Factotum](https://github.com/snowplow/factotum) — [A system to programmatically run data pipelines](https://snowplowanalytics.com/blog/2016/04/09/introducing-factotum-data-pipeline-runner/) [<img src="https://api.travis-ci.org/snowplow/factotum.svg?branch=master">](https://travis-ci.org/snowplow/factotum)
* [fcsonline/drill](https://github.com/fcsonline/drill) — a HTTP load testing application inspired by Ansible syntax [<img src="https://api.travis-ci.org/fcsonline/drill.svg?branch=master">](https://travis-ci.org/fcsonline/drill)
* [Fractalide](https://github.com/fractalide/fractalide) — Simple Rust Microservices
* [habitat](https://www.habitat.sh) — An tool created by [Chef](https://www.chef.io/) to build, deploy, and manage applications.
* [imjacobclark/Herd](https://github.com/imjacobclark/Herd) — an experimental HTTP load testing application
* [intecture/api](https://github.com/intecture/api) — an API-driven server management and configuration tool [<img src="https://api.travis-ci.org/intecture/api.svg?branch=master">](https://travis-ci.org/intecture/api)
* [jedisct1/flowgger](https://github.com/jedisct1/flowgger) — a fast, simple and lightweight data collector
* [jwilm/alacritty](https://github.com/jwilm/alacritty) — a cross-platform, GPU enhanced terminal emulator
* [kbknapp/docli](https://github.com/kbknapp/docli-rs) — a command line utility for managing DigitalOcean infrastructure [<img src="https://api.travis-ci.org/kbknapp/docli-rs.svg?branch=master">](https://travis-ci.org/kbknapp/docli-rs)
* [MaidSafe](https://maidsafe.net) — a decentralized platform.
* [nicohman/eidolon](https://github.com/nicohman/eidolon) — a steam and drm-free game registry and launcher for linux and macosx [<img src="https://api.travis-ci.org/nicohman/eidolon.svg?branch=master">](https://travis-ci.org/nicohman/eidolon)
* [notty](https://github.com/withoutboats/notty) — A new kind of terminal
* [Pijul](https://pijul.org) — a patch-based distributed version control system
* [qmx/limonite](https://github.com/qmx/limonite) — static blog/website generator [<img src="https://api.travis-ci.org/qmx/limonite.svg?branch=master">](https://travis-ci.org/qmx/limonite)
* [Sandstorm Collections App](https://github.com/sandstorm-io/collections-app)
* [Servo](https://github.com/servo/servo) — a prototype web browser engine
* [Weld](https://github.com/serayuzgur/weld) — Full fake REST API generator [<img src="https://api.travis-ci.org/serayuzgur/weld.svg">](https://travis-ci.org/serayuzgur/weld)

### Audio

* [indiscipline/zrtstr](https://github.com/indiscipline/zrtstr) — a command line utility for checking if stereo wav files are faux-stereo (i.e. have identical channels) and converting such files to mono. [<img src="https://api.travis-ci.org/indiscipline/zrtstr.svg?branch=master">](https://travis-ci.org/indiscipline/zrtstr)

### Cryptocurrencies

* [nearprotocol/nearcore](https://github.com/nearprotocol/nearcore) — decentralized smart-contract platform for low-end mobile devices.
* [ethaddrgen](https://github.com/Limeth/ethaddrgen) — Custom Ethereum vanity address generator made in Rust [<img src="https://api.travis-ci.org/Limeth/ethaddrgen.svg?branch=master">](https://travis-ci.org/Limeth/ethaddrgen)
* [coinbase-pro-rs](https://github.com/inv2004/coinbase-pro-rs) — Coinbase pro client in Rust, supports sync/async/websocket [<img src="https://api.travis-ci.org/inv2004/coinbase-pro-rs.svg?branch=master">](https://travis-ci.org/inv2004/coinbase-pro-rs)
* [Grin](https://github.com/mimblewimble/grin/) — Evolution of the MimbleWimble protocol
* [polkadot](https://github.com/paritytech/polkadot) — Heterogeneous multi‑chain technology with pooled security
* [parity-ethereum](https://github.com/paritytech/parity-ethereum) — Fast, light, and robust Ethereum client
* [parity-bitcoin](https://github.com/paritytech/parity-bitcoin) — The Parity Bitcoin client [<img src="https://api.travis-ci.org/paritytech/parity-bitcoin.svg?branch=master">](https://travis-ci.com/paritytech/parity-bitcoin)
* [parity-bridge](https://github.com/paritytech/parity-bridge) — Bridge between any two ethereum-based networks
* [ArgusObserver/wagu](https://github.com/ArgusObserver/wagu) [[wagu](https://crates.io/crates/wagu)] — Generate a wallet for any cryptocurrency [<img src="https://api.travis-ci.com/ArgusObserver/wagu.svg?branch=master">](https://api.travis-ci.com/ArgusObserver/wagu.svg?branch=master)
* [rust-cardano](https://github.com/input-output-hk/rust-cardano) — Rust implementation of Cardano primitives, helpers, and related applications
* [cardano-cli](https://github.com/input-output-hk/cardano-cli) — Cardano Command Line Interface (CLI)
* [Nervos CKB](https://github.com/nervosnetwork/ckb) - Nervos CKB is a public permissionless blockchain, the common knowledge layer of Nervos network.
* [ChainX](https://github.com/chainx-org/ChainX) - Fully Decentralized Interchain Crypto Asset Management on Polkadot.

### Database

* [indradb/indradb](https://github.com/indradb/indradb) — Rust based graph database [<img src="https://api.travis-ci.org/indradb/indradb.svg?branch=master">](https://travis-ci.org/indradb/indradb)
* [Mentat](https://mozilla.github.io/mentat/) — Project Mentat is a persistent, embedded knowledge base. It draws heavily on DataScript and Datomic. Mentat is intended to be a flexible relational (not key-value, not document-oriented) store that makes it easy to describe, grow, and reuse your domain schema.
* [PumpkinDB](https://github.com/PumpkinDB/PumpkinDB) — an event sourcing database engine [<img src="https://api.travis-ci.org/PumpkinDB/PumpkinDB.svg?branch=master">](https://travis-ci.org/PumpkinDB/PumpkinDB)
* [seppo0010/rsedis](https://github.com/seppo0010/rsedis) — a Redis reimplementation in Rust [<img src="https://api.travis-ci.org/seppo0010/rsedis.svg?branch=master">](https://travis-ci.org/seppo0010/rsedis)
* [ParityDB](https://github.com/paritytech/paritydb) — Fast and reliable database, optimised for read operation
* [tikv](https://github.com/tikv/tikv) — a distributed KV database in Rust [<img src="https://circleci.com/gh/tikv/tikv.svg?style=shield&circle-token=36bab0a8e43edb0941b31c38557d2d9d0d58f708">](https://circleci.com/gh/tikv/tikv)

### Emulators

See also [crates matching keyword 'emulator'](https://crates.io/keywords/emulator).

* Commodore 64
  * [kondrak/rust64](https://github.com/kondrak/rust64) — [<img src="https://api.travis-ci.org/kondrak/rust64.svg?branch=master">](https://travis-ci.org/kondrak/rust64)
* Gameboy
  * [Gekkio/mooneye-gb](https://github.com/Gekkio/mooneye-gb) — [<img src="https://api.travis-ci.org/Gekkio/mooneye-gb.svg?branch=master">](https://travis-ci.org/Gekkio/mooneye-gb)
  * [mvdnes/rboy](https://github.com/mvdnes/rboy) — [<img src="https://api.travis-ci.org/mvdnes/rboy.svg?branch=master">](https://travis-ci.org/mvdnes/rboy)
  * [NivenT/RGB](https://github.com/nivent/RGB) — [<img src="https://api.travis-ci.org/NivenT/RGB.svg?branch=master">](https://travis-ci.org/NivenT/RGB)
* NES
  * [iamsix/oxidenes](https://github.com/iamsix/oxidenes) — [<img src="https://api.travis-ci.org/iamsix/oxidenes.svg?branch=master">](https://travis-ci.org/iamsix/oxidenes)
  * [koute/pinky](https://github.com/koute/pinky) — [<img src="https://api.travis-ci.org/koute/pinky.svg?branch=master">](https://travis-ci.org/koute/pinky)
  * [pcwalton/sprocketnes](https://github.com/pcwalton/sprocketnes) — [<img src="https://api.travis-ci.org/pcwalton/sprocketnes.svg?branch=master">](https://travis-ci.org/pcwalton/sprocketnes)
* Playstation
  * [simias/rustation](https://github.com/simias/rustation) — [<img src="https://api.travis-ci.org/simias/rustation.svg?branch=master">](https://travis-ci.org/simias/rustation)
* ZX Spectrum
  * [pacmancoder/rustzx](https://github.com/pacmancoder/rustzx) — [<img src="https://api.travis-ci.org/pacmancoder/rustzx.svg?branch=master">](https://travis-ci.org/pacmancoder/rustzx)
* Virtual Boy
  * [emu-rs/rustual-boy](https://github.com/emu-rs/rustual-boy) — [<img src="https://api.travis-ci.org/emu-rs/rustual-boy.svg?branch=master">](https://travis-ci.org/emu-rs/rustual-boy)
* Emulator Development tools
  * SNES
    * [ioncodes/snesutilities](https://github.com/ioncodes/snesutilities) — ROM analyser/extractor

### Games

See also [Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston).

* [lifthrasiir/angolmois-rust](https://github.com/lifthrasiir/angolmois-rust) — a minimalistic music video game which supports the BMS format [<img src="https://api.travis-ci.org/lifthrasiir/angolmois-rust.svg?branch=master">](https://travis-ci.org/lifthrasiir/angolmois-rust)
* [swatteau/sokoban-rs](https://github.com/swatteau/sokoban-rs) — a Sokoban implementation
* [Zone of Control](https://github.com/ozkriff/zoc) — a turn-based hexagonal strategy game [<img src="https://api.travis-ci.org/ozkriff/zoc.svg?branch=master">](https://travis-ci.org/ozkriff/zoc)
* [rhex](https://github.com/dpc/rhex) — hexagonal ascii roguelike
* [citybound](https://github.com/citybound/citybound) — The city sim you deserve
* [ozkriff/zemeroth](https://github.com/ozkriff/zemeroth) — a small 2D turn-based hexagonal strategy game [<img src="https://api.travis-ci.org/ozkriff/zemeroth.svg?branch=master">](https://travis-ci.org/ozkriff/zemeroth)
* [rsaarelm/magog](https://github.com/rsaarelm/magog) — A roguelike game in Rust
* [cristicbz/rust-doom](https://github.com/cristicbz/rust-doom) — A renderer for Doom, may progress to being a playable game [<img src="https://api.travis-ci.org/cristicbz/rust-doom.svg?branch=master">](https://travis-ci.org/cristicbz/rust-doom)
* [Thinkofname/rust-quake](https://github.com/Thinkofname/rust-quake) — Quake map renderer in Rust
* [aleshaleksey/TGWM](https://github.com/aleshaleksey/TGWM) — An RPG with turned-based mechanics (work in progress) [<img src="https://api.travis-ci.org/aleshaleksey/TGWM.svg?branch=master">](https://travis-ci.org/aleshaleksey/TGWM)
* [garkimasera/rusted-ruins](https://github.com/garkimasera/rusted-ruins) — Extensible open world rogue like game with pixel art [<img src="https://api.travis-ci.org/garkimasera/rusted-ruins.svg?branch=master">](https://travis-ci.org/garkimasera/rusted-ruins)

### Graphics

* [Limeth/euclider](https://github.com/Limeth/euclider) — a real-time 4D CPU ray tracer [<img src="https://api.travis-ci.org/Limeth/euclider.svg?branch=master">](https://travis-ci.org/Limeth/euclider)
* [ivanceras/svgbob](https://github.com/ivanceras/svgbob) — converts ASCII diagrams into SVG graphics [<img src="https://api.travis-ci.org/ivanceras/svgbob.svg">](https://travis-ci.org/ivanceras/svgbob)
* [RazrFalcon/svgcleaner](https://github.com/RazrFalcon/svgcleaner) — tidies SVG graphics
* [Twinklebear/tray_rust](https://github.com/Twinklebear/tray_rust) — a ray tracer [<img src="https://api.travis-ci.org/Twinklebear/tray_rust.svg">](https://travis-ci.org/Twinklebear/tray_rust)
* Image processing
  * [spejss/Image-Processing-CLI-in-Rust](https://github.com/spejss/Image-Processing-CLI-in-Rust) — CLI for processing images, generating histograms. [![Build Status](https://api.travis-ci.org/spejss/Image-Processing-CLI-in-Rust.svg?branch=master)](https://travis-ci.org/spejss/Image-Processing-CLI-in-Rust)

### Industrial automation

* [slowtec/tokio-modbus](https://github.com/slowtec/tokio-modbus) — A [tokio](https://tokio.rs)-based [modbus](http://modbus.org) library. [![Build Status](https://api.travis-ci.org/slowtec/tokio-modbus.svg?branch=master)](https://travis-ci.org/slowtec/tokio-modbus)
* [locka99/opcua](https://github.com/locka99/opcua) — A pure rust [OPC UA](https://opcfoundation.org/about/opc-technologies/opc-ua/) library.
* [BiancoRoyal/modbus-iiot-rust](https://github.com/BiancoRoyal/modbus-iiot-rust) — A pure rust [modbus](http://modbus.org) library with no or less dependencies. [![Build Status](https://api.travis-ci.org/BiancoRoyal/modbus-iiot-rust.svg?branch=master)](https://travis-ci.org/BiancoRoyal/modbus-iiot-rust)

### Operating systems

See also [A comparison of operating systems written in Rust](https://github.com/flosse/rust-os-comparison).

* [redox-os/redox](https://gitlab.redox-os.org/redox-os/redox) — [<img   src="https://api.travis-ci.org/redox-os/redox.svg?branch=master">](https://travis-ci.org/redox-os/redox)
* [thepowersgang/rust_os](https://github.com/thepowersgang/rust_os) — [<img src="https://api.travis-ci.org/thepowersgang/rust_os.svg?branch=master">](https://travis-ci.org/thepowersgang/rust_os)
* [tock/tock](https://github.com/tock/tock) — A secure embedded operating system for Cortex-M based microcontrollers
* [nebulet/nebulet](https://github.com/nebulet/nebulet) — A microkernel that implements a WebAssembly "usermode" that runs in Ring 0.

### Productivity

* [simeg/eureka](https://github.com/simeg/eureka) [[eureka](https://crates.io/crates/eureka)] — a CLI tool to input and store your ideas without leaving the terminal

### Security tools

* [arvancloud/libinjection-rs](https://github.com/arvancloud/libinjection-rs) — Rust bindings for [libinjection](https://github.com/client9/libinjection) [<img src="https://api.travis-ci.org/arvancloud/libinjection-rs.svg?branch=master">](https://travis-ci.org/arvancloud/libinjection-rs)
* [kpcyrd/badtouch](https://github.com/kpcyrd/badtouch) — Scriptable network authentication cracker [<img src="https://api.travis-ci.org/kpcyrd/badtouch.svg?branch=master">](https://travis-ci.org/kpcyrd/badtouch)
* [kpcyrd/rshijack](https://github.com/kpcyrd/rshijack) — tcp connection hijacker, rust rewrite of shijack [<img src="https://api.travis-ci.org/kpcyrd/rshijack.svg?branch=master">](https://travis-ci.org/kpcyrd/rshijack)
* [kpcyrd/sniffglue](https://github.com/kpcyrd/sniffglue) — Secure multithreaded packet sniffer [<img src="https://api.travis-ci.org/kpcyrd/sniffglue.svg?branch=master">](https://travis-ci.org/kpcyrd/sniffglue)
* [kpcyrd/sn0int](https://github.com/kpcyrd/sn0int) — Semi-automatic OSINT framework and package manager [<img src="https://api.travis-ci.org/kpcyrd/sn0int.svg?branch=master">](https://travis-ci.org/kpcyrd/sn0int)

### System tools

* [Aaronepower/tokei](https://github.com/Aaronepower/tokei) — counts the lines of code [<img src="https://img.shields.io/travis/Aaronepower/tokei.svg">](https://travis-ci.org/Aaronepower/tokei)
* [brocode/fblog](https://github.com/brocode/fblog) — Small command-line JSON Log viewer [<img src="https://api.travis-ci.org/brocode/fblog.svg?branch=master">](https://travis-ci.org/brocode/fblog)
* [buster/rrun](https://github.com/buster/rrun) — a command launcher for Linux, similar to gmrun  [<img src="https://api.travis-ci.org/buster/rrun.svg?branch=master">](https://travis-ci.org/buster/rrun)
* [cristianoliveira/funzzy](https://github.com/cristianoliveira/funzzy) — a configurable filesystem watcher inspired by [entr](http://entrproject.org/) [<img src="https://api.travis-ci.org/cristianoliveira/funzzy.svg?branch=master">](https://travis-ci.org/cristianoliveira/funzzy)
* [ddh](https://github.com/darakian/ddh) — Fast duplicate file finder [<img src="https://api.travis-ci.org/darakian/ddh.svg?branch=master">](https://travis-ci.org/darakian/ddh)
* [jhspetersson/fselect](https://github.com/jhspetersson/fselect) [[fselect](https://crates.io/crates/fselect)] — Find files with SQL-like queries [<img src="https://api.travis-ci.org/jhspetersson/fselect.svg?branch=master">](https://travis-ci.org/jhspetersson/fselect)
* [k0pernicus/zou](https://github.com/k0pernicus/zou) — a download accelerator [<img src="https://api.travis-ci.org/k0pernicus/zou.svg?branch=master">](https://travis-ci.org/k0pernicus/zou)
* [lotabout/rargs](https://github.com/lotabout/rargs) [[rargs](https://crates.io/crates/rargs)] — xargs + awk with pattern matching support [<img src="https://api.travis-ci.org/lotabout/rargs.svg?branch=master">](https://travis-ci.org/lotabout/rargs)
* [lotabout/skim](https://github.com/lotabout/skim) — A fuzzy finder in pure rust [<img src="https://api.travis-ci.org/lotabout/skim.svg?branch=master">](https://travis-ci.org/lotabout/skim)
* [mitnk/cicada](https://github.com/mitnk/cicada) — A bash-like Unix shell [<img src="https://api.travis-ci.org/mitnk/cicada.svg?branch=master">](https://travis-ci.org/mitnk/cicada)
* [mmstick/concurr](https://github.com/mmstick/concurr) — Alternative to GNU Parallel w/ a client-server architecture
* [mmstick/fontfinder](https://github.com/mmstick/fontfinder) — GTK3 application for previewing and installing Google's fonts
* [mmstick/parallel](https://github.com/mmstick/parallel) — Reimplementation of GNU Parallel
* [mmstick/systemd-manager](https://github.com/mmstick/systemd-manager) — a systemd service manager written in Rust using GTK-rs.
* [mmstick/tv-renamer](https://github.com/mmstick/tv-renamer) — a tv series renaming application with an optional GTK3 frontend. [<img src="https://api.travis-ci.org/mmstick/tv-renamer.svg?branch=master">](https://travis-ci.org/mmstick/tv-renamer)
* [Peltoche/lsd](https://github.com/Peltoche/lsd) - An ls with a lot of pretty colors and awesome icons [<img src="https://api.travis-ci.org/Peltoche/lsd.svg?branch=master">](https://travis-ci.org/Peltoche/lsd)
* [ogham/exa](https://github.com/ogham/exa) — a replacement for 'ls' [<img src="https://api.travis-ci.org/ogham/exa.svg?branch=master">](https://travis-ci.org/ogham/exa)
* [pop-os/debrep](https://github.com/pop-os/debrepbuild) — APT repository tool for building and managing an APT repo
* [pop-os/popsicle](https://github.com/pop-os/popsicle) — GTK3 & CLI utility for flashing multiple USB devices in parallel
* [pop-os/pop-recovery](https://github.com/pop-os/pop-recovery) — Utility for updating the recovery partition on a Pop!\_OS install.
* [pop-os/system76-power](https://github.com/pop-os/system76-power/) — Linux power management daemon (DBus-interface) with CLI tool.
* [Ralvke/logram](https://github.com/Ralvke/logram) — Push log files' updates to Telegram [<img src="https://api.travis-ci.org/Ralvke/logram.svg?branch=master">](https://travis-ci.org/Ralvke/logram)
* [redox-os/ion](https://github.com/redox-os/ion) — Next-generation system shell [<img src="https://api.travis-ci.org/redox-os/ion.svg?branch=master">](https://travis-ci.org/redox-os/ion)
* [sharkdp/bat](https://github.com/sharkdp/bat) — A cat(1) clone with wings. [<img src="https://api.travis-ci.org/sharkdp/bat.svg?branch=master">](https://travis-ci.org/sharkdp/bat)
* [sharkdp/fd](https://github.com/sharkdp/fd) — A simple, fast and user-friendly alternative to find. [![Build Status](https://api.travis-ci.org/sharkdp/fd.svg?branch=master)](https://travis-ci.org/sharkdp/fd)
* [sitkevij/hex](https://github.com/sitkevij/hex) — A colorized hexdump terminal utility. [<img src="https://api.travis-ci.org/sitkevij/hex.svg?branch=master">](https://travis-ci.org/sitkevij/hex)
* [m4b/bingrep](https://github.com/m4b/bingrep) — Greps through binaries from various OSs and architectures, and colors them. [<img src="https://api.travis-ci.org/m4b/bingrep.svg?branch=master">](https://travis-ci.org/m4b/bingrep)
* [uutils/coreutils](https://github.com/uutils/coreutils) — a cross-platform Rust rewrite of the GNU coreutils [<img src="https://api.travis-ci.org/uutils/coreutils.svg?branch=master">](https://travis-ci.org/uutils/coreutils)
* [watchexec](https://github.com/watchexec/watchexec) — Executes commands in response to file modifications [<img src="https://api.travis-ci.org/mattgreen/watchexec.svg?branch=master">](https://travis-ci.org/mattgreen/watchexec)

### Text editors

* [gchp/iota](https://github.com/gchp/iota) — a simple text editor [<img src="https://api.travis-ci.org/gchp/iota.svg?branch=master">](https://travis-ci.org/gchp/iota)
* [mathall/rim](https://github.com/mathall/rim) — Vim-like text editor written in Rust [<img src="https://api.travis-ci.org/mathall/rim.svg?branch=master">](https://travis-ci.org/mathall/rim)
* [Remacs](https://github.com/Wilfred/remacs) — A community-driven port of Emacs to Rust. [<img src="https://api.travis-ci.org/Wilfred/remacs.svg?branch=master">](https://travis-ci.org/Wilfred/remacs)
* [xi-editor](https://github.com/xi-editor/xi-editor) — a modern editor with a backend written in Rust.
* [xray](https://github.com/atom/xray) — An experimental next-generation Electron-based text editor. [<img src="https://api.travis-ci.org/atom/xray.svg?branch=master">](https://travis-ci.org/atom/xray)

### Text processing
* [BurntSushi/ripgrep](https://github.com/BurntSushi/ripgrep) — combines the usability of The Silver Searcher with the raw speed of grep [<img src="https://api.travis-ci.org/BurntSushi/ripgrep.svg?branch=master">](https://travis-ci.org/BurntSushi/ripgrep)
* [chmln/sd](https://github.com/chmln/sd) [[sd](https://crates.io/crates/sd)] -Intuitive find & replace CLI [<img src="https://api.travis-ci.org/chmln/sd.svg?branch=master">](https://travis-ci.org/chmln/sd)
* [BurntSushi/xsv](https://github.com/BurntSushi/xsv) — a fast CSV command line tool (slicing, indexing, selecting, searching, sampling, etc.) [<img src="https://api.travis-ci.org/BurntSushi/xsv.svg?branch=master">](https://travis-ci.org/BurntSushi/xsv)
* [TankerHQ/ruplacer](https://github.com/TankerHQ/ruplacer) — Find and replace text in source files [<img img src="https://api.travis-ci.org/TankerHQ/ruplacer.svg?branch=master">](https://travis-ci.org/TankerHQ/ruplacer)
* [lavifb/todo_r](https://github.com/lavifb/todo_r) — Find all your TODO notes with one command! [<img src="https://api.travis-ci.org/lavifb/todo_r.svg?branch=master">](https://travis-ci.org/lavifb/todo_r)
* [whitfin/runiq](https://github.com/whitfin/runiq) — an efficient way to filter duplicate lines from unsorted input.
* [whitfin/bytelines](https://github.com/whitfin/bytelines) - Read input lines as byte slices for high efficiency.

### Utilities

* [brycx/checkpwn](https://github.com/brycx/checkpwn) — A Have I Been Pwned (HIBP) command-line utility tool that lets you easily check for compromised accounts and passwords.
* [evansmurithi/cloak](https://github.com/evansmurithi/cloak) — A Command Line OTP (One Time Password) Authenticator application. [<img src="https://api.travis-ci.com/evansmurithi/cloak.svg?branch=master">](https://travis-ci.com/evansmurithi/cloak) [<img src="https://ci.appveyor.com/api/projects/status/9mlfpfru3ng4c689/branch/master?svg=true">](https://ci.appveyor.com/project/evansmurithi/cloak)
* [myfreeweb/freepass](https://github.com/myfreeweb/freepass) — The free password manager for power users. [<img src="https://api.travis-ci.org/myfreeweb/freepass.svg?branch=master">](https://travis-ci.org/myfreeweb/freepass)
* [arthrp/quick-skeleton](https://github.com/arthrp/quick-skeleton) — Project scaffolding tool, similar to Yeoman and Slush. [<img src="https://api.travis-ci.org/arthrp/quick-skeleton.svg?branch=master">](https://travis-ci.org/arthrp/quick-skeleton)
* [yaa110/rubigo](https://github.com/yaa110/rubigo) — Golang dependency tool and package manager, written in Rust [<img src="https://api.travis-ci.org/yaa110/rubigo.svg?branch=master">](https://travis-ci.org/yaa110/rubigo)
* [whitfin/s3-concat](https://github.com/whitfin/s3-concat) — A command line tool to concatenate Amazon S3 files remotely using flexible patterns.
* [whitfin/s3-meta](https://github.com/whitfin/s3-meta) — A command line tool to gather metadata about your Amazon S3 buckets.
* [amar-laksh/workstation](https://github.com/amar-laksh/workstation) — A commandline tool to help you manage your workstation by distancing you from your screen, locking your screen when you aren't there among other things with OPENCV!
* [aleshaleksey/AZDice](https://github.com/aleshaleksey/AZDice) — A dice roll success distribution generator for tabletop homebrewers. [<img src="https://api.travis-ci.org/aleshaleksey/AZDice.svg?branch=master">](https://travis-ci.org/aleshaleksey/AZDice)

### Video

* [yuvadm/slingr](https://github.com/yuvadm/slingr) — A simple CLI for streaming media files over a local network to UPnP media renderers [<img src="https://api.travis-ci.org/yuvadm/slingr.svg?branch=master">](https://travis-ci.org/yuvadm/slingr)

### Virtualization

* [firecracker-microvm/firecracker](https://github.com/firecracker-microvm/firecracker) — a lightweight virtual machine for container workload  [Firecracker Microvm](https://firecracker-microvm.github.io/)
* [oracle/railcar](https://github.com/oracle/railcar) — Docker-like container OCI runtime implementation in Rust [![wercker status](https://app.wercker.com/status/730e874772dc02c6005f4ae4e42b0ca4/s/master "wercker status")](https://app.wercker.com/applications/59696a02ee70670100155ae2 )
* [tailhook/vagga](https://github.com/tailhook/vagga) — a containerization tool without daemons [<img src="https://api.travis-ci.org/tailhook/vagga.svg?branch=master">](https://travis-ci.org/tailhook/vagga)

### Window managers

* [way-cooler/way-cooler](https://github.com/way-cooler/way-cooler) — a customizable Wayland compositor (window manager) [<img src="https://api.travis-ci.org/way-cooler/way-cooler.svg?branch=master">](https://travis-ci.org/way-cooler/way-cooler)

### Web

* [Plume-org/Plume](https://github.com/Plume-org/Plume) — ActivityPub federating blogging application [<img src="https://api.travis-ci.org/Plume-org/Plume.svg?branch=master">](https://travis-ci.org/Plume-org/Plume)
* [rustlang-cn/ruster](https://github.com/rustlang-cn/ruster) — online community in rust for rust community [<img src="https://api.travis-ci.org/rustlang-cn/ruster.svg?branch=master">](https://travis-ci.org/rustlang-cn/ruster)


### Web Servers

* [thecoshman/http](https://github.com/thecoshman/http) — Host These Things Please — a basic http server for hosting a folder fast and simply [<img src="https://api.travis-ci.org/thecoshman/http.svg?branch=master">](https://travis-ci.org/thecoshman/http)
- [svenstaro/miniserve](https://github.com/svenstaro/miniserve) — a small, self-contained cross-platform CLI tool that allows you to just grab the binary and serve some file(s) via HTTP [<img src="https://api.travis-ci.org/svenstaro/miniserve.svg?branch=master">](https://travis-ci.org/svenstaro/miniserve)
- [TheWaWaR/simple-http-server](https://github.com/TheWaWaR/simple-http-server) — simple static http server

## Development tools

* [Clippy](https://github.com/rust-lang/rust-clippy) [[clippy](https://crates.io/crates/clippy)] — Rust lints [<img src="https://api.travis-ci.org/rust-lang/rust-clippy.svg?branch=master">](https://travis-ci.org/rust-lang/rust-clippy)
* [clog-tool/clog-cli](https://github.com/clog-tool/clog-cli) — generates a changelog from git metadata ([conventional changelog](https://blog.thoughtram.io/announcements/tools/2014/09/18/announcing-clog-a-conventional-changelog-generator-for-the-rest-of-us.html)) [<img src="https://api.travis-ci.org/clog-tool/clog-cli.svg?branch=master">](https://travis-ci.org/clog-tool/clog-cli)
* [dan-t/rusty-tags](https://github.com/dan-t/rusty-tags) — create ctags/etags for a cargo project and all of its dependencies [<img src="https://api.travis-ci.org/dan-t/rusty-tags.svg?branch=master">](https://travis-ci.org/dan-t/rusty-tags)
* [frewsxcv/crate-deps](https://github.com/frewsxcv/crate-deps) — generates images of dependency graphs for crates hosted on crates.io
* [git-journal](https://github.com/saschagrunert/git-journal/) — The Git Commit Message and Changelog Generation Framework [<img src="https://api.travis-ci.org/saschagrunert/git-journal.svg?branch=master">](https://travis-ci.org/saschagrunert/git-journal)
* [rust-lang-nursery/rustfix](https://github.com/rust-lang-nursery/rustfix)  — automatically applies the suggestions made by rustc [<img src="https://api.travis-ci.org/killercup/rustfix.svg?branch=master">](https://travis-ci.org/killercup/rustfix)
* [just](https://github.com/casey/just) — A handy command runner for project-specific tasks [<img src="https://api.travis-ci.org/casey/just.svg?branch=master">](https://travis-ci.org/casey/just)
* [Module Linker](https://fiatjaf.alhur.es/module-linker/) — Extension that adds `<a>` links to references in `mod`, `use` and `extern crate` statements at GitHub.
* [ptags](https://github.com/dalance/ptags) — A parallel universal-ctags wrapper for git repository [![Build Status](https://api.travis-ci.org/dalance/ptags.svg?branch=master)](https://travis-ci.org/dalance/ptags)
* [Racer](https://github.com/racer-rust/racer) — code completion for Rust [<img src="https://api.travis-ci.org/phildawes/racer.svg?branch=master">](https://travis-ci.org/phildawes/racer)
* [rustfmt](https://github.com/rust-lang/rustfmt) — a Rust code formatter [<img src="https://api.travis-ci.org/rust-lang/rustfmt.svg?branch=master">](https://travis-ci.org/rust-lang/rustfmt)
* [Rustup](https://github.com/rust-lang/rustup.rs) — the Rust toolchain installer [<img src="https://api.travis-ci.org/rust-lang/rustup.rs.svg?branch=master">](https://travis-ci.org/rust-lang/rustup.rs)
* [Rust Language Server](https://github.com/rust-lang/rls) — a server that runs in the background, providing IDEs, editors, and other tools with information about Rust programs
* [artifact](https://github.com/vitiral/artifact) — the design doc tool made for developers [![Build Status](https://api.travis-ci.org/vitiral/artifact.svg?branch=master)](https://travis-ci.org/vitiral/artifact)
* [semantic-rs](https://github.com/semantic-rs/semantic-rs) — automatic crate publishing [<img src="https://api.travis-ci.org/semantic-rs/semantic-rs.svg?branch=master">](https://travis-ci.org/semantic-rs/semantic-rs)
* [fw](https://github.com/brocode/fw) — workspace productivity booster [<img src="https://api.travis-ci.org/brocode/fw.svg?branch=master">](https://travis-ci.org/brocode/fw)
* [tinyrick](https://github.com/mcandre/tinyrick) a basic task dependency tool emphasizing Rust functions over raw shell commands.

### Build system

* [Cargo](https://crates.io/) — the Rust package manager
  * [BurntSushi/cargo-benchcmp](https://github.com/BurntSushi/cargo-benchcmp) [[cargo-benchcmp](https://crates.io/crates/cargo-benchcmp)] — utility to compare Rust micro-benchmarks [<img src="https://api.travis-ci.org/BurntSushi/cargo-benchcmp.svg?branch=master">](https://travis-ci.org/BurntSushi/cargo-benchcmp)
   * [cardoe/cargo-bitbake](https://github.com/cardoe/cargo-bitbake) [[cargo-bitbake](https://crates.io/crates/cargo-bitbake)] — cargo extension that can generate BitBake recipes utilizing the classes from meta-rust [<img src="https://api.travis-ci.org/cardoe/cargo-bitbake.svg?branch=master">](https://travis-ci.org/cardoe/cargo-bitbake)
  * [rsolomo/cargo-check](https://github.com/rsolomo/cargo-check) [[cargo-check](https://crates.io/crates/cargo-check)] — a wrapper around `cargo rustc -- -Zno-trans` which can be helpful for running a faster compile if you only need correctness checks [<img src="https://api.travis-ci.org/rsolomo/cargo-check.svg?branch=master">](https://travis-ci.org/rsolomo/cargo-check)
  * [kbknapp/cargo-count](https://github.com/kbknapp/cargo-count) [[cargo-count](https://crates.io/crates/cargo-count)] — lists source code counts and details about cargo projects, including unsafe statistics [<img src="https://api.travis-ci.org/kbknapp/cargo-count.svg?branch=master">](https://travis-ci.org/kbknapp/cargo-count)
  * [mmstick/cargo-deb](https://github.com/mmstick/cargo-deb) — generate binary Debian packages [<img src="https://api.travis-ci.org/mmstick/cargo-deb.svg?branch=master">](https://travis-ci.org/mmstick/cargo-deb)
  * [pwoolcoc/cargo-do](https://github.com/pwoolcoc/cargo-do) [[cargo-do](https://crates.io/crates/cargo-do)] — run multiple cargo commands in a row [<img src="https://api.travis-ci.org/pwoolcoc/cargo-do.svg?branch=master">](https://travis-ci.org/pwoolcoc/cargo-do)
  * [maxsnew/cargo-dot](https://github.com/maxsnew/cargo-dot) — generate graphs of a Cargo project's dependencies [<img src="https://api.travis-ci.org/maxsnew/cargo-dot.svg?branch=master">](https://travis-ci.org/maxsnew/cargo-dot)
  * [cardoe/cargo-ebuild](https://github.com/cardoe/cargo-ebuild) [[cargo-ebuild](https://crates.io/crates/cargo-ebuild)] — cargo extension that can generate ebuilds using the in-tree eclasses [<img src="https://api.travis-ci.org/cardoe/cargo-ebuild.svg?branch=master">](https://travis-ci.org/cardoe/cargo-ebuild)
  * [killercup/cargo-edit](https://github.com/killercup/cargo-edit) [[cargo-edit](https://crates.io/crates/cargo-edit)] — allows you to add and list dependencies by reading/writing to your Cargo.toml file from the command line [<img src="https://api.travis-ci.org/killercup/cargo-edit.svg?branch=master">](https://travis-ci.org/killercup/cargo-edit)
  * [Ralvke/cargo-find](https://github.com/Ralvke/cargo-find) [[cargo-find](https://crates.io/crates/cargo-find)] <sup>deprecated</sup> — Find crates from command line [<img src="https://api.travis-ci.org/Ralvke/cargo-find.svg?branch=master">](https://travis-ci.org/Ralvke/cargo-find)
  * [kbknapp/cargo-graph](https://github.com/kbknapp/cargo-graph) [[cargo-graph](https://crates.io/crates/cargo-graph)] — updated fork of `cargo-dot` with additional features [<img src="https://api.travis-ci.org/kbknapp/cargo-graph.svg?branch=master">](https://travis-ci.org/kbknapp/cargo-graph)
  * [imp/cargo-info](https://gitlab.com/imp/cargo-info) [[cargo-info](https://crates.io/crates/cargo-info)] — queries crates.io for crates details from command line [<img src="https://api.travis-ci.org/imp/cargo-info.svg?branch=master">](https://travis-ci.org/imp/cargo-info)
  * [sagiegurari/cargo-make](https://github.com/sagiegurari/cargo-make) [[cargo-make](https://crates.io/crates/cargo-make)] — Rust task runner and build tool. [<img src="https://api.travis-ci.org/sagiegurari/cargo-make.svg?branch=master">](https://travis-ci.org/sagiegurari/cargo-make)
  * [regexident/cargo-modules](https://github.com/regexident/cargo-modules) [[cargo-modules](https://crates.io/crates/cargo-modules)] — A cargo plugin for showing a tree-like overview of a crate's modules. [<img src="https://api.travis-ci.org/regexident/cargo-modules.svg?branch=master">](https://travis-ci.org/regexident/cargo-modules)
  * [imp/cargo-multi](https://gitlab.com/imp/cargo-multi) [[cargo-multi](https://crates.io/crates/cargo-multi)] — runs specified cargo command on multiple crates [<img src="https://api.travis-ci.org/imp/cargo-multi.svg?branch=master">](https://travis-ci.org/imp/cargo-multi)
  * [kbknapp/cargo-outdated](https://github.com/kbknapp/cargo-outdated) [[cargo-outdated](https://crates.io/crates/cargo-outdated)] — displays when newer versions of Rust dependencies are available, or out of date [<img src="https://api.travis-ci.org/kbknapp/cargo-outdated.svg?branch=master">](https://travis-ci.org/kbknapp/cargo-outdated)
  * [sunng87/cargo-release](https://github.com/sunng87/cargo-release) [[cargo-release](https://crates.io/crates/cargo-release)] — tool for releasing git-managed cargo project, build, tag, publish, doc and push [<img src="https://api.travis-ci.org/sunng87/cargo-release.svg?branch=master">](https://travis-ci.org/sunng87/cargo-release)
  * [DanielKeep/cargo-script](https://github.com/DanielKeep/cargo-script) [[cargo-script](https://crates.io/crates/cargo-script)] — lets people quickly and easily run Rust "scripts" which can make use of Cargo's package ecosystem [<img src="https://api.travis-ci.org/DanielKeep/cargo-script.svg?branch=master">](https://travis-ci.org/DanielKeep/cargo-script)
  * [greyblake/cargo-testify](https://github.com/greyblake/cargo-testify) [[cargo-testify](https://crates.io/crates/cargo-testify)] — watches files changes, runs tests and notifies about the result with friendly OS notification [<img src="https://api.travis-ci.org/greyblake/cargo-testify.svg?branch=master">](https://travis-ci.org/greyblake/cargo-testify)
  * [nabijaczleweli/cargo-update](https://github.com/nabijaczleweli/cargo-update) [[cargo-update](https://crates.io/crates/cargo-update)] — cargo subcommand for checking and applying updates to installed executables [<img src="https://api.travis-ci.org/nabijaczleweli/cargo-update.svg?branch=master">](https://travis-ci.org/nabijaczleweli/cargo-update)
  * [passcod/cargo-watch](https://github.com/passcod/cargo-watch) [[cargo-watch](https://crates.io/crates/cargo-watch)] — utility for cargo to compile projects when sources change [<img src="https://api.travis-ci.org/passcod/cargo-watch.svg?branch=master">](https://travis-ci.org/passcod/cargo-watch)
  * [liuchong/cargo-x](https://github.com/liuchong/cargo-x) — A very simple third-party cargo subcommand to execute a custom command [<img src="https://api.travis-ci.org/liuchong/cargo-x.svg?branch=master">](https://travis-ci.org/liuchong/cargo-x)
* CMake
  * [Devolutions/CMakeRust](https://github.com/Devolutions/CMakeRust) — useful for integrating a Rust library into a CMake project
  * [SiegeLord/RustCMake](https://github.com/SiegeLord/RustCMake) — an example project showing usage of CMake with Rust [<img src="https://api.travis-ci.org/SiegeLord/RustCMake.svg?branch=master">](https://travis-ci.org/SiegeLord/RustCMake)
* Github actions
  * [icepuma/rust-github-actions](https://github.com/icepuma/rust-github-actions) — rust github actions
* Webpack
  * [Ralvke/rust-loader](https://github.com/Ralvke/rust-loader) — Webpack Rust loader (wasm)

### Debugging

* GDB
  * [rust-gdb](https://github.com/rust-lang/rust/blob/master/src/etc/rust-gdb)
  * [gdbgui](https://github.com/cs01/gdbgui) — Browser based frontend for gdb to debug C, C++, Rust, and go. [<img src="https://api.travis-ci.org/cs01/gdbgui.svg?branch=master">](https://travis-ci.org/cs01/gdbgui)
* LLDB
  * [lldb_batchmode.py](https://github.com/rust-lang/rust/blob/master/src/etc/lldb_batchmode.py) — allows to use LLDB in a way similar to GDB's batch mode.
  * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) — a LLDB extension for [Visual Studio Code](https://code.visualstudio.com/).
* rr
  * [rr](https://rr-project.org/) — rr is a lightweight tool for recording and replaying execution of applications

### Deployment

* Docker
  * [emk/rust-musl-builder](https://github.com/emk/rust-musl-builder) — Docker images for compiling static Rust binaries using musl-libc and musl-gcc, with static versions of useful C libraries
  * [kpcyrd/mini-docker-rust](https://github.com/kpcyrd/mini-docker-rust) — An example project for very small rust docker images [<img src="https://api.travis-ci.org/kpcyrd/mini-docker-rust.svg?branch=master">](https://travis-ci.org/kpcyrd/mini-docker-rust)
  * [liuchong/docker-rustup](https://github.com/liuchong/docker-rustup) — a multiple version (with musl tools) Rust Docker image
  * [messense/rust-musl-cross](https://github.com/messense/rust-musl-cross) — Docker images for compiling static Rust binaries using musl-cross [<img src="https://api.travis-ci.org/messense/rust-musl-cross.svg?branch=master">](https://travis-ci.org/messense/rust-musl-cross)
  * [rust-lang-nursery/docker-rust](https://github.com/rust-lang/docker-rust) — the official Rust Docker image
* Google App Engine
  * [DenisKolodin/rust-app-engine](https://github.com/DenisKolodin/rust-app-engine) — App Engine Rust boilerplate
* Heroku
  * [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust) — a buildpack for Rust applications on Heroku

### Embedded

[Rust Embedded](https://rust-embedded.org/)

* Cross compiling
  * [japaric/rust-cross](https://github.com/japaric/rust-cross) — everything you need to know about cross compiling Rust programs [<img src="https://api.travis-ci.org/japaric/rust-cross.svg?branch=master">](https://travis-ci.org/japaric/rust-cross)
  * [japaric/xargo](https://github.com/japaric/xargo) — effortless cross compilation of Rust programs to custom bare-metal targets like ARM Cortex-M [<img src="https://api.travis-ci.org/japaric/xargo.svg?branch=master">](https://travis-ci.org/japaric/xargo)
* Raspberry Pi
  * [Ogeon/rust-on-raspberry-pi](https://github.com/Ogeon/rust-on-raspberry-pi) — instructions for how to cross compile Rust projects for the Raspberry Pi .

### FFI

See also [Foreign Function Interface](https://doc.rust-lang.org/book/first-edition/ffi.html),  [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) (a collection of examples of using code written in Rust from other languages) and [FFI examples written in Rust](https://github.com/alexcrichton/rust-ffi-examples).

* C
  * [rlhunt/cbindgen](https://github.com/eqrion/cbindgen) — generates C header files from Rust source files. Used in Gecko for WebRender [<img src="https://api.travis-ci.org/eqrion/cbindgen.svg?branch=master">](https://travis-ci.org/eqrion/cbindgen)
  * [Sean1708/rusty-cheddar](https://github.com/Sean1708/rusty-cheddar) — generates C header files from Rust source files [<img src="https://api.travis-ci.org/Sean1708/rusty-cheddar.svg?branch=master">](https://travis-ci.org/Sean1708/rusty-cheddar)
* C++
  * [rust-lang/rust-bindgen](https://github.com/rust-lang/rust-bindgen) — a Rust bindings generator
* Erlang
  * [hansihe/Rustler](https://github.com/hansihe/Rustler) — safe Rust bridge for creating Erlang NIF functions [<img src="https://api.travis-ci.org/hansihe/rustler.svg?branch=master">](https://travis-ci.org/hansihe/rustler)
* Haskell
  * [mgattozzi/curryrs](https://github.com/mgattozzi/curryrs) — Bridge the gap between Haskell and Rust
  * [mgattozzi/haskellrs](https://github.com/mgattozzi/haskellrs) — Rust in Haskell FFI Example
  * [mgattozzi/rushs](https://github.com/mgattozzi/rushs) — Haskell in Rust FFI Example
* Java
  * [bennettanderson/rjni](https://github.com/benanders/rjni) — use Java from Rust [<img src="https://api.travis-ci.org/bennettanderson/rjni.svg?branch=master">](https://travis-ci.org/GravityScore/RustJNI)
  * [drrb/java-rust-example](https://github.com/drrb/java-rust-example) — use Rust from Java [<img src="https://api.travis-ci.org/drrb/java-rust-example.svg?branch=master">](https://travis-ci.org/drrb/java-rust-example)
  * [kud1ing/rucaja](https://github.com/kud1ing/rucaja) [[rucaja](https://crates.io/crates/rucaja)] — use Java from Rust [<img src="https://api.travis-ci.org/kud1ing/rucaja.svg?branch=master">](https://travis-ci.org/kud1ing/rucaja)
  * [prevoty/jni-rs](https://github.com/jni-rs/jni-rs) [[jni](https://crates.io/crates/jni)] — use Rust from Java [<img src="https://api.travis-ci.org/prevoty/jni-rs.svg?branch=master">](https://travis-ci.org/prevoty/jni-rs)
  * [rawrasaur/rust-jdbc](https://github.com/rawrasaur/rust-jdbc) — uses JDBC from Rust [<img src="https://api.travis-ci.org/rawrasaur/rust-jdbc.svg?branch=master">](https://travis-ci.org/rawrasaur/rust-jdbc)
  * [sfackler/rust-jni-sys](https://github.com/sfackler/rust-jni-sys) [[jni-sys](https://crates.io/crates/jni-sys)] — Rust definitions corresponding to jni.h [<img src="https://api.travis-ci.org/sfackler/rust-jni-sys.svg?branch=master">](https://travis-ci.org/sfackler/rust-jni-sys)
* Lua
  * [jcmoyer/rust-lua53](https://github.com/jcmoyer/rust-lua53) — Lua 5.3 bindings for Rust [<img src="https://api.travis-ci.org/jcmoyer/rust-lua53.svg?branch=master">](https://travis-ci.org/jcmoyer/rust-lua53)
  * [kballard/rust-lua](https://github.com/kballard/rust-lua) — Safe Rust bindings to Lua 5.1 [<img src="https://api.travis-ci.org/kballard/rust-lua.svg">](https://travis-ci.org/kballard/rust-lua)
  * [tickbh/td_rlua](https://github.com/tickbh/td_rlua) — Zero-cost high-level lua 5.3 wrapper for Rust [<img src="https://api.travis-ci.org/tickbh/td_rlua.svg?branch=master">](https://travis-ci.org/tickbh/td_rlua)
  * [tomaka/hlua](https://github.com/tomaka/hlua) — Rust library to interface with Lua [<img src="https://api.travis-ci.org/tomaka/hlua.svg?branch=master">](https://travis-ci.org/tomaka/hlua)
* mruby
  * [anima-engine/mrusty](https://github.com/anima-engine/mrusty) — mruby safe bindings for Rust [<img src="https://api.travis-ci.org/anima-engine/mrusty.svg?branch=master">](https://travis-ci.org/anima-engine/mrusty)
* Node.js
  * [neon-bindings/neon](https://github.com/neon-bindings/neon) — use Rust from Node.js [<img src="https://api.travis-ci.org/neon-bindings/neon.svg?branch=master">](https://travis-ci.org/neon-bindings/neon)
* Objective-C
  * [SSheldon/rust-objc](https://github.com/SSheldon/rust-objc) — Objective-C Runtime bindings and wrapper for Rust
* Perl
  * [vickenty/mi-rust](https://github.com/vickenty/mi-rust) — Adds support to M::I for building modules with Cargo
  * [vickenty/perl-xs](https://github.com/vickenty/perl-xs) — Create Perl XS modules using Rust [<img src="https://api.travis-ci.org/vickenty/perl-xs.svg?branch=master">](https://travis-ci.org/vickenty/perl-xs)
* Python
  * [getsentry/milksnake](https://github.com/getsentry/milksnake) — extension for python setuptools that allows you to distribute dynamic linked libraries in Python wheels in the most portable way imaginable.
  * [dgrunwald/rust-cpython](https://github.com/dgrunwald/rust-cpython) — Python bindings [<img src="https://api.travis-ci.org/dgrunwald/rust-cpython.svg?branch=master">](https://travis-ci.org/dgrunwald/rust-cpython)
  * [PyO3/PyO3](https://github.com/PyO3/PyO3) — Rust bindings for the Python interpreter [<img src="https://api.travis-ci.org/PyO3/pyo3.svg?branch=master">](https://travis-ci.org/PyO3/pyo3)
* R
  * [rustr/rustr](https://github.com/rustr/rustr) — use Rust from R, and use R in Rust [<img src="https://api.travis-ci.org/rustr/rustr.svg?branch=master">](https://travis-ci.org/rustr/rustr)
* Ruby
  * [d-unseductable/ruru](https://github.com/d-unseductable/ruru) — native Ruby extensions written in Rust [<img src="https://api.travis-ci.org/d-unseductable/ruru.svg?branch=master">](https://travis-ci.org/d-unseductable/ruru)
  * [danielpclark/rutie](https://github.com/danielpclark/rutie) — native Ruby extensions written in Rust and vice versa [![Build Status](https://api.travis-ci.org/danielpclark/rutie.svg?branch=master)](https://travis-ci.org/danielpclark/rutie)
  * [tildeio/helix](https://github.com/tildeio/helix) — write Ruby classes in Rust [<img src="https://api.travis-ci.org/tildeio/helix.svg?branch=master">](https://travis-ci.org/tildeio/helix)
* Web Assembly
  * [rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack) — :package: :sparkles: pack up the wasm and publish it to npm! [<img src="https://api.travis-ci.org/rustwasm/wasm-pack.svg?branch=master">](https://travis-ci.org/rustwasm/wasm-packn)
  * [rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — A project for facilitating high-level interactions between wasm modules and JS. [<img src="https://api.travis-ci.org/rustwasm/wasm-bindgen.svg?branch=master">](https://travis-ci.org/rustwasm/wasm-bindgen)

### IDEs

See also [Are we (I)DE yet?](https://areweideyet.com/) and [Rust Tools](https://www.rust-lang.org/tools).

  * [Atom](https://atom.io/)
    * [zargony/atom-language-rust](https://github.com/zargony/atom-language-rust)
  * [Eclipse](https://www.eclipse.org/)
    * [RustDT](https://github.com/RustDT/RustDT) — [<img src="https://api.travis-ci.org/RustDT/RustDT.svg?branch=master">](https://travis-ci.org/RustDT/RustDT)
  * [Emacs](https://www.gnu.org/software/emacs/)
    * [rust-mode](https://github.com/rust-lang/rust-mode) — Rust Major Mode
    * [flycheck-rust](https://github.com/flycheck/flycheck-rust) — Rust support for [Flycheck](https://github.com/flycheck/flycheck)
    * [emacs-racer](https://github.com/racer-rust/emacs-racer) — Autocompletion (see also [company](https://company-mode.github.io) and [auto-complete](https://github.com/auto-complete/auto-complete))
    * [lsp-rust](https://github.com/emacs-lsp/lsp-rust) — Add rls support to [lsp-mode](https://github.com/emacs-lsp/lsp-mode)
  * [gitpod.io](https://gitpod.io) — Online IDE with full Rust support based on Rust Language Server
  * [gnome-builder](https://wiki.gnome.org/Apps/Builder) native support for rust and cargo since Version 3.22.2
  * [Kakoune](http://kakoune.org/)
    * [ul/kak-lsp](https://github.com/ul/kak-lsp/) — [LSP](https://microsoft.github.io/language-server-protocol/) client. Implemented in Rust and supports rls out of the box.
  * [NetBeans](https://netbeans.org/)
    * [drrb/rust-netbeans](https://github.com/drrb/rust-netbeans)
  * [IntelliJ](https://www.jetbrains.com/idea/)
    * [intellij-rust/intellij-rust](https://github.com/intellij-rust/intellij-rust) — [<img src="https://api.travis-ci.org/intellij-rust/intellij-rust.svg?branch=master">](https://travis-ci.org/intellij-rust/intellij-rust)
    * [intellij-rust/intellij-toml](https://github.com/intellij-rust/intellij-toml) — basic Toml support
  * [Ride](https://github.com/madeso/ride) — [<img src="https://api.travis-ci.org/madeso/ride.svg?branch=master">](https://travis-ci.org/madeso/ride)
  * [SolidOak](https://github.com/oakes/SolidOak) — a simple IDE for Rust, based on GTK+ and [Neovim](https://github.com/neovim/neovim)
  * [Sublime Text](https://www.sublimetext.com/)
    * [rust-lang/rust-enhanced](https://github.com/rust-lang/rust-enhanced) — official Rust package
    * [sublimehq/packages](https://github.com/sublimehq/Packages/tree/master/Rust) — native Sublime support (already installed)
  * [Vim](https://vim.sourceforge.io/) — the ubiquitous text editor
	* [rust.vim](https://github.com/rust-lang/rust.vim) — provides file detection, syntax highlighting, formatting, Syntastic integration, and more.
	* [vim-cargo](https://github.com/timonv/vim-cargo) — command bindings to quickly run cargo stuff from vim.
	* [vim-racer](https://github.com/racer-rust/vim-racer) — allows vim to use [Racer](https://github.com/racer-rust/racer) for Rust code completion and navigation.
	* [autozimu/LanguageClient-neovim](https://github.com/autozimu/LanguageClient-neovim) — [LSP](https://microsoft.github.io/language-server-protocol/) client. Implemented in Rust and supports rls out of the box.
  * Visual Studio
    * [PistonDevelopers/VisualRust](https://github.com/PistonDevelopers/VisualRust) — a Visual Studio extension for Rust [<img src="https://api.travis-ci.org/PistonDevelopers/VisualRust.svg?branch=master">](https://travis-ci.org/PistonDevelopers/VisualRust)
  * [Visual Studio Code](https://code.visualstudio.com/)
    * [rust-lang/rls-vscode](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) — Rust support for Visual Studio Code
    * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) — a LLDB extension
    * [crates](https://github.com/serayuzgur/crates) — crates is an extension for crates.io dependencies. [<img src="https://img.shields.io/vscode-marketplace/v/serayuzgur.crates.svg">](https://github.com/serayuzgur/crates) [<img src="https://api.travis-ci.org/serayuzgur/crates.svg?branch=master">](https://travis-ci.org/serayuzgur/crates)
    * [KalitaAlexey/vscode-rust](https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust) — a fork of RustyCode
    * [saviorisdead/RustyCode](https://marketplace.visualstudio.com/items?itemName=saviorisdead.RustyCode) (unmaintained)

### Pattern recognition

* [sfikas/rusteval](https://github.com/sfikas/rusteval) — A tool used to evaluate the output of retrieval algorithms [![Build Status](https://api.travis-ci.org/sfikas/rusteval.svg?branch=master)](https://travis-ci.org/sfikas/rusteval)

### Profiling

* [performancecopilot/hornet](https://github.com/performancecopilot/hornet) — a Performance Co-Pilot memory-mapped values instrumentation library [![crates.io badge](https://img.shields.io/crates/v/hornet.svg)](https://crates.io/crates/hornet) [<img src="https://api.travis-ci.org/performancecopilot/hornet.svg?branch=master">](https://travis-ci.org/performancecopilot/hornet)
* [ellisonch/rust-stopwatch](https://github.com/ellisonch/rust-stopwatch) — a stopwatch library [<img src="https://api.travis-ci.org/ellisonch/rust-stopwatch.svg?branch=master">](https://travis-ci.org/ellisonch/rust-stopwatch)
* FlameGraphs
  * [mrhooray/torch](https://github.com/mrhooray/torch) — generates FlameGraphs based on DWARF Debug Info
  * [TyOverby/flame](https://github.com/TyOverby/flame) — [<img src="https://api.travis-ci.org/TyOverby/flame.svg?branch=master">](https://travis-ci.org/TyOverby/flame)

### Static analysis

[[assert](https://crates.io/keywords/assert), [static](https://crates.io/keywords/static)]

* [nvzqz/static-assertions-rs](https://github.com/nvzqz/static-assertions-rs/) [[static_assertions](https://crates.io/crates/static_assertions)] Compile-time assertions to ensure that invariants are met [![Build Status](https://api.travis-ci.org/nvzqz/static-assertions-rs.svg?branch=master)](https://travis-ci.org/nvzqz/static-assertions-rs/)

### Testing

[[testing](https://crates.io/keywords/testing)]

* [llogiq/mutagen](https://github.com/llogiq/mutagen) [[mutagen](https://crates.io/crates/mutagen)] — A source-level mutation testing framework (nightly only) [<img src="https://api.travis-ci.org/llogiq/mutagen.svg?branch=master">](https://travis-ci.org/llogiq/mutagen)
* [AlKass/polish](https://github.com/AlKass/polish) — Mini Testing/Test-Driven Framework [![Build Status](https://api.travis-ci.org/AlKass/polish.svg?branch=master)](https://travis-ci.org/AlKass/polish) [![Crates Package Status](https://img.shields.io/crates/v/polish.svg)](https://crates.io/crates/polish)
* [altsysrq/proptest](https://github.com/altsysrq/proptest) [[proptest](https://crates.io/crates/proptest)] — property testing framework inspired by the [Hypothesis](https://hypothesis.works/) framework for Python [<img src="https://api.travis-ci.org/altsysrq/proptest.svg?branch=master">](https://travis-ci.org/altsysrq/proptest)
* [BurntSushi/quickcheck](https://github.com/BurntSushi/quickcheck) [[quickcheck](https://crates.io/crates/quickcheck)] — a Rust implementation of [QuickCheck](https://wiki.haskell.org/Introduction_to_QuickCheck1) [<img src="https://api.travis-ci.org/BurntSushi/quickcheck.svg?branch=master">](https://travis-ci.org/BurntSushi/quickcheck)
* [Mockito](https://github.com/lipanski/mockito) [[mockito](https://crates.io/crates/mockito)] — HTTP mocking [<img src="https://api.travis-ci.org/lipanski/mockito.svg?branch=master">](https://travis-ci.org/lipanski/mockito)
* [utkarshkukreti/speculate.rs](https://github.com/utkarshkukreti/speculate.rs) [[speculate](https://crates.io/crates/speculate)] — An RSpec inspired minimal testing framework for Rust
* [rust-fuzz/afl.rs](https://github.com/rust-fuzz/afl.rs) — a Rust fuzzer, using [AFL](http://lcamtuf.coredump.cx/afl/) [<img src="https://api.travis-ci.org/rust-fuzz/afl.rs.svg?branch=master">](https://travis-ci.org/rust-fuzz/afl.rs)
* [trust](https://github.com/japaric/trust) — A Travis CI and AppVeyor template to test your Rust crate on 5 architectures and publish binary releases of it for Linux, macOS and Windows
* [xd009642/tarpaulin](https://github.com/xd009642/tarpaulin) [[tarpaulin](https://crates.io/crates/cargo-tarpaulin)] — A code coverage tool designed for Rust [<img src="https://api.travis-ci.org/repositories/xd009642/tarpaulin.svg?branch=master">](https://travis-ci.org/xd009642/tarpaulin)

### Transpiling

* [immunant/c2rust](https://github.com/immunant/c2rust) — C to Rust translator and cross checker built atop Clang/LLVM. [![Build Status](https://api.travis-ci.org/immunant/c2rust.svg?branch=master)](https://travis-ci.org/immunant/c2rust)
* [jameysharp/corrode](https://github.com/jameysharp/corrode) — A C to Rust translator written in Haskell.

## Libraries

### Artificial Intelligence

#### Genetic algorithms

* [Martin1887/oxigen](https://github.com/Martin1887/oxigen) — Fast, parallel, extensible and adaptable genetic algorithm library. A example using this library solves the N Queens problem for N = 255 in only few seconds and using less than 1 MB of RAM.
* [innoave/genevo](https://github.com/innoave/genevo) — Execute genetic algorithm (GA) simulations in a customizable and extensible way.
* [willi-kappler/darwin-rs](https://github.com/willi-kappler/darwin-rs) — This library allows you to write evolutionary algorithms (EA) using the Rust programming language. Written by Willi Kappler, License: MIT — Version 0.4 (2017.06.26).
* [m-decoster/RsGenetic](https://github.com/m-decoster/RsGenetic) — Genetic Algorithm library in Rust. In maintenance mode.
* [mneumann/evo-rs](https://github.com/mneumann/evo-rs) — Evolutionary Algorithm Library for Rust. Without changes for 3 years.
* [yurytsoy/revonet](https://github.com/yurytsoy/revonet) — Rust implementation of real-coded GA for solving optimization problems and training of neural networks.

#### Machine learning

[[machine learning](https://crates.io/keywords/machine-learning)]

See also [About Rust’s Machine Learning Community](https://medium.com/@autumn_eng/about-rust-s-machine-learning-community-4cda5ec8a790#.hvkp56j3f).

* [AtheMathmo/rusty-machine](https://github.com/AtheMathmo/rusty-machine) — Machine learning library for Rust [![Build Status](https://api.travis-ci.org/AtheMathmo/rusty-machine.svg?branch=master)](https://travis-ci.org/AtheMathmo/rusty-machine)
* [avinashshenoy97/RusticSOM](https://github.com/avinashshenoy97/RusticSOM) — Rust library for Self Organising Maps (SOM). [![Build Status](https://api.travis-ci.org/avinashshenoy97/RusticSOM.svg?branch=master)](https://travis-ci.org/avinashshenoy97/RusticSOM)
* [autumnai/leaf](https://github.com/autumnai/leaf) — Open Machine Intelligence framework. [![Build Status](https://api.travis-ci.org/autumnai/leaf.svg?branch=master)](https://travis-ci.org/autumnai/leaf). Abandoned project. The most updated fork is [spearow/juice]( https://github.com/spearow/juice).
* [tensorflow/rust](https://github.com/tensorflow/rust) — Rust language bindings for TensorFlow. [![Build Status](https://api.travis-ci.org/tensorflow/rust.svg?branch=master)](https://travis-ci.org/tensorflow/rust)
* [maciejkula/rustlearn](https://github.com/maciejkula/rustlearn) — Machine learning crate for Rust. [![Circle CI](https://circleci.com/gh/maciejkula/rustlearn.svg?style=svg)](https://circleci.com/gh/maciejkula/rustlearn)


### Astronomy

[[astronomy](https://crates.io/keywords/astronomy)]

* [saurvs/astro-rust](https://github.com/saurvs/astro-rust) — astronomy for Rust [<img src="https://api.travis-ci.org/saurvs/astro-rust.svg?branch=master">](https://travis-ci.org/saurvs/astro-rust)
* [mindriot101/rust-fitsio](https://github.com/mindriot101/rust-fitsio) [[fitsio](https://crates.io/crates/fitsio)] — fits interface library wrapping cfitsio [<img src="https://api.travis-ci.org/mindriot101/rust-fitsio.svg?branch=master">](https://travis-ci.org/mindriot101/rust-fitsio)
* [flosse/rust-sun](https://github.com/flosse/rust-sun) — A rust port of the JS library suncalc [<img src="https://api.travis-ci.org/flosse/rust-sun.svg?branch=master">](https://travis-ci.org/flosse/rust-sun)

### Asynchronous

* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) — a coroutine I/O library with a working-stealing scheduler [<img src="https://api.travis-ci.org/zonyitoo/coio-rs.svg?branch=master">](https://travis-ci.org/zonyitoo/coio-rs)
* [dpc/mioco](https://github.com/dpc/mioco) — Scalable, coroutine-based, asynchronous IO handling library [<img src="https://img.shields.io/travis/dpc/mioco/master.svg?style=flat-square" alt="Travis CI Build Status">](https://travis-ci.org/dpc/mioco)
* [TeaEntityLab/fpRust](https://github.com/TeaEntityLab/fpRust) — Monad/MonadIO, Handler, Coroutine/doNotation, Functional Programming features for Rust [<img src="https://api.travis-ci.org/TeaEntityLab/fpRust.svg?branch=master" alt="Travis CI Build Status">](https://travis-ci.org/TeaEntityLab/fpRust)
* [rust-lang-nursery/futures-rs](https://github.com/rust-lang-nursery/futures-rs) — Zero-cost futures in Rust [<img src="https://api.travis-ci.org/rust-lang-nursery/futures-rs.svg?branch=master" alt="Travis CI Build Status">](https://travis-ci.org/rust-lang-nursery/futures-rs)
* [carllerche/mio](https://github.com/carllerche/mio) — MIO is a lightweight IO library for Rust with a focus on adding as little overhead as possible over the OS abstractions [<img src="https://api.travis-ci.org/carllerche/mio.svg?branch=master">](https://travis-ci.org/carllerche/mio)

### Audio

[[audio](https://crates.io/keywords/audio)]

* [GuillaumeGomez/rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) — [FMOD](https://www.fmod.com) bindings [![Build Status](https://api.travis-ci.org/GuillaumeGomez/rust-fmod.svg?branch=master)](https://travis-ci.org/GuillaumeGomez/rust-fmod)
* [jhasse/ears](https://github.com/jhasse/ears) — a simple library to play Sounds and Musics, on top of OpenAL and libsndfile [<img src="https://api.travis-ci.org/jhasse/ears.svg?branch=master">](https://travis-ci.org/jhasse/ears)
* [jpernst/alto](https://github.com/jpernst/alto) — OpenAL 1.1 bindings [<img src="https://api.travis-ci.org/jpernst/alto.svg?branch=master">](https://travis-ci.org/jpernst/alto)
* [musitdev/portmidi-rs](https://github.com/musitdev/portmidi-rs) — [PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings [<img src="https://api.travis-ci.org/musitdev/portmidi-rs.svg?branch=master">](https://travis-ci.org/musitdev/portmidi-rs)
* [ruuda/hound](https://github.com/ruuda/hound) [[Hound](https://crates.io/crates/hound)] — A WAV encoding and decoding library [<img src="https://api.travis-ci.org/ruuda/hound.svg?branch=master">](https://travis-ci.org/ruuda/hound)
* [tomaka/rodio](https://github.com/tomaka/rodio) — A Rust audio playback library [![Build Status](https://api.travis-ci.org/tomaka/rodio.svg?branch=master)](https://travis-ci.org/tomaka/rodio)
* [RustAudio](https://github.com/RustAudio)
  * [RustAudio/rust-portaudio](https://github.com/RustAudio/rust-portaudio) — [PortAudio](http://www.portaudio.com/) bindings [<img src="https://api.travis-ci.org/RustAudio/rust-portaudio.svg?branch=master">](https://travis-ci.org/RustAudio/rust-portaudio)

### Authentication

* [Keats/jsonwebtoken](https://github.com/Keats/jsonwebtoken) — [JSON Web Token](https://en.wikipedia.org/wiki/JSON_Web_Token) lib in rust  [![Build Status](https://api.travis-ci.org/Keats/jsonwebtoken.svg?branch=master)](https://travis-ci.org/Keats/jsonwebtoken)

### Automotive

* [Altenius/j2534](https://github.com/Altenius/j2534-rs) [[j2534](https://crates.io/crates/j2534)] - J2534 PassThru bindings
* [jmagnuson/canparse](https://github.com/jmagnuson/canparse) [[canparse](https://crates.io/crates/canparse)] - A CAN signal and definition parser [<img src="https://api.travis-ci.org/jmagnuson/canparse.svg?branch=master">](https://travis-ci.org/jmagnuson/canparse)
* [JulianSchmid/dlt_parse](https://github.com/JulianSchmid/dlt-parse-rs) - Rust DLT (Diagnostic Log and Trace) packet parser [<img src="https://api.travis-ci.org/JulianSchmid/dlt-parse-rs.svg?branch=master">](https://travis-ci.org/JulianSchmid/dlt-parse-rs)
* [JulianSchmid/someip_parse](https://github.com/JulianSchmid/someip-parse-rs) [[someip_parse](https://crates.io/crates/someip_parse)] - A library for parsing the SOME/IP network protocol (without payload interpretation) [<img src="https://api.travis-ci.org/JulianSchmid/someip-parse-rs.svg?branch=master">](https://travis-ci.org/JulianSchmid/someip-parse-rs)
* [LibreTuner/tuneutils](https://github.com/LibreTuner/tuneutils) [[tuneutils](https://crates.io/crates/tuneutils)] - Utilities for interfacing with, diagnosing, and tuning car
* [marcelbuesing/can-dbc](https://github.com/marcelbuesing/can-dbc) [[can-dbc](https://crates.io/crates/can-dbc)] - A parser for the DBC format [<img src="https://api.travis-ci.org/marcelbuesing/can-dbc.svg?branch=dev">](https://travis-ci.org/marcelbuesing/can-dbc)
* [mbr/socketcan](https://github.com/mbr/socketcan-rs) [[socketcan](https://crates.io/crates/socketcan)] - Linux SocketCAN library [<img src="https://api.travis-ci.org/mbr/socketcan-rs.svg?branch=master">](https://travis-ci.org/mbr/socketcan-rs)
* [Sensirion/lin-bus](https://github.com/Sensirion/lin-bus-rs) [[lin-bus](https://crates.io/crates/lin-bus)] - LIN bus driver traits and protocol implementation [<img src="https://circleci.com/gh/Sensirion/lin-bus-rs.svg?style=svg">](https://circleci.com/gh/Sensirion/lin-bus-rs)

### Bioinformatics

* [Rust-Bio](https://github.com/rust-bio) — bioinformatics libraries in Rust.

### Caching

* [jaysonsantos/bmemcached-rs](https://github.com/jaysonsantos/bmemcached-rs) — Memcached library written in pure rust [<img src="https://api.travis-ci.org/jaysonsantos/bmemcached-rs.svg?branch=master">](https://travis-ci.org/jaysonsantos/bmemcached-rs)
* [jaemk/cached](https://github.com/jaemk/cached) — Simple function caching/memoization
* [aisk/rust-memcache](https://github.com/aisk/rust-memcache) — Memcached client library [<img src="https://api.travis-ci.org/aisk/rust-memcache.svg?branch=master">](https://travis-ci.org/aisk/rust-memcache)

### Concurrency

* [aymanmadkour/glock](https://github.com/aymanmadkour/glock) – Granular locking crate for Rust. [<img src="https://api.travis-ci.org/aymanmadkour/glock.svg?branch=master">](https://travis-ci.org/aymanmadkour/glock)
* [crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam) – Support for parallelism and low-level concurrency in Rust [<img src="https://api.travis-ci.org/crossbeam-rs/crossbeam.svg?branch=master">](https://travis-ci.org/crossbeam-rs/crossbeam)
* [pop-os/bus-writer](https://github.com/pop-os/bus-writer) — Generic single-reader, multi-writer
* [Rayon](https://github.com/rayon-rs/rayon) – A data parallelism library for Rust [<img src="https://api.travis-ci.org/rayon-rs/rayon.svg?branch=master">](https://travis-ci.org/rayon-rs/rayon)
* [rustcc/coroutine-rs](https://github.com/rustcc/coroutine-rs) – Coroutine Library in Rust [<img src="https://img.shields.io/travis/rustcc/coroutine-rs.svg">](https://travis-ci.org/rustcc/coroutine-rs)
* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) – Coroutine I/O for Rust [<img src="https://api.travis-ci.org/zonyitoo/coio-rs.svg?branch=master">](https://travis-ci.org/zonyitoo/coio-rs)

### Cloud

* AWS [[aws](https://crates.io/keywords/aws)]
  * [rusoto/rusoto](https://github.com/rusoto/rusoto) — [<img src="https://api.travis-ci.org/rusoto/rusoto.svg?branch=master">](https://travis-ci.org/rusoto/rusoto)
* DigitalOcean
  * [kbknapp/doapi](https://github.com/kbknapp/doapi-rs) — DigitalOcean v2 API bindings [<img src="https://api.travis-ci.org/kbknapp/doapi-rs.svg?branch=master">](https://travis-ci.org/kbknapp/doapi-rs)

### Command-line

* Argument parsing
  * [clap-rs](https://github.com/clap-rs/clap) [[clap](https://crates.io/crates/clap)] — a simple to use, full featured command-line argument parser [<img src="https://api.travis-ci.org/clap-rs/clap-rs.svg?branch=master">](https://travis-ci.org/clap-rs/clap-rs)
  * [docopt/docopt.rs](https://github.com/docopt/docopt.rs) [[docopt](https://crates.io/crates/docopt)] — a Rust implementation of [DocOpt](http://docopt.org) [<img src="https://api.travis-ci.org/docopt/docopt.rs.svg?branch=master">](https://travis-ci.org/docopt/docopt.rs)
  * [TeXitoi/structopt](https://github.com/TeXitoi/structopt) [[structopt](https://crates.io/crates/structopt)] — parse command line argument by defining a struct [<img src="https://api.travis-ci.org/TeXitoi/structopt.svg?branch=master">](https://travis-ci.org/TeXitoi/structopt)
  * [killercup/quicli](https://github.com/killercup/quicli) [[quicli](https://crates.io/crates/quicli)] — quickly build cool CLI apps in Rust [<img src="https://api.travis-ci.org/killercup/quicli.svg">](https://travis-ci.org/killercup/quicli)
* Human-centered design
  * [rust-clique/human-panic](https://github.com/rust-clique/human-panic) [[human-panic](https://crates.io/crates/human-panic)] — panic messages for humans [<img src="https://api.travis-ci.org/rust-clique/human-panic.svg">](https://travis-ci.org/rust-clique/human-panic)
* Line editor
  * [srijs/rust-copperline](https://github.com/srijs/rust-copperline) [[copperline](https://crates.io/crates/copperline)] — pure-Rust command line editing library
  * [MovingtoMars/liner](https://github.com/MovingtoMars/liner) [[liner](https://crates.io/crates/liner)] — a library offering readline-like functionality [<img src="https://api.travis-ci.org/MovingtoMars/liner.svg">](https://travis-ci.org/MovingtoMars/liner)
  * [murarth/linefeed](https://github.com/murarth/linefeed) [[linefeed](https://crates.io/crates/linefeed)] — Configurable, extensible, interactive line reader [<img src="https://api.travis-ci.org/murarth/linefeed.svg">](https://travis-ci.org/murarth/linefeed)
  * [kkawakam/rustyline](https://github.com/kkawakam/rustyline) [[rustyline](https://crates.io/crates/rustyline)] — readline implementation in Rust [<img src="https://api.travis-ci.org/kkawakam/rustyline.svg?branch=master">](https://travis-ci.org/kkawakam/rustyline)
* Pipeline
  * [imp/pager-rs](https://gitlab.com/imp/pager-rs) [[pager](https://crates.io/crates/pager)] — pipe your output through an external pager [<img src="https://gitlab.com/imp/pager-rs/badges/master/build.svg">](https://gitlab.com/imp/pager-rs/pipelines)
  * [hniksic/rust-subprocess](https://github.com/hniksic/rust-subprocess) [[subprocess](https://crates.io/crates/subprocess)] — facilities for interaction with external pipelines [<img src="https://api.travis-ci.org/hniksic/rust-subprocess.svg?branch=master">](https://travis-ci.org/hniksic/rust-subprocess)
  * [oconnor663/duct.rs](https://github.com/oconnor663/duct.rs) [[duct](https://crates.io/crates/duct)] — a builder for subprocess pipelines and IO redirection [<img src="https://api.travis-ci.org/oconnor663/duct.rs.svg?branch=master">](https://travis-ci.org/oconnor663/duct.rs)
  * [philippkeller/rexpect](https://github.com/philippkeller/rexpect) [[rexpect](https://crates.io/crates/rexpect)] — automate interactive applications such as ssh, ftp, passwd, etc [<img src="https://api.travis-ci.org/philippkeller/rexpect.svg?branch=master">](https://travis-ci.org/philippkeller/rexpect)
* Progress
  * [mitsuhiko/indicatif](https://github.com/mitsuhiko/indicatif) [[indicatif](https://crates.io/crates/indicatif)] — indicate progress to users
  * [a8m/pb](https://github.com/a8m/pb) [[pbr](https://crates.io/crates/pbr)] — console progress bar for Rust
  * [FGRibreau/spinners](https://github.com/FGRibreau/spinners) [[spinners](https://crates.io/crates/spinners)] — 60+ elegant terminal spinners
* Prompt
  * [hashmismatch/terminal_cli.rs](https://github.com/hashmismatch/terminal_cli.rs) [[terminal_cli](https://crates.io/crates/terminal_cli)]  — build an interactive command prompt [<img src="https://api.travis-ci.org/hashmismatch/terminal_cli.rs.svg?branch=master">](https://travis-ci.org/hashmismatch/terminal_cli.rs)
* Style
  * [ogham/rust-ansi-term](https://github.com/ogham/rust-ansi-term) [[ansi_term](https://crates.io/crates/ansi_term)] — control colours and formatting on ANSI terminals [<img src="https://api.travis-ci.org/ogham/rust-ansi-term.svg?branch=master">](https://travis-ci.org/ogham/rust-ansi-term)
  * [LukasKalbertodt/term-painter](https://github.com/LukasKalbertodt/term-painter) [[term-painter](https://crates.io/crates/term-painter)] — cross-platform styled terminal output [<img src="https://img.shields.io/travis/LukasKalbertodt/term-painter/master.svg">](https://travis-ci.org/LukasKalbertodt/term-painter)
  * [vitiral/termstyle](https://github.com/vitiral/termstyle) [[termstyle](https://docs.rs/termstyle/0.1.2/termstyle/)] — build (and test) formatted and styled command line applications
  * [SergioBenitez/yansi](https://github.com/SergioBenitez/yansi) [[yansi](https://crates.io/crates/yansi)] — a dead simple ANSI terminal color painting library
* TUI
  * [TimonPost/crossterm](https://github.com/TimonPost/crossterm) [[crossterm](https://crates.io/crates/crossterm)] — crossplatform terminal library
  * [gyscos/Cursive](https://github.com/gyscos/Cursive) [[cursive](https://crates.io/crates/cursive)] — build rich TUI applications [<img src="https://api.travis-ci.org/gyscos/Cursive.svg?branch=master">](https://travis-ci.org/gyscos/Cursive)
  * [ogham/rust-term-grid](https://github.com/ogham/rust-term-grid) [[term_grid](https://crates.io/crates/term_grid)] — Rust library for putting things in a grid [<img src="https://api.travis-ci.org/ogham/rust-term-grid.svg?branch=master">](https://travis-ci.org/ogham/rust-term-grid)
  * [ticki/termion](https://github.com/redox-os/termion) [[termion](https://crates.io/crates/termion)] — bindless library for controlling terminals/TTY  [<img src="https://api.travis-ci.org/ticki/termion.svg?branch=master">](https://travis-ci.org/ticki/termion)
  * [fdehau/tui-rs](https://github.com/fdehau/tui-rs) [[tui](https://crates.io/crates/tui)] — a TUI library inspired by [blessed-contrib](https://github.com/yaronn/blessed-contrib) and [termui](https://github.com/gizak/termui) [<img src="https://api.travis-ci.org/fdehau/tui-rs.svg?branch=master">](https://travis-ci.org/fdehau/tui-rs)
  * BearLibTerminal
    * [cfyzium/bearlibterminal](https://github.com/nabijaczleweli/BearLibTerminal.rs) [[bear-lib-terminal](https://crates.io/crates/bear-lib-terminal)] — [BearLibTerminal](https://bitbucket.org/cfyzium/bearlibterminal) bindings [<img src="https://api.travis-ci.org/nabijaczleweli/BearLibTerminal.rs.svg?branch=master">](https://travis-ci.org/nabijaczleweli/BearLibTerminal.rs)
  * ncurses
    * [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) [[ncurses](https://crates.io/crates/ncurses)] — [ncurses](https://www.gnu.org/software/ncurses/) bindings [<img src="https://api.travis-ci.org/jeaye/ncurses-rs.svg?branch=master">](https://travis-ci.org/jeaye/ncurses-rs)
    * [ihalila/pancurses](https://github.com/ihalila/pancurses) [[pancurses](https://crates.io/crates/pancurses)] — curses library, supports linux and windows [<img src="https://api.travis-ci.org/ihalila/pancurses.svg?branch=master">](https://travis-ci.org/ihalila/pancurses)
  * Termbox
    * [gchp/rustbox](https://github.com/gchp/rustbox) [[rustbox](https://crates.io/crates/rustbox)] — bindings to [Termbox](https://github.com/nsf/termbox) [<img src="https://api.travis-ci.org/gchp/rustbox.svg?branch=master">](https://travis-ci.org/gchp/rustbox)

### Compression

* [Brotli](https://opensource.googleblog.com/2015/09/introducing-brotli-new-compression.html)
  * [ende76/brotli-rs](https://github.com/ende76/brotli-rs) — implementation of Brotli compression
  * [dropbox/rust-brotli](https://github.com/dropbox/rust-brotli) — Brotli decompressor in Rust that optionally avoids the stdlib
* bzip2
  * [alexcrichton/bzip2-rs](https://github.com/alexcrichton/bzip2-rs) — [libbz2](http://www.bzip.org) bindings [<img src="https://api.travis-ci.org/alexcrichton/bzip2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/bzip2-rs)
* gzip
  * [carols10cents/zopfli](https://github.com/carols10cents/zopfli) — implementation of the [Zopfli](https://github.com/google/zopfli) compression algorithm for higher quality deflate or zlib compression
* miniz
  * [alexcrichton/flate2-rs](https://github.com/alexcrichton/flate2-rs) — [miniz](https://code.google.com/archive/p/miniz) bindings [<img src="https://api.travis-ci.org/alexcrichton/flate2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/flate2-rs)
* snappy
  * [JeffBelgum/rust-snappy](https://github.com/JeffBelgum/rust-snappy) — [snappy](https://github.com/google/snappy) bindings [<img src="https://api.travis-ci.org/JeffBelgum/rust-snappy.svg?branch=master">](https://travis-ci.org/JeffBelgum/rust-snappy)
* tar
  * [alexcrichton/tar-rs](https://github.com/alexcrichton/tar-rs) — tar archive reading/writing in Rust [<img src="https://api.travis-ci.org/alexcrichton/tar-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/tar-rs)
* zip
  * [mvdnes/zip-rs](https://github.com/mvdnes/zip-rs) — read and write ZIP archives [![Build Status](https://api.travis-ci.org/mvdnes/zip-rs.svg?branch=master)](https://travis-ci.org/mvdnes/zip-rs)

### Computation

* [argmin-rs/argmin](https://github.com/argmin-rs/argmin) [[argmin](https://crates.io/crates/argmin)] — A pure Rust optimization library [<img src="https://api.travis-ci.org/argmin-rs/argmin.svg?branch=master">](https://travis-ci.org/argmin-rs/argmin)
* [BLAS](https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) [[blas](https://crates.io/keywords/blas)]
  * [mikkyang/rust-blas](https://github.com/mikkyang/rust-blas) — BLAS bindings
  * [stainless-steel/blas](https://github.com/blas-lapack-rs/blas) — BLAS bindings [<img src="https://api.travis-ci.org/blas-lapack-rs/blas.svg?branch=master">](https://travis-ci.org/blas-lapack-rs/blas)
* [Conjugate Gradient](https://en.wikipedia.org/wiki/Limited-memory_BFGS)
  * [noshu/cg-sys](https://github.com/noshu/cg-sys) — Rust binding of fortran CG+ subroutine
* [GMP](https://gmplib.org/)
  * [fizyk20/rust-gmp](https://github.com/fizyk20/rust-gmp) — libgmp bindings [<img src="https://api.travis-ci.org/fizyk20/rust-gmp.svg?branch=master">](https://travis-ci.org/fizyk20/rust-gmp)
* [GSL](http://www.gnu.org/software/gsl/)
  * [GuillaumeGomez/rust-GSL](https://github.com/GuillaumeGomez) — GSL bindings [<img src="https://api.travis-ci.org/GuillaumeGomez/rust-GSL.svg?branch=master">](https://travis-ci.org/GuillaumeGomez/rust-GSL)
* [LAPACK](https://en.wikipedia.org/wiki/LAPACK)
  * [stainless-steel/lapack](https://github.com/blas-lapack-rs/lapack) — LAPACK bindings [<img src="https://api.travis-ci.org/blas-lapack-rs/lapack.svg?branch=master">](https://travis-ci.org/blas-lapack-rs/lapack)
* [L-BFGS-B](https://en.wikipedia.org/wiki/Limited-memory_BFGS)
  * [noshu/lbfgsb-sys](https://github.com/noshu/lbfgsb-sys) — Rust binding of fortran L-BFGS-B subroutine
* [rustsim/nalgebra](https://github.com/rustsim/nalgebra) — low-dimensional linear algebra library [<img src="https://api.travis-ci.org/rustsim/nalgebra.svg?branch=master">](https://travis-ci.org/rustsim/nalgebra)
* Parallel
  * [arrayfire/arrayfire-rust](https://github.com/arrayfire/arrayfire-rust) — [Arrayfire](https://arrayfire.com/) bindings
  * [autumnai/collenchyma](https://github.com/autumnai/collenchyma) — An extensible, pluggable, backend-agnostic framework for parallel, high-performance computations on CUDA, OpenCL and common host CPU. [<img src="https://api.travis-ci.org/autumnai/collenchyma.svg?branch=master">](https://travis-ci.org/autumnai/collenchyma)
  * [luqmana/rust-opencl](https://github.com/luqmana/rust-opencl) — [OpenCL](https://www.khronos.org/opencl/) bindings [<img src="https://api.travis-ci.org/luqmana/rust-opencl.svg?branch=master">](https://travis-ci.org/luqmana/rust-opencl)
* Scirust
  * [indigits/scirust](https://github.com/indigits/scirust) — scientific computing library in Rust [![Build Status](https://api.travis-ci.org/indigits/scirust.svg?branch=master)](https://travis-ci.org/indigits/scirust)
* Statrs
  * [boxtown/statrs](https://github.com/boxtown/statrs) — Robust statistical computation library in Rust [![Build Status](https://api.travis-ci.org/boxtown/statrs.svg?branch=master)](https://travis-ci.org/boxtown/statrs)
* Rustimization [[rustimization](https://crates.io/crates/rustimization)]
  * [noshu/rustimization](https://github.com/noshu/rustimization) — A rust optimization library which includes L-BFGS-B and Conjugate Gradient algorithm

### Configuration

* [mehcode/config-rs](https://github.com/mehcode/config-rs) [[config](https://crates.io/crates/config)] — Layered configuration system for Rust applications (with strong support for 12-factor applications). [<img src="https://api.travis-ci.org/mehcode/config-rs.svg?branch=master">](https://travis-ci.org/mehcode/config-rs)
* [Kixunil/configure_me](https://github.com/Kixunil/configure_me) [[configure_me](https://crates.io/crates/configure_me)] - library for processing application configuration easily

### Cryptography

[[crypto](https://crates.io/keywords/crypto), [cryptography](https://crates.io/keywords/cryptography)]

* [briansmith/ring](https://github.com/briansmith/ring) — Safe, fast, small crypto using Rust and BoringSSL's cryptography primitives. [<img src="https://api.travis-ci.org/briansmith/ring.svg?branch=master">](https://travis-ci.org/briansmith/ring)
* [briansmith/webpki](https://github.com/briansmith/webpki) — Web PKI TLS X.509 certificate validation in Rust. [<img src="https://api.travis-ci.org/briansmith/webpki.svg?branch=master">](https://travis-ci.org/briansmith/webpki)
* [brycx/orion](https://github.com/brycx/orion) — This library aims to provide easy and usable crypto. 'Usable' meaning exposing high-level API's that are easy to use and hard to misuse. [<img src="https://api.travis-ci.org/brycx/orion.svg?branch=master">](https://travis-ci.org/brycx/orion)
* [ctz/rustls](https://github.com/ctz/rustls) — a Rust implementation of TLS
* [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) — cryptographic algorithms in Rust [<img src="https://api.travis-ci.org/DaGenix/rust-crypto.svg?branch=master">](https://travis-ci.org/DaGenix/rust-crypto)
* [dnaq/sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) — [libsodium](https://github.com/jedisct1/libsodium) bindings [<img src="https://api.travis-ci.org/dnaq/sodiumoxide.svg?branch=master">](https://travis-ci.org/dnaq/sodiumoxide)
* [doublify/libblockchain](https://github.com/doublify/libblockchain) — A Blockchain implementation [<img src="https://api.travis-ci.org/doublify/libblockchain.svg?branch=master">](https://travis-ci.org/doublify/libblockchain)
* [exonum/exonum](https://github.com/exonum/exonum) [[exonum](https://crates.io/crates/exonum)] — extensible framework for blockchain projects [<img src="https://api.travis-ci.org/exonum/exonum.svg?branch=master">](https://travis-ci.org/exonum/exonum)
* [klutzy/suruga](https://github.com/klutzy/suruga) — a Rust implementation of [TLS 1.2](https://tools.ietf.org/html/rfc5246)
* [libOctavo/octavo](https://github.com/libOctavo/octavo) — Modular hash and crypto library in Rust [<img src="https://api.travis-ci.org/libOctavo/octavo.svg?branch=master">](https://travis-ci.org/libOctavo/octavo)
* [RustCrypto/hashes](https://github.com/RustCrypto/hashes) — Collection of cryptographic hash functions written in pure Rust [<img src="https://api.travis-ci.org/RustCrypto/hashes.svg?branch=master">](https://travis-ci.org/RustCrypto/hashes)
* [rustindia/mpw-rs](https://github.com/rustindia/mpw-rs) — Pure Rust implementation of the Master Password password manager [<img src="https://api.travis-ci.org/rustindia/mpw-rs.svg?branch=master">](https://travis-ci.org/rustindia/mpw-rs)
* [racum/rust-djangohashers](https://github.com/racum/rust-djangohashers) — A Rust port of the password primitives used in the Django Project. It doesn't require Django, only hashes and validates passwords according to its style. [<img src="https://api.travis-ci.org/Racum/rust-djangohashers.svg?branch=master">](https://travis-ci.org/Racum/rust-djangohashers)
* [RNCryptor/rncryptor-rs](https://github.com/RNCryptor/rncryptor-rs) — Pure Rust implementation of the RNCryptor AES file format
* [conradkdotcom/rooster](https://github.com/conradkdotcom/rooster) [[rooster](https://crates.io/crates/rooster)] — Simple password manager to use in your terminal
* [sfackler/rust-native-tls](https://github.com/sfackler/rust-native-tls) — Bindings for native TLS libraries
* [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) — [OpenSSL](https://www.openssl.org/) bindings [<img src="https://api.travis-ci.org/sfackler/rust-openssl.svg?branch=master">](https://travis-ci.org/sfackler/rust-openssl)
* [kornelski/rust-security-framework](https://github.com/kornelski/rust-security-framework) — Bindings for Security Framework (OSX native crypto)
* [steffengy/schannel-rs](https://github.com/steffengy/schannel-rs) — Bindings for Schannel (Windows native TLS)
* [zebradil/rustotpony](https://github.com/zebradil/rustotpony) — CLI manager of one-time password generators aka Google Authenticator (TOTP)

### Database

[[database](https://crates.io/keywords/database)]

* [sfackler/r2d2](https://github.com/sfackler/r2d2) — generic connection pool [<img src="https://api.travis-ci.org/sfackler/r2d2.svg?branch=master">](https://travis-ci.org/sfackler/r2d2)
* NoSQL [[nosql](https://crates.io/keywords/nosql)]
  * [Cassandra](http://cassandra.apache.org) [[cassandra](https://crates.io/keywords/cassandra), [cql](https://crates.io/keywords/cql)]
    * [AlexPikalov/cdrs](https://github.com/AlexPikalov/cdrs) [[cdrs](https://crates.io/crates/cdrs)] — native client written in Rust [<img src="https://api.travis-ci.org/AlexPikalov/cdrs.svg?branch=master">](https://travis-ci.org/AlexPikalov/cdrs)
    * [Metaswitch/cassandra-rs](https://github.com/Metaswitch/cassandra-rs) —  bindings to the DataStax C/C++ client [<img src="https://api.travis-ci.org/Metaswitch/cassandra-rs.svg?branch=master">](https://travis-ci.org/Metaswitch/cassandra-rs)
  * CouchDB [[couchdb](https://crates.io/keywords/couchdb)]
    * [chill-rs/chill](https://github.com/chill-rs/chill) [[couchdb](https://crates.io/crates/chill)] — a Rust client for the CouchDB REST API [<img src="https://api.travis-ci.org/chill-rs/chill.svg?branch=master">](https://travis-ci.org/chill-rs/chill)
  * Elasticsearch [[elasticsearch](https://crates.io/keywords/elasticsearch)]
    * [benashford/rs-es](https://github.com/benashford/rs-es) [[rs-es](https://crates.io/crates/rs-es)] — a Rust client for the [Elastic](https://www.elastic.co/) REST API [<img src="https://api.travis-ci.org/benashford/rs-es.svg?branch=master">](https://travis-ci.org/benashford/rs-es)
    * [elastic-rs/elastic-reqwest](https://github.com/elastic-rs/elastic-reqwest) [[elastic_reqwest](https://crates.io/crates/elastic_reqwest)] — a lightweight implementation of the Elasticsearch API based on Reqwest [<img src="https://api.travis-ci.org/elastic-rs/elastic-reqwest.svg">](https://travis-ci.org/elastic-rs/elastic-reqwest)
  * etcd
    * [jimmycuadra/rust-etcd](https://github.com/jimmycuadra/rust-etcd) [[etcd](https://crates.io/crates/etcd)] — A client library for CoreOS's etcd. [<img src="https://api.travis-ci.org/jimmycuadra/rust-etcd.svg?branch=master">](https://travis-ci.org/jimmycuadra/rust-etcd)
  * ForestDB
    * [vhbit/sherwood](https://github.com/vhbit/sherwood) — [ForestDB](https://github.com/couchbase/forestdb) bindings [<img src="https://api.travis-ci.org/vhbit/sherwood.svg?branch=master">](https://travis-ci.org/vhbit/sherwood)
  * [InfluxDB](https://www.influxdata.com/)
    * [panoptix-za/influxdb-rs](https://github.com/panoptix-za/influxdb-rs) — asynchronous interface [<img src="https://api.travis-ci.org/panoptix-za/influxdb-rs.svg?branch=master">](https://travis-ci.org/panoptix-za/influxdb-rs)
    * [driftluo/InfluxDBClient-rs](https://github.com/driftluo/InfluxDBClient-rs) — Synchronization interface [<img src="https://api.travis-ci.org/driftluo/InfluxDBClient-rs.svg?branch=master">](https://travis-ci.org/driftluo/InfluxDBClient-rs)
  * LevelDB
    * [skade/leveldb](https://github.com/skade/leveldb) — [LevelDB](https://github.com/google/leveldb) bindings [<img src="https://api.travis-ci.org/skade/leveldb.svg?branch=master">](https://travis-ci.org/skade/leveldb)
  * LMDB [[lmdb](https://crates.io/keywords/lmdb)]
    * [vhbit/lmdb-rs](https://github.com/vhbit/lmdb-rs) [[lmdb-rs](https://crates.io/crates/lmdb-rs)] — [LMDB](https://symas.com/lmdb/) bindings [<img src="https://api.travis-ci.org/vhbit/lmdb-rs.svg?branch=master">](https://travis-ci.org/vhbit/lmdb-rs)
  * MongoDB [[mongodb](https://crates.io/keywords/mongodb)]
    * [mongodb-labs/mongo-rust-driver-prototype](https://github.com/mongodb-labs/mongo-rust-driver-prototype) [[mongodb](https://crates.io/crates/mongodb)] — [MongoDB](https://www.mongodb.com/) bindings [<img src="https://api.travis-ci.org/mongodb-labs/mongo-rust-driver-prototype.svg">](https://travis-ci.org/mongodb-labs/mongo-rust-driver-prototype)
  * Neo4j [[cypher](https://crates.io/keywords/cypher), [neo4j](https://crates.io/keywords/neo4j)]
  * Redis [[redis](https://crates.io/keywords/redis)]
    * [mitsuhiko/redis-rs](https://github.com/mitsuhiko/redis-rs) — [Redis](https://redis.io/) library in Rust [<img src="https://api.travis-ci.org/mitsuhiko/redis-rs.svg?branch=master">](https://travis-ci.org/mitsuhiko/redis-rs)
  * [RocksDB](https://rocksdb.org/)
    * [rust-rocksdb/rust-rocksdb](https://github.com/rust-rocksdb/rust-rocksdb) — RocksDB bindings [<img src="https://api.travis-ci.org/rust-rocksdb/rust-rocksdb.svg?branch=master">](https://travis-ci.org/rust-rocksdb/rust-rocksdb)
  * [UnQLite](https://unqlite.org/)
    * [zitsen/unqlite.rs](https://github.com/zitsen/unqlite.rs) — UnQLite bindings [<img src="https://api.travis-ci.org/zitsen/unqlite.rs.svg?branch=master">](https://travis-ci.org/zitsen/unqlite.rs)
  * [ZooKeeper](https://zookeeper.apache.org/)
    * [bonifaido/rust-zookeeper](https://github.com/bonifaido/rust-zookeeper) [[zookeeper](https://crates.io/crates/zookeeper)] — A client library for Apache ZooKeeper. [<img src="https://api.travis-ci.org/bonifaido/rust-zookeeper.svg?branch=master">](https://travis-ci.org/bonifaido/rust-zookeeper)
* SQL [[sql](https://crates.io/keywords/sql)]
  * Microsoft SQL
    * [steffengy/tiberius](https://github.com/steffengy/tiberius) — [<img src="https://api.travis-ci.org/steffengy/tiberius.svg?branch=master">](https://travis-ci.org/steffengy/tiberius)
  * MySql [[mysql](https://crates.io/keywords/mysql)]
    * [AgilData/mysql-proxy-rs](https://github.com/AgilData/mysql-proxy-rs) — a MySQL Proxy [<img src="https://api.travis-ci.org/AgilData/mysql-proxy-rs.svg?branch=master">](https://travis-ci.org/AgilData/mysql-proxy-rs)
    * [blackbeam/mysql_async](https://github.com/blackbeam/mysql_async) [[mysql_async](https://crates.io/crates/mysql_async)] — asyncronous Rust Mysql driver based on Tokio. [<img src="https://api.travis-ci.org/blackbeam/mysql_async.svg?branch=master">](https://travis-ci.org/blackbeam/mysql_async)
    * [blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) [[mysql](https://crates.io/crates/mysql)] — a native MySql client [<img src="https://api.travis-ci.org/blackbeam/rust-mysql-simple.svg?branch=master">](https://travis-ci.org/blackbeam/rust-mysql-simple)
  * PostgreSql [[postgres](https://crates.io/keywords/postgres), [postgresql](https://crates.io/keywords/postgresql)]
    * [sfackler/rust-postgres](https://github.com/sfackler/rust-postgres) [[postgres](https://crates.io/crates/postgres)] — a native [PostgreSQL](https://www.postgresql.org/) client [<img src="https://api.travis-ci.org/sfackler/rust-postgres.svg?branch=master">](https://travis-ci.org/sfackler/rust-postgres)
  * Sqlite [[sqlite](https://crates.io/keywords/sqlite)]
    * [jgallagher/rusqlite](https://github.com/jgallagher/rusqlite) — [Sqlite3](https://www.sqlite.org/index.html) bindings [<img src="https://api.travis-ci.org/jgallagher/rusqlite.svg?branch=master">](https://travis-ci.org/jgallagher/rusqlite)
* ORM [[orm](https://crates.io/keywords/orm)]
  * [diesel-rs/diesel](https://github.com/diesel-rs/diesel) — an ORM and Query builder for Rust [![Build Status](https://api.travis-ci.org/diesel-rs/diesel.svg)](https://travis-ci.org/diesel-rs/diesel)
  * [ivanceras/rustorm](https://github.com/ivanceras/rustorm) — an ORM for Rust [![Build Status](https://api.travis-ci.org/ivanceras/rustorm.svg)](https://travis-ci.org/ivanceras/rustorm)

### Data processing

* [bluss/ndarray](https://github.com/rust-ndarray/ndarray) — N-dimensional array with array views, multidimensional slicing, and efficient operations
* [kernelmachine/utah](https://github.com/kernelmachine/utah) — Dataframe structure and operations in Rust
* [weld-project/weld](https://github.com/weld-project/weld) — High-performance runtime for data analytics applications

### Data structures

* [bluss/rust-itertools](https://github.com/bluss/rust-itertools) — [<img src="https://api.travis-ci.org/bluss/rust-itertools.svg?branch=master">](https://travis-ci.org/bluss/rust-itertools)
* [contain-rs](https://github.com/contain-rs) — Extension of Rust's std::collections
* [danielpclark/array_tool](https://github.com/danielpclark/array_tool) — Array helpers for Rust. Some of the most common methods you would use on Arrays made available on Vectors. Polymorphic implementations for handling most of your use cases. [<img src="https://api.travis-ci.org/danielpclark/array_tool.svg?branch=master">](https://travis-ci.org/danielpclark/array_tool)
* [fizyk20/generic-array](https://github.com/fizyk20/generic-array) – a hack to allow for arrays sized by typenums [<img src="https://api.travis-ci.org/fizyk20/generic-array.svg?branch=master">](https://travis-ci.org/fizyk20/generic-array)
* [garro95/priority-queue](https://github.com/garro95/priority-queue)[[priority-queue](https://crates.io/crates/priority-queue)] — A priority queue that implements priority changes. [<img src="https://api.travis-ci.org/garro95/priority-queue.svg?branch=master">](https://travis-ci.org/garro95/priority-queue)
* [Nemo157/roaring-rs](https://github.com/Nemo157/roaring-rs) – Roaring Bitmaps in Rust [<img src="https://api.travis-ci.org/Nemo157/roaring-rs.svg?branch=master">](https://travis-ci.org/Nemo157/roaring-rs)
* [orium/rpds](https://github.com/orium/rpds) [[rpds](https://crates.io/crates/rpds)] — Persistent data structures in Rust. [<img src="https://api.travis-ci.org/orium/rpds.svg?branch=master">](https://travis-ci.org/orium/rpds)
* [pop-os/progress-streams](https://github.com/pop-os/progress-streams) — Progress callbacks for types which implement `dyn io::Read` or `dyn io::Write`.
* [reem/rust-typemap](https://github.com/reem/rust-typemap) — [<img src="https://api.travis-ci.org/reem/rust-typemap.svg?branch=master">](https://travis-ci.org/reem/rust-typemap)
* [xfix/enum-map](https://github.com/xfix/enum-map) [[enum-map](https://crates.io/crates/enum-map)] — An optimized map implementation for enums using an array to store values. [<img src="https://api.travis-ci.org/xfix/enum-map.svg?branch=master">](https://travis-ci.org/xfix/enum-map)

### Data visualization

* [saresend/gust](https://github.com/saresend/Gust) — [<img src="https://api.travis-ci.org/saresend/Gust.svg?branch=master">](https://travis-ci.org/saresend/Gust)

### Date and time

[[date](https://crates.io/keywords/date), [time](https://crates.io/keywords/time)]

* [chronotope/chrono](https://github.com/chronotope/chrono) — [<img src="https://api.travis-ci.org/chronotope/chrono.svg?branch=master">](https://travis-ci.org/chronotope/chrono)
* [yaa110/rust-persian-calendar](https://github.com/yaa110/rust-persian-calendar) — [<img src="https://api.travis-ci.org/yaa110/rust-persian-calendar.svg?branch=master">](https://travis-ci.org/yaa110/rust-persian-calendar)

### Distributed systems

* Antimony
  * [antimonyproject/antimony](https://github.com/antimonyproject/antimony) [[antimony](https://crates.io/crates/antimony)] — stream processing / distributed computation platform [<img src="https://api.travis-ci.org/antimonyproject/antimony.svg?branch=master">](https://travis-ci.org/antimonyproject/antimony)
* Apache Hadoop
  * [whitfin/efflux](https://github.com/whitfin/efflux) — Easy Hadoop Streaming and MapReduce interfaces in Rust.
* Apache Kafka
  * [fede1024/rust-rdkafka](https://github.com/fede1024/rust-rdkafka) [[rdkafka](https://crates.io/crates/rdkafka)] — [librdkafka](https://github.com/edenhill/librdkafka) bindings [<img src="https://api.travis-ci.org/fede1024/rust-rdkafka.svg?branch=master">](https://travis-ci.org/fede1024/rust-rdkafka)
  * [spicavigo/kafka-rust](https://github.com/spicavigo/kafka-rust) — [<img src="https://api.travis-ci.org/spicavigo/kafka-rust.svg?branch=master">](https://travis-ci.org/spicavigo/kafka-rust)
* Beanstalkd
  * [schickling/rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) — [Beanstalkd](https://github.com/beanstalkd/beanstalkd) bindings [<img src="https://api.travis-ci.org/schickling/rust-beanstalkd.svg?branch=master">](https://travis-ci.org/schickling/rust-beanstalkd)
* HDFS
  * [hyunsik/hdfs-rs](https://github.com/hyunsik/hdfs-rs) — libhdfs bindings [<img src="https://api.travis-ci.org/hyunsik/hdfs-rs.svg?branch=master">](https://travis-ci.org/hyunsik/hdfs-rs)

### Email

[[email](https://crates.io/keywords/email), [imap](https://crates.io/keywords/imap), [smtp](https://crates.io/keywords/smtp)]

* [GildedHonour/atarashii_imap](https://github.com/GildedHonour/atarashii_imap) — 新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections [<img src="https://api.travis-ci.org/GildedHonour/atarashii_imap.svg?branch=master">](https://travis-ci.org/GildedHonour/atarashii_imap)
* [gsquire/sendgrid-rs](https://github.com/gsquire/sendgrid-rs) — unofficial Rust library for SendGrid API [<img src="https://api.travis-ci.org/gsquire/sendgrid-rs.svg?branch=master">](https://travis-ci.org/gsquire/sendgrid-rs)
* [lettre/lettre](https://github.com/lettre/lettre) — an SMTP-library for Rust [<img src="https://api.travis-ci.org/lettre/lettre.svg?branch=master">](https://travis-ci.org/lettre/lettre)
* [staktrace/mailparse](https://github.com/staktrace/mailparse) [[mailparse](https://crates.io/crates/mailparse)] — a library for parsing real-world email files [<img src="https://api.travis-ci.org/staktrace/mailparse.svg?branch=master">](https://travis-ci.org/staktrace/mailparse)

### Encoding

[[encoding](https://crates.io/keywords/encoding)]

* ASN.1
  * [alex/rust-asn1](https://github.com/alex/rust-asn1) — a Rust ASN.1 (DER) serializer [<img src="https://api.travis-ci.org/alex/rust-asn1.svg?branch=master">](https://travis-ci.org/alex/rust-asn1)
* Bencode
  * [arjantop/rust-bencode](https://github.com/arjantop/rust-bencode) — [Bencode](https://en.wikipedia.org/wiki/Bencode) implementation in Rust [<img src="https://api.travis-ci.org/arjantop/rust-bencode.svg?branch=master">](https://travis-ci.org/arjantop/rust-bencode)
* Binary
  * [arcnmx/nue](https://github.com/arcnmx/nue) — I/O and binary data encoding for Rust [<img src="https://api.travis-ci.org/arcnmx/nue.svg?branch=master">](https://travis-ci.org/arcnmx/nue)
  * [TyOverby/bincode](https://github.com/TyOverby/bincode) — a binary encoder/decoder in Rust [<img src="https://api.travis-ci.org/TyOverby/bincode.svg?branch=master">](https://travis-ci.org/TyOverby/bincode)
  * [m4b/goblin](https://github.com/m4b/goblin) [[goblin](https://crates.io/crates/goblin)] —  cross-platform, zero-copy, and endian-aware binary parsing [<img src="https://api.travis-ci.org/m4b/goblin.svg?branch=master">](https://travis-ci.org/m4b/goblin)
* BSON
  * [zonyitoo/bson-rs](https://github.com/zonyitoo/bson-rs) — [<img src="https://api.travis-ci.org/zonyitoo/bson-rs.svg?branch=master">](https://travis-ci.org/zonyitoo/bson-rs)
* Byte swapping
  * [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder) — Supports big-endian, little-endian and native byte orders [<img src="https://api.travis-ci.org/BurntSushi/byteorder.svg?branch=master">](https://travis-ci.org/BurntSushi/byteorder)
* Cap'n Proto
  * [capnproto/capnproto-rust](https://github.com/capnproto/capnproto-rust) — [<img src="https://api.travis-ci.org/capnproto/capnproto-rust.svg?branch=master">](https://travis-ci.org/capnproto/capnproto-rust)
* CBOR
  * [BurntSushi/rust-cbor](https://github.com/BurntSushi/rust-cbor) — Supports JSON conversion and type-based encoding/decoding [<img src="https://api.travis-ci.org/BurntSushi/rust-cbor.svg?branch=master">](https://travis-ci.org/BurntSushi/rust-cbor)
* Character Encoding
  * [hsivonen/encoding_rs](https://github.com/hsivonen/encoding_rs) [[encoding_rs](https://crates.io/crates/encoding_rs)] — A Gecko-oriented implementation of the Encoding Standard in Rust [<img src="https://api.travis-ci.org/hsivonen/encoding_rs.svg?branch=master">](https://travis-ci.org/hsivonen/encoding_rs)
  * [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding) — [<img src="https://api.travis-ci.org/lifthrasiir/rust-encoding.svg?branch=master">](https://travis-ci.org/lifthrasiir/rust-encoding)
* CRC
  * [mrhooray/crc-rs](https://github.com/mrhooray/crc-rs) — [<img src="https://api.travis-ci.org/mrhooray/crc-rs.svg?branch=master">](https://travis-ci.org/mrhooray/crc-rs)
* CSV
  * [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) — [<img src="https://api.travis-ci.org/BurntSushi/rust-csv.svg?branch=master">](https://travis-ci.org/BurntSushi/rust-csv)
* HAR
  * [mandrean/har-rs](https://github.com/mandrean/har-rs) — A HTTP Archive Format (HAR) serialization & deserialization library [![Build Status](https://api.travis-ci.org/mandrean/har-rs.svg?branch=master)](https://travis-ci.org/mandrean/har-rs)
* HTML
  * [servo/html5ever](https://github.com/servo/html5ever) — High-performance browser-grade HTML5 parser [<img src="https://api.travis-ci.org/servo/html5ever.svg?branch=master">](https://travis-ci.org/servo/html5ever)
  * [veddan/rust-htmlescape](https://github.com/veddan/rust-htmlescape) — encoding/decoding HTML entities [<img src="https://api.travis-ci.org/veddan/rust-htmlescape.svg?branch=master">](https://travis-ci.org/veddan/rust-htmlescape)
* JSON
  * [pikkr/pikkr](https://github.com/pikkr/pikkr) [[pikkr](https://crates.io/crates/pikkr)] — JSON parser which picks up values directly without performing tokenization in Rust
  * [serde-rs/json](https://github.com/serde-rs/json) [[serde\_json](https://crates.io/crates/serde_json)] — JSON support for [Serde](https://github.com/serde-rs/serde) framework [<img src="https://api.travis-ci.org/serde-rs/json.svg?branch=master">](https://travis-ci.org/serde-rs/json)
  * [maciejhirsz/json-rust](https://github.com/maciejhirsz/json-rust) [[json](https://crates.io/crates/json)] — JSON implementation in Rust [<img src="https://api.travis-ci.org/maciejhirsz/json-rust.svg?branch=master">](https://travis-ci.org/maciejhirsz/json-rust)
* Jsonnet
  * [Qihoo360/rust-jsonnet](https://github.com/Qihoo360/rust-jsonnet) —  [<img src="https://api.travis-ci.org/Qihoo360/rust-jsonnet.svg?branch=master">](https://travis-ci.org/Qihoo360/rust-jsonnet)
* MsgPack
  * [3Hren/msgpack-rust](https://github.com/3Hren/msgpack-rust) — a pure Rust low/high level MessagePack implementation [<img src="https://api.travis-ci.org/3Hren/msgpack-rust.svg?branch=master">](https://travis-ci.org/3Hren/msgpack-rust)
* PEM
  * [jcreekmore/pem-rs](https://github.com/jcreekmore/pem-rs) [[pem](https://crates.io/crates/pem)] — a Rust based way to parse and encode PEM-encoded data [<img src="https://api.travis-ci.org/jcreekmore/pem-rs.svg?branch=master">](https://travis-ci.org/jcreekmore/pem-rs)
* Postman Collection
  * [mandrean/postman-collection-rs](https://github.com/mandrean/postman-collection-rs) — A Postman Collection v1, v2 & v2.1 serialization & deserialization library [![Build Status](https://api.travis-ci.org/mandrean/postman-collection-rs.svg?branch=master)](https://travis-ci.org/mandrean/postman-collection-rs)
* ProtocolBuffers
  * [danburkert/prost](https://github.com/danburkert/prost) — [<img src="https://api.travis-ci.org/danburkert/prost.svg?branch=master">](https://travis-ci.org/danburkert/prost)
  * [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf) — [<img src="https://api.travis-ci.org/stepancheg/rust-protobuf.svg?branch=master">](https://travis-ci.org/stepancheg/rust-protobuf)
* RON (Rusty Object Notation)
  * [https://github.com/ron-rs/ron](https://github.com/ron-rs/ron) — [<img src="https://api.travis-ci.org/ron-rs/ron.svg?branch=master">](https://travis-ci.org/https://github.com/ron-rs/ron)
* Tnetstring
  * [erickt/rust-tnetstring](https://github.com/erickt/rust-tnetstring) — [<img src="https://api.travis-ci.org/erickt/rust-tnetstring.svg?branch=master">](https://travis-ci.org/erickt/rust-tnetstring)
* TOML
  * [alexcrichton/toml-rs](https://github.com/alexcrichton/toml-rs) — [<img src="https://api.travis-ci.org/alexcrichton/toml-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/toml-rs)
* XML
  * [Florob/RustyXML](https://github.com/Florob/RustyXML) — an XML parser written in Rust [<img src="https://api.travis-ci.org/Florob/RustyXML.svg?branch=master">](https://travis-ci.org/Florob/RustyXML)
  * [shepmaster/sxd-document](https://github.com/shepmaster/sxd-document) — An XML library in Rust [<img src="https://api.travis-ci.org/shepmaster/sxd-document.svg?branch=master">](https://travis-ci.org/shepmaster/sxd-document)
  * [shepmaster/sxd-xpath](https://github.com/shepmaster/sxd-xpath) — An XPath library in Rust [<img src="https://api.travis-ci.org/shepmaster/sxd-xpath.svg?branch=master">](https://travis-ci.org/shepmaster/sxd-xpath)
  * [netvl/xml-rs](https://github.com/netvl/xml-rs) — a streaming XML library [<img src="https://api.travis-ci.org/netvl/xml-rs.svg?branch=master">](https://travis-ci.org/netvl/xml-rs)
  * [media-io/yaserde](https://github.com/media-io/yaserde) — Yet Another Serializer/Deserializer specialized for XML [<img src="https://api.travis-ci.org/media-io/yaserde.svg?branch=master">](https://travis-ci.org/media-io/yaserde)
* YAML
  * [chyh1990/yaml-rust](https://github.com/chyh1990/yaml-rust) — The missing YAML 1.2 implementation for Rust. [<img src="https://api.travis-ci.org/chyh1990/yaml-rust.svg?branch=master">](https://travis-ci.org/chyh1990/yaml-rust)
  * [dtolnay/serde-yaml](https://github.com/dtolnay/serde-yaml) [[serde\_yaml](https://crates.io/crates/serde_yaml)] — YAML support for [Serde](https://github.com/serde-rs/serde) framework [<img src="https://api.travis-ci.org/dtolnay/serde-yaml.svg?branch=master">](https://travis-ci.org/dtolnay/serde-yaml)
  * [kimhyunkang/libyaml-rust](https://github.com/kimhyunkang/libyaml-rust) — [libyaml](https://pyyaml.org/wiki/LibYAML) bindings [<img src="https://api.travis-ci.org/kimhyunkang/libyaml-rust.svg?branch=master">](https://travis-ci.org/kimhyunkang/libyaml-rust)
  * [vitiral/stfu8](https://github.com/vitiral/stfu8) — Sorta Text Format in UTF-8 [<img src="https://api.travis-ci.org/vitiral/stfu8.svg?branch=master">](https://travis-ci.org/vitiral/stfu8)

### Filesystem

[[filesystem](https://crates.io/keywords/filesystem)]
* Operations
  * [pop-os/dbus-udisks2](https://github.com/pop-os/dbus-udisks2) -> UDisks2 DBus API
  * [pop-os/sys-mount](https://github.com/pop-os/sys-mount) — High level abstraction for the `mount` / `umount2` system calls.
  * [vitiral/path_abs](https://github.com/vitiral/path_abs) — Absolute serializable path types and associated methods. [<img src="https://api.travis-ci.org/vitiral/path_abs.svg?branch=master">](https://travis-ci.org/webdesus/fs_extr://travis-ci.org/vitiral/path_abs)
  * [webdesus/fs_extra](https://github.com/webdesus/fs_extra) — expanding opportunities standard library std::fs and std::io [<img src="https://api.travis-ci.org/webdesus/fs_extra.svg?branch=master">](https://travis-ci.org/webdesus/fs_extra)
* Temporary Files
  * [rust-lang-deprecated/tempdir](https://github.com/rust-lang-deprecated/tempdir) — temporary directory library [<img src="https://api.travis-ci.org/rust-lang-nursery/tempdir.svg?branch=master">](https://travis-ci.org/rust-lang-nursery/tempdir)
  * [Stebalien/tempfile](https://github.com/Stebalien/tempfile) — temporary file library [<img src="https://api.travis-ci.org/Stebalien/tempfile.svg?branch=master">](https://travis-ci.org/Stebalien/tempfile)
  * [Stebalien/xattr](https://github.com/Stebalien/xattr) [[xattr](https://crates.io/crates/xattr)] — list and manipulate unix extended file attributes [<img src="https://api.travis-ci.org/Stebalien/xattr.svg?branch=master">](https://travis-ci.org/Stebalien/xattr)
  * [zboxfs/zbox](https://github.com/zboxfs/zbox) [[zbox](https://crates.io/crates/zbox)] — Zero-details, privacy-focused embeddable file system. [<img src="https://api.travis-ci.org/zboxfs/zbox.svg?branch=master">](https://travis-ci.org/zboxfs/zbox)

### Game development

See also [Are we game yet?](http://arewegameyet.com)
* Allegro
  * [SiegeLord/RustAllegro](https://github.com/SiegeLord/RustAllegro) — [Allegro 5](https://liballeg.org/) bindings [<img src="https://api.travis-ci.org/SiegeLord/RustAllegro.svg?branch=master">](https://travis-ci.org/SiegeLord/RustAllegro)
* Challonge
  * [vityafx/challonge-rs](https://github.com/vityafx/challonge-rs) [[challonge](https://crates.io/crates/challonge)] — Client library for the Challonge REST API. Helps to organize tournaments. [<img src="https://api.travis-ci.org/vityafx/challonge-rs.svg?branch=master">](https://travis-ci.org/vityafx/challonge-rs)
* Corange
  * [lucidscape/corange-rs](https://github.com/lucidscape/corange-rs) — [Corange](https://github.com/orangeduck/Corange) bindings
* Entity-Component Systems (ECS)
  * [slide-rs/specs](https://github.com/slide-rs/specs) — Specs Parallel ECS [<img src="https://api.travis-ci.org/slide-rs/specs.svg">](httpsL//github.com/travis-ci.org/slide-rs/specs)
* Game Engines
  * [Amethyst](https://www.amethyst.rs) — Data-oriented game engine [<img src="https://api.travis-ci.org/amethyst/amethyst.svg?branch=master">](https://travis-ci.org/amethyst/amethyst)
  * [Piston](https://www.piston.rs/) — [<img src="https://api.travis-ci.org/PistonDevelopers/piston.svg?branch=master">](https://travis-ci.org/PistonDevelopers/piston)
  * [ggez](https://github.com/ggez/ggez) — a lightweight game framework for making 2D games with minimum friction [<img src="https://api.travis-ci.org/ggez/ggez.svg?branch=master">](https://travis-ci.org/ggez/ggez)
  * [Kiss3D](http://kiss3d.org/) — a Keep It Simple, Stupid 3d graphics engine written with Rust
  * [Unrust](https://github.com/unrust/unrust) — unrust — A pure rust based (webgl 2.0 / native) game engine
  * [Vulkust](https://github.com/Hossein-Noroozpour/vulkust) — Vulkust — A safe, highly multithreaded, Vulkan based game engine written with Rust.
* [SDL](http://www.libsdl.org/) [[sdl](https://crates.io/keywords/sdl)]
  * [brson/rust-sdl](https://github.com/brson/rust-sdl) — SDL1 bindings [<img src="https://api.travis-ci.org/brson/rust-sdl.svg?branch=master">](https://travis-ci.org/brson/rust-sdl)
  * [Rust-SDL2/rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) — SDL2 bindings [<img src="https://api.travis-ci.org/Rust-SDL2/rust-sdl2.svg?branch=master">](https://travis-ci.org/Rust-SDL2/rust-sdl2)
* SFML
  * [jeremyletang/rust-sfml](https://github.com/jeremyletang/rust-sfml) — [SFML](https://www.sfml-dev.org/) bindings [<img src="https://api.travis-ci.org/jeremyletang/rust-sfml.svg?branch=master">](https://travis-ci.org/jeremyletang/rust-sfml)
* Tcod-rs
  * [tomassedovic/tcod-rs](https://github.com/tomassedovic/tcod-rs) — Libtcod bindings for Rust.
* Victorem
  * [VictoremWinbringer/Victorem](https://github.com/VictoremWinbringer/Victorem) [[Victorem](https://crates.io/crates/Victorem)] — Easy UDP Game Server and UDP Client framework for creating simple 2D and 3D online game prototype [<img src="https://api.travis-ci.org/VictoremWinbringer/Victorem.svg?branch=master">](https://travis-ci.org/VictoremWinbringer/Victorem)
* Voxlap
  * [bbodi/rust-voxlap](https://github.com/bbodi/rust-voxlap) — [Voxlap](http://advsys.net/ken/voxlap.htm) bindings

### Geospatial

[[geo](https://crates.io/keywords/geo), [gis](https://crates.io/keywords/gis)]

* [DaveKram/coord_transforms](https://github.com/DaveKram/coord_transforms) [[coord_transforms](https://crates.io/crates/coord_transforms)] — coordinate transformations (2-d, 3-d, and geospatial) [<img src="https://api.travis-ci.org/DaveKram/coord_transforms.svg?branch=master">](https://travis-ci.org/DaveKram/coord_transforms)
* [vlopes11/geomorph](https://github.com/vlopes11/geomorph) [[geomorph](https://crates.io/crates/geomorph)] — conversion between UTM, LatLon and MGRS coordinates [<img src="https://api.travis-ci.org/vlopes11/geomorph.svg?branch=master">](https://travis-ci.org/vlopes11/geomorph)
* [Georust](https://github.com/georust) — geospatial tools and libraries written in Rust
* [rust-reverse-geocoder](https://github.com/llambda/rrgeo) — a fast, offline reverse geocoder in Rust, inspired by https://github.com/thampiman/reverse-geocoder

### Graphics

[[graphics](https://crates.io/keywords/graphics)]

* [gfx-rs/gfx](https://github.com/gfx-rs/gfx) — A high-performance, bindless graphics API for Rust. [<img src="https://img.shields.io/travis/gfx-rs/gfx/master.svg">](https://travis-ci.org/gfx-rs/gfx)
* Font
  * [redox-os/rusttype](https://github.com/redox-os/rusttype) — a pure Rust alternative to libraries like FreeType [<img src="https://img.shields.io/travis/dredox-os/rusttype/master.svg">](https://travis-ci.org/redox-os/rusttype)
* OpenGL [[opengl](https://crates.io/keywords/opengl)]
  * [brendanzab/gl-rs](https://github.com/brendanzab/gl-rs) — [<img src="https://api.travis-ci.org/brendanzab/gl-rs.svg?branch=master">](https://travis-ci.org/brendanzab/gl-rs)
  * [glium/glium](https://github.com/glium/glium) — safe OpenGL wrapper for the Rust language. [<img src="https://api.travis-ci.org/glium/glium.svg?branch=master">](https://travis-ci.org/glium/glium)
  * [Kiss3d](http://kiss3d.org) — draw simple geometric figures and play with them with one-liners [<img src="https://api.travis-ci.org/repositories/sebcrozet/kiss3d.json.svg?branch=master">](https://api.travis-ci.org/repositories/sebcrozet/kiss3d.json)
  * [PistonDevelopers/glfw-rs](https://github.com/PistonDevelopers/glfw-rs) — [<img src="https://api.travis-ci.org/PistonDevelopers/glfw-rs.svg?branch=master">](https://travis-ci.org/PistonDevelopers/glfw-rs)
  * [tomaka/glutin](https://github.com/tomaka/glutin) — Rust alternative to [GLFW](https://www.glfw.org/) [<img src="https://api.travis-ci.org/tomaka/glutin.svg?branch=master">](https://travis-ci.org/tomaka/glutin)
* PDF
  * [kaj/rust-pdf](https://github.com/kaj/rust-pdf) — [<img src="https://api.travis-ci.org/kaj/rust-pdf.svg?branch=master">](https://travis-ci.org/kaj/rust-pdf)
  * [fschutt/printpdf](https://github.com/fschutt/printpdf) — PDF writing library [<img src="https://api.travis-ci.org/fschutt/printpdf.svg?branch=master">](https://travis-ci.org/fschutt/printpdf)
  * [J-F-Liu/lopdf](https://github.com/J-F-Liu/lopdf) — PDF document manipulation [<img src="https://api.travis-ci.org/J-F-Liu/lopdf.svg?branch=master">](https://travis-ci.org/J-F-Liu/lopdf)
* [Vulkan](https://www.khronos.org/vulkan/) [[vulkan](https://crates.io/keywords/vulkan)]
  * [vulkano](https://github.com/vulkano-rs/vulkano) [[vulkano](https://crates.io/crates/vulkano)] — [<img src="https://api.travis-ci.org/vulkano-rs/vulkano.svg?branch=master">](https://travis-ci.org/vulkano-rs/vulkano)

### Graph processing

* [kud1ing/tinkerpop-rs](https://github.com/kud1ing/tinkerpop-rs) — an example how to use Apache TinkerPop from Rust [<img src="https://api.travis-ci.org/kud1ing/tinkerpop-rs.svg?branch=master">](https://travis-ci.org/kud1ing/tinkerpop-rs)

### GUI

[[gui](https://crates.io/keywords/gui)]

* [autopilot-rs/autopilot-rs](https://github.com/autopilot-rs/autopilot-rs) — A simple, cross-platform GUI automation library for Rust.
* [maps4print/azul](https://github.com/maps4print/azul) — A free, functional, IMGUI-oriented GUI framework for rapid development of desktop applications written in Rust, supported by the Mozilla WebRender rendering engine. [<img src="https://api.travis-ci.org/maps4print/azul.svg?branch=master">](https://travis-ci.org/maps4print/azul)
* [PistonDevelopers/conrod](https://github.com/PistonDevelopers/conrod/) — An easy-to-use, immediate-mode, 2D GUI library written entirely in Rust [<img src="https://api.travis-ci.org/PistonDevelopers/conrod.svg?branch=master">](https://travis-ci.org/PistonDevelopers/conrod)
* [rise-ui](https://github.com/rise-ui/rise) - Simple component-based cross-Platform GUI Toolkit for developing beautiful and user-friendly interfaces.


* Cocoa
  * [kylewlacy/sorbet-cocoa](https://github.com/kylewlacy/sorbet-cocoa) — [<img src="https://api.travis-ci.org/kylewlacy/sorbet-cocoa.svg?branch=master">](https://travis-ci.org/kylewlacy/sorbet-cocoa)
  * [servo/core-foundation-rs](https://github.com/servo/core-foundation-rs) — [<img src="https://api.travis-ci.org/servo/core-foundation-rs.svg?branch=master">](https://travis-ci.org/servo/core-foundation-rs)
* [GTK+](https://www.gtk.org/) [[gtk](https://crates.io/keywords/gtk)]
  * [gtk-rs/gtk](https://github.com/gtk-rs/gtk) — GTK+ bindings [<img src="https://api.travis-ci.org/gtk-rs/gtk.svg?branch=master">](https://travis-ci.org/gtk-rs/gtk)
  * [relm](https://github.com/antoyo/relm) — Asynchronous, GTK+-based, GUI library, inspired by Elm [<img src="https://api.travis-ci.org/antoyo/relm.svg?branch=master">](https://travis-ci.org/antoyo/relm)
* [ImGui](https://github.com/ocornut/imgui)
  * [imgui-rs](https://github.com/Gekkio/imgui-rs) — Rust bindings for ImGui [<img src="https://api.travis-ci.org/Gekkio/imgui-rs.svg?branch=master">](https://travis-ci.org/Gekkio/imgui-rs)
* [IUP](http://webserver2.tecgraf.puc-rio.br/iup/)
  * [clear-coat](https://github.com/jminer/clear-coat) — Clear Coat is a Rust wrapper for the IUP GUI library
  * [dcampbell24/iup-rust](https://github.com/dcampbell24/iup-rust) — IUP bindings [<img src="https://api.travis-ci.org/dcampbell24/iup-rust.svg?branch=master">](https://travis-ci.org/dcampbell24/iup-rust)
  * [Kiss-ui](https://github.com/KISS-UI/kiss-ui) — a simple UI framework built on IUP [![Build Status](https://api.travis-ci.org/cybergeek94/kiss-ui.svg?branch=master)](https://travis-ci.org/cybergeek94/kiss-ui)
* [libui](https://github.com/andlabs/libui)
  * [pcwalton/libui-rs](https://github.com/pcwalton/libui-rs) — libui bindings [<img src="https://api.travis-ci.org/pcwalton/libui-rs.svg?branch=master">](https://travis-ci.org/pcwalton/libui-rs). Abandoned project. The most updated fork is [LeoTindall/libui-rs](https://github.com/LeoTindall/libui-rs).
* [Nuklear](https://github.com/vurtun/nuklear)
  * [nuklear-rust](https://github.com/snuk182/nuklear-rust) — Rust bindings for Nuklear [<img src="https://api.travis-ci.org/snuk182/nuklear-rust.svg?branch=master">](https://travis-ci.org/snuk182/nuklear-rust)
* [Qt](http://doc.qt.io)
  * [cyndis/qmlrs](https://github.com/cyndis/qmlrs) — QtQuick bindings [<img src="https://api.travis-ci.org/cyndis/qmlrs.svg?branch=master">](https://travis-ci.org/cyndis/qmlrs)
  * [kitech/qt.rs](https://github.com/kitech/qt.rs) — Qt5 bindings [<img src="https://api.travis-ci.org/kitech/qt.rs.svg?branch=master">](https://travis-ci.org/kitech/qt.rs)
  * [Rust Qt Binding Generator](https://phabricator.kde.org/source/rust-qt-binding-generator/) — Binding generator hosted by KDE.
  * [rust-qt](https://github.com/rust-qt) —
  * [White-Oak/qml-rust](https://github.com/White-Oak/qml-rust) — QML bindings. [<img src="https://api.travis-ci.org/White-Oak/qml-rust.svg?branch=master">](https://travis-ci.org/White-Oak/qml-rust)
* [saurvs/nfd-rs](https://github.com/saurvs/nfd-rs) — [nativefiledialog](https://github.com/mlabbe/nativefiledialog) bindings
* [Sciter](https://sciter.com/)
  * [sciter-sdk/rust-sciter](https://github.com/sciter-sdk/rust-sciter) — Sciter bindings [<img src="https://ci.appveyor.com/api/projects/status/github/sciter-sdk/rust-sciter?svg=true">](https://ci.appveyor.com/project/sciter-sdk/rust-sciter)
* [flutter-rs](https://github.com/gliheng/flutter-rs) - Build flutter desktop app in dart & rust.

### Image processing

* [abonander/img_hash](https://github.com/abonander/img_hash) — Perceptual image hashing and comparison for equality and similarity.
* [PistonDevelopers/imageproc](https://github.com/PistonDevelopers/imageproc) — An image processing library, based on the `image` library. [![Build Status](https://api.travis-ci.org/PistonDevelopers/imageproc.svg?branch=master)](https://travis-ci.org/PistonDevelopers/imageproc)
* [PistonDevelopers/image](https://github.com/PistonDevelopers/image) — Basic imaging processing functions and methods for converting to and from image formats [<img src="https://api.travis-ci.org/PistonDevelopers/image.svg?branch=master">](https://travis-ci.org/PistonDevelopers/image)
* [kali/opencv-rust](https://github.com/kali/opencv-rust) — Rust bindings for OpenCV [<img src="https://api.travis-ci.org/kali/opencv-rust.svg?branch=cv2">](https://travis-ci.org/kali/opencv-rust)
* [teovoinea/steganography](https://github.com/teovoinea/steganography) [[steganography](https://crates.io/crates/steganography)] — A simple steganography library [<img src="https://api.travis-ci.org/teovoinea/steganography.svg?branch=master">](https://travis-ci.org/teovoinea/steganography)

### Language specification

* [shnewto/bnf](https://github.com/shnewto/bnf) — A library for parsing Backus–Naur form context-free grammars. [<img src="https://api.travis-ci.org/shnewto/bnf.svg?branch=master">](https://travis-ci.org/shnewto/bnf)

### Logging

[[log](https://crates.io/keywords/log)]

* [rust-lang-nursery/log](https://github.com/rust-lang-nursery/log) — Logging implementation for Rust [![Build Status](https://api.travis-ci.org/rust-lang-nursery/log.svg?branch=master)](https://travis-ci.org/rust-lang-nursery/log)
* [slog-rs/slog](https://github.com/slog-rs/slog) — Structured, composable logging for Rust [![Build Status](https://api.travis-ci.org/slog-rs/slog.svg?branch=master)](https://travis-ci.org/slog-rs/slog)
* [sfackler/log4rs](https://github.com/sfackler/log4rs) — highly configurable logging framework modeled after Java's Logback and log4j libraries [![Build Status](https://api.travis-ci.org/sfackler/log4rs.svg?branch=master)](https://travis-ci.org/sfackler/log4rs)

### Macro

* cute
  * [mattgathu/cute](https://github.com/mattgathu/cute) — Macro for Python-esque list comprehensions in Rust. [![Build Status](https://api.travis-ci.org/mattgathu/cute.svg?branch=master)](https://travis-ci.org/tensorflow/rust)
* hado
  * [ludat/hado-rs](https://github.com/ludat/hado-rs) — A little macro for writing haskell-like do expressions without too much ceremony

### Markup language

* CommonMark
  * [raphlinus/pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) — [CommonMark](https://commonmark.org/) parser in Rust

### Mobile

[Geal/rust_on_mobile](https://github.com/Geal/rust_on_mobile)

* Android
  * [tomaka/android-rs-glue](https://github.com/tomaka/android-rs-glue) — glue between Rust and Android [<img src="https://api.travis-ci.org/tomaka/android-rs-glue.svg?branch=master">](https://travis-ci.org/tomaka/android-rs-glue)
* iOS
  * [TimNN/cargo-lipo](https://github.com/TimNN/cargo-lipo) — a cargo lipo subcommand which automatically creates a universal library for use with your iOS application. [<img src="https://api.travis-ci.org/TimNN/cargo-lipo.svg?branch=master">](https://travis-ci.org/TimNN/cargo-lipo)
  * [vhbit/ObjCrust](https://github.com/vhbit/ObjCrust) — using Rust to create an iOS static library [<img src="https://api.travis-ci.org/vhbit/ObjCrust.svg?branch=master">](https://travis-ci.org/vhbit/ObjCrust)
* Pebble
  * [andars/pebble.rs](https://github.com/andars/pebble.rs) — a crate that allows Rust to be used to develop Pebble applications.

### Network programming

* HTTP
  * [pop-os/parallel-getter](https://github.com/pop-os/parallel-getter) — Download a file with parallel GET requests to maximize bandwidth usage.
  * [pop-os/url-crawler](https://github.com/pop-os/url-crawler) — A configurable parallel web crawler, designed to crawl a website for content.
  * [pop-os/url-scraper](https://github.com/pop-os/url-scraper) — Scrape URLs from HTML pages
* FTP
  * [mattnenterprise/rust-ftp](https://github.com/mattnenterprise/rust-ftp) — an [FTP](https://en.wikipedia.org/wiki/File_Transfer_Protocol) client for Rust [<img src="https://api.travis-ci.org/mattnenterprise/rust-ftp.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-ftp)
* IPNetwork
  * [achanda/ipnetwork](https://github.com/achanda/ipnetwork) — A library to work with IP networks in pure Rust [<img src="https://api.travis-ci.org/achanda/ipnetwork.svg?branch=master">](https://travis-ci.org/achanda/ipnetwork)
* Low level
  * [libpnet/libpnet](https://github.com/libpnet/libpnet) — a cross-platform, low level networking [<img src="https://api.travis-ci.org/libpnet/libpnet.svg?branch=master">](https://travis-ci.org/libpnet/libpnet)
  * [tokio-rs/tokio](https://github.com/tokio-rs/tokio) — a network application framework for rapid development and highly scalable production deployments of clients and servers.
  * [dylanmckay/protocol](https://github.com/dylanmckay/protocol) — Custom TCP/UDP protocol definitions
  * [actix/actix](https://github.com/actix/actix) — Actor library for Rust [<img src="https://api.travis-ci.org/actix/actix.svg?branch=master">](https://travis-ci.org/actix/actix)
* NanoMsg
  * [thehydroimpulse/nanomsg.rs](https://github.com/thehydroimpulse/nanomsg.rs) — [nanomsg](https://nanomsg.org/) bindings [<img src="https://api.travis-ci.org/thehydroimpulse/nanomsg.rs.svg?branch=master">](https://travis-ci.org/thehydroimpulse/nanomsg.rs)
* NNTP
  * [mattnenterprise/rust-nntp](https://github.com/mattnenterprise/rust-nntp) — an [NNTP](https://en.wikipedia.org/wiki/Network_News_Transfer_Protocol) client for Rust [<img src="https://api.travis-ci.org/mattnenterprise/rust-nntp.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-nntp)
* POP3
  * [mattnenterprise/rust-pop3](https://github.com/mattnenterprise/rust-pop3) — a [POP3](https://en.wikipedia.org/wiki/Post_Office_Protocol) client for Rust [<img src="https://api.travis-ci.org/mattnenterprise/rust-pop3.svg?branch=master">](https://travis-ci.org/mattnenterprise/rust-pop3)
* SSH
  * [alexcrichton/ssh2-rs](https://github.com/alexcrichton/ssh2-rs) — [libssh2](https://www.libssh2.org/) bindings [<img src="https://api.travis-ci.org/alexcrichton/ssh2-rs.svg?branch=master">](https://travis-ci.org/alexcrichton/ssh2-rs)
  * [Thrussh](https://github.com/pijul-scm/thrussh/) — an SSH library written from scratch in Rust, backed by [libsodium](https://download.libsodium.org/doc/)
* Stomp
  * [zslayton/stomp-rs](https://github.com/zslayton/stomp-rs) — a [STOMP 1.2](http://stomp.github.io/stomp-specification-1.2.html) client implementation in Rust [<img src="https://api.travis-ci.org/zslayton/stomp-rs.svg?branch=master">](https://travis-ci.org/zslayton/stomp-rs)
* uTP
  * [meqif/rust-utp](https://github.com/meqif/rust-utp) — a [uTP](http://www.bittorrent.org/beps/bep_0029.html) (Micro Transport Protocol) library for Rust. [<img src="https://api.travis-ci.org/meqif/rust-utp.svg?branch=master">](https://travis-ci.org/meqif/rust-utp)
* ZeroMQ
  * [erickt/rust-zmq](https://github.com/erickt/rust-zmq) — [ZeroMQ](http://zeromq.org/) bindings [<img src="https://api.travis-ci.org/erickt/rust-zmq.svg?branch=master">](https://travis-ci.org/erickt/rust-zmq)
* CoAP
  * [Covertness/coap-rs](https://github.com/Covertness/coap-rs) — A [Constrained Application Protocol(CoAP)](https://tools.ietf.org/html/rfc7252) library for Rust. [<img src="https://api.travis-ci.org/Covertness/coap-rs.svg?branch=master">](https://travis-ci.org/Covertness/coap-rs)

### Parser

  * [Geal/nom](https://github.com/Geal/nom) — parser combinator library [<img src="https://api.travis-ci.org/Geal/nom.svg?branch=master">](https://travis-ci.org/Geal/nom)
  * [ivanceras/inquerest](https://github.com/ivanceras/inquerest) — an URL parameter parser for rest filter inquiry [![Build Status](https://api.travis-ci.org/ivanceras/inquerest.svg?branch=master)](https://travis-ci.org/ivanceras/inquerest)
  * [kevinmehall/rust-peg](https://github.com/kevinmehall/rust-peg) — Parsing Expression Grammar (PEG) parser generator
  * [m4rw3r/chomp](https://github.com/m4rw3r/chomp) – A fast monadic-style parser combinator [<img src="https://api.travis-ci.org/m4rw3r/chomp.svg?branch=master">](https://travis-ci.org/m4rw3r/chomp)
  * [Marwes/combine](https://github.com/Marwes/combine) — parser combinator library [<img src="https://api.travis-ci.org/Marwes/combine.svg?branch=master">](https://travis-ci.org/Marwes/combine)
  * [lalrpop/lalrpop](https://github.com/lalrpop/lalrpop) — LR(1) parser generator for Rust [![Build status](https://api.travis-ci.org/lalrpop/lalrpop.svg?branch=master)](https://travis-ci.org/lalrpop/lalrpop)
  * [nrc/zero](https://github.com/nrc/zero) — zero-allocation parsing of binary data [<img src="https://api.travis-ci.org/nrc/zero.svg?branch=master">](https://travis-ci.org/nrc/zero)
  * [pest-parser/pest](https://github.com/pest-parser/pest) — The Elegant Parser [![Build Status](https://api.travis-ci.org/pest-parser/pest.svg?branch=master)](https://travis-ci.org/pest-parser/pest)
  * [ptal/oak](https://github.com/ptal/oak) — a typed PEG parser generator (compiler plugin)
  * [rustless/queryst](https://github.com/rustless/queryst) — a query string parsing library for Rust inspired by https://github.com/ljharb/qs [![Build Status](https://api.travis-ci.org/rustless/queryst.svg?branch=master)](https://travis-ci.org/rustless/queryst)

### Packaging formats

- [pop-os/debarchive](https://github.com/pop-os/debarchive) Library for reading and extracting debian archives

### Platform specific

* Linux
  * [frol/cgroups-fs](https://github.com/frol/cgroups-fs) — Rust bindings to Linux Control Groups (cgroups) [<img src="https://api.travis-ci.org/frol/cgroups-fs.svg?branch=master">](https://travis-ci.org/frol/cgroups-fs)
  * [pop-os/dbus-udisks2](https://github.com/pop-os/dbus-udisks2) — UDisks2 DBus API
  * [pop-os/distinst](https://github.com/pop-os/distinst/) — Linux distribution installer library
  * [inotify-rs/inotify](https://github.com/inotify-rs/inotify) — [inotify](https://en.wikipedia.org/wiki/Inotify) bindings [<img src="https://api.travis-ci.org/inotify-rs/inotify.svg?branch=master">](https://travis-ci.org/inotify-rs/inotify)
  * [arvancloud/nginx-rs](https://github.com/arvancloud/nginx-rs) — [Nginx](https://www.nginx.com) bindings [<img src="https://api.travis-ci.org/arvancloud/nginx-rs.svg?branch=master">](https://travis-ci.org/arvancloud/nginx-rs)
  * [yaa110/rust-iptables](https://github.com/yaa110/rust-iptables) — [iptables](https://www.netfilter.org/projects/iptables/index.html) bindings [<img src="https://api.travis-ci.org/yaa110/rust-iptables.svg?branch=master">](https://travis-ci.org/yaa110/rust-iptables)
* Unix-like
  * [nix-rust/nix](https://github.com/nix-rust/nix) — Unix-like API bindings [<img src="https://api.travis-ci.org/nix-rust/nix.svg?branch=master">](https://travis-ci.org/nix-rust/nix)
  * [zargony/rust-fuse](https://github.com/zargony/rust-fuse) — [FUSE](https://github.com/libfuse/libfuse) bindings <img src="https://api.travis-ci.org/zargony/rust-fuse.svg?branch=master">
* Windows
  * [retep998/winapi-rs](https://github.com/retep998/winapi-rs) — Windows API bindings [<img src="https://api.travis-ci.org/retep998/winapi-rs.svg?branch=master">](https://travis-ci.org/retep998/winapi-rs)

### Scripting
[[scripting](https://crates.io/keywords/scripting)]
* [PistonDevelopers/dyon](https://github.com/PistonDevelopers/dyon) — A rusty dynamically typed scripting language
* [gluon-lang/gluon](https://github.com/gluon-lang/gluon) —  A small, statically-typed, functional programming language
* [murarth/ketos](https://github.com/murarth/ketos) — A Lisp dialect functional programming language serving as a scripting and extension language for rust
* [JohnBSmith/moss](https://github.com/JohnBSmith/moss) [[moss](https://crates.io/crates/moss)] — A dynamically typed scripting language
* [jonathandturner/rhai](https://github.com/jonathandturner/rhai) — A tiny and fast embedded scripting language resembling a combination of JS and Rust

### Template engine

* Handlebars
  * [sunng87/handlebars-rust](https://github.com/sunng87/handlebars-rust) — Handlebars template engine with inheritance, custom helper support. [<img src="https://api.travis-ci.org/sunng87/handlebars-rust.svg?branch=master">](https://travis-ci.org/sunng87/handlebars-rust)
* HTML
  * [lfairy/maud](https://github.com/lfairy/maud) — compile-time HTML templates [<img src="https://api.travis-ci.org/lfairy/maud.svg?branch=master">](https://travis-ci.org/lfairy/maud)
  * [Stebalien/horrorshow-rs](https://github.com/Stebalien/horrorshow-rs) — compile-time HTML templates [<img src="https://api.travis-ci.org/Stebalien/horrorshow-rs.svg?branch=master">](https://travis-ci.org/Stebalien/horrorshow-rs)
  * [kaj/ructe](https://github.com/kaj/ructe) — HTML template system for Rust [<img src="https://api.travis-ci.org/kaj/ructe.svg?branch=master">](https://travis-ci.org/kaj/ructe)
  * [Keats/tera](https://github.com/Keats/tera) — template engine based on Jinja2 and the Django template language. [<img src="https://api.travis-ci.org/Keats/tera.svg?branch=master">](https://travis-ci.org/Keats/tera)
  * [djc/askama](https://github.com/djc/askama) — template rendering engine based on Jinja [<img src="https://api.travis-ci.org/djc/askama.svg?branch=master">](https://travis-ci.org/djc/askama)
* Mustache
  * [rustache/rustache](https://github.com/rustache/rustache) — [<img src="https://api.travis-ci.org/rustache/rustache.svg?branch=master">](https://travis-ci.org/rustache/rustache)
* [tailhook/marafet](https://github.com/tailhook/marafet) — Compiler for Jade-like template language to cito.js-based virtual dom

### Text processing

* [BurntSushi/suffix](https://github.com/BurntSushi/suffix) — Linear time suffix array construction (with Unicode support) [<img src="https://api.travis-ci.org/BurntSushi/suffix.svg?branch=master">](https://travis-ci.org/BurntSushi/suffix)
* [BurntSushi/tabwriter](https://github.com/BurntSushi/tabwriter) — Elastic tab stops (i.e., text column alignment) [<img src="https://api.travis-ci.org/BurntSushi/tabwriter.svg?branch=master">](https://travis-ci.org/BurntSushi/tabwriter)
* [dguo/strsim-rs](https://github.com/dguo/strsim-rs) [[strsim-rs](https://crates.io/crates/strsim)] — String similarity metrics [<img src="https://api.travis-ci.org/dguo/strsim-rs.svg?branch=master">](https://travis-ci.org/dguo/strsim-rs)
* [mgeisler/textwrap](https://github.com/mgeisler/textwrap) [[textwrap](https://crates.io/crates/textwrap)] — Word wrap text (with support for hyphenation) [<img src="https://api.travis-ci.org/mgeisler/textwrap.svg?branch=master">](https://travis-ci.org/mgeisler/textwrap)
* [pwoolcoc/ngrams](https://github.com/pwoolcoc/ngrams) — Construct [n-grams](https://en.wikipedia.org/wiki/N-gram) from arbitrary iterators [<img src="https://api.travis-ci.org/pwoolcoc/ngrams.svg?branch=master">](https://travis-ci.org/pwoolcoc/ngrams)
* [ps1dr3x/easy_reader](https://github.com/ps1dr3x/easy_reader) — A reader that allows forwards, backwards and random navigations through the lines of huge files without consuming iterators [<img src="https://api.travis-ci.org/ps1dr3x/easy_reader.svg?branch=master">](https://travis-ci.org/ps1dr3x/easy_reader)
* [rust-lang/regex](https://github.com/rust-lang/regex) — Regular expressions (RE2 style) [<img src="https://api.travis-ci.org/rust-lang/regex.svg?branch=master">](https://travis-ci.org/rust-lang/regex)
* [greyblake/whatlang-rs](https://github.com/greyblake/whatlang-rs) — Natural language detection library based on trigrams [<img src="https://api.travis-ci.org/greyblake/whatlang-rs.svg?branch=master">](https://travis-ci.org/greyblake/whatlang-rs)
* [yaa110/rake-rs](https://github.com/yaa110/rake-rs) — Multilingual implementation of RAKE algorithm for Rust [<img src="https://api.travis-ci.org/yaa110/rake-rs.svg?branch=master">](https://travis-ci.org/yaa110/rake-rs)
* [Lucretiel/joinery](https://github.com/Lucretiel/joinery) [[joinery](https://crates.io/crates/joinery)] – Generic string + iterable joining [<img src="https://api.travis-ci.org/Lucretiel/joinery.svg?branch=master">](https://travis-ci.org/Lucretiel/joinery)

### Text search

* [BurntSushi/fst](https://github.com/BurntSushi/fst) [[fst](https://crates.io/crates/fst)] — [<img src="https://api.travis-ci.org/BurntSushi/fst.svg?branch=master">](https://travis-ci.org/BurntSushi/fst)
* [CurrySoftware/perlin](https://github.com/CurrySoftware/perlin) [[perlin](https://crates.io/crates/perlin)] — [<img src="https://api.travis-ci.org/CurrySoftware/perlin.svg?branch=master">](https://travis-ci.org/CurrySoftware/perlin)
* [tantivy-search/tantivy](https://github.com/tantivy-search/tantivy) [[tantivy](https://crates.io/crates/tantivy)] — [<img src="https://api.travis-ci.org/tantivy-search/tantivy.svg?branch=master">](https://travis-ci.org/tantivy-search/tantivy)

### Virtualization

* [beneills/quantum](https://github.com/beneills/quantum) — Advanced Rust quantum computer simulator [<img src="https://api.travis-ci.org/beneills/quantum.svg?branch=master">](https://travis-ci.org/beneills/quantum)
* [ekse/unicorn-rs](https://github.com/ekse/unicorn-rs) — Rust bindings for the unicorn CPU emulator
* [saurvs/hypervisor-rs](https://github.com/saurvs/hypervisor-rs) — Hardware-accelerated virtualization on OS X

### Web programming

See also [Are we web yet?](http://www.arewewebyet.org) and [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison).

* Client-side / WASM
  * [DenisKolodin/yew](https://github.com/DenisKolodin/yew) [[yew](https://crates.io/crates/yew)] — Rust framework for making client web apps
  * [koute/cargo-web](https://github.com/koute/cargo-web) [[cargo-web](https://crates.io/crates/cargo-web)] — A Cargo subcommand for the client-side Web [![Build Status](https://api.travis-ci.org/koute/cargo-web.svg)](https://travis-ci.org/koute/cargo-web)
  * [koute/stdweb](https://github.com/koute/stdweb) [[stdweb](https://crates.io/crates/stdweb)] — A standard library for the client-side Web [![Build Status](https://api.travis-ci.org/koute/stdweb.svg)](https://travis-ci.org/koute/stdweb)
* HTTP Client
  * [alexcrichton/curl-rust](https://github.com/alexcrichton/curl-rust) — [libcurl](https://curl.haxx.se/libcurl/) bindings [<img src="https://api.travis-ci.org/alexcrichton/curl-rust.svg?branch=master">](https://travis-ci.org/alexcrichton/curl-rust)
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [<img src="https://api.travis-ci.org/hyperium/hyper.svg?branch=master">](https://travis-ci.org/hyperium/hyper)
  * [seanmonstar/reqwest](https://github.com/seanmonstar/reqwest) — an ergonomic HTTP Client for Rust. [<img src="https://api.travis-ci.org/seanmonstar/reqwest.svg?branch=master">](https://travis-ci.org/seanmonstar/reqwest)
  * [DoumanAsh/yukikaze](https://gitlab.com/Douman/yukikaze) — Beautiful and elegant Yukikaze is little HTTP client library based on hyper. [<img src="https://gitlab.com/Douman/yukikaze/badges/master/build.svg">](https://gitlab.com/Douman/yukikaze)
* HTTP Server
  * [actix/actix-web](https://github.com/actix/actix-web) — A lightweight async web framework for Rust with websocket support [<img src="https://api.travis-ci.org/actix/actix-web.svg?branch=master">](https://travis-ci.org/actix/actix-web)
  * [return/branca](https://github.com/return/branca) [[branca](https://crates.io/crates/branca)] — A Pure Rust implementation of Branca for Authenticated and Encrypted API tokens. [<img src="https://api.travis-ci.org/return/branca.svg?branch=master">](https://travis-ci.org/return/branca)
  * [Gotham](https://github.com/gotham-rs/gotham) — A flexible web framework that does not sacrifice safety, security or speed. [<img src="https://api.travis-ci.org/gotham-rs/gotham.svg?branch=master">](https://travis-ci.org/gotham-rs/gotham)
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [<img src="https://api.travis-ci.org/hyperium/hyper.svg?branch=master">](https://travis-ci.org/hyperium/hyper)
  * [GildedHonour/frank_jwt](https://github.com/GildedHonour/frank_jwt) — JSON Web Token implementation in Rust. [<img src="https://api.travis-ci.org/GildedHonour/frank_jwt.svg?branch=master">](https://travis-ci.org/GildedHonour/frank_jwt)
  * [Iron](https://github.com/iron/iron) — a middleware-based server framework [<img src="https://api.travis-ci.org/GildedHonour/frank_jwt.svg?branch=master">](https://travis-ci.org/GildedHonour/frank_jwt)
  * [sunng87/handlebars-iron](https://github.com/sunng87/handlebars-iron) — [Handlebars-rust](https://github.com/sunng87/handlebars-rust) as an Iron web framework middleware. [<img src="https://api.travis-ci.org/sunng87/handlebars-iron.svg?branch=master">](https://travis-ci.org/sunng87/handlebars-iron)
  * [Nickel](https://github.com/nickel-org/nickel.rs/) — inspired by [Express](http://expressjs.com/) [<img src="https://api.travis-ci.org/nickel-org/nickel.rs.svg?branch=master">](https://travis-ci.org/nickel-org/nickel.rs)
  * [Ogeon/rustful](https://github.com/Ogeon/rustful) — a RESTful web framework for Rust  [<img src="https://api.travis-ci.org/Ogeon/rustful.svg?branch=master">](https://travis-ci.org/Ogeon/rustful)
  * [Rocket](https://github.com/SergioBenitez/Rocket) — Rocket is web framework for Rust (nightly) with a focus on ease-of-use, expressability, and speed [<img src="https://api.travis-ci.org/SergioBenitez/Rocket.svg?branch=master">](https://travis-ci.org/SergioBenitez/Rocket)
  * [Rustless](https://github.com/rustless/rustless) — a REST-like API micro-framework inspired by [Grape](https://github.com/ruby-grape/grape) and [Hyper](https://github.com/hyperium/hyper) [<img src="https://api.travis-ci.org/rustless/rustless.svg?branch=master">](https://travis-ci.org/rustless/rustless)
  * [Saphir](https://github.com/richerarc/saphir) — A progressive web framework with low-level control, without the pain.
  * [sappworks/sapper](https://github.com/sappworks/sapper) — A lightweight web framework built on async hyper, implemented in Rust language. [<img src="https://api.travis-ci.org/sappworks/sapper.svg?branch=master">](https://travis-ci.org/sappworks/sapper)
  * [tiny-http](https://github.com/tiny-http/tiny-http) — Low level HTTP server library [<img src="https://api.travis-ci.org/frewsxcv/tiny-http.svg?branch=master">](https://travis-ci.org/frewsxcv/tiny-http)
  * [tomaka/rouille](https://github.com/tomaka/rouille) — Web framework in Rust [<img src="https://api.travis-ci.org/tomaka/rouille.svg?branch=master">](https://travis-ci.org/tomaka/rouille)
  * [carllerche/tower-web](https://github.com/carllerche/tower-web) [[tower-web](https://crates.io/crates/tower-web)] — A fast, boilerplate free, web framework for Rust [<img src="https://api.travis-ci.org/carllerche/tower-web.svg?branch=master">](https://travis-ci.org/carllerche/tower-web)
  * [danclive/sincere](https://github.com/danclive/sincere) — A micro web framework for Rust(stable) based on hyper and multithreading. [<img src="https://api.travis-ci.org/danclive/sincere.svg?branch=master">](https://travis-ci.org/danclive/sincere)
  * [oltdaniel/zap](https://github.com/oltdaniel/zap) — A lightning fast http framework for Rust [<img src="https://api.travis-ci.org/oltdaniel/zap.svg?branch=master">](https://travis-ci.org/oltdaniel/zap)
* [WebSocket](https://datatracker.ietf.org/doc/rfc6455/)
  * [cyderize/rust-websocket](https://github.com/cyderize/rust-websocket) — a framework for dealing with WebSocket connections (both clients and servers) [<img src="https://api.travis-ci.org/cyderize/rust-websocket.svg?branch=master">](https://travis-ci.org/cyderize/rust-websocket)
  * [housleyjk/ws-rs](https://github.com/housleyjk/ws-rs) — lightweight, event-driven WebSockets for Rust [<img src="https://api.travis-ci.org/housleyjk/ws-rs.svg?branch=stable">](https://travis-ci.org/housleyjk/ws-rs)
  * [snapview/tungstenite-rs](https://github.com/snapview/tungstenite-rs) — Lightweight stream-based WebSocket implementation for Rust.
  * [actix/sockjs](https://github.com/actix/sockjs) — [SockJS](https://github.com/sockjs) server for Rust [<img src="https://api.travis-ci.org/actix/sockjs.svg?branch=master">](https://travis-ci.org/actix/sockjs)
  * [vityafx/urlshortener-rs](https://github.com/vityafx/urlshortener-rs) [[urlshortener](https://crates.io/crates/urlshortener)] — A very simple urlshortener library for Rust. [<img src="https://api.travis-ci.org/vityafx/urlshortener-rs.svg?branch=master">](https://travis-ci.org/vityafx/urlshortener-rs)
* Miscellaneous
  * [cargonauts](https://github.com/cargonauts-rs/cargonauts) — A web framework intended for building maintainable, well-factored web apps.
  * [pyros2097/rust-embed](https://github.com/pyros2097/rust-embed) — A macro to embed static assets into the rust binary
  * [utkarshkukreti/select.rs](https://github.com/utkarshkukreti/select.rs) [[select](https://crates.io/crates/select)] — A library to extract useful data from HTML documents, suitable for web scraping. [![Build Status](https://api.travis-ci.org/utkarshkukreti/select.rs.svg?branch=master)](https://travis-ci.org/utkarshkukreti/select.rs)
* Reverse Proxy
  * [sozu-proxy/sozu](https://github.com/sozu-proxy/sozu) [[sozu](https://crates.io/crates/sozu)] — A HTTP reverse proxy. [![Build Status](https://api.travis-ci.org/sozu-proxy/sozu.svg?branch=master)](https://api.travis-ci.org/sozu-proxy/sozu)
* Static Site Generators
  * [getzola/zola](https://github.com/getzola/zola) [[zola](https://www.getzola.org/)] — An opinionated static site generator with everything built-in. [![Build Status](https://api.travis-ci.org/getzola/zola.svg?branch=master)](https://travis-ci.org/getzola/zola)
  * [cobalt-org/cobalt.rs](https://github.com/cobalt-org/cobalt.rs) — Static site generator written in Rust [<img src="https://api.travis-ci.org/cobalt-org/cobalt.rs.svg?branch=master">](https://travis-ci.org/cobalt-org/cobalt.rs)
  * [FuGangqiang/mdblog.rs](https://github.com/FuGangqiang/mdblog.rs) — Static site generator from markdown files.
  * [leven-the-blog/leven](https://github.com/leven-the-blog/leven) [[leven](https://crates.io/crates/leven)] — a simple, parallelized blog generator. [<img src="https://api.travis-ci.org/leven-the-blog/leven.svg?branch=master">](https://travis-ci.org/leven-the-blog/leven)

### Peripherals

* Serial Port
  * [Susurrus/serialport-rs](https://github.com/Susurrus/serialport-rs) [[serialport](https://docs.rs/serialport/3.0.0/serialport/)] — A cross-platform library that provides access to a serial port

## Resources

* Benchmarks
  * [TeXitoi/benchmarksgame-rs](https://github.com/TeXitoi/benchmarksgame-rs) — Rust implementations for the [The Computer Language Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/) [<img src="https://api.travis-ci.org/TeXitoi/benchmarksgame-rs.svg?branch=master">](https://travis-ci.org/TeXitoi/benchmarksgame-rs)
* Decks & Presentations
  * [Learning systems programming with Rust](https://speakerdeck.com/jvns/learning-systems-programming-with-rust) — Presented by [Julia Evans](https://twitter.com/@b0rk) @ Rustconf 2016.
  * [Shipping a Solid Rust Crate](https://www.youtube.com/watch?v=t4CyEKb-ywA) — Presented by [Michael Gattozzi](https://github.com/mgattozzi) @ RustConf 2017
  * [Rust: Hack Without Fear!](https://www.youtube.com/watch?v=lO1z-7cuRYI) — Presented by [Nicholas Matsakis](https://github.com/nikomatsakis) @ C++Now 2018
* Learning
  * [Programming Community Curated Resources for Learning Rust](https://hackr.io/tutorials/learn-rust) — A list of recommended resources voted by the programming community.
  * [exercism.io](https://exercism.io/tracks/rust) — programming exercises that help you learn new concepts in Rust.
  * [Idiomatic Rust](https://github.com/mre/idiomatic-rust) —  A peer-reviewed collection of articles/talks/repos which teach idiomatic Rust.
  * [Learning Rust With Entirely Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/) — in-depth exploration of Rust's memory management rules, through implementing a few different types of list structures.
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  * [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) — a collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem.
  * [Rust in Motion](https://www.manning.com/livevideo/rust-in-motion?a_aid=cnichols&a_bid=6a993c2e) — A video series by [Carol Nichols](https://github.com/carols10cents) and [Jake Goulding](https://github.com/shepmaster) (paid)
  * [rust-learning](https://github.com/ctjhoa/rust-learning) — a collection of useful resources to learn Rust
  * [Rustlings](https://github.com/rustlings/rustlings) — small exercises to get you used to reading and writing Rust code
  * [stdx](https://github.com/brson/stdx) — Learn these crates first as an extension to std
  * [University of Pennsylvania's Comp Sci Rust Programming Course](http://cis198-2016s.github.io/schedule/)
  * [Build a language VM](https://blog.subnetzero.io/post/building-language-vm-part-00/)
* Podcasts
  * [New Rustacean](https://newrustacean.com) — a podcast about learning Rust
* [RustCamp 2015 Talks](http://confreaks.tv/events/rustcamp2015)
* [Rust Design Patterns](https://github.com/rust-unofficial/patterns)
* [Rust Guidelines](http://aturon.github.io/)
* [RustBooks](https://github.com/sger/RustBooks) — list of RustBooks
* [Rust Subreddit](https://www.reddit.com/r/rust/) — a subreddit(forum) where rust related questions, articles and resources are posted and discussed

## License

[![CC0](https://licensebuttons.net/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)
