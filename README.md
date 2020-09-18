# Awesome Rust

A curated list of Rust code and resources.

If you want to contribute, please read [this](CONTRIBUTING.md).

## Table of contents

  - [Applications](#applications)
    - [Audio and Music](#audio-and-music)
    - [Cryptocurrencies](#cryptocurrencies)
    - [Database](#database)
    - [Emulators](#emulators)
    - [Games](#games)
    - [Graphics](#graphics)
    - [Industrial automation](#industrial-automation)
    - [Observability](#observability)
    - [Operating systems](#operating-systems)
    - [Productivity](#productivity)
    - [Security tools](#security-tools)
    - [System tools](#system-tools)
    - [Text editors](#text-editors)
    - [Text processing](#text-processing)
    - [Image processing](#image-processing)
    - [Utilities](#utilities)
    - [Video](#video)
    - [Virtualization](#virtualization)
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
    - [Services](#services)
    - [Static analysis](#static-analysis)
    - [Testing](#testing)
    - [Transpiling](#transpiling)
  - [Libraries](#libraries)
    - [Artificial Intelligence](#artificial-intelligence)
      - [Genetic algorithms](#genetic-algorithms)
      - [Machine learning](#machine-learning)
    - [Astronomy](#astronomy)
    - [Asynchronous](#asynchronous)
    - [Audio and Music](#audio-and-music-1)
    - [Authentication](#authentication)
    - [Automotive](#automotive)
    - [Bioinformatics](#bioinformatics)
    - [Caching](#caching)
    - [Concurrency](#concurrency)
    - [Cloud](#cloud)
    - [Command-line](#command-line)
    - [Compression](#compression)
    - [Computation](#computation)
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
    - [Functional Programming](#functional-programming)
    - [Game development](#game-development)
    - [Geospatial](#geospatial)
    - [Graphics](#graphics-1)
    - [Graph processing](#graph-processing)
    - [GUI](#gui)
    - [Image processing](#image-processing)
    - [Language specification](#language-specification)
    - [Logging](#logging)
    - [Macro](#macro)
    - [Markup language](#markup-language)
    - [Mobile](#mobile)
    - [Network programming](#network-programming)
    - [Parsing](#parsing)
    - [Packaging formats](#packaging-formats)
    - [Peripherals](#peripherals)
    - [Platform specific](#platform-specific)
    - [Scripting](#scripting)
    - [Simulation](#simulation)
    - [Template engine](#template-engine)
    - [Text processing](#text-processing-1)
    - [Text search](#text-search)
    - [Unsafe](#unsafe)
    - [Virtualization](#virtualization-1)
    - [Web programming](#web-programming)
  - [Registries](#registries)
  - [Resources](#resources)
  - [License](#license)

## Applications

See also [Rust — Production](https://www.rust-lang.org/production) organizations running Rust in production.

* [alacritty](https://github.com/alacritty/alacritty) — A cross-platform, GPU enhanced terminal emulator
* [AnderEnder/s3find-rs](https://github.com/AnderEnder/s3find-rs) — A command line utility to walk an Amazon S3 hierarchy, an analog of find for Amazon S3
* [andschwa/rust-genetic-algorithm](https://github.com/andschwa/rust-genetic-algorithm) — A genetic algorithm for academic benchmark problems
* [asm-cli-rust](https://github.com/cch123/asm-cli-rust) — An interactive assembly shell written in rust.
* [ballista](https://github.com/ballista-compute/ballista) — PoC of distributed compute platform using Rust, Apache Arrow, and Kubernetes!
* [cloudflare/boringtun](https://github.com/cloudflare/boringtun) — A Userspace WireGuard VPN Implementation
* [darrint/device-blocker](https://github.com/darrint/device-blocker) — Limit screen time to children's various mobile devices by blocking internet access on the family Wifi router.
* [denoland/deno](https://github.com/denoland/deno) — A secure JavaScript/TypeScript runtime built with V8, Rust, and Tokio
* [dlecan/generic-dns-update](https://github.com/dlecan/generic-dns-update) — A tool to update DNS zonefiles with your IP address
* [Factotum](https://github.com/snowplow/factotum) — [A system to programmatically run data pipelines](https://snowplowanalytics.com/blog/2016/04/09/introducing-factotum-data-pipeline-runner/)
* [fcsonline/drill](https://github.com/fcsonline/drill) — A HTTP load testing application inspired by Ansible syntax
* [Fractalide](https://github.com/fractalide/fractalide) — Simple Rust Microservices
* [habitat](https://community.chef.io/products/chef-habitat/) — An tool created by [Chef](https://www.chef.io/) to build, deploy, and manage applications.
* [Herd](https://github.com/imjacobclark/Herd) — an experimental HTTP load testing application
* [intecture/api](https://github.com/intecture/api) — an API-driven server management and configuration tool
* [ivanceras/diwata](https://github.com/ivanceras/diwata) — A database administration tool for postgresql
* [jedisct1/flowgger](https://github.com/awslabs/flowgger) — A fast, simple and lightweight data collector
* [kbknapp/docli](https://github.com/kbknapp/docli-rs) — A command line utility for managing DigitalOcean infrastructure
* [kytan](https://github.com/changlan/kytan) — High Performance Peer-to-Peer VPN
* [limonite](https://crates.io/crates/limonite) — static blog/website generator
* [linkerd/linkerd2-proxy](https://github.com/linkerd/linkerd2-proxy) — Ultralight service mesh for Kubernetes.
* [MaidSafe](https://maidsafe.net) — A decentralized platform.
* [mdBook](https://crates.io/crates/mdbook) — A command line utility to create books from markdown files
* [nicohman/eidolon](https://github.com/nicohman/eidolon) — A steam and drm-free game registry and launcher for linux and macosx
* [notty](https://github.com/withoutboats/notty) — A new kind of terminal
* [Pijul](https://pijul.org) — A patch-based distributed version control system
* [rsign](https://crates.io/crates/rsign) — A simple command-line tool used to generate/sign/verify digital signatures designed to be compatible with Minisign
* [Rudr](https://github.com/oam-dev/rudr) — A Kubernetes implementation of the [Open Application Model](https://oam.dev/) specification
* [rx](https://github.com/cloudhead/rx) — Vi inspired Modern Pixel Art Editor
* [Sandstorm Collections App](https://github.com/sandstorm-io/collections-app)
* [Servo](https://github.com/servo/servo) — A prototype web browser engine
* [tiny](https://github.com/osa1/tiny) — A terminal IRC client
* [trust-dns](https://crates.io/crates/trust-dns) — A DNS-server
* [updns](https://github.com/wyhaya/updns) — DNS proxy tool
* [Weld](https://github.com/serayuzgur/weld) — Full fake REST API generator
* [wezterm](https://github.com/wez/wezterm) — A GPU-accelerated cross-platform terminal emulator and multiplexer

### Audio and Music

* [enginesound](https://github.com/DasEtwas/enginesound) — A GUI and command line application used to procedurally generate semi-realistic engine sounds. Featuring in-depth configuration, variable sample rate and a frequency analysis window.
* [indiscipline/zrtstr](https://github.com/indiscipline/zrtstr) — A command line utility for checking if stereo wav files are faux-stereo (i.e. have identical channels) and converting such files to mono.
* [Polaris](https://github.com/agersant/polaris) — A music streaming application.
* [Spotify TUI](https://github.com/Rigellute/spotify-tui) — A Spotify client for the terminal written in Rust.
* [Spotifyd](https://github.com/Spotifyd/spotifyd) — An open source Spotify client running as a UNIX daemon.

### Cryptocurrencies

* [Bitcoin Satoshi's Vision](https://github.com/brentongunning/rust-sv) — A Rust library for working with Bitcoin SV .
* [cardano-cli](https://github.com/input-output-hk/cardano-cli) — Cardano Command Line Interface (CLI)
* [ChainX](https://github.com/chainx-org/ChainX) — Fully Decentralized Interchain Crypto Asset Management on Polkadot.
* [CITA](https://github.com/citahub/cita) — A high performance blockchain kernel for enterprise users.
* [coinbase-pro-rs](https://github.com/inv2004/coinbase-pro-rs) — Coinbase pro client in Rust, supports sync/async/websocket
* [ethaddrgen](https://github.com/Limeth/ethaddrgen) — Custom Ethereum vanity address generator made in Rust
* [Grin](https://github.com/mimblewimble/grin/) — Evolution of the MimbleWimble protocol
* [hdwallet](https://github.com/jjyr/hdwallet) — BIP-32 HD wallet related key derivation utilities.
* [Holochain](https://github.com/holochain/holochain-rust) — Scalable P2P alternative to blockchain for all those distributed apps you always wanted to build
* [ibc-rs](https://github.com/informalsystems/ibc-rs) - Rust implementation of the [Interblockchain Communication](https://cosmos.network/ibc/) protocol
* [infincia/bip39-rs](https://github.com/infincia/bip39-rs) — Rust implementation of BIP39.
* [Joystream](https://github.com/Joystream/joystream) — A user governed video platform
* [Libra](https://github.com/libra/libra) — Libra’s mission is to enable a simple global currency and financial infrastructure that empowers billions of people.
* [Lighthouse](https://github.com/sigp/lighthouse) — Rust Ethereum 2.0 Client
* [nearprotocol/nearcore](https://github.com/nearprotocol/nearcore) — decentralized smart-contract platform for low-end mobile devices.
* [Nervos CKB](https://github.com/nervosnetwork/ckb) — Nervos CKB is a public permissionless blockchain, the common knowledge layer of Nervos network.
* [Nimiq](https://github.com/nimiq/core-rs) — Rust implementation of Nimiq node
* [Parity-Bitcoin](https://github.com/paritytech/parity-bitcoin) — The Parity Bitcoin client
* [Parity-Bridge](https://github.com/paritytech/parity-bridge) — Bridge between any two ethereum-based networks
* [Parity-Ethereum](https://github.com/openethereum/openethereum) — Fast, light, and robust Ethereum client
* [Parity-Zcash](https://github.com/paritytech/parity-zcash) — Rust implementation of the Zcash protocol
* [Polkadot](https://github.com/paritytech/polkadot) — Heterogeneous multi‑chain technology with pooled security
* [rbtc](https://github.com/lucawen/rbtc) — Convert BTC to any currency and vice-versa.
* [rust-cardano](https://github.com/input-output-hk/rust-cardano) — Rust implementation of Cardano primitives, helpers, and related applications
* [Substrate](https://github.com/paritytech/substrate) — Generic modular blockchain template written in Rust
* [tendermint-rs](https://github.com/informalsystems/tendermint-rs) - Rust implementation of Tendermint blockchain data structures and clients
* [wagyu](https://github.com/AleoHQ/wagyu) [[wagyu](https://crates.io/crates/wagyu)] — Rust library for generating cryptocurrency wallets
* [zcash](https://github.com/zcash/zcash) — Zcash is an implementation of the "Zerocash" protocol.
* [YeeCo yeeroot](https://github.com/yeeco/yeeroot) — YeeCo yeeroot is a permissionless, secure, high performance and scalable public blockchain platform powered by full sharding technology on PoW consensus written in Rust

### Database

* [indradb](https://crates.io/crates/indradb) — Rust based graph database
* [Materialize](https://github.com/MaterializeInc/materialize) - Streaming SQL database powered by Timely Dataflow
* [noria](https://crates.io/crates/noria) — Dynamically changing, partially-stateful data-flow for web application backends
* [Lucid](https://github.com/lucid-kv/lucid) — High performance and distributed KV store accessible through a HTTP API.
* [ParityDB](https://github.com/paritytech/parity-db) — Fast and reliable database, optimised for read operation
* [PumpkinDB](https://github.com/PumpkinDB/PumpkinDB) — an event sourcing database engine
* [seppo0010/rsedis](https://github.com/seppo0010/rsedis) — A Redis reimplementation in Rust
* [TerrabaseDB](https://github.com/terrabasedb/terrabasedb) — A multi-model NoSQL database
* [tikv](https://github.com/tikv/tikv) — A distributed KV database in Rust
* [sled](https://crates.io/crates/sled) — A (beta) modern embedded database

### Emulators

See also [crates matching keyword 'emulator'](https://crates.io/keywords/emulator).

* Commodore 64
  * [kondrak/rust64](https://github.com/kondrak/rust64)
* Flash Player
  * [Ruffle](https://github.com/ruffle-rs/ruffle) — Ruffle is an Adobe Flash Player emulator written in the Rust programming language. Ruffle targets both the desktop and the web using WebAssembly.
* Gameboy
  * [Gekkio/mooneye-gb](https://github.com/Gekkio/mooneye-gb)
  * [mvdnes/rboy](https://github.com/mvdnes/rboy)
  * [NivenT/RGB](https://github.com/nivent/RGB)
  * [mohanson/gameboy](https://github.com/mohanson/gameboy) — Full featured Cross-platform GameBoy emulator. Forever boys!.
* Gameboy Advance
  * [michelhe/rustboyadvance-ng](https://github.com/michelhe/rustboyadvance-ng) - RustboyAdvance-ng is a Gameboy Advance emulator with desktop, android and [WebAssembly](https://michelhe.github.io/rustboyadvance-ng/) support.
* NES
  * [iamsix/oxidenes](https://github.com/iamsix/oxidenes)
  * [koute/pinky](https://github.com/koute/pinky)
  * [pcwalton/sprocketnes](https://github.com/pcwalton/sprocketnes)
* Playstation
  * [rustation-ng](https://gitlab.com/flio/rustation-ng/) — Playstation emulator using Rust
* ZX Spectrum
  * [pacmancoder/rustzx](https://github.com/pacmancoder/rustzx)
  * [rodrigorc/raze](https://github.com/rodrigorc/raze) — For WebAssembly, [live version here](https://rodrigorc.github.io/raze/)
* Virtual Boy
  * [emu-rs/rustual-boy](https://github.com/emu-rs/rustual-boy)
* Intel 8080 CPU
  * [mohanson/i8080](https://github.com/mohanson/i8080) — Intel 8080 cpu emulator by Rust
* Emulator Development tools
  * SNES
    * [ioncodes/snesutilities](https://github.com/ioncodes/snesutilities) — ROM analyser/extractor

### Games

See also [Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston).

* [lifthrasiir/angolmois-rust](https://github.com/lifthrasiir/angolmois-rust) — A minimalistic music video game which supports the BMS format
* [citybound](https://github.com/citybound/citybound) — The city sim you deserve
* [schulke-214/connect-four](https://github.com/schulke-214/connect-four) — A simple connect four implementation.
* [rsaarelm/magog](https://github.com/rsaarelm/magog) — A roguelike game in Rust
* [schulke-214/rsnake](https://github.com/schulke-214/rsnake) — Snake written in Rust.
* [soydos](https://github.com/soydos/pusoy_dos2) — A wasm implementation of Pusoy Dos
* [cristicbz/rust-doom](https://github.com/cristicbz/rust-doom) — A renderer for Doom, may progress to being a playable game
* [Thinkofname/rust-quake](https://github.com/Thinkofname/rust-quake) — Quake map renderer in Rust
* [rhex](https://github.com/dpc/rhex) — hexagonal ascii roguelike
* [garkimasera/rusted-ruins](https://github.com/garkimasera/rusted-ruins) — Extensible open world rogue like game with pixel art
* [Veloren](https://gitlab.com/veloren/veloren) — An open world, open source multiplayer voxel RPG game currently in alpha development
* [swatteau/sokoban-rs](https://github.com/swatteau/sokoban-rs) — A Sokoban implementation
* [aleshaleksey/TGWM](https://github.com/aleshaleksey/TGWM) — An RPG with turned-based mechanics (work in progress)
* [ozkriff/zemeroth](https://github.com/ozkriff/zemeroth) — A small 2D turn-based hexagonal strategy game
* [Zone of Control](https://github.com/ozkriff/zoc) — A turn-based hexagonal strategy game
* [KostasKyriakou/snake_game](https://github.com/KostasKyriakou/snake_game) - Simple terminal snake game written in Rust.

### Graphics

* [Limeth/euclider](https://github.com/Limeth/euclider) — A real-time 4D CPU ray tracer
* [RazrFalcon/resvg](https://github.com/RazrFalcon/resvg) — An SVG rendering library.
* [ivanceras/svgbob](https://github.com/ivanceras/svgbob) — converts ASCII diagrams into SVG graphics
* [RazrFalcon/svgcleaner](https://github.com/RazrFalcon/svgcleaner) — tidies SVG graphics
* [Twinklebear/tray_rust](https://github.com/Twinklebear/tray_rust) — A ray tracer
* [turnage/valora](https://crates.io/crates/valora) — A library for generative fine art
* Image processing:
  * [mikigraf/Image-Processing-CLI-in-Rust](https://github.com/mikigraf/Image-Processing-CLI-in-Rust) — CLI for processing images, generating histograms.

### Industrial automation
* [locka99/opcua](https://github.com/locka99/opcua) — A pure rust [OPC UA](https://opcfoundation.org/about/opc-technologies/opc-ua/) library.
* [slowtec/tokio-modbus](https://github.com/slowtec/tokio-modbus) — A [tokio](https://tokio.rs)-based [modbus](https://modbus.org) library.
* [BiancoRoyal/modbus-iiot-rust](https://github.com/BiancoRoyal/modbus-iiot-rust) — A pure rust [modbus](https://modbus.org) library with no or less dependencies.

### Observability

* [timberio/vector](https://github.com/timberio/vector) — A High-Performance, Logs, Metrics, & Events Router.
* [Mnwa/gtsa](https://github.com/Mnwa/gtsa) — A simple solution to proxy gelf messages (messages for Graylog) to Sentry
* [OpenTelemetry](https://crates.io/crates/opentelemetry) — OpenTelemetry provides a single set of APIs, libraries, agents, and collector services to capture distributed traces and metrics from your application. You can analyze them using Prometheus, Jaeger, and other observability tools.

### Operating systems

See also [A comparison of operating systems written in Rust](https://github.com/flosse/rust-os-comparison).

* [nebulet/nebulet](https://github.com/nebulet/nebulet) — A microkernel that implements a WebAssembly "usermode" that runs in Ring 0.
* [redox-os/redox](https://gitlab.redox-os.org/redox-os/redox)
* [thepowersgang/rust_os](https://github.com/thepowersgang/rust_os)
* [tock/tock](https://github.com/tock/tock) — A secure embedded operating system for Cortex-M based microcontrollers

### Productivity
* [espanso](https://github.com/federico-terzi/espanso) — A cross-platform Text Expander written in Rust
* [eureka](https://crates.io/crates/eureka) — A CLI tool to input and store your ideas without leaving the terminal
* [pier-cli/pier](https://github.com/pier-cli/pier) — A central repository to manage (add, search metadata, etc.) all your one-liners, scripts, tools, and CLIs
* [subilo](https://github.com/Bansco/subilo) - A continuous deployment agent

### Security tools

* [kpcyrd/badtouch](https://github.com/kpcyrd/badtouch) — A scriptable network authentication cracker
* [lethe](https://github.com/kostassoid/lethe) — A secure cross-platform drive wiping utility
* [arvancloud/libinjection-rs](https://github.com/arvancloud/libinjection-rs) — Rust bindings for [libinjection](https://github.com/client9/libinjection)
* [ripasso](https://github.com/cortex/ripasso/) — A password manager, filesystem compatible with pass
* [kpcyrd/rshijack](https://github.com/kpcyrd/rshijack) — A TCP connection hijacker, rust rewrite of shijack
* [rustscan/rustscan](https://github.com/RustScan/RustScan) — Make Nmap faster with this port scanning tool
* [kpcyrd/sniffglue](https://github.com/kpcyrd/sniffglue) — A secure multithreaded packet sniffer
* [kpcyrd/sn0int](https://github.com/kpcyrd/sn0int) — A semi-automatic OSINT framework and package manager
* [phra/rustbuster](https://github.com/phra/rustbuster) — A Comprehensive Web Fuzzer and Content Discovery Tool

### System tools

* [ajeetdsouza/zoxide](https://github.com/ajeetdsouza/zoxide/) — A fast alternative to `cd` that learns your habits
* [bandwhich](https://github.com/imsnif/bandwhich) — Terminal bandwidth utilization tool
* [brocode/fblog](https://github.com/brocode/fblog) — Small command-line JSON Log viewer
* [buster/rrun](https://github.com/buster/rrun) — A command launcher for Linux, similar to gmrun
* [cristianoliveira/funzzy](https://github.com/cristianoliveira/funzzy) — A configurable filesystem watcher inspired by [entr](http://entrproject.org/)
* [dalance/procs](https://github.com/dalance/procs) — A modern replacement for 'ps' written by Rust
* [diskonaut](https://github.com/imsnif/diskonaut) — Terminal visual disk space navigator
* [dust](https://github.com/bootandy/dust) — A more intuitive version of du
* [ddh](https://github.com/darakian/ddh) — Fast duplicate file finder
* [fselect](https://crates.io/crates/fselect) — Find files with SQL-like queries
* [gitui](https://github.com/extrawurst/gitui) - Blazing fast terminal client for git written in Rust.
* [k0pernicus/zou](https://github.com/k0pernicus/zou) — A download accelerator
* [Kondo](https://github.com/tbillington/kondo) - CLI & GUI tool for deleting software project artifacts and reclaiming disk space
* [lotabout/rargs](https://github.com/lotabout/rargs) [[rargs](https://crates.io/crates/rargs)] — xargs + awk with pattern matching support
* [lotabout/skim](https://github.com/lotabout/skim) — A fuzzy finder in pure rust
* [mitnk/cicada](https://github.com/mitnk/cicada) — A bash-like Unix shell
* [mmstick/concurr](https://github.com/mmstick/concurr) — Alternative to GNU Parallel w/ a client-server architecture
* [mmstick/fontfinder](https://github.com/mmstick/fontfinder) — GTK3 application for previewing and installing Google's fonts
* [mmstick/parallel](https://github.com/mmstick/parallel) — Reimplementation of GNU Parallel
* [mmstick/tv-renamer](https://github.com/mmstick/tv-renamer) — A tv series renaming application with an optional GTK3 frontend.
* [organize-rt](https://gitlab.com/FixFromDarkness/organize-rt) — Organize files based on regex rules (file extensions by default).
* [orhun/kmon](https://github.com/orhun/kmon) — Linux Kernel Manager and Activity Monitor
* [Peltoche/lsd](https://github.com/Peltoche/lsd) — An ls with a lot of pretty colors and awesome icons
* [ogham/exa](https://github.com/ogham/exa) — A replacement for 'ls'
* [pop-os/debrep](https://github.com/pop-os/debrepbuild) — APT repository tool for building and managing an APT repo
* [pop-os/popsicle](https://github.com/pop-os/popsicle) — GTK3 & CLI utility for flashing multiple USB devices in parallel
* [Luminarys/synapse](https://github.com/Luminarys/synapse) — Flexible and fast BitTorrent daemon.
* [pop-os/system76-power](https://github.com/pop-os/system76-power/) — Linux power management daemon (DBus-interface) with CLI tool.
* [Ralvke/logram](https://github.com/Ralvke/logram) — Push log files' updates to Telegram
* [redox-os/ion](https://github.com/redox-os/ion) — Next-generation system shell
* [unwrittenfun/hotkey-rs](https://github.com/unwrittenfun/hotkey-rs) — A library to listen to global hotkeys in Rust
* [nivekuil/rip](https://github.com/nivekuil/rip) - A safe and ergonomic alternative to `rm`
* [sharkdp/bat](https://github.com/sharkdp/bat) — A cat(1) clone with wings.
* [sharkdp/fd](https://github.com/sharkdp/fd) — A simple, fast and user-friendly alternative to find.
* [sitkevij/hex](https://github.com/sitkevij/hex) — A colorized hexdump terminal utility.
* [slai11/goto](https://github.com/slai11/goto) — A simple and user-friendly way to jump to your indexed directories. [
* [m4b/bingrep](https://github.com/m4b/bingrep) — Greps through binaries from various OSs and architectures, and colors them.
* [uutils/coreutils](https://github.com/uutils/coreutils) — A cross-platform Rust rewrite of the GNU coreutils
* [watchexec](https://github.com/watchexec/watchexec) — Executes commands in response to file modifications
* [XAMPPRocky/tokei](https://github.com/XAMPPRocky/tokei) — counts the lines of code
* [yake](https://crates.io/crates/yake) — Yake is a task runner based on yaml files
* [ytop](https://github.com/cjbassi/ytop) - A TUI system monitor written in Rust

### Text editors

* [amp](https://amp.rs) — Inspired by Vi/Vim.
* [gchp/iota](https://github.com/gchp/iota) — A simple text editor
* [ilai-deutel/kibi](https://github.com/ilai-deutel/kibi) — A tiny (≤1024 LOC) text editor with syntax highlighting, incremental search and more.
* [mathall/rim](https://github.com/mathall/rim) — Vim-like text editor written in Rust
* [Remacs](https://github.com/remacs/remacs) — A community-driven port of Emacs to Rust.
* [xi-editor](https://github.com/xi-editor/xi-editor) — A modern editor with a backend written in Rust.
* [xray](https://github.com/atom-archive/xray) — An experimental next-generation Electron-based text editor.

### Text processing

* [cpc](https://github.com/probablykasper/cpc) - Parses and calculates strings of math with support for units and unit conversion, from `1+2` to `1% of round(1 lightyear / 14!s to km/h)`.
* [grex](https://github.com/pemistahl/grex) — A command-line tool and library for generating regular expressions from user-provided test cases
* [TankerHQ/ruplacer](https://github.com/TankerHQ/ruplacer) — Find and replace text in source files
* [ripgrep](https://crates.io/crates/ripgrep) — combines the usability of The Silver Searcher with the raw speed of grep
* [phiresky/ripgrep-all](https://github.com/phiresky/ripgrep-all) — ripgrep, but also search in PDFs, E-Books, Office documents, zip, tar.gz, etc.
* [replicadse/complate](https://github.com/replicadse/complate) — An in-terminal text templating tool designed for standardizing messages (like for GIT commits).
* [sd](https://crates.io/crates/sd) — Intuitive find & replace CLI
* [lavifb/todo_r](https://github.com/lavifb/todo_r) — Find all your TODO notes with one command!
* [whitfin/runiq](https://github.com/whitfin/runiq) — an efficient way to filter duplicate lines from unsorted input.
* [whitfin/bytelines](https://github.com/whitfin/bytelines) — Read input lines as byte slices for high efficiency.
* [vishaltelangre/ff](https://github.com/vishaltelangre/ff) — Find files (ff) by name!
* [xsv](https://crates.io/crates/xsv) — A fast CSV command line tool (slicing, indexing, selecting, searching, sampling, etc.)
* [Lisprez/so_stupid_search](https://github.com/Lisprez/so_stupid_search) — A simple and fast string search tool for human beings

### Image processing

* [Imager](https://github.com/imager-io/imager) — Automated image optimization.

### Utilities

* [aleshaleksey/AZDice](https://github.com/aleshaleksey/AZDice) — A dice roll success distribution generator for tabletop homebrewers.
* [yaa110/cb](https://github.com/yaa110/cb) — Command line interface to manage clipboard
* [brycx/checkpwn](https://github.com/brycx/checkpwn) — A Have I Been Pwned (HIBP) command-line utility tool that lets you easily check for compromised accounts and passwords.
* [evansmurithi/cloak](https://github.com/evansmurithi/cloak) — A Command Line OTP (One Time Password) Authenticator application.
* [arthrp/consoletimer](https://github.com/arthrp/consoleTimer) — Simple timer for your terminal.
* [tversteeg/emplace](https://github.com/tversteeg/emplace) — Synchronize installed packages on multiple machines
* [myfreeweb/freepass](https://github.com/myfreeweb/freepass) — The free password manager for power users.
* [yoannfleurydev/gitweb](https://github.com/yoannfleurydev/gitweb) — Open the current remote repository in your browser.
* [mme](https://github.com/GoberInfinity/mme) — Command line tool to remember you commands that you sometimes forget.
* [raftario/licensor](https://github.com/raftario/licensor) — write licenses to stdout [
* [arthrp/quick-skeleton](https://github.com/arthrp/quick-skeleton) — Project scaffolding tool, similar to Yeoman and Slush.
* [repoch](https://github.com/lucawen/repoch) — Convert epoch to datetime and datetime to epoch
* [whitfin/s3-concat](https://github.com/whitfin/s3-concat) — A command line tool to concatenate Amazon S3 files remotely using flexible patterns.
* [whitfin/s3-meta](https://github.com/whitfin/s3-meta) — A command line tool to gather metadata about your Amazon S3 buckets.
* [whitfin/s3-utils](https://github.com/whitfin/s3-utils) — A small tool containing utilities based around Amazon S3 to provide additional convenience APIs.
* [gorros/s3-edit-rs](https://github.com/gorros/s3-edit-rs) — A command line tool to edit a file directly on Amazon S3.
* [fcsonline/tmux-thumbs](https://github.com/fcsonline/tmux-thumbs) — A lightning fast version of tmux-fingers written in Rust, copy/pasting tmux like vimium/vimperator.
* [amar-laksh/workstation](https://github.com/amar-laksh/workstation) — A commandline tool to help you manage your workstation by distancing you from your screen, locking your screen when you aren't there among other things with OPENCV!
* [guoxbin/dtool](https://github.com/guoxbin/dtool) — A useful command-line tool collection to assist development including conversion, codec, hashing, encryption, etc.
* [nomino](https://github.com/yaa110/nomino) — Batch rename utility for developers
* [barberousse](https://github.com/zeapo/barberousse) — AWS Secrets Manager editor ![build](https://github.com/zeapo/barberousse/workflows/build/badge.svg?branch=master)

### Video

* [tgotwig/vidmerger](https://github.com/tgotwig/vidmerger) —  A wrapper around ffmpeg which simplifies merging multiple videos 🎞
* [xiph/rav1e](https://github.com/xiph/rav1e) — The fastest and safest AV1 encoder.
* [yuvadm/slingr](https://github.com/yuvadm/slingr) — A simple CLI for streaming media files over a local network to UPnP media renderers
* [yuvadm/streamlib](https://github.com/streamlib/streamlib) — Play your favorite live video and audio streams from command line

### Virtualization

* [firecracker-microvm/firecracker](https://github.com/firecracker-microvm/firecracker) — A lightweight virtual machine for container workload  [Firecracker Microvm](https://firecracker-microvm.github.io/)
* [oracle/railcar](https://github.com/oracle/railcar) — Docker-like container OCI runtime implementation in Rust
* [tailhook/vagga](https://github.com/tailhook/vagga) — A containerization tool without daemons

### Web

* [Plume-org/Plume](https://github.com/Plume-org/Plume) — ActivityPub federating blogging application
* [LemmyNet/lemmy](https://github.com/LemmyNet/lemmy) — A link aggregator / reddit clone for the fediverse

### Web Servers

* [thecoshman/http](https://github.com/thecoshman/http) — Host These Things Please — A basic http server for hosting a folder fast and simply
* [svenstaro/miniserve](https://github.com/svenstaro/miniserve) — A small, self-contained cross-platform CLI tool that allows you to just grab the binary and serve some file(s) via HTTP
* [TheWaWaR/simple-http-server](https://github.com/TheWaWaR/simple-http-server) — simple static http server
* [wyhaya/see](https://github.com/wyhaya/see) — Static HTTP file server

## Development tools

* [clippy](https://crates.io/crates/clippy) — Rust lints
* [clog-tool/clog-cli](https://github.com/clog-tool/clog-cli) — generates a changelog from git metadata ([conventional changelog](https://blog.thoughtram.io/announcements/tools/2014/09/18/announcing-clog-a-conventional-changelog-generator-for-the-rest-of-us.html))
* [dan-t/rusty-tags](https://github.com/dan-t/rusty-tags) — create ctags/etags for a cargo project and all of its dependencies
* [delta](https://crates.io/crates/git-delta) — A syntax-highlighter for git and diff output
* [dotenv-linter](https://github.com/dotenv-linter/dotenv-linter) — Linter for `.env` files
* [frewsxcv/crate-deps](https://github.com/frewsxcv/crate-deps) — generates images of dependency graphs for crates hosted on crates.io
* [git-journal](https://github.com/saschagrunert/git-journal/) — The Git Commit Message and Changelog Generation Framework
* [gstats](https://github.com/boonshift/gstats/) — command line tool to print a developer handy summary of all git repositories below current directory
* [rust-lang/rustfix](https://github.com/rust-lang/rustfix)  — automatically applies the suggestions made by rustc
* [just](https://github.com/casey/just) — A handy command runner for project-specific tasks
* [mask](https://github.com/jakedeichert/mask) — A CLI task runner defined by a simple markdown file
* [Module Linker](https://github.com/fiatjaf/module-linker) — Extension that adds `<a>` links to references in `mod`, `use` and `extern crate` statements at GitHub.
* [ptags](https://github.com/dalance/ptags) — A parallel universal-ctags wrapper for git repository
* [Racer](https://github.com/racer-rust/racer) — code completion for Rust
* [rustfmt](https://github.com/rust-lang/rustfmt) — A Rust code formatter
* [Rustup](https://github.com/rust-lang/rustup) — the Rust toolchain installer
* [Rust Language Server](https://github.com/rust-lang/rls) — A server that runs in the background, providing IDEs, editors, and other tools with information about Rust programs
* [Rust Regex Playground](https://2fd.github.io/rust-regex-playground/#method=find&regex=%5Cw%2B&text=abc) — Web tool to evaluate rust regular expressions
* [Rust Search Extension](https://github.com/huhu/rust-search-extension) — A handy browser extension to search crates and docs in address bar (omnibox).
* [artifact](https://github.com/vitiral/artifact) — the design doc tool made for developers
* [semantic-rs](https://github.com/semantic-rs/semantic-rs) — automatic crate publishing
* [fw](https://github.com/brocode/fw) — workspace productivity booster
* [tinyrick](https://github.com/mcandre/tinyrick) a basic task dependency tool emphasizing Rust functions over raw shell commands.
* [scriptisto](https://github.com/igor-petruk/scriptisto) A language-agnostic "shebang interpreter" that enables you to write one file scripts in compiled languages.

### Build system

* [Cargo](https://crates.io/) — the Rust package manager
  * [cargo-benchcmp](https://crates.io/crates/cargo-benchcmp) — A utility to compare Rust micro-benchmarks
  * [cargo-bitbake](https://crates.io/crates/cargo-bitbake) — A cargo extension that can generate BitBake recipes utilizing the classes from meta-rust
  * [cargo-cache](https://crates.io/crates/cargo-cache) — inspect/manage/clean your cargo cache (`~/.cargo/`/`${CARGO_HOME}`), print sizes etc
  * [cargo-check](https://crates.io/crates/cargo-check) — A wrapper around `cargo rustc -- -Zno-trans` which can be helpful for running a faster compile if you only need correctness checks
  * [cargo-count](https://crates.io/crates/cargo-count) — lists source code counts and details about cargo projects, including unsafe statistics
  * [cargo-deb](https://crates.io/crates/cargo-deb) — Generates binary Debian packages
  * [cargo-deps](https://crates.io/crates/cargo-deps) — build dependency graphs of Rust projects
  * [cargo-do](https://crates.io/crates/cargo-do) — run multiple cargo commands in a row
  * [cargo-ebuild](https://crates.io/crates/cargo-ebuild) — cargo extension that can generate ebuilds using the in-tree eclasses
  * [cargo-edit](https://crates.io/crates/cargo-edit) — allows you to add and list dependencies by reading/writing to your Cargo.toml file from the command line
  * [cargo-find](https://crates.io/crates/cargo-find) <sup>deprecated</sup> — Find crates from command line
  * [cargo-generate](https://github.com/ashleygwilliams/cargo-generate) A generator of a rust project by leveraging a pre-existing git repository as a template
  * [cargo-graph](https://crates.io/crates/cargo-graph) — updated fork of `cargo-dot` with additional features. Unmaintained, see `cargo-deps`
  * [cargo-info](https://crates.io/crates/cargo-info) — queries crates.io for crates details from command line
  * [cargo-license](https://crates.io/crates/cargo-license) — A cargo subcommand to quickly view the licenses of all dependencies.
  * [cargo-make](https://crates.io/crates/cargo-make) — Rust task runner and build tool.
  * [cargo-modules](https://crates.io/crates/cargo-modules) — A cargo plugin for showing a tree-like overview of a crate's modules.
  * [cargo-multi](https://crates.io/crates/cargo-multi) — runs specified cargo command on multiple crates
  * [cargo-outdated](https://crates.io/crates/cargo-outdated) — displays when newer versions of Rust dependencies are available, or out of date
  * [cargo-release](https://crates.io/crates/cargo-release) — tool for releasing git-managed cargo project, build, tag, publish, doc and push
  * [cargo-script](https://crates.io/crates/cargo-script) — lets people quickly and easily run Rust "scripts" which can make use of Cargo's package ecosystem
  * [cargo-testify](https://crates.io/crates/cargo-testify) — watches files changes, runs tests and notifies about the result with friendly OS notification
  * [cargo-tree](https://github.com/sfackler/cargo-tree) – Cargo subcommand that visualizes a crate's dependency graph in a tree-like format
  * [cargo-update](https://crates.io/crates/cargo-update) — cargo subcommand for checking and applying updates to installed executables
  * [cargo-watch](https://crates.io/crates/cargo-watch) — utility for cargo to compile projects when sources change
  * [liuchong/cargo-x](https://github.com/liuchong/cargo-x) — A very simple third-party cargo subcommand to execute a custom command
  * [dtolnay/cargo-expand](https://github.com/dtolnay/cargo-expand) — Expand macros in your source code
* CMake
  * [Devolutions/CMakeRust](https://github.com/Devolutions/CMakeRust) — useful for integrating a Rust library into a CMake project
  * [SiegeLord/RustCMake](https://github.com/SiegeLord/RustCMake) — an example project showing usage of CMake with Rust
* Github actions
  * [icepuma/rust-action](https://github.com/icepuma/rust-action) — rust github action
  * [peaceiris/actions-mdbook](https://github.com/peaceiris/actions-mdbook) — GitHub Actions for mdBook
* GitHub webhooks
  * [snare](https://tratt.net/laurie/src/snare/) — GitHub webhooks runner daemon
* Webpack
  * [Ralvke/rust-loader](https://github.com/Ralvke/rust-loader) — Webpack Rust loader (wasm)

### Debugging

* GDB
  * [rust-gdb](https://github.com/rust-lang/rust/blob/master/src/etc/rust-gdb)
  * [gdbgui](https://github.com/cs01/gdbgui) — Browser based frontend for gdb to debug C, C++, Rust, and go.
* LLDB
  * [lldb_batchmode.py](https://github.com/rust-lang/rust/blob/master/src/etc/lldb_batchmode.py) — allows to use LLDB in a way similar to GDB's batch mode.
  * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) — A LLDB extension for [Visual Studio Code](https://code.visualstudio.com/).

### Deployment

* Docker
  * [emk/rust-musl-builder](https://github.com/emk/rust-musl-builder) — Docker images for compiling static Rust binaries using musl-libc and musl-gcc, with static versions of useful C libraries
  * [kpcyrd/mini-docker-rust](https://github.com/kpcyrd/mini-docker-rust) — An example project for very small rust docker images
  * [liuchong/docker-rustup](https://github.com/liuchong/docker-rustup) — A multiple version (with musl tools) Rust Docker image
  * [messense/rust-musl-cross](https://github.com/messense/rust-musl-cross) — Docker images for compiling static Rust binaries using musl-cross
  * [rust-lang-nursery/docker-rust](https://github.com/rust-lang/docker-rust) — the official Rust Docker image
* Google App Engine
  * [DenisKolodin/rust-app-engine](https://github.com/DenisKolodin/rust-app-engine) — App Engine Rust boilerplate
* Heroku
  * [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust) — A buildpack for Rust applications on Heroku

### Embedded

[Rust Embedded](https://rust-embedded.org/)

* Cross compiling
  * [japaric/rust-cross](https://github.com/japaric/rust-cross) — everything you need to know about cross compiling Rust programs
  * [japaric/xargo](https://github.com/japaric/xargo) — effortless cross compilation of Rust programs to custom bare-metal targets like ARM Cortex-M
* Raspberry Pi
  * [Ogeon/rust-on-raspberry-pi](https://github.com/Ogeon/rust-on-raspberry-pi) — instructions for how to cross compile Rust projects for the Raspberry Pi .
* Arduino
  * [avr-rust/ruduino](https://github.com/avr-rust/ruduino) ^`^t Reusable components for the Arduino Uno.

### FFI

See also [Foreign Function Interface](https://doc.rust-lang.org/book/first-edition/ffi.html),  [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) (a collection of examples of using code written in Rust from other languages) and [FFI examples written in Rust](https://github.com/alexcrichton/rust-ffi-examples).

* C
  * [rlhunt/cbindgen](https://github.com/eqrion/cbindgen) — generates C header files from Rust source files. Used in Gecko for WebRender
  * [Sean1708/rusty-cheddar](https://github.com/Sean1708/rusty-cheddar) — generates C header files from Rust source files
* C++
  * [rust-lang/rust-bindgen](https://github.com/rust-lang/rust-bindgen) — A Rust bindings generator
  * [dtolnay/cxx](https://github.com/dtolnay/cxx) — Safe interop between Rust and C++
  * [rust-cpp](https://crates.io/crates/cpp) - Embed C++ code directly in Rust.
* Erlang
  * [rusterlium/rustler](https://github.com/rusterlium/rustler) — safe Rust bridge for creating Erlang NIF functions
* Haskell
  * [mgattozzi/curryrs](https://github.com/mgattozzi/curryrs) — Bridge the gap between Haskell and Rust
  * [mgattozzi/haskellrs](https://github.com/mgattozzi/haskellrs) — Rust in Haskell FFI Example
  * [mgattozzi/rushs](https://github.com/mgattozzi/rushs) — Haskell in Rust FFI Example
* Java
  * [j4rs](https://crates.io/crates/j4rs) — use Java from Rust
  * [bennettanderson/rjni](https://github.com/benanders/rjni) — use Java from Rust
  * [drrb/java-rust-example](https://github.com/drrb/java-rust-example) — use Rust from Java
  * [jni](https://crates.io/crates/jni) — use Rust from Java
  * [jni-sys](https://crates.io/crates/jni-sys) — Rust definitions corresponding to jni.h
  * [rucaja](https://crates.io/crates/rucaja) — use Java from Rust
  * [rawrafox/rust-jdbc](https://github.com/rawrafox/rust-jdbc) — uses JDBC from Rust
* Lua
  * [jcmoyer/rust-lua53](https://github.com/jcmoyer/rust-lua53) — Lua 5.3 bindings for Rust
  * [lilyball/rust-lua](https://github.com/lilyball/rust-lua) — Safe Rust bindings to Lua 5.1
  * [tickbh/td_rlua](https://github.com/tickbh/td_rlua) — Zero-cost high-level lua 5.3 wrapper for Rust
  * [tomaka/hlua](https://github.com/tomaka/hlua) — Rust library to interface with Lua
* mruby
  * [anima-engine/mrusty](https://github.com/anima-engine/mrusty) — mruby safe bindings for Rust
* Node.js
  * [neon-bindings/neon](https://github.com/neon-bindings/neon) — Rust bindings for writing safe and fast native Node.js modules
  * [infinyon/node-bindgen](https://github.com/infinyon/node-bindgen) - Easy way to generate nodejs module using Rust
* Objective-C
  * [SSheldon/rust-objc](https://github.com/SSheldon/rust-objc) — Objective-C Runtime bindings and wrapper for Rust
* Perl
  * [vickenty/mi-rust](https://github.com/vickenty/mi-rust) — Adds support to M::I for building modules with Cargo
  * [vickenty/perl-xs](https://github.com/vickenty/perl-xs) — Create Perl XS modules using Rust
* Python
  * [getsentry/milksnake](https://github.com/getsentry/milksnake) — extension for python setuptools that allows you to distribute dynamic linked libraries in Python wheels in the most portable way imaginable.
  * [dgrunwald/rust-cpython](https://github.com/dgrunwald/rust-cpython) — Python bindings
  * [PyO3/PyO3](https://github.com/PyO3/PyO3) — Rust bindings for the Python interpreter
* Ruby
  * [d-unseductable/ruru](https://github.com/d-unseductable/ruru) — native Ruby extensions written in Rust
  * [danielpclark/rutie](https://github.com/danielpclark/rutie) — native Ruby extensions written in Rust and vice versa
  * [tildeio/helix](https://github.com/tildeio/helix) — write Ruby classes in Rust
* Web Assembly
  * [rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack) — :package: :sparkles: pack up the wasm and publish it to npm!
  * [rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — A project for facilitating high-level interactions between wasm modules and JS.
  * [rhysd/wain](https://github.com/rhysd/wain) - wain: WebAssembly INterpreter from scratch in Safe Rust with zero dependency

### IDEs

See also [Are we (I)DE yet?](https://areweideyet.com/) and [Rust Tools](https://www.rust-lang.org/tools).

  * [Atom](https://atom.io/)
    * [zargony/atom-language-rust](https://github.com/zargony/atom-language-rust)
    * [rust-lang/atom-ide-rust](https://github.com/rust-lang/atom-ide-rust) — Rust IDE support for Atom, powered by the Rust Language Server (RLS)
  * [Eclipse](https://www.eclipse.org/)
    * [Eclipse Corrosion](https://github.com/eclipse/corrosion)
    * [RustDT](https://github.com/RustDT/RustDT)
  * [Emacs](https://www.gnu.org/software/emacs/)
    * [rust-mode](https://github.com/rust-lang/rust-mode) — Rust Major Mode
    * [rustic](https://github.com/brotzeit/rustic) - Rust development environment for Emacs
    * [flycheck-rust](https://github.com/flycheck/flycheck-rust) — Rust support for [Flycheck](https://github.com/flycheck/flycheck)
    * [emacs-racer](https://github.com/racer-rust/emacs-racer) — Autocompletion (see also [company](https://company-mode.github.io) and [auto-complete](https://github.com/auto-complete/auto-complete))
  * [gitpod.io](https://gitpod.io) — Online IDE with full Rust support based on Rust Language Server
  * [gnome-builder](https://wiki.gnome.org/Apps/Builder) native support for rust and cargo since Version 3.22.2
  * [Kakoune](http://kakoune.org/)
    * [ul/kak-lsp](https://github.com/ul/kak-lsp/) — [LSP](https://microsoft.github.io/language-server-protocol/) client. Implemented in Rust and supports rls out of the box.
  * [NetBeans](https://netbeans.org/)
    * [drrb/rust-netbeans](https://github.com/drrb/rust-netbeans)
  * [IntelliJ](https://www.jetbrains.com/idea/)
    * [intellij-rust/intellij-rust](https://github.com/intellij-rust/intellij-rust)
    * [intellij-rust/intellij-toml](https://github.com/intellij-rust/intellij-toml) — basic Toml support
  * [Ride](https://github.com/madeso/ride)
  * [SolidOak](https://github.com/oakes/SolidOak) — A simple IDE for Rust, based on GTK+ and [Neovim](https://github.com/neovim/neovim)
  * [Sublime Text](https://www.sublimetext.com/)
    * [rust-lang/rust-enhanced](https://github.com/rust-lang/rust-enhanced) — official Rust package
    * [sublimehq/packages](https://github.com/sublimehq/Packages/tree/master/Rust) — native Sublime support (already installed)
  * [Vim](https://vim.sourceforge.io/) — the ubiquitous text editor
    * [rust.vim](https://github.com/rust-lang/rust.vim) — provides file detection, syntax highlighting, formatting, Syntastic integration, and more.
    * [vim-cargo](https://github.com/timonv/vim-cargo) — command bindings to quickly run cargo stuff from vim.
    * [vim-racer](https://github.com/racer-rust/vim-racer) — allows vim to use [Racer](https://github.com/racer-rust/racer) for Rust code completion and navigation.
    * [autozimu/LanguageClient-neovim](https://github.com/autozimu/LanguageClient-neovim) — [LSP](https://microsoft.github.io/language-server-protocol/) client. Implemented in Rust and supports rls out of the box.
  * Visual Studio
    * [PistonDevelopers/VisualRust](https://github.com/PistonDevelopers/VisualRust) — A Visual Studio extension for Rust
    * [dgriffen/rls-vs2017](https://github.com/ZoeyR/rls-vs2017) — Rust support for Visual Studio 2017 Preview
  * [Visual Studio Code](https://code.visualstudio.com/)
    * [rust-lang/rls-vscode](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust) — Rust support for Visual Studio Code
    * [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) — An alternative rust language server to the RLS
    * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) — A LLDB extension
    * [crates](https://github.com/serayuzgur/crates) — crates is an extension for crates.io dependencies.

### Pattern recognition

* [sfikas/rusteval](https://github.com/sfikas/rusteval) — A tool used to evaluate the output of retrieval algorithms

### Profiling

* [bheisler/criterion.rs](https://github.com/bheisler/criterion.rs) — Statistics-driven benchmarking library for Rust
* [sharkdp/hyperfine](https://github.com/sharkdp/hyperfine) — A command-line benchmarking tool
* [performancecopilot/hornet](https://github.com/performancecopilot/hornet) — A Performance Co-Pilot memory-mapped values instrumentation library
* [koute/memory-profiler](https://github.com/koute/memory-profiler) — A memory profiler for Linux
* [ellisonch/rust-stopwatch](https://github.com/ellisonch/rust-stopwatch) — A stopwatch library
* FlameGraphs
  * [mrhooray/torch](https://github.com/mrhooray/torch) — generates FlameGraphs based on DWARF Debug Info
  * [llogiq/flame](https://github.com/llogiq/flame)

### Services

* [deps.rs](https://github.com/srijs/deps.rs) — Detect outdated or insecure dependencies
* [docs.rs](https://docs.rs) — Automatic documentation generation of crates

### Static analysis

[[assert](https://crates.io/keywords/assert), [static](https://crates.io/keywords/static)]

* [facebookexperimental/MIRAI](https://github.com/facebookexperimental/mirai) — an abstract interpreter operating on Rust's mid-level intermediate representation (MIR)
* [static_assertions](https://crates.io/crates/static_assertions) — Compile-time assertions to ensure that invariants are met

### Testing

[[testing](https://crates.io/keywords/testing)]

* [demonstrate](https://crates.io/crates/demonstrate) — Declarative Testing Framework
* [httpmock](https://github.com/alexliesenfeld/httpmock) — HTTP mocking
* [mockiato](https://crates.io/crates/mockiato) — A strict, yet friendly mocking library for Rust 2018
* [mutagen](https://crates.io/crates/mutagen) — A source-level mutation testing framework (nightly only)
* [AlKass/polish](https://github.com/AlKass/polish) — Mini Testing/Test-Driven Framework
* [proptest](https://crates.io/crates/proptest) — property testing framework inspired by the [Hypothesis](https://hypothesis.works/) framework for Python
* [quickcheck](https://crates.io/crates/quickcheck) — A Rust implementation of [QuickCheck](https://wiki.haskell.org/Introduction_to_QuickCheck1)
* [mockito](https://crates.io/crates/mockito) — HTTP mocking
* [speculate](https://crates.io/crates/speculate) — An RSpec inspired minimal testing framework for Rust
* [rstest](https://crates.io/crates/rstest) — Fixture-based test framework for Rust
* [ruspec](https://crates.io/crates/ruspec) — Write like Rspec testing framework with rust
* [rust-fuzz/afl.rs](https://github.com/rust-fuzz/afl.rs) — A Rust fuzzer, using [AFL](https://lcamtuf.coredump.cx/afl/)
* [tarpaulin](https://crates.io/crates/cargo-tarpaulin) — A code coverage tool designed for Rust
* [trust](https://github.com/japaric/trust) — A Travis CI and AppVeyor template to test your Rust crate on 5 architectures and publish binary releases of it for Linux, macOS and Windows
* [fake-rs](https://github.com/cksac/fake-rs) —  A library for generating fake data
* [goldenfile](https://github.com/calder/rust-goldenfile) - A library providing a simple API for goldenfile testing.

### Transpiling

* [immunant/c2rust](https://github.com/immunant/c2rust) — C to Rust translator and cross checker built atop Clang/LLVM.
* [jameysharp/corrode](https://github.com/jameysharp/corrode) — A C to Rust translator written in Haskell.


## Libraries

### Artificial Intelligence

#### Genetic algorithms

* [Martin1887/oxigen](https://github.com/Martin1887/oxigen) — Fast, parallel, extensible and adaptable genetic algorithm library. A example using this library solves the N Queens problem for N = 255 in only few seconds and using less than 1 MB of RAM.
* [innoave/genevo](https://github.com/innoave/genevo) — Execute genetic algorithm (GA) simulations in a customizable and extensible way.
* [willi-kappler/darwin-rs](https://github.com/willi-kappler/darwin-rs) — Evolutionary algorithms with Rust
* [m-decoster/RsGenetic](https://github.com/m-decoster/RsGenetic) — Genetic Algorithm library in Rust. In maintenance mode.
* [mneumann/evo-rs](https://github.com/mneumann/evo-rs) — Evolutionary Algorithm Library for Rust. Without changes for 3 years.
* [yurytsoy/revonet](https://github.com/yurytsoy/revonet) — Rust implementation of real-coded GA for solving optimization problems and training of neural networks.
* [pkalivas/radiate](https://github.com/pkalivas/radiate) — A customizable parallel genetic programming engine capable of evolving solutions for supervised, unsupervised, and reinforcement learning problems. Comes with complete and customizable implementation of NEAT and Evtree.

#### Machine learning

[[machine learning](https://crates.io/keywords/machine-learning)]

See also [About Rust’s Machine Learning Community](https://medium.com/@autumn_eng/about-rust-s-machine-learning-community-4cda5ec8a790#.hvkp56j3f).

* [AtheMathmo/rusty-machine](https://github.com/AtheMathmo/rusty-machine) — Machine learning library for Rust
* [avinashshenoy97/RusticSOM](https://github.com/avinashshenoy97/RusticSOM) — Rust library for Self Organising Maps (SOM).
* [autumnai/leaf](https://github.com/autumnai/leaf) — Open Machine Intelligence framework.
* [tensorflow/rust](https://github.com/tensorflow/rust) — Rust language bindings for TensorFlow.
* [maciejkula/rustlearn](https://github.com/maciejkula/rustlearn) — Machine learning crate for Rust.
* [LaurentMazare/tch-rs](https://github.com/LaurentMazare/tch-rs) — Rust language bindings for PyTorch.
* [huggingface/tokenizers](https://github.com/huggingface/tokenizers) - Hugging Face's tokenizers for modern NLP pipelines written in Rust (original implementation) with bindings for Python.

### Astronomy

[[astronomy](https://crates.io/keywords/astronomy)]

* [saurvs/astro-rust](https://github.com/saurvs/astro-rust) — astronomy for Rust
* [fitsio](https://crates.io/crates/fitsio) — fits interface library wrapping cfitsio
* [flosse/rust-sun](https://github.com/flosse/rust-sun) — A rust port of the JS library suncalc

### Asynchronous

* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) — A coroutine I/O library with a working-stealing scheduler
* [dpc/mioco](https://github.com/dpc/mioco) — Scalable, coroutine-based, asynchronous IO handling library
* [TeaEntityLab/fpRust](https://github.com/TeaEntityLab/fpRust) — Monad/MonadIO, Handler, Coroutine/doNotation, Functional Programming features for Rust
* [rust-lang/futures-rs](https://github.com/rust-lang/futures-rs) — Zero-cost futures in Rust
* [mio](https://github.com/tokio-rs/mio) — MIO is a lightweight IO library for Rust with a focus on adding as little overhead as possible over the OS abstractions
* [Xudong-Huang/may](https://github.com/Xudong-Huang/may) — rust stackful coroutine library
* [rustasync/runtime](https://github.com/rustasync/runtime) — A runtime agnostic API designed to make async feel like its part of stdlib

### Audio and Music

[[audio](https://crates.io/keywords/audio)]

* [GuillaumeGomez/rust-fmod](https://github.com/GuillaumeGomez/rust-fmod) — [FMOD](https://www.fmod.com) bindings
* [jhasse/ears](https://github.com/jhasse/ears) — A simple library to play Sounds and Musics, on top of OpenAL and libsndfile
* [jpernst/alto](https://github.com/jpernst/alto) — OpenAL 1.1 bindings
* [musitdev/portmidi-rs](https://github.com/musitdev/portmidi-rs) — [PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings
* [hound](https://crates.io/crates/hound) — A WAV encoding and decoding library
* [RustAudio](https://github.com/RustAudio)
  * [RustAudio/cpal](https://github.com/RustAudio/cpal) - Low-level cross-platform audio I/O library in pure Rust.
  * [RustAudio/rodio](https://github.com/RustAudio/rodio) — A Rust audio playback library
  * [RustAudio/rust-portaudio](https://github.com/RustAudio/rust-portaudio) — [PortAudio](http://www.portaudio.com/) bindings
* [ozankasikci/rust-music-theory](https://github.com/ozankasikci/rust-music-theory) — A Rust music theory library
* [MoAlyousef/soloud-rs](https://github.com/MoAlyousef/soloud-rs) — Rust bindings for the [soloud audio engine library](https://sol.gfxile.net/soloud/)

### Authentication

* [Keats/jsonwebtoken](https://github.com/Keats/jsonwebtoken) — [JSON Web Token](https://en.wikipedia.org/wiki/JSON_Web_Token) lib in rust
* [sgrust01/jwtvault](https://github.com/sgrust01/jwtvault) — Async library to manage and orchestrate JWT workflow
* [oauth2](https://github.com/ramosbugs/oauth2-rs) — Extensible, strongly-typed Rust OAuth2 client library
* [oxide-auth](https://github.com/HeroicKatora/oxide-auth) — A OAuth2 server library, for use in combination with actix or other frontends, featuring a set of configurable and pluggable backends
* [yup-oauth2](https://github.com/dermesser/yup-oauth2) — An oauth2 client implementation providing the Device, Installed and Service Account flows

### Automotive

* [canparse](https://crates.io/crates/canparse) — A CAN signal and definition parser
* [j2534](https://crates.io/crates/j2534) — J2534 PassThru bindings
* [JulianSchmid/dlt_parse](https://github.com/JulianSchmid/dlt-parse-rs) — Rust DLT (Diagnostic Log and Trace) packet parser
* [JulianSchmid/someip_parse](https://github.com/JulianSchmid/someip-parse-rs) [[someip_parse](https://crates.io/crates/someip_parse)] — A library for parsing the SOME/IP network protocol (without payload interpretation)
* [LibreTuner/tuneutils](https://github.com/LibreTuner/tuneutils) [[tuneutils](https://crates.io/crates/tuneutils)] — Utilities for interfacing with, diagnosing, and tuning car
* [marcelbuesing/can-dbc](https://github.com/marcelbuesing/can-dbc) [[can-dbc](https://crates.io/crates/can-dbc)] — A parser for the DBC format
* [marcelbuesing/tokio-socketcan-bcm](https://github.com/marcelbuesing/tokio-socketcan-bcm) [[tokio-socketcan-bcm](https://crates.io/crates/tokio-socketcan-bcm)] — Linux SocketCAN BCM support for tokio
* [mbr/socketcan](https://github.com/mbr/socketcan-rs) [[socketcan](https://crates.io/crates/socketcan)] — Linux SocketCAN library
* [oefd/tokio-socketcan](https://github.com/oefd/tokio-socketcan) [[tokio-socketcan]](https://crates.io/crates/tokio-socketcan)] — Linux SocketCAN support for tokio based on the socketcan crate
* [Sensirion/lin-bus](https://github.com/Sensirion/lin-bus-rs) [[lin-bus](https://crates.io/crates/lin-bus)] — LIN bus driver traits and protocol implementation

### Bioinformatics

* [Rust-Bio](https://github.com/rust-bio) — bioinformatics libraries in Rust.

### Caching

* [jaysonsantos/bmemcached-rs](https://github.com/jaysonsantos/bmemcached-rs) — Memcached library written in pure rust
* [jaemk/cached](https://github.com/jaemk/cached) — Simple function caching/memoization
* [aisk/rust-memcache](https://github.com/aisk/rust-memcache) — Memcached client library

### Concurrency

* [aymanmadkour/glock](https://github.com/aymanmadkour/glock) – Granular locking crate for Rust.
* [crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam) – Support for parallelism and low-level concurrency in Rust
* [orium/archery](https://github.com/orium/archery) [[archery](https://crates.io/crates/archery)] — Library to abstract from `Rc`/`Arc` pointer types.
* [pop-os/bus-writer](https://github.com/pop-os/bus-writer) — Generic single-reader, multi-writer
* [Rayon](https://github.com/rayon-rs/rayon) – A data parallelism library for Rust
* [rustcc/coroutine-rs](https://github.com/rustcc/coroutine-rs) – Coroutine Library in Rust
* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) – Coroutine I/O for Rust

### Cloud

* AWS [[aws](https://crates.io/keywords/aws)]
  * [rusoto/rusoto](https://github.com/rusoto/rusoto)

### Command-line

* Argument parsing
  * [clap-rs](https://github.com/clap-rs/clap) [[clap](https://crates.io/crates/clap)] — A simple to use, full featured command-line argument parser
  * [docopt/docopt.rs](https://github.com/docopt/docopt.rs) [[docopt](https://crates.io/crates/docopt)] — A Rust implementation of [DocOpt](http://docopt.org)
  * [z5labs/pflag](https://github.com/z5labs/pflag) [[pflag](https://crates.io/crates/pflag)] — A port of @spf13's amazing POSIX compliant arg parsing library in Go.
  * [TeXitoi/structopt](https://github.com/TeXitoi/structopt) [[structopt](https://crates.io/crates/structopt)] — parse command line argument by defining a struct
  * [killercup/quicli](https://github.com/killercup/quicli) [[quicli](https://crates.io/crates/quicli)] — quickly build cool CLI apps in Rust
  * [ksk001100/seahorse](https://github.com/ksk001100/seahorse) [[seahorse](https://crates.io/crates/seahorse)] — A minimal CLI framework written in Rust
* Data visualization
  * [reugn/rspark](https://github.com/reugn/rspark) [[rspark](https://crates.io/crates/rspark)] — ▁▂▆▇▁▄█▁ Sparklines for Rust apps
* Human-centered design
  * [rust-cli/human-panic](https://github.com/rust-cli/human-panic) [[human-panic](https://crates.io/crates/human-panic)] — panic messages for humans
* Line editor
  * [srijs/rust-copperline](https://github.com/srijs/rust-copperline) [[copperline](https://crates.io/crates/copperline)] — pure-Rust command line editing library
  * [MovingtoMars/liner](https://github.com/MovingtoMars/liner) [[liner](https://crates.io/crates/liner)] — A library offering readline-like functionality
  * [murarth/linefeed](https://github.com/murarth/linefeed) [[linefeed](https://crates.io/crates/linefeed)] — Configurable, extensible, interactive line reader
  * [kkawakam/rustyline](https://github.com/kkawakam/rustyline) [[rustyline](https://crates.io/crates/rustyline)] — readline implementation in Rust
* Pipeline
  * [imp/pager-rs](https://gitlab.com/imp/pager-rs) [[pager](https://crates.io/crates/pager)] — pipe your output through an external pager
  * [hniksic/rust-subprocess](https://github.com/hniksic/rust-subprocess) [[subprocess](https://crates.io/crates/subprocess)] — facilities for interaction with external pipelines
  * [oconnor663/duct.rs](https://github.com/oconnor663/duct.rs) [[duct](https://crates.io/crates/duct)] — A builder for subprocess pipelines and IO redirection
  * [philippkeller/rexpect](https://github.com/philippkeller/rexpect) [[rexpect](https://crates.io/crates/rexpect)] — automate interactive applications such as ssh, ftp, passwd, etc
* Progress
  * [mitsuhiko/indicatif](https://github.com/mitsuhiko/indicatif) [[indicatif](https://crates.io/crates/indicatif)] — indicate progress to users
  * [a8m/pb](https://github.com/a8m/pb) [[pbr](https://crates.io/crates/pbr)] — console progress bar for Rust
  * [FGRibreau/spinners](https://github.com/FGRibreau/spinners) [[spinners](https://crates.io/crates/spinners)] — 60+ elegant terminal spinners
* Prompt
  * [hashmismatch/terminal_cli.rs](https://github.com/hashmismatch/terminal_cli.rs) [[terminal_cli](https://crates.io/crates/terminal_cli)]  — build an interactive command prompt
  * [starship/starship](https://starship.rs/) [[starship](https://crates.io/crates/starship)]  — A minimal, blazing fast, and extremely customizable prompt for any shell
* Style
  * [ogham/rust-ansi-term](https://github.com/ogham/rust-ansi-term) [[ansi_term](https://crates.io/crates/ansi_term)] — control colours and formatting on ANSI terminals
  * [LukasKalbertodt/term-painter](https://github.com/LukasKalbertodt/term-painter) [[term-painter](https://crates.io/crates/term-painter)] — cross-platform styled terminal output
  * [vitiral/termstyle](https://github.com/vitiral/termstyle) [[termstyle](https://docs.rs/termstyle/0.1.2/termstyle/)] — build (and test) formatted and styled command line applications
  * [SergioBenitez/yansi](https://github.com/SergioBenitez/yansi) [[yansi](https://crates.io/crates/yansi)] — A dead simple ANSI terminal color painting library
  * [mackwic/colored](https://github.com/mackwic/colored) [[colored](https://crates.io/crates/colored)] — Coloring terminal so simple, you already know how to do it!
* TUI
  * [TimonPost/crossterm](https://github.com/crossterm-rs/crossterm) [[crossterm](https://crates.io/crates/crossterm)] — crossplatform terminal library
  * [gyscos/Cursive](https://github.com/gyscos/Cursive) [[cursive](https://crates.io/crates/cursive)] — build rich TUI applications
  * [ogham/rust-term-grid](https://github.com/ogham/rust-term-grid) [[term_grid](https://crates.io/crates/term_grid)] — Rust library for putting things in a grid
  * [redox-os/termion](https://github.com/redox-os/termion) [[termion](https://crates.io/crates/termion)] — bindless library for controlling terminals/TTY
  * [fdehau/tui-rs](https://github.com/fdehau/tui-rs) [[tui](https://crates.io/crates/tui)] — A TUI library inspired by [blessed-contrib](https://github.com/yaronn/blessed-contrib) and [termui](https://github.com/gizak/termui)
  * BearLibTerminal
    * [cfyzium/bearlibterminal](https://github.com/nabijaczleweli/BearLibTerminal.rs) [[bear-lib-terminal](https://crates.io/crates/bear-lib-terminal)] — [BearLibTerminal](https://github.com/tommyettinger/BearLibTerminal) bindings
  * ncurses
    * [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) [[ncurses](https://crates.io/crates/ncurses)] — [ncurses](https://www.gnu.org/software/ncurses/) bindings
    * [ihalila/pancurses](https://github.com/ihalila/pancurses) [[pancurses](https://crates.io/crates/pancurses)] — curses library, supports linux and windows
  * Termbox
    * [gchp/rustbox](https://github.com/gchp/rustbox) [[rustbox](https://crates.io/crates/rustbox)] — bindings to [Termbox](https://github.com/nsf/termbox)
  * [ivanceras/titik](https://github.com/ivanceras/titik) - a crossplatform TUI widget library with the goal of providing interactive widgets

### Compression

* [Brotli](https://opensource.googleblog.com/2015/09/introducing-brotli-new-compression.html)
  * [ende76/brotli-rs](https://github.com/ende76/brotli-rs) — implementation of Brotli compression
  * [dropbox/rust-brotli](https://github.com/dropbox/rust-brotli) — Brotli decompressor in Rust that optionally avoids the stdlib
* bzip2
  * [alexcrichton/bzip2-rs](https://github.com/alexcrichton/bzip2-rs) — [libbz2](https://www.sourceware.org/bzip2/) bindings
* Columnar compression
  * [velvia/compressed-vec](https://github.com/velvia/compressed-vec) - SIMD Floating point and integer compressed vector library
* gzip
  * [carols10cents/zopfli](https://github.com/carols10cents/zopfli) — implementation of the [Zopfli](https://github.com/google/zopfli) compression algorithm for higher quality deflate or zlib compression
* miniz
  * [alexcrichton/flate2-rs](https://github.com/alexcrichton/flate2-rs) — [miniz](https://code.google.com/archive/p/miniz) bindings
* snappy
  * [JeffBelgum/rust-snappy](https://github.com/JeffBelgum/rust-snappy) — [snappy](https://github.com/google/snappy) bindings
* tar
  * [alexcrichton/tar-rs](https://github.com/alexcrichton/tar-rs) — tar archive reading/writing in Rust
* zip
  * [zip-rs/zip](https://github.com/zip-rs/zip) — read and write ZIP archives

### Computation

* [argmin-rs/argmin](https://github.com/argmin-rs/argmin) [[argmin](https://crates.io/crates/argmin)] — A pure Rust optimization library
* [BLAS](https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) [[blas](https://crates.io/keywords/blas)]
  * [mikkyang/rust-blas](https://github.com/mikkyang/rust-blas) — BLAS bindings
  * [stainless-steel/blas](https://github.com/blas-lapack-rs/blas) — BLAS bindings
* [Conjugate Gradient](https://en.wikipedia.org/wiki/Limited-memory_BFGS)
  * [noshu/cg-sys](https://github.com/noshu/cg-sys) — Rust binding of fortran CG+ subroutine
* [GMP](https://gmplib.org/)
  * [fizyk20/rust-gmp](https://github.com/fizyk20/rust-gmp) — libgmp bindings
* [GSL](http://www.gnu.org/software/gsl/)
  * [GuillaumeGomez/rust-GSL](https://github.com/GuillaumeGomez) — GSL bindings
* [LAPACK](https://en.wikipedia.org/wiki/LAPACK)
  * [stainless-steel/lapack](https://github.com/blas-lapack-rs/lapack) — LAPACK bindings
* [L-BFGS-B](https://en.wikipedia.org/wiki/Limited-memory_BFGS)
  * [noshu/lbfgsb-sys](https://github.com/noshu/lbfgsb-sys) — Rust binding of fortran L-BFGS-B subroutine
* [dimforge/nalgebra](https://github.com/dimforge/nalgebra) — low-dimensional linear algebra library
* Parallel
  * [arrayfire/arrayfire-rust](https://github.com/arrayfire/arrayfire-rust) — [Arrayfire](https://github.com/arrayfire) bindings
  * [autumnai/collenchyma](https://github.com/autumnai/collenchyma) — An extensible, pluggable, backend-agnostic framework for parallel, high-performance computations on CUDA, OpenCL and common host CPU.
  * [luqmana/rust-opencl](https://github.com/luqmana/rust-opencl) — [OpenCL](https://www.khronos.org/opencl/) bindings
* Scirust
  * [indigits/scirust](https://github.com/indigits/scirust) — scientific computing library in Rust
* Statrs
  * [boxtown/statrs](https://github.com/boxtown/statrs) — Robust statistical computation library in Rust
* Rustimization [[rustimization](https://crates.io/crates/rustimization)]
  * [noshu/rustimization](https://github.com/noshu/rustimization) — A rust optimization library which includes L-BFGS-B and Conjugate Gradient algorithm
* [calebwin/emu](https://github.com/calebwin/emu) — A language for GPGPU numerical computing from a Rust macro

### Configuration

* [mehcode/config-rs](https://github.com/mehcode/config-rs) [[config](https://crates.io/crates/config)] — Layered configuration system for Rust applications (with strong support for 12-factor applications).
* [theimpossibleastronaut/configster](https://github.com/theimpossibleastronaut/configster) [[configster](https://crates.io/crates/configster)] — Rust library for parsing configuration files
* [Kixunil/configure_me](https://github.com/Kixunil/configure_me) [[configure_me](https://crates.io/crates/configure_me)] — library for processing application configuration easily
* [andoriyu/uclicious](https://github.com/andoriyu/uclicious) [[uclicious](https://crates.io/crates/uclicious)] — [libUCL](https://github.com/vstakhov/libucl) based feature-rich configuration library.
* [FlashSystems/justconfig](https://github.com/FlashSystems/just-config) - Easily extendable, layered configuration library that introduces no additional dependencies into your project.

### Cryptography

[[crypto](https://crates.io/keywords/crypto), [cryptography](https://crates.io/keywords/cryptography)]

* [briansmith/ring](https://github.com/briansmith/ring) — Safe, fast, small crypto using Rust and BoringSSL's cryptography primitives.
* [briansmith/webpki](https://github.com/briansmith/webpki) — Web PKI TLS X.509 certificate validation in Rust.
* [brycx/orion](https://github.com/brycx/orion) — This library aims to provide easy and usable crypto. 'Usable' meaning exposing high-level API's that are easy to use and hard to misuse.
* [cossacklabs/themis](https://github.com/cossacklabs/themis) [[themis](https://crates.io/crates/themis)] — a high-level cryptographic library for solving typical data security tasks, best fit for multi-platform apps.
* [ctz/rustls](https://github.com/ctz/rustls) — A Rust implementation of TLS
* [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) — cryptographic algorithms in Rust
* [dalek-cryptography/curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) — Pure Rust implementation of Curve25519 operations
* [dalek-cryptography/ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) — Pure Rust implementation of Ed25519 digital signatures
* [dalek-cryptography/x25519-dalek](https://github.com/dalek-cryptography/x25519-dalek) — Pure Rust implementation of X25519 key exchange
* [debris/tiny-keccak](https://github.com/debris/tiny-keccak) — Pure Rust implementation of the Keccak family (SHA3)
* [defund/pw](https://github.com/defund/pw) — CLI password manager with no frills
* [sodiumoxide/sodiumoxide](https://github.com/sodiumoxide/sodiumoxide) — [libsodium](https://github.com/jedisct1/libsodium) bindings
* [doublify/libblockchain](https://github.com/doublify/libblockchain) — A Blockchain implementation
* [exonum/exonum](https://github.com/exonum/exonum) [[exonum](https://crates.io/crates/exonum)] — extensible framework for blockchain projects
* [klutzy/suruga](https://github.com/klutzy/suruga) — A Rust implementation of [TLS 1.2](https://tools.ietf.org/html/rfc5246)
* [libOctavo/octavo](https://github.com/libOctavo/octavo) — Modular hash and crypto library in Rust
* [novifinancial/opaque-ke](https://github.com/novifinancial/opaque-ke) — Pure Rust implementation of the recent [OPAQUE](https://datatracker.ietf.org/doc/draft-krawczyk-cfrg-opaque/) password-authenticated key exchange.
* [RustCrypto/hashes](https://github.com/RustCrypto/hashes) — Collection of cryptographic hash functions written in pure Rust
* [rustindia/mpw-rs](https://github.com/rustindia/mpw-rs) — Pure Rust implementation of the Master Password password manager
* [Fraunhofer-AISEC/rabe](https://github.com/Fraunhofer-AISEC/rabe) — Library providing several Attribute-Based Encryption (ABE) schemes
* [racum/rust-djangohashers](https://github.com/racum/rust-djangohashers) — A Rust port of the password primitives used in the Django Project. It doesn't require Django, only hashes and validates passwords according to its style.
* [RNCryptor/rncryptor-rs](https://github.com/RNCryptor/rncryptor-rs) — Pure Rust implementation of the RNCryptor AES file format
* [conradkleinespel/rooster](https://github.com/conradkleinespel/rooster) [[rooster](https://crates.io/crates/rooster)] — Simple password manager to use in your terminal
* [sfackler/rust-native-tls](https://github.com/sfackler/rust-native-tls) — Bindings for native TLS libraries
* [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) — [OpenSSL](https://www.openssl.org/) bindings
* [kornelski/rust-security-framework](https://github.com/kornelski/rust-security-framework) — Bindings for Security Framework (OSX native)
* [steffengy/schannel-rs](https://github.com/steffengy/schannel-rs) — Bindings for Schannel (Windows native TLS)
* [zebradil/rustotpony](https://github.com/zebradil/rustotpony) — CLI manager of one-time password generators aka Google Authenticator (TOTP)

### Database

[[database](https://crates.io/keywords/database)]

* [sfackler/r2d2](https://github.com/sfackler/r2d2) — generic connection pool

* NoSQL [[nosql](https://crates.io/keywords/nosql)]

  * [ArangoDB](https://www.arangodb.com)
     * [Rincon](https://github.com/innoave/rincon) — An ArangoDB (NoSQL and Graph store) driver for Rust
  * [Cassandra](https://cassandra.apache.org) [[cassandra](https://crates.io/keywords/cassandra), [cql](https://crates.io/keywords/cql)]
    * [AlexPikalov/cdrs](https://github.com/AlexPikalov/cdrs) [[cdrs](https://crates.io/crates/cdrs)] — native client written in Rust
    * [Metaswitch/cassandra-rs](https://github.com/Metaswitch/cassandra-rs) —  bindings to the DataStax C/C++ client
  * CouchDB [[couchdb](https://crates.io/keywords/couchdb)]
    * [chill-rs/chill](https://github.com/chill-rs/chill) [[couchdb](https://crates.io/crates/chill)] — A Rust client for the CouchDB REST API
    * [Sofa](https://github.com/66Origin/sofa) — an interface to CouchDB HTTP REST API for stable rust
  * Crux [[crux](https://crates.io/keywords/crux)]
    * [naomijub/transistor](https://github.com/naomijub/transistor) — A Crux Database Client.
  * [DynamoDB](https://aws.amazon.com/dynamodb/) [[dynamodb](https://crates.io/keywords/dynamodb)]
    * [softprops/dynomite](https://github.com/softprops/dynomite) - A library for strongly-typed and convenient interaction with `rusoto_dynamodb`
  * Elasticsearch [[elasticsearch](https://crates.io/keywords/elasticsearch)]
    * [benashford/rs-es](https://github.com/benashford/rs-es) [[rs-es](https://crates.io/crates/rs-es)] — A Rust client for the [Elastic](https://www.elastic.co/) REST API
    * [elastic-rs/elastic](https://github.com/elastic-rs/elastic) [[elastic](https://crates.io/crates/elastic)] — elastic is an efficient, modular API client for Elasticsearch written in Rust
  * etcd
    * [jimmycuadra/rust-etcd](https://github.com/jimmycuadra/rust-etcd) [[etcd](https://crates.io/crates/etcd)] — A client library for CoreOS's etcd.
    * [luncj/etcd-rs](https://github.com/luncj/etcd-rs) — An asynchronous etcd client for rust
  * ForestDB
    * [vhbit/sherwood](https://github.com/vhbit/sherwood) — [ForestDB](https://github.com/couchbase/forestdb) bindings
  * [InfluxDB](https://www.influxdata.com/)
    * [panoptix-za/influxdb-rs](https://github.com/panoptix-za/influxdb-rs) — asynchronous interface
    * [driftluo/InfluxDBClient-rs](https://github.com/driftluo/InfluxDBClient-rs) — Synchronization interface
  * LevelDB
    * [skade/leveldb](https://github.com/skade/leveldb) — [LevelDB](https://github.com/google/leveldb) bindings
  * LMDB [[lmdb](https://crates.io/keywords/lmdb)]
    * [vhbit/lmdb-rs](https://github.com/vhbit/lmdb-rs) [[lmdb-rs](https://crates.io/crates/lmdb-rs)] — [LMDB](https://symas.com/lmdb/) bindings
  * MHdb
    * [MHmorgan/mhdb](https://github.com/MHmorgan/mhdb) [[mhdb](https://crates.io/crates/mhdb)] — Pure rust embeddable key-value store database.
  * MongoDB [[mongodb](https://crates.io/keywords/mongodb)]
    * [mongodb/mongo-rust-driver](https://github.com/mongodb/mongo-rust-driver) [[mongodb](https://crates.io/crates/mongodb)] — [MongoDB](https://www.mongodb.com/) bindings
  * Neo4j [[cypher](https://crates.io/keywords/cypher), [neo4j](https://crates.io/keywords/neo4j)]
  * Redis [[redis](https://crates.io/keywords/redis)]
    * [mitsuhiko/redis-rs](https://github.com/mitsuhiko/redis-rs) — [Redis](https://redis.io/) library in Rust
  * [RocksDB](https://rocksdb.org/)
    * [rust-rocksdb/rust-rocksdb](https://github.com/rust-rocksdb/rust-rocksdb) — RocksDB bindings
  * [UnQLite](https://unqlite.org/)
    * [zitsen/unqlite.rs](https://github.com/zitsen/unqlite.rs) — UnQLite bindings
  * [ZooKeeper](https://zookeeper.apache.org/)
    * [bonifaido/rust-zookeeper](https://github.com/bonifaido/rust-zookeeper) [[zookeeper](https://crates.io/crates/zookeeper)] — A client library for Apache ZooKeeper.
  * [PickleDB](https://pythonhosted.org/pickleDB/)
    * [seladb/pickledb-rs](https://github.com/seladb/pickledb-rs) — a lightweight and simple key-value store, heavily inspired by Python's PickleDB.
* SQL [[sql](https://crates.io/keywords/sql)]
  * Generic
    * [launchbadge/sqlx](https://github.com/launchbadge/sqlx) - async PostgreSQL/MySQL/SQLite connection pool with strong typing support
  * Microsoft SQL
    * [prisma/tiberius](https://github.com/prisma/tiberius)
  * MySql [[mysql](https://crates.io/keywords/mysql)]
    * [AgilData/mysql-proxy-rs](https://github.com/AgilData/mysql-proxy-rs) — A MySQL Proxy
    * [blackbeam/mysql_async](https://github.com/blackbeam/mysql_async) [[mysql_async](https://crates.io/crates/mysql_async)] — asyncronous Rust Mysql driver based on Tokio.
    * [blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) [[mysql](https://crates.io/crates/mysql)] — A native MySql client
  * PostgreSql [[postgres](https://crates.io/keywords/postgres), [postgresql](https://crates.io/keywords/postgresql)]
    * [sfackler/rust-postgres](https://github.com/sfackler/rust-postgres) [[postgres](https://crates.io/crates/postgres)] — A native [PostgreSQL](https://www.postgresql.org/) client
  * Sqlite [[sqlite](https://crates.io/keywords/sqlite)]
    * [rusqlite](https://github.com/rusqlite/rusqlite) — [Sqlite3](https://www.sqlite.org/index.html) bindings
* ORM [[orm](https://crates.io/keywords/orm)]
  * [diesel-rs/diesel](https://github.com/diesel-rs/diesel) — an ORM and Query builder for Rust
  * [ivanceras/rustorm](https://github.com/ivanceras/rustorm) — an ORM for Rust

### Data processing

* [bluss/ndarray](https://github.com/rust-ndarray/ndarray) — N-dimensional array with array views, multidimensional slicing, and efficient operations
* [kernelmachine/utah](https://github.com/kernelmachine/utah) — Dataframe structure and operations in Rust
* [ritchie46/polars](https://github.com/ritchie46/polars) - Fast feature complete DataFrame library
* [weld-project/weld](https://github.com/weld-project/weld) — High-performance runtime for data analytics applications

### Data structures

* [billyevans/tst](https://github.com/billyevans/tst) [[tst](https://crates.io/crates/tst)] — Ternary search tree collection
* [rust-itertools/itertools](https://github.com/rust-itertools/itertools)
* [contain-rs](https://github.com/contain-rs) — Extension of Rust's std::collections
* [danielpclark/array_tool](https://github.com/danielpclark/array_tool) — Array helpers for Rust. Some of the most common methods you would use on Arrays made available on Vectors. Polymorphic implementations for handling most of your use cases.
* [fizyk20/generic-array](https://github.com/fizyk20/generic-array) – a hack to allow for arrays sized by typenums
* [garro95/priority-queue](https://github.com/garro95/priority-queue)[[priority-queue](https://crates.io/crates/priority-queue)] — A priority queue that implements priority changes.
* [mrhooray/kdtree-rs](https://github.com/mrhooray/kdtree-rs) — K-dimensional tree in Rust for fast geospatial indexing and nearest neighbors lookup
* [Nemo157/roaring-rs](https://github.com/Nemo157/roaring-rs) – Roaring Bitmaps in Rust
* [orium/rpds](https://github.com/orium/rpds) [[rpds](https://crates.io/crates/rpds)] — Persistent data structures in Rust.
* [pop-os/progress-streams](https://github.com/pop-os/progress-streams) — Progress callbacks for types which implement `dyn io::Read` or `dyn io::Write`.
* [whitfin/usher](https://github.com/whitfin/usher) [[usher](https://crates.io/crates/usher)] — Parameterized routing for generic resources in Rust.
* [xfix/enum-map](https://github.com/xfix/enum-map) [[enum-map](https://crates.io/crates/enum-map)] — An optimized map implementation for enums using an array to store values.

### Data visualization

* [saresend/gust](https://github.com/saresend/Gust)
* [milliams/plotlib](https://github.com/milliams/plotlib)
* [igiagkiozis/plotly](https://github.com/igiagkiozis/plotly) — Plotly for Rust.
* [38/plotters](https://github.com/38/plotters)

### Date and time

[[date](https://crates.io/keywords/date), [time](https://crates.io/keywords/time)]

* [chronotope/chrono](https://github.com/chronotope/chrono)
* [yaa110/rust-persian-calendar](https://github.com/yaa110/rust-persian-calendar)
* [Mnwa/ms](https://github.com/Mnwa/ms) [[ms-converter](https://crates.io/crates/ms-converter)] — it's a library for converting human-like times to milliseconds

### Distributed systems

* Antimony
  * [antimonyproject/antimony](https://github.com/antimonyproject/antimony) [[antimony](https://crates.io/crates/antimony)] — stream processing / distributed computation platform
* Apache Hadoop
  * [whitfin/efflux](https://github.com/whitfin/efflux) — Easy Hadoop Streaming and MapReduce interfaces in Rust.
* Apache Kafka
  * [fede1024/rust-rdkafka](https://github.com/fede1024/rust-rdkafka) [[rdkafka](https://crates.io/crates/rdkafka)] — [librdkafka](https://github.com/edenhill/librdkafka) bindings
  * [gklijs/schema_registry_converter](https://github.com/gklijs/schema_registry_converter) — to integrate with [confluent schema registry](https://www.confluent.io/product/confluent-platform/data-compatibility/)
  * [spicavigo/kafka-rust](https://github.com/spicavigo/kafka-rust)
* Beanstalkd
  * [schickling/rust-beanstalkd](https://github.com/schickling/rust-beanstalkd) — [Beanstalkd](https://github.com/beanstalkd/beanstalkd) bindings
* HDFS
  * [hyunsik/hdfs-rs](https://github.com/hyunsik/hdfs-rs) — libhdfs bindings

### Email

[[email](https://crates.io/keywords/email), [imap](https://crates.io/keywords/imap), [smtp](https://crates.io/keywords/smtp)]

* [GildedHonour/atarashii_imap](https://github.com/GildedHonour/atarashii_imap) — 新しい (atarashii/new) IMAP client in Rust. It supports plain and secure connections
* [gsquire/sendgrid-rs](https://github.com/gsquire/sendgrid-rs) — unofficial Rust library for SendGrid API
* [lettre/lettre](https://github.com/lettre/lettre) — an SMTP-library for Rust
* [staktrace/mailparse](https://github.com/staktrace/mailparse) [[mailparse](https://crates.io/crates/mailparse)] — A library for parsing real-world email files
* [meli](https://git.meli.delivery/meli/meli.git) [[meli](https://crates.io/crates/meli)] — terminal email client

### Encoding

[[encoding](https://crates.io/keywords/encoding)]

* ASN.1
  * [alex/rust-asn1](https://github.com/alex/rust-asn1) — A Rust ASN.1 (DER) serializer
* Bencode
  * [arjantop/rust-bencode](https://github.com/arjantop/rust-bencode) — [Bencode](https://en.wikipedia.org/wiki/Bencode) implementation in Rust
* Binary
  * [arcnmx/nue](https://github.com/arcnmx/nue) — I/O and binary data encoding for Rust
  * [servo/bincode](https://github.com/servo/bincode) — A binary encoder/decoder in Rust
  * [m4b/goblin](https://github.com/m4b/goblin) [[goblin](https://crates.io/crates/goblin)] —  cross-platform, zero-copy, and endian-aware binary parsing
* BSON
  * [mongodb/bson-rust](https://github.com/mongodb/bson-rust) — Encoding and decoding support for BSON in Rust
* Byte swapping
  * [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder) — Supports big-endian, little-endian and native byte orders
* Cap'n Proto
  * [capnproto/capnproto-rust](https://github.com/capnproto/capnproto-rust)
* CBOR
  * [serde_cbor](https://crates.io/crates/serde_cbor) — CBOR support for serde
* Character Encoding
  * [hsivonen/encoding_rs](https://github.com/hsivonen/encoding_rs) [[encoding_rs](https://crates.io/crates/encoding_rs)] — A Gecko-oriented implementation of the Encoding Standard in Rust
  * [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding)
* CRC
  * [mrhooray/crc-rs](https://github.com/mrhooray/crc-rs)
* CSV
  * [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) — A fast and flexible CSV reader and writer, with support for Serde
* [FlatBuffers](https://google.github.io/flatbuffers/)
  * [frol/flatc-rust](https://github.com/frol/flatc-rust) — FlatBuffers compiler (flatc) integration for Cargo build scripts
* EDN
  * [naomijub/edn-rs](https://github.com/naomijub/edn-rs) — crate to parse and emit EDN format into Rust types.
* HAR
  * [mandrean/har-rs](https://github.com/mandrean/har-rs) — A HTTP Archive Format (HAR) serialization & deserialization library
* HTML
  * [servo/html5ever](https://github.com/servo/html5ever) — High-performance browser-grade HTML5 parser
  * [veddan/rust-htmlescape](https://github.com/veddan/rust-htmlescape) — encoding/decoding HTML entities
* JSON
  * [pikkr/pikkr](https://github.com/pikkr/pikkr) [[pikkr](https://crates.io/crates/pikkr)] — JSON parser which picks up values directly without performing tokenization in Rust
  * [serde-rs/json](https://github.com/serde-rs/json) [[serde\_json](https://crates.io/crates/serde_json)] — JSON support for [Serde](https://github.com/serde-rs/serde) framework
  * [simd-lite/simd-json](https://github.com/simd-lite/simd-json) [[simd-json](https://crates.io/crates/simd-json)] — High performance JSON parser based on a port of simdjson
  * [maciejhirsz/json-rust](https://github.com/maciejhirsz/json-rust) [[json](https://crates.io/crates/json)] — JSON implementation in Rust
  * [importcjj/rust-ajson](https://github.com/importcjj/rust-ajson) [[ajson]](https://crates.io/crates/ajson) —  Get JSON values quickly
* Jsonnet
  * [Qihoo360/rust-jsonnet](https://github.com/Qihoo360/rust-jsonnet)
* MsgPack
  * [3Hren/msgpack-rust](https://github.com/3Hren/msgpack-rust) — A pure Rust low/high level MessagePack implementation
* PEM
  * [jcreekmore/pem-rs](https://github.com/jcreekmore/pem-rs) [[pem](https://crates.io/crates/pem)] — A Rust based way to parse and encode PEM-encoded data
* Postman Collection
  * [mandrean/postman-collection-rs](https://github.com/mandrean/postman-collection-rs) — A Postman Collection v1, v2 & v2.1 serialization & deserialization library
* ProtocolBuffers
  * [danburkert/prost](https://github.com/danburkert/prost)
  * [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf)
* RON (Rusty Object Notation)
  * [https://github.com/ron-rs/ron](https://github.com/ron-rs/ron)
* Tnetstring
  * [erickt/rust-tnetstring](https://github.com/erickt/rust-tnetstring)
* TOML
  * [alexcrichton/toml-rs](https://github.com/alexcrichton/toml-rs)
* XML
  * [tafia/quick-xml](https://github.com/tafia/quick-xml) — High performance XML pull reader/writer
  * [Florob/RustyXML](https://github.com/Florob/RustyXML) — an XML parser written in Rust
  * [shepmaster/sxd-document](https://github.com/shepmaster/sxd-document) — An XML library in Rust
  * [shepmaster/sxd-xpath](https://github.com/shepmaster/sxd-xpath) — An XPath library in Rust
  * [netvl/xml-rs](https://github.com/netvl/xml-rs) — A streaming XML library
  * [media-io/yaserde](https://github.com/media-io/yaserde) — Yet Another Serializer/Deserializer specialized for XML
* YAML
  * [chyh1990/yaml-rust](https://github.com/chyh1990/yaml-rust) — The missing YAML 1.2 implementation for Rust.
  * [dtolnay/serde-yaml](https://github.com/dtolnay/serde-yaml) [[serde\_yaml](https://crates.io/crates/serde_yaml)] — YAML support for [Serde](https://github.com/serde-rs/serde) framework
  * [kimhyunkang/libyaml-rust](https://github.com/kimhyunkang/libyaml-rust) — [libyaml](https://pyyaml.org/wiki/LibYAML) bindings
  * [vitiral/stfu8](https://github.com/vitiral/stfu8) — Sorta Text Format in UTF-8

### Filesystem

[[filesystem](https://crates.io/keywords/filesystem)]
* Libraries
  * [jonhkr/rust-file-seq](https://github.com/jonhkr/rust-file-seq) -> Fail-safe sequence implementation that uses the file system as store
* Operations
  * [pop-os/dbus-udisks2](https://github.com/pop-os/dbus-udisks2) -> UDisks2 DBus API
  * [pop-os/sys-mount](https://github.com/pop-os/sys-mount) — High level abstraction for the `mount` / `umount2` system calls.
  * [vitiral/path_abs](https://github.com/vitiral/path_abs) — Absolute serializable path types and associated methods.
  * [webdesus/fs_extra](https://github.com/webdesus/fs_extra) — expanding opportunities standard library std::fs and std::io
* Temporary Files
  * [rust-lang-deprecated/tempdir](https://github.com/rust-lang-deprecated/tempdir) — temporary directory library
  * [Stebalien/tempfile](https://github.com/Stebalien/tempfile) — temporary file library
  * [Stebalien/xattr](https://github.com/Stebalien/xattr) [[xattr](https://crates.io/crates/xattr)] — list and manipulate unix extended file attributes
  * [zboxfs/zbox](https://github.com/zboxfs/zbox) [[zbox](https://crates.io/crates/zbox)] — Zero-details, privacy-focused embeddable file system.

### Functional Programming
[[functional programming](https://crates.io/keywords/fp)]
* Prelude
  * [JasonShin/fp-core.rs](https://github.com/JasonShin/fp-core.rs) — A library for functional programming in Rust

### Game development

See also [Are we game yet?](https://arewegameyet.rs)
* Allegro
  * [SiegeLord/RustAllegro](https://github.com/SiegeLord/RustAllegro) — [Allegro 5](https://liballeg.org/) bindings
* Challonge
  * [vityafx/challonge-rs](https://github.com/vityafx/challonge-rs) [[challonge](https://crates.io/crates/challonge)] — Client library for the Challonge REST API. Helps to organize tournaments.
* Corange
  * [lucidscape/corange-rs](https://github.com/lucidscape/corange-rs) — [Corange](https://github.com/orangeduck/Corange) bindings
* Entity-Component Systems (ECS)
  * [amethyst/specs](https://github.com/amethyst/specs) — Specs Parallel ECS
  * [legion](https://github.com/amethyst/legion) — A feature rich high performance ECS library with minimal boilerplate
* Game Engines
  * [Amethyst](https://amethyst.rs) — Data-oriented game engine -
  * [Bevy](https://github.com/bevyengine/bevy) is a refreshingly simple data-driven game engine built in Rust.
  * [ggez](https://github.com/ggez/ggez) — A lightweight game framework for making 2D games with minimum friction -
  * [harmony](https://github.com/StarArawn/harmony) — A modern 3D/2D game engine that uses wgpu
  * [Kiss3d](http://kiss3d.org) — A Keep It Simple, Stupid 3d graphics engine written with Rust
  * [oxidator](https://github.com/Ruddle/oxidator) — A real time strategy game/engine written with Rust and WebGPU
  * [Piston](https://www.piston.rs/)
  * [Unrust](https://github.com/unrust/unrust) — unrust — A pure rust based (webgl 2.0 / native) game engine
* [SDL](http://www.libsdl.org/) [[sdl](https://crates.io/keywords/sdl)]
  * [brson/rust-sdl](https://github.com/brson/rust-sdl) — SDL1 bindings
  * [Rust-SDL2/rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) — SDL2 bindings
* SFML
  * [jeremyletang/rust-sfml](https://github.com/jeremyletang/rust-sfml) — [SFML](https://www.sfml-dev.org/) bindings
* Tcod-rs
  * [tomassedovic/tcod-rs](https://github.com/tomassedovic/tcod-rs) — Libtcod bindings for Rust.
* Victorem
  * [VictoremWinbringer/Victorem](https://github.com/VictoremWinbringer/Victorem) [[Victorem](https://crates.io/crates/Victorem)] — Easy UDP Game Server and UDP Client framework for creating simple 2D and 3D online game prototype
* Voxlap
  * [bbodi/rust-voxlap](https://github.com/bbodi/rust-voxlap) — [Voxlap](http://advsys.net/ken/voxlap.htm) bindings
* [Awesome wgpu](https://github.com/rofrol/awesome-wgpu) — A curated list of wgpu code and resources

### Geospatial

[[geo](https://crates.io/keywords/geo), [gis](https://crates.io/keywords/gis)]

* [DaveKram/coord_transforms](https://github.com/DaveKram/coord_transforms) [[coord_transforms](https://crates.io/crates/coord_transforms)] — coordinate transformations (2-d, 3-d, and geospatial)
* [Georust](https://github.com/georust) — geospatial tools and libraries written in Rust
* [rust-reverse-geocoder](https://github.com/llambda/rrgeo) — A fast, offline reverse geocoder in Rust, inspired by https://github.com/thampiman/reverse-geocoder
* [vlopes11/geomorph](https://github.com/vlopes11/geomorph) [[geomorph](https://crates.io/crates/geomorph)] — conversion between UTM, LatLon and MGRS coordinates

### Graphics

[[graphics](https://crates.io/keywords/graphics)]

* [gfx-rs/wgpu](https://github.com/gfx-rs/wgpu) - Native WebGPU implementation based on gfx-hal.
* [gfx-rs/gfx](https://github.com/gfx-rs/gfx) — A high-performance, bindless graphics API for Rust.
* Font
  * [redox-os/rusttype](https://github.com/redox-os/rusttype) — A pure Rust alternative to libraries like FreeType
  * [RazrFalcon/rustybuzz](https://github.com/RazrFalcon/rustybuzz) - An incremental harfbuzz port to Rust
* OpenGL [[opengl](https://crates.io/keywords/opengl)]
  * [brendanzab/gl-rs](https://github.com/brendanzab/gl-rs)
  * [glium/glium](https://github.com/glium/glium) — safe OpenGL wrapper for the Rust language.
  * [Kiss3d](http://kiss3d.org) — draw simple geometric figures and play with them with one-liners
  * [PistonDevelopers/glfw-rs](https://github.com/PistonDevelopers/glfw-rs)
  * [glutin](https://crates.io/crates/glutin) — Rust alternative to [GLFW](https://www.glfw.org/)
* PDF
  * [kaj/rust-pdf](https://github.com/kaj/rust-pdf)
  * [fschutt/printpdf](https://github.com/fschutt/printpdf) — PDF writing library
  * [J-F-Liu/lopdf](https://github.com/J-F-Liu/lopdf) — PDF document manipulation
  * [WASM-PDF](https://github.com/jussiniinikoski/wasm-pdf) – Generates PDF files with JavaScript and WASM (WebAssembly)
* [Vulkan](https://www.khronos.org/vulkan/) [[vulkan](https://crates.io/keywords/vulkan)]
  * [vulkano](https://github.com/vulkano-rs/vulkano) [[vulkano](https://crates.io/crates/vulkano)]

### Graph processing

* [kud1ing/tinkerpop-rs](https://github.com/kud1ing/tinkerpop-rs) — an example how to use Apache TinkerPop from Rust

### GUI

[[gui](https://crates.io/keywords/gui)]

* [autopilot-rs/autopilot-rs](https://github.com/autopilot-rs/autopilot-rs) — A simple, cross-platform GUI automation library for Rust.
* [maps4print/azul](https://github.com/maps4print/azul) — A free, functional, IMGUI-oriented GUI framework for rapid development of desktop applications written in Rust, supported by the Mozilla WebRender rendering engine.
* [OrbTk](https://github.com/redox-os/orbtk) — The Orbital Widget Toolkit is a multi platform (G)UI toolkit using SDL2
* [PistonDevelopers/conrod](https://github.com/PistonDevelopers/conrod/) — An easy-to-use, immediate-mode, 2D GUI library written entirely in Rust
* [rise-ui](https://github.com/rise-ui/rise) — Simple component-based cross-Platform GUI Toolkit for developing beautiful and user-friendly interfaces.


* Cocoa
  * [kylewlacy/sorbet-cocoa](https://github.com/kylewlacy/sorbet-cocoa)
  * [servo/core-foundation-rs](https://github.com/servo/core-foundation-rs)
* [FLTK](https://www.fltk.org/)
  * [fltk-rs](https://github.com/MoAlyousef/fltk-rs) — FLTK Rust bindings
* [Flutter](https://flutter.dev/)
  * [flutter-rs](https://github.com/flutter-rs/flutter-rs) — Build flutter desktop app in dart & rust.
* [GTK+](https://www.gtk.org/) [[gtk](https://crates.io/keywords/gtk)]
  * [gtk-rs/gtk](https://github.com/gtk-rs/gtk) — GTK+ bindings
  * [relm](https://github.com/antoyo/relm) — Asynchronous, GTK+-based, GUI library, inspired by Elm
* [ImGui](https://github.com/ocornut/imgui)
  * [imgui-rs](https://github.com/Gekkio/imgui-rs) — Rust bindings for ImGui
* [IUP](http://webserver2.tecgraf.puc-rio.br/iup/)
  * [clear-coat](https://github.com/jminer/clear-coat) — Clear Coat is a Rust wrapper for the IUP GUI library
  * [dcampbell24/iup-rust](https://github.com/dcampbell24/iup-rust) — IUP bindings
  * [Kiss-ui](https://github.com/KISS-UI/kiss-ui) — A simple UI framework built on IUP
* [libui](https://github.com/andlabs/libui)
  * [rust-native-ui/libui-rs](https://github.com/rust-native-ui/libui-rs) — libui bindings
* [Nuklear](https://github.com/Immediate-Mode-UI/Nuklear)
  * [nuklear-rust](https://github.com/snuk182/nuklear-rust) — Rust bindings for Nuklear
* [Qt](https://doc.qt.io)
  * [woboq/qmetaobject-rs](https://github.com/woboq/qmetaobject-rs) — Integrate Qml and Rust by building the QMetaObject at compile time.
  * [cyndis/qmlrs](https://github.com/cyndis/qmlrs) — QtQuick bindings
  * [kitech/qt.rs](https://github.com/kitech/qt.rs) — Qt5 bindings
  * [Rust Qt Binding Generator](https://phabricator.kde.org/source/rust-qt-binding-generator/) — Binding generator hosted by KDE.
  * [rust-qt](https://github.com/rust-qt)
  * [White-Oak/qml-rust](https://github.com/White-Oak/qml-rust) — QML bindings.
* [saurvs/nfd-rs](https://github.com/saurvs/nfd-rs) — [nativefiledialog](https://github.com/mlabbe/nativefiledialog) bindings
* [Sciter](https://sciter.com/)
  * [sciter-sdk/rust-sciter](https://github.com/sciter-sdk/rust-sciter) — Sciter bindings
* [hecrj/iced](https://github.com/hecrj/iced) — A cross-platform GUI library for Rust focused on simplicity and type-safety. Inspired by Elm.
* [ivanceras/sauron-native](https://github.com/ivanceras/sauron-native) - A truly native and cross platform GUI library. One unified code can be run as native GUI, Html Web and TUI.
* [tauri-apps/tauri](https://github.com/tauri-apps/tauri) — Toolchain for building highly secure native apps that have tiny binaries and are very fast from HTML, JS and CSS layer, powered by [webview](https://github.com/webview/webview).

### Image processing

* [abonander/img_hash](https://github.com/abonander/img_hash) — Perceptual image hashing and comparison for equality and similarity.
* [image-rs/image](https://github.com/image-rs/image) — Basic imaging processing functions and methods for converting to and from image formats
* [image-rs/imageproc](https://github.com/image-rs/imageproc) — An image processing library, based on the `image` library.
* [twistedfall/opencv-rust](https://github.com/twistedfall/opencv-rust) — Rust bindings for OpenCV
* [teovoinea/steganography](https://github.com/teovoinea/steganography) [[steganography](https://crates.io/crates/steganography)] — A simple steganography library

### Language specification

* [shnewto/bnf](https://github.com/shnewto/bnf) — A library for parsing Backus–Naur form context-free grammars.

### Logging

[[log](https://crates.io/keywords/log)]

* [seanmonstar/pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger) — A pretty, easy-to-use logger for Rust.
* [rust-lang/log](https://github.com/rust-lang/log) — Logging implementation for Rust
* [slog-rs/slog](https://github.com/slog-rs/slog) — Structured, composable logging for Rust
* [estk/log4rs](https://github.com/estk/log4rs) — highly configurable logging framework modeled after Java's Logback and log4j libraries
* [tokio-rs/tracing](https://github.com/tokio-rs/tracing) — An application level tracing framework for async-aware structured logging, error handling, metrics, and more

### Macro

* cute
  * [mattgathu/cute](https://github.com/mattgathu/cute) — Macro for Python-esque list comprehensions in Rust.
* hado
  * [ludat/hado-rs](https://github.com/ludat/hado-rs) — A little macro for writing haskell-like do expressions without too much ceremony
* [Linq-in-Rust](https://github.com/StardustDL/Linq-in-Rust) - Macro and methods for C#-LINQ-like expressions.

### Markup language

* CommonMark
  * [raphlinus/pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) — [CommonMark](https://commonmark.org/) parser in Rust

### Mobile

[Geal/rust_on_mobile](https://github.com/Geal/rust_on_mobile)

* Android
  * [rust-windowing/android-rs-glue](https://github.com/rust-windowing/android-rs-glue) — glue between Rust and Android
* iOS
  * [TimNN/cargo-lipo](https://github.com/TimNN/cargo-lipo) — A cargo lipo subcommand which automatically creates a universal library for use with your iOS application.
  * [vhbit/ObjCrust](https://github.com/vhbit/ObjCrust) — using Rust to create an iOS static library
* Pebble
  * [andars/pebble.rs](https://github.com/andars/pebble.rs) — A crate that allows Rust to be used to develop Pebble applications.
* Android / iOS
  * [i-schuetz/rust_android_ios](https://github.com/i-schuetz/rust_android_ios) — An example of using a shared Rust lib for Android and iOS using rust-swig and cbindgen respectively.

### Network programming

* HTTP
  * [pop-os/parallel-getter](https://github.com/pop-os/parallel-getter) — Download a file with parallel GET requests to maximize bandwidth usage.
  * [pop-os/url-crawler](https://github.com/pop-os/url-crawler) — A configurable parallel web crawler, designed to crawl a website for content.
  * [pop-os/url-scraper](https://github.com/pop-os/url-scraper) — Scrape URLs from HTML pages
* FTP
  * [mattnenterprise/rust-ftp](https://github.com/mattnenterprise/rust-ftp) — an [FTP](https://en.wikipedia.org/wiki/File_Transfer_Protocol) client for Rust
* gRPC
  * [tikv/grpc-rs](https://github.com/tikv/grpc-rs) — The gRPC library for Rust built on C Core library and futures
* IPNetwork
  * [achanda/ipnetwork](https://github.com/achanda/ipnetwork) — A library to work with IP networks in pure Rust
  * [candrew/netsim](https://github.com/canndrew/netsim) — A Rust library for network simulation and testing
* JSON-RPC
  * [vlopes11/futures-jsonrpc](https://github.com/vlopes11/futures-jsonrpc) [[futures-jsonrpc](https://crates.io/crates/futures-jsonrpc)] — Futures implementation for JSON-RPC
* Low level
  * [libpnet/libpnet](https://github.com/libpnet/libpnet) — A cross-platform, low level networking
  * [smoltcp-rs/smoltcp](https://github.com/smoltcp-rs/smoltcp) — A standalone, event-driven TCP/IP stack that is designed for bare-metal, real-time systems
  * [tokio-rs/tokio](https://github.com/tokio-rs/tokio) — A network application framework for rapid development and highly scalable production deployments of clients and servers.
  * [dylanmckay/protocol](https://github.com/dylanmckay/protocol) — Custom TCP/UDP protocol definitions
  * [actix/actix](https://github.com/actix/actix) — Actor library for Rust
* NanoMsg
  * [thehydroimpulse/nanomsg.rs](https://github.com/thehydroimpulse/nanomsg.rs) — [nanomsg](https://nanomsg.org/) bindings
* Nng
  * [neachdainn/nng-rs](https://gitlab.com/neachdainn/nng-rs) [[Nng](https://crates.io/crates/nng)] — [Nng (nanomsg v2)](https://nng.nanomsg.org/index.html) bindings
* NNTP
  * [mattnenterprise/rust-nntp](https://github.com/mattnenterprise/rust-nntp) — an [NNTP](https://en.wikipedia.org/wiki/Network_News_Transfer_Protocol) client for Rust
* POP3
  * [mattnenterprise/rust-pop3](https://github.com/mattnenterprise/rust-pop3) — A [POP3](https://en.wikipedia.org/wiki/Post_Office_Protocol) client for Rust
* SSH
  * [alexcrichton/ssh2-rs](https://github.com/alexcrichton/ssh2-rs) — [libssh2](https://www.libssh2.org/) bindings
  * [Thrussh](https://github.com/pijul-scm/thrussh/) — an SSH library written from scratch in Rust, backed by [libsodium](https://doc.libsodium.org/)
* Stomp
  * [zslayton/stomp-rs](https://github.com/zslayton/stomp-rs) — A [STOMP 1.2](http://stomp.github.io/stomp-specification-1.2.html) client implementation in Rust
* uTP
  * [meqif/rust-utp](https://github.com/meqif/rust-utp) — A [uTP](http://www.bittorrent.org/beps/bep_0029.html) (Micro Transport Protocol) library for Rust.
* ZeroMQ
  * [erickt/rust-zmq](https://github.com/erickt/rust-zmq) — [ZeroMQ](https://zeromq.org/) bindings
* CoAP
  * [Covertness/coap-rs](https://github.com/Covertness/coap-rs) — A [Constrained Application Protocol(CoAP)](https://tools.ietf.org/html/rfc7252) library for Rust.
* Docker
  * [fussybeaver/bollard](https://github.com/fussybeaver/bollard) — Docker daemon API in Rust
* RPC
  * [smallnest/rpcx-rs](https://github.com/smallnest/rpcx-rs) — A RPC library for Rust for developing microservices in easy and simple way.
* QUIC
  * [cloudflare/quiche](https://github.com/cloudflare/quiche) — cloudflare implementation of the QUIC transport protocol and HTTP/3
  * [djc/quinn](https://github.com/djc/quinn) — Futures-based QUIC implementation in Rust
  * [mozilla/neqo](https://github.com/mozilla/neqo) — an Implementation of QUIC written in Rust
* P2P
  * [libp2p/rust-libp2p](https://github.com/libp2p/rust-libp2p) — The Rust Implementation of libp2p networking stack.

### Parsing

  * [aleshaleksey/libazdice](https://github.com/aleshaleksey/libazdice) — A parser for parsing dice strings for TTRPGs and TTRPG development
  * [Folyd/robotstxt](https://github.com/Folyd/robotstxt) - A native Rust port of Google's robots.txt parser and matcher C++ library
  * [Geal/nom](https://github.com/Geal/nom) — parser combinator library
  * [ivanceras/inquerest](https://github.com/ivanceras/inquerest) — an URL parameter parser for rest filter inquiry
  * [kevinmehall/rust-peg](https://github.com/kevinmehall/rust-peg) — Parsing Expression Grammar (PEG) parser generator
  * [m4rw3r/chomp](https://github.com/m4rw3r/chomp) – A fast monadic-style parser combinator
  * [Marwes/combine](https://github.com/Marwes/combine) — parser combinator library
  * [lalrpop/lalrpop](https://github.com/lalrpop/lalrpop) — LR(1) parser generator for Rust
  * [nrc/zero](https://github.com/nrc/zero) — zero-allocation parsing of binary data
  * [pest-parser/pest](https://github.com/pest-parser/pest) — The Elegant Parser
  * [ptal/oak](https://github.com/ptal/oak) — A typed PEG parser generator (compiler plugin)
  * [replicadse/wavefront_rs](https://github.com/replicadse/wavefront_rs) — A parser for the Wavefront OBJ format.
  * [s-panferov/queryst](https://github.com/s-panferov/queryst) — A query string parsing library for Rust inspired by https://github.com/ljharb/qs
  * [freestrings/jsonpath](https://github.com/freestrings/jsonpath) — [JsonPath](https://goessner.net/articles/JsonPath/) engine written in Rust. Webassembly and Javascript support too

### Packaging formats

- [pop-os/debarchive](https://github.com/pop-os/debarchive) Library for reading and extracting debian archives

### Peripherals

* Serial Port
  * [Susurrus/serialport-rs](https://github.com/Susurrus/serialport-rs) [[serialport](https://docs.rs/serialport/3.0.0/serialport/)] — A cross-platform library that provides access to a serial port

### Platform specific

* Cross-platform
  * [svartalf/rust-battery](https://crates.io/crates/battery) — Cross-platform information about the notebook batteries

* Linux
  * [frol/cgroups-fs](https://github.com/frol/cgroups-fs) — Rust bindings to Linux Control Groups (cgroups)
  * [pop-os/dbus-udisks2](https://github.com/pop-os/dbus-udisks2) — UDisks2 DBus API
  * [pop-os/distinst](https://github.com/pop-os/distinst/) — Linux distribution installer library
  * [hannobraun/inotify](https://github.com/hannobraun/inotify) — [inotify](https://en.wikipedia.org/wiki/Inotify) bindings
  * [arvancloud/nginx-rs](https://github.com/arvancloud/nginx-rs) — [Nginx](https://www.nginx.com) bindings
  * [yaa110/rust-iptables](https://github.com/yaa110/rust-iptables) — [iptables](https://www.netfilter.org/projects/iptables/index.html) bindings
* Unix-like
  * [nix-rust/nix](https://github.com/nix-rust/nix) — Unix-like API bindings
  * [zargony/fuse-rs](https://github.com/zargony/fuse-rs) — [FUSE](https://github.com/libfuse/libfuse) bindings
* Windows
  * [retep998/winapi-rs](https://github.com/retep998/winapi-rs) — Windows API bindings
* FreeBSD
  * [fubarnetes/libjail-rs](https://github.com/fubarnetes/libjail-rs/) — Rust implementation of a FreeBSD jail library
  * [dlrobertson/capsicum-rs](https://github.com/dlrobertson/capsicum-rs) — Rust bindings for the FreeBSD capsicum framework

### Scripting

[[scripting](https://crates.io/keywords/scripting)]

* [duckscript](https://crates.io/crates/duckscript) — [Simple, extendable and embeddable scripting language.](https://github.com/sagiegurari/duckscript)
* [PistonDevelopers/dyon](https://github.com/PistonDevelopers/dyon) — A rusty dynamically typed scripting language
* [gluon-lang/gluon](https://github.com/gluon-lang/gluon) —  A small, statically-typed, functional programming language
* [murarth/ketos](https://github.com/murarth/ketos) — A Lisp dialect functional programming language serving as a scripting and extension language for rust
* [moss](https://crates.io/crates/moss) — A dynamically typed scripting language
* [mun](https://github.com/mun-lang/mun) — A compiled, statically-typed scripting language with first class hot reloading support
* [jonathandturner/rhai](https://github.com/jonathandturner/rhai) — A tiny and fast embedded scripting language resembling a combination of JS and Rust

### Simulation

[[simulation](https://crates.io/keywords/simulation)]

* [bigbang](https://crates.io/crates/bigbang) - Gravitational and collisional n-body simulation with optional GPU acceleration
* [nyx-space](https://crates.io/crates/nyx-space) - High fidelity, fast, reliable and validated astrodynamical toolkit library, used for spacecraft mission design and orbit determination

### Template engine

* Handlebars
  * [sunng87/handlebars-rust](https://github.com/sunng87/handlebars-rust) — Handlebars template engine with inheritance, custom helper support.
  * [botika/yarte](https://github.com/botika/yarte) — Yarte stands for **Y**et **A**nother **R**ust **T**emplate **E**ngine, is the fastest template engine.
* HTML
  * [lambda-fairy/maud](https://github.com/lambda-fairy/maud) — compile-time HTML templates
  * [Stebalien/horrorshow-rs](https://github.com/Stebalien/horrorshow-rs) — compile-time HTML templates
  * [kaj/ructe](https://github.com/kaj/ructe) — HTML template system for Rust
  * [Keats/tera](https://github.com/Keats/tera) — template engine based on Jinja2 and the Django template language.
  * [djc/askama](https://github.com/djc/askama) — template rendering engine based on Jinja
  * [naomijub/hiccup](https://github.com/naomijub/hiccup) — template engine inpired by Clojure's Hiccup.
* Mustache
  * [rustache/rustache](https://github.com/rustache/rustache)
* [tailhook/marafet](https://github.com/tailhook/marafet) — Compiler for Jade-like template language to cito.js-based virtual dom

### Text processing

* [BurntSushi/suffix](https://github.com/BurntSushi/suffix) — Linear time suffix array construction (with Unicode support)
* [BurntSushi/tabwriter](https://github.com/BurntSushi/tabwriter) — Elastic tab stops (i.e., text column alignment)
* [mgeisler/textwrap](https://github.com/mgeisler/textwrap) [[textwrap](https://crates.io/crates/textwrap)] — Word wrap text (with support for hyphenation)
* [pwoolcoc/ngrams](https://github.com/pwoolcoc/ngrams) — Construct [n-grams](https://en.wikipedia.org/wiki/N-gram) from arbitrary iterators
* [ps1dr3x/easy_reader](https://github.com/ps1dr3x/easy_reader) — A reader that allows forwards, backwards and random navigations through the lines of huge files without consuming iterators
* [rust-lang/regex](https://github.com/rust-lang/regex) — Regular expressions (RE2 style)
* [strsim-rs](https://crates.io/crates/strsim) — String similarity metrics
* [greyblake/whatlang-rs](https://github.com/greyblake/whatlang-rs) — Natural language detection library based on trigrams
* [yaa110/rake-rs](https://github.com/yaa110/rake-rs) — Multilingual implementation of RAKE algorithm for Rust
* [Lucretiel/joinery](https://github.com/Lucretiel/joinery) [[joinery](https://crates.io/crates/joinery)] – Generic string + iterable joining
* [Daniel-Liu-c0deb0t/triple_accel](https://github.com/Daniel-Liu-c0deb0t/triple_accel) [[triple_accel](https://crates.io/crates/triple_accel)] - Rust edit distance routines accelerated using SIMD; supports fast Hamming, Levenshtein, restricted Damerau-Levenshtein, etc. distance calculations and string search

### Text search

* [andylokandy/simsearch-rs](https://github.com/andylokandy/simsearch-rs) [[simsearch](https://crates.io/crates/simsearch)] — A simple and lightweight fuzzy search engine that works in memory, searching for similar strings
* [BurntSushi/fst](https://github.com/BurntSushi/fst) [[fst](https://crates.io/crates/fst)]
* [meilisearch/MeiliSearch](https://github.com/meilisearch/MeiliSearch) — Ultra relevant, instant and typo-tolerant full-text search API.
* [minio/minsql](https://github.com/minio/minsql) — High-performance log search engine.
* [CurrySoftware/perlin](https://github.com/CurrySoftware/perlin) [[perlin](https://crates.io/crates/perlin)]
* [tantivy-search/tantivy](https://github.com/tantivy-search/tantivy) [[tantivy](https://crates.io/crates/tantivy)]

### Unsafe

* [zerocopy](https://crates.io/crates/zerocopy) — Utilities for safely reinterpreting arbitrary byte sequences as native Rust types

### Virtualization

* [beneills/quantum](https://github.com/beneills/quantum) — Advanced Rust quantum computer simulator
* [chromium/chromiumos/platform/crosvm](https://chromium.googlesource.com/chromiumos/platform/crosvm/) CrOSVM — Enables Chrome OS to run Linux apps inside a fast, secure virtualized environment
* [unicorn-rs/unicorn-rs](https://github.com/unicorn-rs/unicorn-rs) — Rust bindings for the unicorn CPU emulator
* [saurvs/hypervisor-rs](https://github.com/saurvs/hypervisor-rs) — Hardware-accelerated virtualization on OS X

### Web programming

See also [Are we web yet?](http://www.arewewebyet.org) and [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison).

* Client-side / WASM
  * [cargo-web](https://crates.io/crates/cargo-web) — A Cargo subcommand for the client-side Web
  * [seed](https://seed-rs.org/) — A Rust framework for creating web apps
  * [stdweb](https://crates.io/crates/stdweb) — A standard library for the client-side Web
  * [yew](https://crates.io/crates/yew) — Rust framework for making client web apps
  * [sauron](https://github.com/ivanceras/sauron) - Client side web framework which closely adheres to The Elm Architecture.
* HTTP Client
  * [alexcrichton/curl-rust](https://github.com/alexcrichton/curl-rust) — [libcurl](https://curl.haxx.se/libcurl/) bindings
  * [async-graphql](https://github.com/async-graphql/async-graphql) - A GraphQL server library implemented in Rust
  * [graphql-client](https://github.com/graphql-rust/graphql-client) — Typed, correct GraphQL requests and responses in Rust.
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation
  * [seanmonstar/reqwest](https://github.com/seanmonstar/reqwest) — an ergonomic HTTP Client for Rust.
  * [DoumanAsh/yukikaze](https://gitlab.com/Douman/yukikaze) — Beautiful and elegant Yukikaze is little HTTP client library based on hyper.
* HTTP Server
  * [actix/actix-web](https://github.com/actix/actix-web) — A lightweight async web framework for Rust with websocket support
  * [branca](https://crates.io/crates/branca) — A Pure Rust implementation of Branca for Authenticated and Encrypted API tokens.
  * [Gotham](https://github.com/gotham-rs/gotham) — A flexible web framework that does not sacrifice safety, security or speed.
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation
  * [GildedHonour/frank_jwt](https://github.com/GildedHonour/frank_jwt) — JSON Web Token implementation in Rust.
  * [handlebars-rust](https://github.com/sunng87/handlebars-rust) — an Iron web framework middleware.
  * [Iron](https://github.com/iron/iron) — A middleware-based server framework
  * [Juniper](https://github.com/graphql-rust/juniper) — GraphQL server library for Rust
  * [Nickel](https://github.com/nickel-org/nickel.rs/) — inspired by [Express](http://expressjs.com/)
  * [Ogeon/rustful](https://github.com/Ogeon/rustful) — A RESTful web framework for Rust
  * [Rocket](https://github.com/SergioBenitez/Rocket) — Rocket is web framework for Rust (nightly) with a focus on ease-of-use, expressability, and speed
  * [Rustless](https://github.com/rustless/rustless) — A REST-like API micro-framework inspired by [Grape](https://github.com/ruby-grape/grape) and [Hyper](https://github.com/hyperium/hyper)
  * [Saphir](https://github.com/richerarc/saphir) — A progressive web framework with low-level control, without the pain.
  * [daogangtang/sapper](https://github.com/daogangtang/sapper) — A lightweight web framework built on async hyper, implemented in Rust language.
  * [tiny-http](https://github.com/tiny-http/tiny-http) — Low level HTTP server library
  * [tomaka/rouille](https://github.com/tomaka/rouille) — Web framework in Rust
  * [carllerche/tower-web](https://github.com/carllerche/tower-web) [[tower-web](https://crates.io/crates/tower-web)] — A fast, boilerplate free, web framework for Rust
  * [danclive/sincere](https://github.com/danclive/sincere) — A micro web framework for Rust(stable) based on hyper and multithreading.
  * [oltdaniel/zap](https://github.com/oltdaniel/zap) — A lightning fast http framework for Rust
* [WebSocket](https://datatracker.ietf.org/doc/rfc6455/)
  * [actix/sockjs](https://github.com/actix/sockjs) — A [SockJS](https://github.com/sockjs) server for Rust
  * [rust-websocket](https://github.com/websockets-rs/rust-websocket) — A framework for dealing with WebSocket connections (both clients and servers)
  * [housleyjk/ws-rs](https://github.com/housleyjk/ws-rs) — lightweight, event-driven WebSockets for Rust
  * [snapview/tungstenite-rs](https://github.com/snapview/tungstenite-rs) — Lightweight stream-based WebSocket implementation for Rust.
  * [vi/websocat](https://github.com/vi/websocat) — CLI for interacting with WebSockets, with functionality of Netcat, Curl and Socat.
  * [vityafx/urlshortener-rs](https://github.com/vityafx/urlshortener-rs) [[urlshortener](https://crates.io/crates/urlshortener)] — A very simple urlshortener library for Rust.
  * [bitwyre/websocket_core](https://github.com/bitwyre/websocket_core) — Websocket generic server library for periodic message broadcast.
* Miscellaneous
  * [cargonauts](https://github.com/cargonauts-rs/cargonauts) — A web framework intended for building maintainable, well-factored web apps.
  * [pyros2097/rust-embed](https://github.com/pyros2097/rust-embed) — A macro to embed static assets into the rust binary
  * [utkarshkukreti/select.rs](https://github.com/utkarshkukreti/select.rs) [[select](https://crates.io/crates/select)] — A library to extract useful data from HTML documents, suitable for web scraping.
  * [pwoolcoc/soup](https://gitlab.com/pwoolcoc/soup) [[soup](https://crates.io/crates/soup)] — A library similar to Python's BeautifulSoup, designed to enable quick and easy manipulation and querying of HTML documents.
  * [softprops/openapi](https://github.com/softprops/openapi) — A library for processing openapi spec files
  * [tbot](https://gitlab.com/SnejUgal/tbot) - Make cool Telegram bots with Rust easily
  * [teloxide/teloxide](https://github.com/teloxide/teloxide/) - An elegant Telegram bots framework for Rust
* Reverse Proxy
  * [sozu-proxy/sozu](https://github.com/sozu-proxy/sozu) [[sozu](https://crates.io/crates/sozu)] — A HTTP reverse proxy.
* Static Site Generators
  * [getzola/zola](https://github.com/getzola/zola) [[zola](https://www.getzola.org/)] — An opinionated static site generator with everything built-in.
  * [cobalt-org/cobalt.rs](https://github.com/cobalt-org/cobalt.rs) — Static site generator written in Rust
  * [FuGangqiang/mdblog.rs](https://github.com/FuGangqiang/mdblog.rs) — Static site generator from markdown files.
  * [leven-the-blog/leven](https://github.com/leven-the-blog/leven) [[leven](https://crates.io/crates/leven)] — A simple, parallelized blog generator.

## Registries

A registry allows you to publish your Rust libraries as crate packages, to share them with others publicly and privately.

* [Crates](https://crates.io) — The official public registry for Rust/Cargo.
* [Cloudsmith :heavy_dollar_sign:](https://cloudsmith.com/cargo-registry/) — A fully managed package management SaaS, with first-class support for public and private Cargo/Rust registries (plus many others). Has a generous free-tier and is also completely free for open-source.

## Resources

* Benchmarks
  * [TeXitoi/benchmarksgame-rs](https://github.com/TeXitoi/benchmarksgame-rs) — Rust implementations for the [The Computer Language Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/)
* Decks & Presentations
  * [Learning systems programming with Rust](https://speakerdeck.com/jvns/learning-systems-programming-with-rust) — Presented by [Julia Evans](https://twitter.com/@b0rk) @ Rustconf 2016.
  * [Shipping a Solid Rust Crate](https://www.youtube.com/watch?v=t4CyEKb-ywA) — Presented by [Michael Gattozzi](https://github.com/mgattozzi) @ RustConf 2017
  * [Rust: Hack Without Fear!](https://www.youtube.com/watch?v=lO1z-7cuRYI) — Presented by [Nicholas Matsakis](https://github.com/nikomatsakis) @ C++Now 2018
* Learning
  * [Programming Community Curated Resources for Learning Rust](https://hackr.io/tutorials/learn-rust) — A list of recommended resources voted by the programming community.
  * [awesome-rust-mentors](https://rustbeginners.github.io/awesome-rust-mentors/) — A list of helpful Rust mentors willing to take mentees and eductate them about Rust and programming.
  * [Awesome Rust Streaming](https://github.com/jamesmunns/awesome-rust-streaming) - A community curated list of livestreams about Rust.
  * [exercism.io](https://exercism.io/tracks/rust) — programming exercises that help you learn new concepts in Rust.
  * [Idiomatic Rust](https://github.com/mre/idiomatic-rust) —  A peer-reviewed collection of articles/talks/repos which teach idiomatic Rust.
  * [Learning Rust With Entirely Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/) — in-depth exploration of Rust's memory management rules, through implementing a few different types of list structures.
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  * [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) — A collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem.
  * [Rust Online Courses at Classpert](https://classpert.com/rust-programming) — A list of Rust online courses (paid) from Classpert Online Course Search
  * [Rust for professionals](https://overexact.com/rust-for-professionals/) — A quick introduction to Rust for experienced software developers.
  * [Rust in Motion](https://www.manning.com/livevideo/rust-in-motion?a_aid=cnichols&a_bid=6a993c2e) — A video series by [Carol Nichols](https://github.com/carols10cents) and [Jake Goulding](https://github.com/shepmaster) (paid)
  * [rust-learning](https://github.com/ctjhoa/rust-learning) — A collection of useful resources to learn Rust
  * [Rustlings](https://github.com/rust-lang/rustlings) — small exercises to get you used to reading and writing Rust code
  * [stdx](https://github.com/brson/stdx) — Learn these crates first as an extension to std
  * [University of Pennsylvania's Comp Sci Rust Programming Course](http://cis198-2016s.github.io/schedule/)
  * [Build a language VM](https://blog.subnetzero.io/post/building-language-vm-part-00/)
  * [Code Playground](https://codeplayground.app) - Interactively edit & play rust snippets on your iPhone and iPad devices.
* Podcasts
  * [New Rustacean](https://newrustacean.com) — A podcast about learning Rust
  * [Rustacean Station](https://rustacean-station.org/) — A community project for creating podcast content for Rust
  * [Rusty Spike](https://rusty-spike.blubrry.net) — news on all things Rust
* [RustCamp 2015 Talks](https://www.youtube.com/playlist?list=PLE7tQUdRKcybdIw61JpCoo89i4pWU5f_t)
* [Rust Design Patterns](https://github.com/rust-unofficial/patterns)
* [Rust Guidelines](http://aturon.github.io/)
* [RustBooks](https://github.com/sger/RustBooks) — list of RustBooks
* [Rust Subreddit](https://www.reddit.com/r/rust/) — A subreddit(forum) where rust related questions, articles and resources are posted and discussed

## License

[![CC0](https://licensebuttons.net/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)
