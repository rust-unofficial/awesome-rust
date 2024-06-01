# Awesome Rust [![build badge](https://github.com/rust-unofficial/awesome-rust/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rust-unofficial/awesome-rust/actions/workflows/rust.yml) [![Track Awesome List](https://www.trackawesomelist.com/badge.svg)](https://www.trackawesomelist.com/rust-unofficial/awesome-rust/)

A curated list of Rust code and resources.

If you want to contribute, please read [this](CONTRIBUTING.md).

## Table of contents

<!-- toc -->

- [Applications](#applications)
  * [Audio and Music](#audio-and-music)
  * [Blockchain](#blockchain)
  * [Database](#database)
  * [Emulators](#emulators)
  * [File manager](#file-manager)
  * [Games](#games)
  * [Graphics](#graphics)
  * [Image processing](#image-processing)
  * [Industrial automation](#industrial-automation)
  * [Observability](#observability)
  * [Operating systems](#operating-systems)
  * [Package Managers](#package-managers)
  * [Payments](#payments)
  * [Productivity](#productivity)
  * [Routing protocols](#routing-protocols)
  * [Security tools](#security-tools)
  * [Social networks](#social-networks)
  * [System tools](#system-tools)
  * [Task scheduling](#task-scheduling)
  * [Text editors](#text-editors)
  * [Text processing](#text-processing)
  * [Utilities](#utilities)
  * [Video](#video)
  * [Virtualization](#virtualization)
  * [Web](#web)
  * [Web Servers](#web-servers)
- [Development tools](#development-tools)
  * [Build system](#build-system)
  * [Debugging](#debugging)
  * [Deployment](#deployment)
  * [Embedded](#embedded)
  * [FFI](#ffi)
  * [Formatters](#formatters)
  * [IDEs](#ides)
  * [Profiling](#profiling)
  * [Services](#services)
  * [Static analysis](#static-analysis)
  * [Testing](#testing)
  * [Transpiling](#transpiling)
- [Libraries](#libraries)
  * [Artificial Intelligence](#artificial-intelligence)
    + [Genetic algorithms](#genetic-algorithms)
    + [Machine learning](#machine-learning)
    + [OpenAI](#openai)
  * [Astronomy](#astronomy)
  * [Asynchronous](#asynchronous)
  * [Audio and Music](#audio-and-music-1)
  * [Authentication](#authentication)
  * [Automotive](#automotive)
  * [Bioinformatics](#bioinformatics)
  * [Caching](#caching)
  * [Cloud](#cloud)
  * [Command-line](#command-line)
  * [Compression](#compression)
  * [Computation](#computation)
  * [Concurrency](#concurrency)
  * [Configuration](#configuration)
  * [Cryptography](#cryptography)
  * [Data processing](#data-processing)
  * [Data streaming](#data-streaming)
  * [Data structures](#data-structures)
  * [Data visualization](#data-visualization)
  * [Database](#database-1)
  * [Date and time](#date-and-time)
  * [Distributed systems](#distributed-systems)
  * [Domain driven design](#domain-driven-design)
  * [eBPF](#ebpf)
  * [Email](#email)
  * [Encoding](#encoding)
  * [Filesystem](#filesystem)
  * [Finance](#finance)
  * [Functional Programming](#functional-programming)
  * [Game development](#game-development)
  * [Geospatial](#geospatial)
  * [Graph algorithms](#graph-algorithms)
  * [Graphics](#graphics-1)
  * [GUI](#gui)
  * [Image processing](#image-processing-1)
  * [Language specification](#language-specification)
  * [Logging](#logging)
  * [Macro](#macro)
  * [Markup language](#markup-language)
  * [Mobile](#mobile)
  * [Network programming](#network-programming)
  * [Parsing](#parsing)
  * [Peripherals](#peripherals)
  * [Platform specific](#platform-specific)
  * [Scripting](#scripting)
  * [Simulation](#simulation)
  * [System](#system)
  * [Task scheduling](#task-scheduling-1)
  * [Template engine](#template-engine)
  * [Text processing](#text-processing-1)
  * [Text search](#text-search)
  * [Unsafe](#unsafe)
  * [Video](#video-1)
  * [Virtualization](#virtualization-1)
  * [Web programming](#web-programming)
- [Registries](#registries)
- [Resources](#resources)
- [License](#license)

<!-- tocstop -->

## Applications

See also [Rust — Production](https://www.rust-lang.org/production) organizations running Rust in production.

* [alacritty](https://github.com/alacritty/alacritty) — A cross-platform, GPU enhanced terminal emulator
* [Arti](https://gitlab.torproject.org/tpo/core/arti) — An implementation of Tor. (So far, it's a not-very-complete client. But watch this space!) [![Crates.io](https://img.shields.io/crates/v/arti.svg)](https://crates.io/crates/arti)
* [asm-cli-rust](https://github.com/cch123/asm-cli-rust) — An interactive assembly shell.
* [cloudflare/boringtun](https://github.com/cloudflare/boringtun) — A Userspace WireGuard VPN Implementation [![build badge](https://img.shields.io/badge/crates.io-v0.2.0-orange.svg)](https://crates.io/crates/boringtun)
* [datafusion](https://github.com/apache/datafusion) — Apache Arrow DataFusion and Ballista query engines
* [defguard](https://github.com/defguard/defguard) — Enterprise Open Source SSO & WireGuard VPN with real 2FA/MFA
* [denoland/deno](https://github.com/denoland/deno) — A secure JavaScript/TypeScript runtime built with V8 and Tokio [![Build Status](https://github.com/denoland/deno/workflows/ci/badge.svg?branch=master&event=push)](https://github.com/denoland/deno/actions)
* [doprz/dipc](https://github.com/doprz/dipc) — Convert your favorite images and wallpapers with your favorite color palettes/themes [![crates.io](https://img.shields.io/crates/v/dipc)](https://crates.io/crates/dipc)
* [EasyTier](https://github.com/EasyTier/EasyTier) - A simple, full-featured and decentralized mesh VPN with WireGuard support. [![crates.io](https://img.shields.io/crates/v/easytier)](https://crates.io/crates/easytier) [![GitHub actions](https://github.com/EasyTier/EasyTier/actions/workflows/core.yml/badge.svg)](https://github.com/EasyTier/EasyTier/actions/)[![GitHub actions](https://github.com/EasyTier/EasyTier/actions/workflows/gui.yml/badge.svg)](https://github.com/EasyTier/EasyTier/actions/)
* [Factotum](https://github.com/snowplow/factotum) — A system to programmatically run data pipelines
* [fcsonline/drill](https://github.com/fcsonline/drill) — A HTTP load testing application inspired by Ansible syntax
* [fend](https://github.com/printfn/fend) - Arbitrary-precision unit-aware calculator [![build](https://github.com/printfn/fend/workflows/build/badge.svg)](https://github.com/printfn/fend)
* [Fractalide](https://github.com/fractalide/fractalide) — Simple microservices
* [habitat](https://github.com/habitat-sh/habitat) — A tool created by Chef to build, deploy, and manage applications.
* [Herd](https://github.com/imjacobclark/Herd) — an experimental HTTP load testing application
* [hickory-dns](https://crates.io/crates/trust-dns) — A DNS-server [![Build Status](https://github.com/hickory-dns/hickory-dns/workflows/test/badge.svg?branch=main)](https://github.com/hickory-dns/hickory-dns/actions?query=workflow%3Atest)
* [innernet](https://github.com/tonarino/innernet) - An overlay or private mesh network that uses Wireguard under the hood
* [jedisct1/flowgger](https://github.com/awslabs/flowgger) — A fast, simple and lightweight data collector
* [kalker](https://github.com/PaddiM8/kalker) - A scientific calculator that supports math-like syntax with user-defined variables, functions, derivation, integration, and complex numbers. Cross-platform + WASM support [![Build Status](https://github.com/PaddiM8/kalker/workflows/Release/badge.svg)](https://github.com/PaddiM8/kalker/actions)
* [kftray](https://github.com/hcavarsan/kftray) - A cross-platform system tray app for managing and sharing multiple kubectl port-forward configurations. [![Build Status](https://github.com/hcavarsan/kftray/workflows/Release/badge.svg)](https://github.com/hcavarsan/kftray/actions)
* [kytan](https://github.com/changlan/kytan) — High Performance Peer-to-Peer VPN
* [linkerd/linkerd2-proxy](https://github.com/linkerd/linkerd2-proxy) — Ultralight service mesh for Kubernetes.
* [MaidSafe](https://github.com/maidsafe) — A decentralized platform.
* [mdBook](https://github.com/rust-lang/mdBook) — A command line utility to create books from markdown files [![Build Status](https://github.com/rust-lang/mdBook/workflows/CI/badge.svg?branch=master)](https://github.com/rust-lang/mdBook/actions)
* [mirrord](https://github.com/metalbear-co/mirrord) — Connect your local process and your cloud environment, and run local code in cloud conditions
* [nicohman/eidolon](https://github.com/nicohman/eidolon) — A steam and drm-free game registry and launcher for linux and macosx
* [notty](https://github.com/withoutboats/notty) — A new kind of terminal
* [Pijul](https://pijul.org) — A patch-based distributed version control system
* [Rauthy](https://github.com/sebadob/rauthy) — OpenID Connect Single Sign-On Identity & Access Management
* [Rio](https://github.com/raphamorim/rio) - A hardware-accelerated GPU terminal emulator powered by WebGPU, focusing to run in desktops and browsers.
* [rx](https://github.com/cloudhead/rx) — Vi inspired Modern Pixel Art Editor
* [Servo](https://github.com/servo/servo) — A prototype web browser engine
* [shoes](https://github.com/cfal/shoes) - A multi-protocol proxy server
* [shuttle](https://github.com/shuttle-hq/shuttle) — A serverless platform.
* [Sniffnet](https://github.com/GyulyVGC/sniffnet) — Cross-platform application to monitor your network traffic with ease [![build badge](https://img.shields.io/github/actions/workflow/status/gyulyvgc/sniffnet/rust.yml?logo=github)](https://github.com/GyulyVGC/sniffnet/blob/main/.github/workflows/rust.yml) [![crate](https://img.shields.io/crates/v/sniffnet?logo=rust)](https://crates.io/crates/sniffnet)
* [SWC](https://github.com/swc-project/swc) — super-fast TypeScript / JavaScript compiler
* [tiny](https://github.com/osa1/tiny) — A terminal IRC client
* [UpVPN](https://github.com/upvpn/upvpn-app)  — WireGuard VPN client for macOS, Linux, and Windows built on Tauri.
* [wasmer](https://github.com/wasmerio/wasmer) — A safe and fast WebAssembly runtime supporting WASI and Emscripten [![Build Status](https://github.com/wasmerio/wasmer/workflows/build/badge.svg?style=flat-square)](https://github.com/wasmerio/wasmer/actions)
* [Weld](https://github.com/serayuzgur/weld) — Full fake REST API generator
* [wezterm](https://github.com/wez/wezterm) — A GPU-accelerated cross-platform terminal emulator and multiplexer
* [WinterJS](https://github.com/wasmerio/winterjs) — A secure JavaScript runtime built with SpiderMonkey and Axum
* [zellij](https://github.com/zellij-org/zellij) — A terminal multiplexer (workspace) with batteries included

### Audio and Music

* [dano](https://github.com/kimono-koans/dano) — A hashdeep/md5tree (but much more) for media files
* [enginesound](https://github.com/DasEtwas/enginesound) — A GUI and command line application used to procedurally generate semi-realistic engine sounds. Featuring in-depth configuration, variable sample rate and a frequency analysis window.
* [Festival](https://github.com/hinto-janai/festival) — A local music player/server/client [![build-badge](https://github.com/hinto-janai/festival/actions/workflows/ci.yml/badge.svg)](https://github.com/hinto-janai/festival/actions/workflows/ci.yml)
* [figsoda/mmtc](https://github.com/figsoda/mmtc) [[mmtc](https://crates.io/crates/mmtc)] — Minimal mpd terminal client that aims to be simple yet highly configurable [![build-badge](https://github.com/figsoda/mmtc/actions/workflows/ci.yml/badge.svg)](https://github.com/figsoda/mmtc/actions/workflows/ci.yml)
* [Glicol](https://github.com/chaosprint/glicol) — Graph-oriented live coding language, for collaborative musicking in browsers.
* [ncspot](https://github.com/hrkfdn/ncspot) - Cross-platform ncurses Spotify client, inspired by ncmpc and the likes. [![build badge](https://github.com/hrkfdn/ncspot/workflows/Build/badge.svg)](https://github.com/hrkfdn/ncspot/actions?query=workflow%3ABuild)
* [Pinepods](https://github.com/madeofpendletonwool/PinePods) — A rust based podcast management system with multi-user support. Pinepods utilizes a central database so aspects like listen time and themes follow from device to device. With clients built using Tauri, it's a full cross-platform listening solution! ![Docker Container Build](https://github.com/madeofpendletonwool/PinePods/actions/workflows/docker-publish.yml/badge.svg)
* [Polaris](https://github.com/agersant/polaris) — A music streaming application.
* [Spotify Player](https://github.com/aome510/spotify-player) — A Spotify player in the terminal with full feature parity.
* [Spotifyd](https://github.com/Spotifyd/spotifyd) — An open source Spotify client running as a UNIX daemon. ![Continuous Integration](https://github.com/Spotifyd/spotifyd/workflows/Continuous%20Integration/badge.svg?branch=master)
* [termusic](https://github.com/tramhao/termusic) - Music Player TUI written
* [WhatBPM](https://github.com/sergree/whatbpm) — A daily statically generated information resource for electronic dance music producers. Provides daily analytics on the most frequently used values for each EDM genre: tempos, keys, root notes, and so on, using publicly available data such as Beatport and Spotify. ![Continuous Integration](https://github.com/sergree/whatbpm/actions/workflows/website_build_deploy.yml/badge.svg?branch=main)

### Blockchain

* [artemis](https://github.com/paradigmxyz/artemis) - A simple, modular, and fast framework for writing MEV bots.
* [beerus](https://github.com/eigerco/beerus) - Beerus is a trustless StarkNet Light Client, ⚡blazing fast ⚡ [![GitHub Workflow Status](https://github.com/eigerco/beerus/actions/workflows/test.yml/badge.svg)](https://github.com/eigerco/beerus/actions/workflows/test.yml)
* [Bitcoin Satoshi's Vision](https://github.com/brentongunning/rust-sv) [[sv](https://crates.io/crates/sv)] — A library for working with Bitcoin SV.
* [cairo](https://github.com/starkware-libs/cairo) - Cairo is the first Turing-complete language for creating provable programs for general computation. This is also the native language of [StarkNet](https://www.starknet.io), a ZK-Rollup using STARK proofs ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/starkware-libs/cairo/CI?style=flat-square&logo=github)
* [cairo-vm](https://github.com/lambdaclass/cairo-vm) — Implementation of the Cairo VM [![rust](https://github.com/lambdaclass/cairo-vm/actions/workflows/rust.yml/badge.svg)](https://github.com/lambdaclass/cairo-vm/actions/workflows/rust.yml)
* [ChainX](https://github.com/chainx-org/ChainX) — Fully Decentralized Interchain Crypto Asset Management on Polkadot.
* [CITA](https://github.com/citahub/cita) — A high performance blockchain kernel for enterprise users.
* [coinbase-pro-rs](https://github.com/inv2004/coinbase-pro-rs) — Coinbase pro client, supports sync/async/websocket
* [Diem](https://github.com/diem/diem) — Diem’s mission is to enable a simple global currency and financial infrastructure that empowers billions of people.
* [electrumrs](https://github.com/romanz/electrs) — An efficient re-implementation of Electrum Server.
* [ethabi](https://github.com/rust-ethereum/ethabi) - Encode and decode smart contract invocations.
* [ethaddrgen](https://github.com/Limeth/ethaddrgen) — Custom Ethereum vanity address generator
* [ethers-rs](https://github.com/gakonst/ethers-rs) - Complete Ethereum & Celo library and wallet implementation. ![Build Status](https://github.com/gakonst/ethers-rs/workflows/Tests/badge.svg)
* [etk](https://github.com/quilt/etk) - etk is a collection of tools for writing, reading, and analyzing EVM bytecode.
* [Forest](https://github.com/ChainSafe/forest) - Filecoin implementation [![Build Status](https://img.shields.io/circleci/build/gh/ChainSafe/forest/main?branch=master)](https://app.circleci.com/pipelines/github/ChainSafe/forest?branch=main)
* [Foundry](https://github.com/foundry-rs/foundry) - Foundry is a blazing fast, portable and modular toolkit for Ethereum application development. ![Build Status](https://img.shields.io/github/workflow/status/foundry-rs/foundry/test?style=flat-square)
* [Grin](https://github.com/mimblewimble/grin/) — Evolution of the MimbleWimble protocol
* [hdwallet](https://github.com/jjyr/hdwallet) [[hdwallet](https://crates.io/crates/hdwallet)] — BIP-32 HD wallet related key derivation utilities.
* [Holochain](https://github.com/holochain/holochain) — Scalable P2P alternative to blockchain for all those distributed apps you always wanted to build. [![detect critical check failures](https://github.com/holochain/holochain/actions/workflows/check_run_detect_release_pr_failure.yml/badge.svg)](https://github.com/holochain/holochain/actions/workflows/check_run_detect_release_pr_failure.yml)
* [Hyperlane](https://github.com/hyperlane-xyz/hyperlane-monorepo).
  Framework for permissionless, modular interoperability. The offchain clients are written in Rust, as well as the smart contracts for Solana VM and CosmWasm.
* [ibc-rs](https://github.com/informalsystems/hermes) - Implementation of the [Interblockchain Communication](https://ibc.cosmos.network/) protocol
* [infincia/bip39-rs](https://github.com/infincia/bip39-rs) [[bip39](https://crates.io/crates/bip39)] — Implementation of BIP39.
* [interBTC](https://github.com/interlay/interbtc) — Trustless and fully decentralized Bitcoin bridge to Polkadot and Kusama.
* [Joystream](https://github.com/Joystream/joystream) — A user governed video platform
* [Lighthouse](https://github.com/sigp/lighthouse) — Ethereum Consensus Layer (CL) Client [![Build Status](https://github.com/sigp/lighthouse/workflows/test-suite/badge.svg?branch=master)](https://github.com/sigp/lighthouse/actions)
* [madara](https://github.com/keep-starknet-strange/madara) - Kaioshin is a ⚡ blazing fast ⚡ Starknet sequencer, based on substrate. [![GitHub Workflow Status](https://github.com/keep-starknet-strange/madara/actions/workflows/test.yml/badge.svg)](https://github.com/keep-starknet-strange/madara/actions/workflows/test.yml)
* [mev-inspect-rs](https://github.com/flashbots/mev-inspect-rs) - Ethereum MEV Inspector.
* [near/nearcore](https://github.com/near/nearcore) — decentralized smart-contract platform for low-end mobile devices.
* [Nervos CKB](https://github.com/nervosnetwork/ckb) — Nervos CKB is a public permissionless blockchain, the common knowledge layer of Nervos network.
* [opensea-rs](https://github.com/gakonst/opensea-rs) - Bindings & CLI to the Opensea API and Contracts.
* [Parity-Bitcoin](https://github.com/paritytech/parity-bitcoin) — The Parity Bitcoin client
* [Phala-Network/phala-blockchain](https://github.com/Phala-Network/phala-blockchain) — Confidential smart contract blockchain based on Intel SGX and Substrate
* [polkadot-sdk](https://github.com/paritytech/polkadot-sdk) — The Parity Polkadot Blockchain SDK
* [revm](https://github.com/bluealloy/revm) - Revolutionary Machine (revm) is a fast Ethereum virtual machine.
* [rust-bitcoin](https://github.com/rust-bitcoin/rust-bitcoin) — Library with support for de/serialization, parsing and executing on data structures and network messages related to Bitcoin.
* [rust-lightning](https://github.com/lightningdevkit/rust-lightning) [![Crate](https://img.shields.io/crates/v/lightning.svg?logo=rust)](https://crates.io/crates/lightning) — Bitcoin Lightning library. The main crate,`lightning`, does not handle networking, persistence, or any other I/O. Thus,it is runtime-agnostic, but users must implement basic networking logic, chain interactions, and disk storage.po on linking crate.
* [sigma-rust](https://github.com/ergoplatform/sigma-rust) — ErgoTree interpreter and wallet-related features.
* [Solana](https://github.com/solana-labs/solana) — Incredibly fast, highly scalable blockchain using Proof-of-History.
* [Subspace](https://github.com/subspace/subspace) - The first layer-one blockchain that can fully resolve the blockchain trilemma by simultaneously achieving scalability, security, and decentralization.
* [Sui](https://github.com/MystenLabs/sui) — A next-generation smart contract platform with high throughput, low latency, and an asset-oriented programming model powered by the Move programming language.
* [svm-rs](https://github.com/alloy-rs/svm-rs) - Solidity-Compiler Version Manager.
* [tendermint-rs](https://github.com/informalsystems/tendermint-rs) - Tendermint blockchain data structures and clients
* [wagyu](https://github.com/howardwu/wagyu) [[wagyu](https://crates.io/crates/wagyu)] — Library for generating cryptocurrency wallets
* [zcash](https://github.com/zcash/zcash) — Zcash is an implementation of the "Zerocash" protocol.

### Database

* [Atomic-Server](https://github.com/atomicdata-dev/atomic-server/) [[atomic-server](https://crates.io/crates/atomic_server)] - NoSQL graph database with realtime updates, dynamic indexing and easy-to-use GUI for CMS purposes. [![Release](https://github.com/atomicdata-dev/atomic-server/actions/workflows/docker.yml/badge.svg)](https://github.com/atomicdata-dev/atomic-server/actions/workflows/docker.yml)
* [CozoDB](https://github.com/cozodb/cozo) - A transactional, relational database that uses Datalog and focuses on graph data and algorithms. Time-travel-capable, and fast! [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/cozodb/cozo/build.yml?branch=main)](https://github.com/cozodb/cozo/actions/workflows/build.yml)
* [darkbird](https://github.com/Rustixir/darkbird) [[darkbird](https://crates.io/crates/darkbird)] - HighConcurrency, RealTime, InMemory storage inspired by erlang mnesia
* [Databend](https://github.com/datafuselabs/databend) - A Modern Real-Time Data Processing & Analytics DBMS with Cloud-Native Architecture [![Release](https://github.com/datafuselabs/databend/actions/workflows/databend-release.yml/badge.svg)](https://github.com/datafuselabs/databend/actions/workflows/databend-release.yml)
* [DB3 Network](https://github.com/dbpunk-labs/db3) — DB3 is a community-driven blockchain layer2 decentralized database network ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/dbpunk-labs/db3/ci.yml?branch=main&style=flat-square)
* [erikgrinaker/toydb](https://github.com/erikgrinaker/toydb) — Distributed SQL database, written as a learning project.
* [FnckSQL](https://github.com/KipData/FnckSQL) — SQL as a Function for Rust
* [Garage](https://github.com/deuxfleurs-org/garage) [[garage](https://crates.io/crates/garage)] — S3-compatible distributed object storage service designed for self-hosting at a small-to-medium scale. [![status-badge](https://woodpecker.deuxfleurs.fr/api/badges/1/status.svg)](https://woodpecker.deuxfleurs.fr/repos/1)
* [GreptimeDB](https://github.com/grepTimeTeam/greptimedb/) - An open-source, cloud-native, distributed time-series database with PromQL/SQL/Python supported.[![CI](https://github.com/greptimeTeam/greptimedb/actions/workflows/develop.yml/badge.svg)](https://github.com/greptimeTeam/greptimedb/actions/workflows/develop.yml)
* [indradb](https://crates.io/crates/indradb) — Graph database
* [lancedb](https://github.com/lancedb/lancedb) [[vectordb](https://crates.io/crates/vectordb)] — A serverless, low-latency vector database for AI applications
* [Lucid](https://github.com/lucid-kv/lucid) — High performance and distributed KV store accessible through a HTTP API. [![Build Status](https://github.com/lucid-kv/lucid/workflows/Lucid/badge.svg?branch=master)](https://github.com/lucid-kv/lucid/actions?workflow=Lucid)
* [Materialize](https://github.com/MaterializeInc/materialize) - Streaming SQL database powered by Timely Dataflow :heavy_dollar_sign: [![Build status](https://badge.buildkite.com/97d6604e015bf633d1c2a12d166bb46f3b43a927d3952c999a.svg?branch=main)](https://buildkite.com/materialize/test)
* [native_db](https://github.com/vincent-herlemont/native_db) [[native_db](https://crates.io/crates/native_db)] - Drop-in, embedded database for multi-platform apps (server, desktop, mobile). Sync Rust types effortlessly
* [Neon](https://github.com/neondatabase/neon) Serverless Postgres. We separated storage and compute to offer autoscaling, branching, and bottomless storage.
* [noria](https://github.com/mit-pdos/noria) [[noria](https://crates.io/crates/noria)] — Dynamically changing, partially-stateful data-flow for web application backends
* [ParadeDB](https://github.com/paradedb/paradedb/) - ParadeDB is an Elasticsearch alternative built on Postgres, designed for real-time search and analytics.
* [ParityDB](https://github.com/paritytech/parity-db) — Fast and reliable database, optimised for read operation
* [PumpkinDB](https://github.com/PumpkinDB/PumpkinDB) — an event sourcing database engine
* [Qdrant](https://github.com/qdrant/qdrant) - An open source vector similarity search engine with extended filtering support [![Tests](https://github.com/qdrant/qdrant/workflows/Tests/badge.svg)](https://github.com/qdrant/qdrant/actions)
* [Qrlew/qrlew](https://github.com/Qrlew/qrlew) [[qrlew](https://crates.io/crates/qrlew)] - The SQL-to-SQL Differential Privacy layer [![Qrlew](https://github.com/Qrlew/qrlew/actions/workflows/ci.yml/badge.svg)](https://github.com/Qrlew/qrlew/actions) ![Crates.io Version](https://img.shields.io/crates/v/qrlew?logo=Rust)
* [RisingWaveLabs/RisingWave](https://github.com/RisingWaveLabs/risingwave) - the next-generation streaming database in the cloud [![CI](https://github.com/RisingWaveLabs/risingwave/actions/workflows/main.yml/badge.svg)](https://github.com/RisingWaveLabs/risingwave/actions/workflows/main.yml/badge.svg?branch=main)
* [seppo0010/rsedis](https://github.com/seppo0010/rsedis) — A Redis reimplementation.
* [Skytable](https://github.com/skytable/skytable) — A multi-model NoSQL database ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/skytable/skytable/Tests?style=flat-square)
* [sled](https://crates.io/crates/sled) — A (beta) modern embedded database [![Build Status](https://github.com/spacejam/sled/workflows/Rust/badge.svg?branch=master)](https://github.com/spacejam/sled/actions?workflow=Rust)
* [SQLSync](https://github.com/orbitinghail/sqlsync) — Multiplayer offline-first SQLite [![GitHub Workflow Status](https://github.com/orbitinghail/sqlsync/actions/workflows/actions.yaml/badge.svg?branch=main)](https://github.com/orbitinghail/sqlsync/actions?query=branch%3Amain)
* [SurrealDB](https://github.com/surrealdb/surrealdb) — A scalable, distributed, document-graph database [![Build Status](https://img.shields.io/github/workflow/status/surrealdb/surrealdb/Continuous%20integration/main)](https://github.com/surrealdb/surrealdb/actions)
* [TerminusDB](https://github.com/terminusdb/terminusdb-store) - open source graph database and document store [![Build Status](https://github.com/terminusdb/terminusdb-store/workflows/Build/badge.svg?branch=master)](https://github.com/terminusdb/terminusdb-store/actions)
* [tikv](https://github.com/tikv/tikv) — A distributed KV database in Rust [![Build Status](https://ci.pingcap.net/job/tikv_ghpr_test/badge/icon)](https://ci.pingcap.net/job/tikv_ghpr_test/)
* [USearch](https://github.com/unum-cloud/usearch) - Similarity Search Engine for Vectors and Strings [![crates.io](https://img.shields.io/crates/v/usearch.svg)](https://crates.io/crates/usearch)
* [vorot93/libmdbx-rs](https://github.com/vorot93/libmdbx-rs) [[mdbx-sys](https://crates.io/crates/mdbx-sys)] — Bindings for MDBX, a "fast, compact, powerful, embedded, transactional key-value database, with permissive license". This is a fork of mozilla/lmdb-rs with patches to make it work with libmdbx.
* [WooriDB](https://github.com/naomijub/wooridb) - General purpose time serial database inspired by Crux and Datomic.

### Emulators

See also [crates matching keyword 'emulator'](https://crates.io/keywords/emulator).

* CHIP-8
  * [ColinEberhardt/wasm-rust-chip8](https://github.com/ColinEberhardt/wasm-rust-chip8) — A WebAssembly CHIP-8 emulator.
  * [starrhorne/chip8-rust](https://github.com/starrhorne/chip8-rust) — chip8 emulator
* Commodore 64
  * [kondrak/rust64](https://github.com/kondrak/rust64) —
* Flash Player
  * [Ruffle](https://github.com/ruffle-rs/ruffle) — Ruffle is an Adobe Flash Player emulator. Ruffle targets both the desktop and the web using WebAssembly. [![CI](https://github.com/ruffle-rs/ruffle/actions/workflows/test_rust.yml/badge.svg)](https://github.com/ruffle-rs/ruffle/actions/workflows/test_rust.yml)[![CI](https://github.com/ruffle-rs/ruffle/actions/workflows/test_web.yml/badge.svg)](https://github.com/ruffle-rs/ruffle/actions/workflows/test_web.yml)
* Gameboy
  * [Gekkio/mooneye-gb](https://github.com/Gekkio/mooneye-gb) —
  * [joamag/boytacean](https://github.com/joamag/boytacean) — GameBoy Color emulator that runs on the Web using WebAssembly.
  * [mohanson/gameboy](https://github.com/mohanson/gameboy) — Full featured Cross-platform GameBoy emulator. Forever boys!.
  * [mvdnes/rboy](https://github.com/mvdnes/rboy) —
* Gameboy Advance
  * [michelhe/rustboyadvance-ng](https://github.com/michelhe/rustboyadvance-ng) - RustboyAdvance-ng is a Gameboy Advance emulator with desktop, android and [WebAssembly](https://michelhe.github.io/rustboyadvance-ng/) support. [![build badge](https://github.com/michelhe/rustboyadvance-ng/workflows/Deploy/badge.svg?branch=master)](https://github.com/michelhe/rustboyadvance-ng/actions?query=workflow%3ADeploy)
* GameMaker
  * [OpenGMK](https://github.com/OpenGMK/OpenGMK) — OpenGMK is a modern rewrite of the proprietary GameMaker Classic engines, providing a full sourceport of the runner, a decompiler, a TASing framework, and libraries for working with gamedata yourself.
* IBM PC
  * [MartyPC](https://github.com/dbalsom/martypc) — An IBM PC/XT emulator written in Rust.
* Intel 8080 CPU
  * [mohanson/i8080](https://github.com/mohanson/i8080) — Intel 8080 CPU emulator
* iOS
  * [touchHLE](https://github.com/touchHLE/touchHLE) — High-level emulator for iPhone OS apps
* iPod
  * [clicky](https://github.com/daniel5151/clicky) — A clickwheel iPod emulator (WIP)
* NES
  * [koute/pinky](https://github.com/koute/pinky) —
  * [pcwalton/sprocketnes](https://github.com/pcwalton/sprocketnes)
* Nintendo 64
  * [gopher64](https://github.com/gopher64/gopher64) — N64 emulator written in Rust
* Nintendo DS
  * [dust](https://github.com/kelpsyberry/dust) — A Nintendo DS emulator
* PlayStation 4
  * [Obliteration](https://github.com/obhq/obliteration) — Experimental PS4 emulator for Windows, macOS and Linux [![CI](https://github.com/obhq/obliteration/actions/workflows/main.yml/badge.svg)](https://github.com/obhq/obliteration/actions/workflows/main.yml)
* ZX Spectrum
  * [rustzx/rustzx](https://github.com/rustzx/rustzx) — [![RustZX CI](https://github.com/rustzx/rustzx/actions/workflows/ci.yml/badge.svg)](https://github.com/rustzx/rustzx/actions/workflows/ci.yml)

### File manager

* [broot](https://github.com/Canop/broot) - A new way to see and navigate directory trees (get an overview of a directory, even a big one; find a directory then `cd` to it; never lose track of file hierarchy while you search; manipulate your files, ...), further reading [dystroy.org/broot](https://dystroy.org/broot/) [![Latest Version](https://img.shields.io/crates/v/broot.svg)](https://crates.io/crates/broot)
* [joshuto](https://github.com/kamiyaa/joshuto) - ranger-like terminal file manager
* [xplr](https://github.com/sayanarijit/xplr) - A hackable, minimal, fast TUI file explorer
* [yazi](https://github.com/sxyazi/yazi) - Blazing fast terminal file manager, based on async I/O.

### Games

See also [Games Made With Piston](https://github.com/PistonDevelopers/piston/wiki/Games-Made-With-Piston).

* [chess-tui](https://github.com/thomas-mauran/chess-tui) — A Chess TUI implementation ♟️
* [citybound](https://github.com/citybound/citybound) — The city sim you deserve
* [cristicbz/rust-doom](https://github.com/cristicbz/rust-doom) — A renderer for Doom, may progress to being a playable game
* [doukutsu-rs](https://github.com/doukutsu-rs/doukutsu-rs) — Reimplementation of Cave Story engine with some enhancements.
* [garkimasera/rusted-ruins](https://github.com/garkimasera/rusted-ruins) — Extensible open world rogue like game with pixel art
* [gorilla-devs/ferium](https://github.com/gorilla-devs/ferium) — Ferium is a fast and feature rich CLI program for downloading and updating Minecraft mods from Modrinth, CurseForge, and GitHub Releases, and modpacks from Modrinth and CurseForge ![ferium build](https://github.com/gorilla-devs/ferium/actions/workflows/build.yml/badge.svg?branch=main)
* [lifthrasiir/angolmois-rust](https://github.com/lifthrasiir/angolmois-rust) — A minimalistic music video game which supports the BMS format
* [maras-archive/rsnake](https://github.com/maras-archive/rsnake) — Snake.
* [mcthesw/game-save-manager](https://github.com/mcthesw/game-save-manager) - A user-friendly tool for managing game saves [![build badge](https://github.com/mcthesw/game-save-manager/actions/workflows/tauri.yml/badge.svg)](https://github.com/mcthesw/game-save-manager/actions/workflows/tauri.yml)
* [mtkennerly/ludusavi](https://github.com/mtkennerly/ludusavi) — Backup tool for PC game saves [![build badge](https://img.shields.io/github/actions/workflow/status/mtkennerly/ludusavi/main.yaml?logo=github)](https://github.com/mtkennerly/ludusavi/actions/workflows/main.yaml) [![crate](https://img.shields.io/crates/v/ludusavi?logo=rust)](https://crates.io/crates/ludusavi)
* [ozkriff/zemeroth](https://github.com/ozkriff/zemeroth) — A small 2D turn-based hexagonal strategy game
* [rhex](https://github.com/dpc/rhex) — hexagonal ascii roguelike
* [rsaarelm/magog](https://github.com/rsaarelm/magog) — A roguelike game.
* [SoftbearStudios/mk48](https://github.com/SoftbearStudios/mk48) — Mk48.io is an online multiplayer naval combat game
* [swatteau/sokoban-rs](https://github.com/swatteau/sokoban-rs) — A Sokoban implementation
* [thetawavegame/thetawave-legacy](https://github.com/thetawavegame/thetawave-legacy) - A space shooter game that strives to be an entry point for new game developers to make their first contributions. ![build badge](https://github.com/thetawavegame/thetawave-legacy/actions/workflows/ci.yml/badge.svg?branch=master)
* [Thinkofname/rust-quake](https://github.com/Thinkofname/rust-quake) — Quake map renderer.
* [ttyperacer/terminal-typeracer](https://gitlab.com/ttyperacer/terminal-typeracer) - Single player typing test game written for the terminal
* [Veloren](https://gitlab.com/veloren/veloren) — An open world, open source multiplayer voxel RPG game currently in alpha development [![build badge](https://gitlab.com/veloren/veloren/badges/master/pipeline.svg)](https://gitlab.com/veloren/veloren/-/pipelines)
* [Zone of Control](https://github.com/ozkriff/zoc) — A turn-based hexagonal strategy game

### Graphics

* [dps/rust-raytracer](https://github.com/dps/rust-raytracer) - An implementation of a very simple raytracer based on Ray Tracing in One Weekend by Peter Shirley.
* [flxzt/rnote](https://github.com/flxzt/rnote) - Sketch and take handwritten notes.
* [ivanceras/svgbob](https://github.com/ivanceras/svgbob) — converts ASCII diagrams into SVG graphics
* [KaminariOS/rustracer](https://github.com/KaminariOS/rustracer) — A PBR glTF 2.0 renderer based on Vulkan ray-tracing.
* [Limeth/euclider](https://github.com/Limeth/euclider) — A real-time 4D CPU ray tracer
* [RazrFalcon/resvg](https://github.com/RazrFalcon/resvg) — An SVG rendering library.
* [rodrigorc/papercraft](https://github.com/rodrigorc/papercraft) - A tool to unwrap 3D models and create them in paper with scissors and glue.
* [rustq/vue-skia](https://github.com/rustq/vue-skia) — Skia based 2d graphics vue rendering library. It is based on Rust to implement software rasterization to perform rendering.
* [turnage/valora](https://crates.io/crates/valora) — A library for generative fine art ![Rust](https://github.com/turnage/valora/workflows/Rust/badge.svg?branch=master)
* [Twinklebear/tray_rust](https://github.com/Twinklebear/tray_rust) — A ray tracer
* [wahn/rs_pbrt](https://github.com/wahn/rs_pbrt) — Implements a counterpart to the PBRT book's (3rd edition) C++ code.

### Image processing

* [Imager](https://github.com/imager-io/imager) — Automated image optimization.
* [shssoichiro/oxipng](https://github.com/shssoichiro/oxipng) [[oxipng](https://crates.io/crates/oxipng)] — Multithreaded PNG optimizer written in Rust. [![Build Status](https://github.com/shssoichiro/oxipng/workflows/oxipng/badge.svg)](https://github.com/shssoichiro/oxipng/actions?query=branch%3Amaster) [![Version](https://img.shields.io/crates/v/oxipng.svg)](https://crates.io/crates/oxipng)

### Industrial automation

* [locka99/opcua](https://github.com/locka99/opcua) — A [OPC UA](https://opcfoundation.org/about/opc-technologies/opc-ua/) library.
* [slowtec/tokio-modbus](https://github.com/slowtec/tokio-modbus) — A [tokio](https://tokio.rs)-based [modbus](https://modbus.org) library.

### Observability

* [avito-tech/bioyino](https://github.com/avito-tech/bioyino) — A high-performance scalable StatsD compatible server.
* [openobserve](https://github.com/openobserve/openobserve) - 10x easier, 140x lower storage cost, high performance, petabyte scale - Elasticsearch/Splunk/Datadog alternative.
* [OpenTelemetry](https://crates.io/crates/opentelemetry) — OpenTelemetry provides a single set of APIs, libraries, agents, and collector services to capture distributed traces and metrics from your application. You can analyze them using Prometheus, Jaeger, and other observability tools. [![GitHub Actions CI](https://github.com/open-telemetry/opentelemetry-rust/workflows/CI/badge.svg?branch=master)](https://github.com/open-telemetry/opentelemetry-rust/actions?query=workflow%3ACI+branch%3Amaster)
* [Quickwit-oss/quickwit](https://github.com/quickwit-oss/quickwit) - Cloud-native and highly cost-efficient search engine for log management. [![CI](https://github.com/quickwit-oss/quickwit/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/quickwit-oss/quickwit/actions?query=workflow%3ACI)
* [Scaphandre](https://github.com/hubblo-org/scaphandre) - A power consumption monitoring agent, to track host and each service power consumption and enable designing systems and applications for more sustainability. Designed to fit any monitoring toolchain (already supports prometheus, warp10, riemann...).
* [vectordotdev/vector](https://github.com/vectordotdev/vector) — A High-Performance, Logs, Metrics, & Events Router.

### Operating systems

See also [A comparison of operating systems written in Rust](https://github.com/flosse/rust-os-comparison).

* [0x59616e/SteinsOS](https://github.com/0x59616e/SteinsOS) — An OS for armv8-a architecture.
* [Andy-Python-Programmer/aero](https://github.com/Andy-Python-Programmer/aero) — A modern, unix-like operating system following the monolithic kernel design.
* [DragonOS-Community/DragonOS](https://github.com/DragonOS-Community/DragonOS) — An operating system with a self-developed kernel from scratch and Linux compatibility.
* [redox-os/redox](https://gitlab.redox-os.org/redox-os/redox) —
* [thepowersgang/rust_os](https://github.com/thepowersgang/rust_os) —
* [theseus-os/Theseus](https://github.com/theseus-os/Theseus) — A safe-language, single address space and single privilege level OS written from scratch - [![build badge](https://img.shields.io/github/workflow/status/theseus-os/Theseus/Documentation?label=docs%20build)](https://www.theseus-os.com/Theseus/book/index.html)
* [tock/tock](https://github.com/tock/tock) — A secure embedded operating system for Cortex-M based microcontrollers

### Package Managers

* [helsing-ai/buffrs](https://github.com/helsing-ai/buffrs) [[buffrs](https://crates.io/crates/buffrs)] — A modern package manager for protocol buffers and gRPC architectures.

### Payments

* [hyperswitch](https://github.com/juspay/hyperswitch) — An open source payments orchestrator that lets you connect with multiple payment processors and route payment traffic effortlessly, all with a single API integration ![GitHub last commit](https://img.shields.io/github/last-commit/juspay/hyperswitch?style=flat-square)

### Productivity

* [ast-grep](https://github.com/ast-grep/ast-grep) - A CLI tool for code structural search, lint and rewriting.
* [Bartib](https://github.com/nikolassv/bartib) [[Bartib](https://crates.io/crates/bartib)] - A simple timetracker for the command line [![Tests](https://github.com/nikolassv/bartib/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/nikolassv/bartib/actions/workflows/test.yml)
* [espanso](https://github.com/espanso/espanso) — A cross-platform Text Expander. [![CI](https://github.com/espanso/espanso/actions/workflows/ci.yml/badge.svg?branch=dev&event=push)](https://github.com/espanso/espanso/actions/workflows/ci.yml)
* [eureka](https://crates.io/crates/eureka) — A CLI tool to input and store your ideas without leaving the terminal
* [Furtherance](https://github.com/lakoliu/Furtherance) - Time tracking app built with GTK4
* [illacloud/illa](https://github.com/illacloud/illa) [[ILLA Cloud](https://illacloud.com)] - Low-code internal tool builder.
* [LLDAP](https://github.com/lldap/lldap) - Simplified LDAP interface for authentication.
* [pier-cli/pier](https://github.com/pier-cli/pier) — A central repository to manage (add, search metadata, etc.) all your one-liners, scripts, tools, and CLIs
* [ShadoySV/work-break](https://github.com/ShadoySV/work-break) [[work-break](https://crates.io/crates/work-break)] - Work and rest time balancer taking into account your current and today strain [![Build](https://github.com/shadoysv/work-break/actions/workflows/release.yml/badge.svg)](https://github.com/ShadoySV/work-break/releases)
* [yashs662/rust_kanban](https://github.com/yashs662/rust_kanban) [[rust-kanban](https://crates.io/crates/rust-kanban)] [![Build](https://github.com/yashs662/rust_kanban/actions/workflows/build.yml/badge.svg)](https://github.com/yashs662/rust_kanban/releases) — A Kanban App for the terminal

### Routing protocols

* [Holo](https://github.com/holo-routing/holo) - Holo is a suite of routing protocols designed to support high-scale and automation-driven networks
* [RustyBGP](https://github.com/osrg/rustybgp) - BGP

### Security tools

* [AFLplusplus/LibAFL](https://github.com/AFLplusplus/LibAFL) - Advanced Fuzzing Library - Slot your Fuzzer together in Rust! Scales across cores and machines. For Windows, Android, MacOS, Linux, no_std, etc. [![build and test](https://github.com/AFLplusplus/LibAFL/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/AFLplusplus/LibAFL/actions/workflows/build_and_test.yml)
* [arp-scan-rs](https://github.com/kongbytes/arp-scan-rs) - A minimalistic ARP scan tool for fast local network scans
* [cargo-audit](https://crates.io/crates/cargo-audit) - Audit Cargo.lock for crates with security vulnerabilities
* [cargo-auditable](https://crates.io/crates/cargo-auditable) - Make production Rust binaries auditable
* [cargo-crev](https://crates.io/crates/cargo-crev) - A cryptographically verifiable code review system for the cargo package manager.
* [cargo-deny](https://crates.io/crates/cargo-deny) - Cargo plugin to help you manage large dependency graphs
* [Cherrybomb](https://github.com/blst-security/cherrybomb) - Stop half-done API specifications with a CLI tool that helps you avoid undefined user behaviour by validating your API specifications.
* [cotp](https://github.com/replydev/cotp) - Trustworthy, encrypted, command-line TOTP/HOTP authenticator app with import functionality.
* [entropic-security/xgadget](https://github.com/entropic-security/xgadget) [[xgadget](https://crates.io/crates/xgadget)] — Fast, parallel, cross-variant ROP/JOP gadget search [![GitHub Actions](https://github.com/entropic-security/xgadget/workflows/test/badge.svg)](https://github.com/entropic-security/xgadget/actions)
* [epi052/feroxbuster](https://github.com/epi052/feroxbuster) - A simple, fast, recursive content discovery tool.
* [Inspektor](https://github.com/inspektor-dev/inspektor) - A database protocol-aware proxy that is used to enforce access policies 👮
* [kpcyrd/authoscope](https://github.com/kpcyrd/authoscope) — A scriptable network authentication cracker
* [kpcyrd/rshijack](https://github.com/kpcyrd/rshijack) — A TCP connection hijacker; rewrite of shijack
* [kpcyrd/sn0int](https://github.com/kpcyrd/sn0int) — A semi-automatic OSINT framework and package manager
* [kpcyrd/sniffglue](https://github.com/kpcyrd/sniffglue) — A secure multithreaded packet sniffer
* [ObserverWard](https://github.com/0x727/ObserverWard) — Community based web technologies analysis tool.
* [Raspirus](https://github.com/Raspirus/Raspirus) - User- and resources-friendly signatures-based malware scanner [![status](https://github.com/Raspirus/Raspirus/actions/workflows/testproject.yml/badge.svg)](https://github.com/Raspirus/Raspirus/actions/workflows/testproject.yml)
* [ripasso](https://github.com/cortex/ripasso/) — A password manager, filesystem compatible with pass
* [rustscan/rustscan](https://github.com/RustScan/RustScan) — Make Nmap faster with this port scanning tool [![build badge](https://github.com/RustScan/RustScan/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/RustScan/RustScan/actions?query=workflow%3A%22Continuous+integration%22)

### Social networks

* Mastodon
  * [Rustodon](https://github.com/rustodon/rustodon) - A Mastodon-compatible, ActivityPub-speaking server.

### System tools

* [ajeetdsouza/zoxide](https://github.com/ajeetdsouza/zoxide/) — A fast alternative to `cd` that learns your habits [![release](https://github.com/ajeetdsouza/zoxide/workflows/.github/workflows/release.yml/badge.svg)](https://github.com/ajeetdsouza/zoxide/actions)
* [atuin](https://github.com/atuinsh/atuin) [[atuin](https://crates.io/crates/atuin)] — Atuin replaces your existing shell history with a SQLite database, and records additional context for your commands. Additionally, it provides optional and fully encrypted synchronisation of your history between machines, via an Atuin server.
* [bandwhich](https://github.com/imsnif/bandwhich) — Terminal bandwidth utilization tool
* [bottom](https://github.com/ClementTsang/bottom) - Yet another cross-platform graphical process/system monitor. [![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/ClementTsang/bottom/ci/master)](https://github.com/ClementTsang/bottom/actions?query=branch%3Amaster)
* [brocode/fblog](https://github.com/brocode/fblog) — Small command-line JSON Log viewer
* [bustd](https://github.com/vrmiguel/bustd) - Lightweight process killer daemon to handle out-of-memory scenarios on Linux. [![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/vrmiguel/bustd/build-and-test)](https://github.com/vrmiguel/bustd/actions?query=branch%3Amaster)
* [buster/rrun](https://github.com/buster/rrun) — A command launcher for Linux, similar to gmrun
* [cantino/mcfly](https://github.com/cantino/mcfly) - Fly through your shell history. Great Scott!
* [crabz](https://github.com/sstadick/crabz) - Multi-threaded compression and decompression CLI tool [![Build Status](https://github.com/sstadick/crabz/workflows/Check/badge.svg)](https://github.com/sstadick/crabz/actions?query=workflow%3ACheck)
* [cristianoliveira/funzzy](https://github.com/cristianoliveira/funzzy) — A configurable filesystem watcher inspired by [entr](http://eradman.com/entrproject/)
* [dalance/procs](https://github.com/dalance/procs) — A modern replacement for 'ps' [![Regression](https://github.com/dalance/procs/actions/workflows/regression.yml/badge.svg)](https://github.com/dalance/procs/actions/workflows/regression.yml)
* [ddh](https://github.com/darakian/ddh) — Fast duplicate file finder
* [diskonaut](https://github.com/imsnif/diskonaut) — Terminal visual disk space navigator
* [dust](https://github.com/bootandy/dust) — A more intuitive version of du
* [eza-community/eza](https://github.com/eza-community/eza) — A replacement for 'ls'
* [fselect](https://crates.io/crates/fselect) — Find files with SQL-like queries
* [gitui](https://github.com/extrawurst/gitui) - Blazing fast terminal client for git. [![build](https://github.com/extrawurst/gitui/workflows/CI/badge.svg?branch=master)](https://github.com/extrawurst/gitui/actions)
* [GQL](https://github.com/amrdeveloper/gql) — A SQL like query language to run on .git files.
* [httm](https://github.com/kimono-koans/httm) - Interactive, file-level Time Machine-like tool for ZFS/btrfs/nilfs2 (and even actual Time Machine backups!)
* [j0ru/kickoff](https://github.com/j0ru/kickoff) - Fast and snappy wayland program launcher [![build](https://github.com/j0ru/kickoff/actions/workflows/ci.yml/badge.svg)](https://github.com/j0ru/kickoff/actions)
* [Kondo](https://github.com/tbillington/kondo) - CLI & GUI tool for deleting software project artifacts and reclaiming disk space
* [LACT](https://github.com/ilya-zlobintsev/LACT) - Linux AMDGPU Controller
* [lodosgroup/lpm](https://github.com/lodosgroup/lpm) — An experimental system package manager
* [lotabout/rargs](https://github.com/lotabout/rargs) [[rargs](https://crates.io/crates/rargs)] — xargs + awk with pattern matching support
* [lotabout/skim](https://github.com/lotabout/skim) — A fuzzy finder
* [lsd](https://github.com/lsd-rs/lsd) — An ls with a lot of pretty colors and awesome icons [![build](https://github.com/lsd-rs/lsd/workflows/CICD/badge.svg?branch=master)](https://github.com/lsd-rs/lsd/actions)
* [Luminarys/synapse](https://github.com/Luminarys/synapse) — Flexible and fast BitTorrent daemon.
* [m4b/bingrep](https://github.com/m4b/bingrep) — Greps through binaries from various OSs and architectures, and colors them.
* [mdgaziur/findex](https://github.com/mdgaziur/findex) - Findex is a highly customizable application finder using GTK3
* [mitnk/cicada](https://github.com/mitnk/cicada) — A bash-like Unix shell
* [mmstick/concurr](https://github.com/mmstick/concurr) — Alternative to GNU Parallel w/ a client-server architecture
* [mmstick/fontfinder](https://github.com/mmstick/fontfinder) — GTK3 application for previewing and installing Google's fonts
* [mmstick/tv-renamer](https://github.com/mmstick/tv-renamer) — A tv series renaming application with an optional GTK3 frontend.
* [mxseev/logram](https://github.com/mxseev/logram) — Push log files' updates to Telegram
* [netscanner](https://github.com/Chleba/netscanner) - TUI Network Scanner
* [nickgerace/gfold](https://github.com/nickgerace/gfold) [[gfold](https://crates.io/crates/gfold)] - CLI tool to help keep track of multiple Git repositories [![build](https://img.shields.io/github/workflow/status/nickgerace/gfold/merge/main)](https://github.com/nickgerace/gfold/actions?query=workflow%3Amerge+branch%3Amain)
* [nivekuil/rip](https://github.com/nivekuil/rip) - A safe and ergonomic alternative to `rm`
* [nushell/nushell](https://github.com/nushell/nushell) - A new type of shell
* [orhun/kmon](https://github.com/orhun/kmon) — Linux Kernel Manager and Activity Monitor ![https://github.com/orhun/kmon/actions](https://img.shields.io/github/actions/workflow/status/orhun/kmon/ci.yml?branch=master&label=build)
* [orhun/systeroid](https://github.com/orhun/systeroid) — A more powerful alternative to sysctl(8) with a terminal user interface ![https://github.com/orhun/systeroid/actions](https://img.shields.io/github/actions/workflow/status/orhun/systeroid/ci.yml?branch=main&label=build)
* [ouch](https://github.com/ouch-org/ouch) - Painless compression and decompression on the command-line [![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/ouch-org/ouch/build-and-test)](https://github.com/ouch-org/ouch/actions?query=branch%3Amaster)
* [pkolaczk/fclones](https://github.com/pkolaczk/fclones) — Efficient duplicate file finder and remover
* [pop-os/popsicle](https://github.com/pop-os/popsicle) — GTK3 & CLI utility for flashing multiple USB devices in parallel
* [pop-os/system76-power](https://github.com/pop-os/system76-power/) — Linux power management daemon (DBus-interface) with CLI tool.
* [pueue](https://github.com/nukesor/pueue) — Manage your long running shell commands. [![GitHub Actions Workflow](https://github.com/nukesor/pueue/workflows/Test%20build/badge.svg?branch=master)](https://github.com/nukesor/pueue/actions)
* [qarmin/czkawka](https://github.com/qarmin/czkawka) - Multi-functional app to find duplicates, empty folders, similar images, etc. [![GitHub Actions Workflow](https://github.com/qarmin/czkawka/actions/workflows/pages/pages-build-deployment/badge.svg?branch=master)](https://github.com/qarmin/czkawka/actions)
* [redox-os/ion](https://github.com/redox-os/ion) — Next-generation system shell
* [sharkdp/bat](https://github.com/sharkdp/bat) — A cat(1) clone with wings. [![CICD](https://github.com/sharkdp/bat/actions/workflows/CICD.yml/badge.svg?branch=master)](https://github.com/sharkdp/bat/actions/workflows/CICD.yml)
* [sharkdp/fd](https://github.com/sharkdp/fd) — A simple, fast and user-friendly alternative to find. [![CICD](https://github.com/sharkdp/fd/actions/workflows/CICD.yml/badge.svg)](https://github.com/sharkdp/fd/actions/workflows/CICD.yml)
* [sitkevij/hex](https://github.com/sitkevij/hex) — A colorized hexdump terminal utility.
* [supercilex/fuc](https://github.com/supercilex/fuc) - Fast `cp` and `rm` commands
* [trippy](https://github.com/fujiapple852/trippy) - A network diagnostic tool [![build badge](https://github.com/fujiapple852/trippy/workflows/CI/badge.svg)](https://github.com/fujiapple852/trippy/actions/workflows/ci.yml)
* [uutils/coreutils](https://github.com/uutils/coreutils) — A cross-platform rewrite of the GNU coreutils [![CICD](https://github.com/uutils/coreutils/actions/workflows/CICD.yml/badge.svg)](https://github.com/uutils/coreutils/actions/workflows/CICD.yml)
* [watchexec](https://github.com/watchexec/watchexec) — Executes commands in response to file modifications
* [XAMPPRocky/tokei](https://github.com/XAMPPRocky/tokei) — counts the lines of code
* [ynqa/jnv](https://github.com/ynqa/jnv) — Interactive JSON filter using jq [![ci](https://github.com/ynqa/jnv/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ynqa/jnv/actions/workflows/ci.yml)
* [ynqa/sig](https://github.com/ynqa/sig) — Interactive grep (for streaming) [![ci](https://github.com/ynqa/sig/actions/workflows/ci.yml/badge.svg)](https://github.com/ynqa/sig/actions/workflows/ci.yml)

### Task scheduling

* [delicate](https://github.com/BinChengZhao/delicate) — A lightweight and distributed task scheduling platform. [![Build Status](https://github.com/BinChengZhao/delicate/workflows/CI/badge.svg)](https://github.com/BinChengZhao/delicate/actions)
* [tasklet](https://github.com/stav121/tasklet) [[tasklet](https://crates.io/crates/tasklet)] - A task scheduling library written in Rust ![Build Status](https://img.shields.io/github/actions/workflow/status/stav121/tasklet/rust.yml)

### Text editors

* [amp](https://amp.rs) — Inspired by Vi/Vim.
* [emacs-ng](https://github.com/emacs-ng/emacs-ng) — Complementing the C codebase with rust code to introduce new features.
* [gchp/iota](https://github.com/gchp/iota) — A simple text editor
* [helix](https://github.com/helix-editor/helix) — A post-modern modal text editor inspired by Neovim/Kakoune. [![build badge](https://github.com/helix-editor/helix/actions/workflows/build.yml/badge.svg)](https://github.com/helix-editor/helix/actions)
* [ilai-deutel/kibi](https://github.com/ilai-deutel/kibi) — A tiny (≤1024 LOC) text editor with syntax highlighting, incremental search and more. [![build badge](https://github.com/ilai-deutel/kibi/workflows/CI/badge.svg?branch=master)](https://github.com/ilai-deutel/kibi/actions?query=branch%3Amaster)
* [Lapce](https://github.com/lapce/lapce) — A modern editor with a backend. Taking inspiration from the discontinued [xi-editor](https://github.com/xi-editor/xi-editor).
* [mathall/rim](https://github.com/mathall/rim) — Vim-like text editor.
* [ox](https://github.com/curlpipe/ox) — An independent Rust text editor that runs in your terminal!
* [vamolessa/pepper](https://git.sr.ht/~lessa/pepper) [[pepper](https://crates.io/crates/pepper)] — An opinionated modal editor to simplify code editing from the terminal
* [zed](https://github.com/zed-industries/zed) — A high-performance, multiplayer code editor from the creators of Atom and Tree-sitter.

### Text processing

* [ashvardanian/stringzilla](https://github.com/ashvardanian/StringZilla) - SIMD-accelerated string search, sort, edit distances, alignments, and generators for x86 AVX2 & AVX-512, and Arm NEON [![crates.io](https://img.shields.io/crates/v/stringzilla.svg)](https://crates.io/crates/stringzilla)
* [dominikwilkowski/cfonts](https://github.com/dominikwilkowski/cfonts) [[cfonts](https://crates.io/crates/cfonts)] — Sexy ANSI fonts for the console ![build badge](https://github.com/dominikwilkowski/cfonts/actions/workflows/testing.yml/badge.svg)
* [grex](https://github.com/pemistahl/grex) — A command-line tool and library for generating regular expressions from user-provided test cases
* [jqnatividad/qsv](https://github.com/jqnatividad/qsv) [[qsv](https://crates.io/crates/qsv)] — A high performance CSV data-wrangling toolkit. Forked from xsv, with 34+ additional commands & more. [![Linux build status](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml) [![Windows build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml) [![macOS build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml)
* [Lisprez/so_stupid_search](https://github.com/Lisprez/so_stupid_search) — A simple and fast string search tool for human beings
* [Melody](https://github.com/yoav-lavi/melody) - A language that compiles to regular expressions and aims to be more easily readable and maintainable [![build badge](https://github.com/yoav-lavi/melody/actions/workflows/rust.yml/badge.svg)](https://github.com/yoav-lavi/melody/actions/workflows/rust.yml) [![crates.io](https://img.shields.io/crates/v/melody_compiler?label=compiler)](https://crates.io/crates/melody_compiler)
* [phiresky/ripgrep-all](https://github.com/phiresky/ripgrep-all) — ripgrep, but also search in PDFs, E-Books, Office documents, zip, tar.gz, etc.
* [replicadse/complate](https://github.com/replicadse/complate) — An in-terminal text templating tool designed for standardizing messages (like for GIT commits). [![crates.io](https://img.shields.io/crates/v/complate.svg)](https://crates.io/crates/complate) [![crates.io](https://img.shields.io/crates/d/complate?label=crates.io%20downloads)](https://crates.io/crates/complate) [![build badge](https://github.com/replicadse/complate/workflows/pipeline/badge.svg?branch=master)](https://github.com/replicadse/complate/actions)
* [ripgrep](https://crates.io/crates/ripgrep) — combines the usability of The Silver Searcher with the raw speed of grep
* [ruplacer](https://github.com/your-tools/ruplacer) — Find and replace text in source files [![Run tests](https://github.com/your-tools/ruplacer/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/your-tools/ruplacer/actions/workflows/test.yml)
* [sd](https://crates.io/crates/sd) — Intuitive find & replace CLI
* [sstadick/hck](https://github.com/sstadick/hck) - A faster and more featureful drop in replacement for `cut` [![build badge](https://github.com/sstadick/hck/workflows/Check/badge.svg?branch=master)](https://github.com/sstadick/hck)
* [vishaltelangre/ff](https://github.com/vishaltelangre/ff) — Find files (ff) by name!
* [whitfin/bytelines](https://github.com/whitfin/bytelines) [[bytelines](https://crates.io/crates/bytelines)] — Read input lines as byte slices for high efficiency.
* [whitfin/runiq](https://github.com/whitfin/runiq) — an efficient way to filter duplicate lines from unsorted input.
* [xsv](https://crates.io/crates/xsv) — A fast CSV command line tool (slicing, indexing, selecting, searching, sampling, etc.)

### Utilities

* [1History](https://github.com/1History/1History) — Command line interface to backup Firefox/Chrome/Safari history to one SQLite file [![Build Status](https://github.com/1History/1History/actions/workflows/CI.yml/badge.svg)](https://github.com/1History/1History/actions/workflows/CI.yml)
* [brycx/checkpwn](https://github.com/brycx/checkpwn) — A Have I Been Pwned (HIBP) command-line utility tool that lets you easily check for compromised accounts and passwords.
* [Epic Asset Manager](https://github.com/AchetaGames/Epic-Asset-Manager) — An unofficial client to install Unreal Engine, download and manage purchased assets, projects, plugins and games from the Epic Games Store.
* [evansmurithi/cloak](https://github.com/evansmurithi/cloak) — A Command Line OTP (One Time Password) Authenticator application.
![CI](https://github.com/evansmurithi/cloak/workflows/CI/badge.svg) [![build badge](https://ci.appveyor.com/api/projects/status/9mlfpfru3ng4c689/branch/master?svg=true)](https://ci.appveyor.com/project/evansmurithi/cloak)
* [fcsonline/tmux-thumbs](https://github.com/fcsonline/tmux-thumbs) — A lightning fast version of tmux-fingers, copy/pasting tmux like vimium/vimperator.
* [guoxbin/dtool](https://github.com/guoxbin/dtool) — A useful command-line tool collection to assist development including conversion, codec, hashing, encryption, etc.
* [Mobslide](https://github.com/thewh1teagle/mobslide) — Desktop application that turns your smartphone into presentation remote controller.
* [mprocs](https://github.com/pvolok/mprocs) — TUI for running multiple processes
* [mrjackwills/oxker](https://github.com/mrjackwills/oxker) [[oxker](https://crates.io/crates/oxker)] - A simple tui to view & control docker containers.
* [nix-community/nix-init](https://github.com/nix-community/nix-init) — Generate Nix packages from URLs with hash prefetching, dependency inference, license detection, and more [![build-badge](https://github.com/nix-community/nix-init/actions/workflows/ci.yml/badge.svg)](https://github.com/nix-community/nix-init/actions/workflows/ci.yml)
* [nix-community/nix-melt](https://github.com/nix-community/nix-melt) — A ranger-like flake.lock viewer [![build-badge](https://github.com/nix-community/nix-melt/actions/workflows/ci.yml/badge.svg)](https://github.com/nix-community/nix-melt/actions/workflows/ci.yml)
* [nix-community/nurl](https://github.com/nix-community/nurl) [[nurl](https://crates.io/crates/nurl)] — Generate Nix fetcher calls from repository URLs [![build-badge](https://github.com/nix-community/nurl/actions/workflows/ci.yml/badge.svg)](https://github.com/nix-community/nurl/actions/workflows/ci.yml)
* [nomino](https://github.com/yaa110/nomino) — Batch rename utility for developers
* [raftario/licensor](https://github.com/raftario/licensor) — write licenses to stdout [![GitHub Actions](https://github.com/raftario/licensor/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/raftario/licensor/actions/workflows/build.yml)
* [rust-parallel](https://github.com/aaronriekenberg/rust-parallel) - Fast command line app using Tokio to execute commands in parallel.  Similar interface to GNU Parallel or xargs. [![Crate](https://img.shields.io/crates/v/rust-parallel.svg?logo=rust)](https://crates.io/crates/rust-parallel) [![Build Status](https://github.com/aaronriekenberg/rust-parallel/actions/workflows/CI.yml/badge.svg)](https://github.com/aaronriekenberg/rust-parallel/actions/workflows/CI.yml)
* [rustdesk/rustdesk](https://github.com/rustdesk/rustdesk) — A remote desktop software, great alternative to TeamViewer and AnyDesk.
* [rustic-rs/rustic](https://github.com/rustic-rs/rustic) [[rustic-rs](https://crates.io/crates/rustic-rs)] — Fast, encrypted, deduplicated backups powered by Rust. [![Version](https://img.shields.io/crates/v/rustic-rs.svg)](https://crates.io/crates/rustic-rs)
* [sorairolake/qrtool](https://github.com/sorairolake/qrtool) [[qrtool](https://crates.io/crates/qrtool)] — A utility for encoding and decoding QR code images. [![CI](https://github.com/sorairolake/qrtool/workflows/CI/badge.svg?branch=develop)](https://github.com/sorairolake/qrtool/actions?query=workflow%3ACI)
* [str4d/rage](https://github.com/str4d/rage) [[rage](https://crates.io/crates/rage)] — Rust implementation of [age](https://github.com/FiloSottile/age).
* [suckit](https://github.com/Skallwar/suckit) - Recursively visit and download a website's content to your disk. [![Crate](https://img.shields.io/crates/v/suckit.svg?logo=rust)](https://crates.io/crates/suckit) [![Build Status](https://github.com/Skallwar/suckit/workflows/Build%20and%20test/badge.svg)](https://github.com/Skallwar/suckit/blob/master/.github/workflows/build_and_test.yml)
* [tversteeg/emplace](https://github.com/tversteeg/emplace) — Synchronize installed packages on multiple machines
* [vamolessa/verco](https://github.com/vamolessa/verco) [[verco](https://crates.io/crates/verco)] — A simple Git/Hg tui client focused on keyboard shortcuts
* [vaultwarden](https://github.com/dani-garcia/vaultwarden#readme) [![Build](https://github.com/dani-garcia/vaultwarden/actions/workflows/build.yml/badge.svg)](https://github.com/dani-garcia/vaultwarden/actions/workflows/build.yml) — Alternative implementation of the Bitwarden server API written in Rust
* [Vibe](https://github.com/thewh1teagle/vibe) — Transcribe audio or video in every language on every platform.
* [warpdotdev/Warp](https://github.com/warpdotdev/Warp) :heavy_dollar_sign: — Warp is a blazingly-fast modern GPU-accelerated terminal built to make you and your team more productive.
* [wrestic](https://github.com/alvaro17f/wrestic) —  👽 A wrapper around restic.

### Video

* [dertuxmalwieder/yaydl](https://github.com/dertuxmalwieder/yaydl) [[yaydl](https://crates.io/crates/yaydl)] - A simple video downloader
* [gyroflow/gyroflow](https://github.com/gyroflow/gyroflow) - Video stabilization application using gyroscope data
* [harlanc/xiu](https://github.com/harlanc/xiu) — A powerful and secure live server (rtmp/httpflv/hls/relay). [![crates.io](https://img.shields.io/crates/v/xiu.svg)](https://crates.io/crates/xiu)
* [vidmerger](https://github.com/TGotwig/vidmerger) — 📼 Merge video & audio files via CLI
* [xiph/rav1e](https://github.com/xiph/rav1e) — The fastest and safest AV1 encoder.

### Virtualization

* [containers/youki](https://github.com/containers/youki) — A container runtime [![build badge](https://github.com/containers/youki/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/containers/youki/actions)
* [firecracker-microvm/firecracker](https://github.com/firecracker-microvm/firecracker) — A lightweight virtual machine for container workload [Firecracker Microvm](https://firecracker-microvm.github.io/)
* [kata-containers/kata-containers](https://github.com/kata-containers/kata-containers) - A implementation of lightweight Virtual Machines (VMs) that feel and perform like containers, but provide the workload isolation and security advantages of VMs.
* [tailhook/vagga](https://github.com/tailhook/vagga) — A containerization tool without daemons

### Web

* [cfal/tobaru](https://github.com/cfal/tobaru) - Port forwarder with allowlists, IP and TLS SNI/ALPN rule-based routing, iptables support, round-robin forwarding (load balancing), and hot reloading.
* [LemmyNet/lemmy](https://github.com/LemmyNet/lemmy) — A link aggregator / reddit clone for the fediverse [![Build Status](https://cloud.drone.io/api/badges/LemmyNet/lemmy/status.svg)](https://cloud.drone.io/LemmyNet/lemmy)
* [libreddit](https://github.com/libreddit/libreddit) - An alternative private front-end to Reddit
* [MASQ-Project/Node](https://github.com/MASQ-Project/Node) — MASQ Node software provides a decentralized mesh-network of nodes for global users to access normal internet content - next evolution of tech beyond Tor & VPN [![build badge](https://github.com/MASQ-Project/Node/actions/workflows/ci-matrix.yml/badge.svg)](https://github.com/MASQ-Project/Node/actions)
* [Plume-org/Plume](https://github.com/Plume-org/Plume) — ActivityPub federating blogging application
* [Revolt/backend](https://github.com/revoltchat/backend) - User-first chat platform built with modern web technologies.

### Web Servers

* [cloudflare/pingora](https://github.com/cloudflare/pingora) - A library for building fast, reliable and evolvable network services.
* [emanuele-em/proxelar](https://github.com/emanuele-em/proxelar) — A MITM Proxy 🦀! Toolkit for HTTP/1, HTTP/2, and WebSockets with SSL/TLS Capabilities [![Rust](https://github.com/emanuele-em/proxelar/actions/workflows/rust.yml/badge.svg)](https://github.com/emanuele-em/proxelar/actions/workflows/rust.yml)
* [mu-arch/skyfolder](https://github.com/mu-arch/skyfolder) - 🪂 Beautiful HTTP/Bittorrent server without the hassle. Secure - GUI - Pretty - Fast
* [mufeedvh/binserve](https://github.com/mufeedvh/binserve) — A blazingly fast static web server with routing, templating, and security in a single binary you can set up with zero code [![build badge](https://github.com/mufeedvh/binserve/workflows/CICD/badge.svg?branch=master)](https://github.com/mufeedvh/binserve/actions)
* [orhun/rustypaste](https://github.com/orhun/rustypaste) — A minimal file upload/pastebin service ![https://github.com/orhun/rustypaste/actions](https://img.shields.io/github/actions/workflow/status/orhun/rustypaste/ci.yml?branch=master&label=build)
* [ronanyeah/rust-hasura](https://github.com/ronanyeah/rust-hasura) — A demonstration of how a GraphQL server can be used as a remote schema with [Hasura](https://hasura.io/) ![Rust](https://github.com/ronanyeah/rust-hasura/workflows/Rust/badge.svg?branch=master)
* [static-web-server](https://github.com/static-web-server/static-web-server) — A blazing fast and asynchronous web server for static files-serving. ⚡ [![CI](https://github.com/static-web-server/static-web-server/actions/workflows/devel.yml/badge.svg)](https://github.com/static-web-server/static-web-server/actions/workflows/devel.yml?query=branch%3Amaster)
* [svenstaro/miniserve](https://github.com/svenstaro/miniserve) — A small, self-contained cross-platform CLI tool that allows you to just grab the binary and serve some file(s) via HTTP [![build badge](https://github.com/svenstaro/miniserve/workflows/CI/badge.svg?branch=master)](https://github.com/svenstaro/miniserve/actions)
* [thecoshman/http](https://github.com/thecoshman/http) — Host These Things Please — A basic http server for hosting a folder fast and simply
* [TheWaWaR/simple-http-server](https://github.com/TheWaWaR/simple-http-server) — simple static http server
* [wyhaya/see](https://github.com/wyhaya/see) — Static HTTP file server

## Development tools

* [ATAC](https://github.com/Julien-cpsn/ATAC) — A feature-full TUI API client made in Rust. ATAC is free, open-source, offline and account-less.
* [bacon](https://github.com/Canop/bacon) — background rust code checker, similar to cargo-watch
* [clippy](https://crates.io/crates/clippy) — Rust lints
* [clog-tool/clog-cli](https://github.com/clog-tool/clog-cli) — generates a changelog from git metadata ([conventional changelog](https://blog.thoughtram.io/announcements/tools/2014/09/18/announcing-clog-a-conventional-changelog-generator-for-the-rest-of-us.html))
* [comtrya](https://github.com/comtrya/comtrya) — A configuration management tool for localhost / dotfiles [![build badge](https://github.com/comtrya/comtrya/actions/workflows/main.yaml/badge.svg)](https://github.com/comtrya/comtrya/actions)
* [create-rust-app](https://github.com/Wulf/create-rust-app) — Set up a modern rust+react web app by running one command. [![crate](https://img.shields.io/crates/v/create-rust-app.svg)](https://crates.io/crates/create-rust-app)
* [dan-t/rusty-tags](https://github.com/dan-t/rusty-tags) — create ctags/etags for a cargo project and all of its dependencies
* [datanymizer/datanymizer](https://github.com/datanymizer/datanymizer) - Powerful database anonymizer with flexible rules [![build badge](https://github.com/datanymizer/datanymizer/workflows/CI/badge.svg?branch=main)](https://github.com/datanymizer/datanymizer/actions?query=workflow%3ACI+branch%3Amain)
* [delta](https://crates.io/crates/git-delta) — A syntax-highlighter for git and diff output[![build badge](https://github.com/dandavison/delta/workflows/Continuous%20Integration/badge.svg)](https://github.com/dandavison/delta//actions)
* [dotenv-linter](https://github.com/dotenv-linter/dotenv-linter) — Linter for `.env` files [![build badge](https://github.com/dotenv-linter/dotenv-linter/workflows/CI/badge.svg?branch=master)](https://github.com/dotenv-linter/dotenv-linter/actions?query=workflow%3ACI+branch%3Amaster)
* [envio-cli/envio](https://github.com/envio-cli/envio) - A Modern And Secure CLI Tool For Managing Environment Variables [![build badge](https://github.com/envio-cli/envio/actions/workflows/CICD.yml/badge.svg?branch=main)](https://github.com/envio-cli/envio/actions/workflows/CICD.yml)
* [frolic](https://github.com/FrolicOrg/Frolic)  — An API layer to build customer facing dashboards 10x faster
* [fw](https://github.com/brocode/fw) — workspace productivity booster [![Rust](https://github.com/brocode/fw/actions/workflows/rust.yml/badge.svg)](https://github.com/brocode/fw/actions/workflows/rust.yml)
* [fzf-make](https://github.com/kyu08/fzf-make) [[fzf-make](https://crates.io/crates/fzf-make)] — A command line tool that executes make target using fuzzy finder with preview window. [![crates.io](https://img.shields.io/crates/v/fzf-make?style=flatflat-square)](https://crates.io/crates/fzf-make)
* [geiger](https://github.com/geiger-rs/cargo-geiger) — A program that list statistics related to usage of unsafe code in a crate and all its dependencies [![Build Status](https://dev.azure.com/cargo-geiger/cargo-geiger/_apis/build/status/geiger-rs.cargo-geiger?branchName=master)](https://dev.azure.com/cargo-geiger/cargo-geiger/_build/latest?definitionId=1&branchName=master)
* [git-cliff](https://github.com/orhun/git-cliff) — A highly customizable Changelog Generator that follows Conventional Commit specifications ![https://github.com/orhun/git-cliff/actions](https://img.shields.io/github/actions/workflow/status/orhun/git-cliff/ci.yml?branch=main&label=build)
* [git-journal](https://github.com/saschagrunert/git-journal/) — The Git Commit Message and Changelog Generation Framework
* [hot-lib-reloader](https://github.com/rksm/hot-lib-reloader-rs) — Hot reload Rust code [![build badge](https://github.com/rksm/hot-lib-reloader-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/rksm/hot-lib-reloader-rs/actions/workflows/ci.yml)
* [intelli-shell](https://github.com/lasantosr/intelli-shell) - Bookmark commands with placeholders and search or autocomplete at any time [![crate](https://img.shields.io/crates/v/intelli-shell.svg)](https://crates.io/crates/intelli-shell) [![build badge](https://github.com/lasantosr/intelli-shell/actions/workflows/release.yml/badge.svg)](https://github.com/lasantosr/intelli-shell/actions/workflows/release.yml)
* [just](https://github.com/casey/just) — A handy command runner for project-specific tasks
* [mask](https://github.com/jacobdeichert/mask) — A CLI task runner defined by a simple markdown file [![build badge](https://github.com/jacobdeichert/mask/workflows/CI/badge.svg?branch=master)](https://github.com/jacobdeichert/mask/actions?query=workflow%3ACI)
* [Module Linker](https://github.com/fiatjaf/module-linker) — Extension that adds `<a>` links to references in `mod`, `use` and `extern crate` statements at GitHub.
* [ptags](https://github.com/dalance/ptags) — A parallel universal-ctags wrapper for git repository
* [Racer](https://github.com/racer-rust/racer) — code completion for Rust
* [Rust Search Extension](https://github.com/huhu/rust-search-extension) — A handy browser extension to search crates and docs in address bar (omnibox). [![Build Status](https://github.com/huhu/rust-search-extension/workflows/build/badge.svg?branch=master)](https://github.com/huhu/rust-search-extension/actions)
* [Rustup](https://github.com/rust-lang/rustup) — the Rust toolchain installer [![build badge](https://github.com/rust-lang/rustup/workflows/Linux%20(master)/badge.svg?branch=master)](https://github.com/rust-lang/rustup/actions)
* [scriptisto](https://github.com/igor-petruk/scriptisto) A language-agnostic "shebang interpreter" that enables you to write one file scripts in compiled languages. [![Build Status](https://cloud.drone.io/api/badges/igor-petruk/scriptisto/status.svg)](https://cloud.drone.io/igor-petruk/scriptisto)
* [typos](https://github.com/crate-ci/typos) [[typos-cli](https://crates.io/crates/typos-cli)] — Source code spell checker

### Build system

* [Cargo](https://crates.io/) — the Rust package manager
  * [cargo-all-features](https://github.com/frewsxcv/cargo-all-features) - A configurable subcommand to simplify testing, building and much more for all combinations of features [![CI](https://github.com/frewsxcv/cargo-all-features/actions/workflows/ci.yml/badge.svg)](https://github.com/frewsxcv/cargo-all-features/actions/workflows/ci.yml)
  * [cargo-benchcmp](https://crates.io/crates/cargo-benchcmp) — A utility to compare micro-benchmarks
  * [cargo-bitbake](https://crates.io/crates/cargo-bitbake) — A cargo extension that can generate BitBake recipes utilizing the classes from meta-rust
  * [cargo-cache](https://crates.io/crates/cargo-cache) — inspect/manage/clean your cargo cache (`~/.cargo/`/`${CARGO_HOME}`), print sizes etc [![Build Status](https://github.com/matthiaskrgr/cargo-cache/workflows/ci/badge.svg?branch=master)](https://github.com/matthiaskrgr/cargo-cache/actions)
  * [cargo-check](https://crates.io/crates/cargo-check) — A wrapper around `cargo rustc -- -Zno-trans` which can be helpful for running a faster compile if you only need correctness checks
  * [cargo-commander](https://crates.io/crates/cargo-commander) — A subcommand for `cargo` to run CLI commands similar to how the scripts section in `package.json` works [![Build and test](https://github.com/simonhyll/cargo-commander/actions/workflows/build.yml/badge.svg)](https://github.com/simonhyll/cargo-commander/actions/workflows/build.yml)
  * [cargo-count](https://crates.io/crates/cargo-count) — lists source code counts and details about cargo projects, including unsafe statistics
  * [cargo-deb](https://crates.io/crates/cargo-deb) — Generates binary Debian packages
  * [cargo-deps](https://crates.io/crates/cargo-deps) — build dependency graphs
  * [cargo-do](https://crates.io/crates/cargo-do) — run multiple cargo commands in a row
  * [cargo-ebuild](https://crates.io/crates/cargo-ebuild) — cargo extension that can generate ebuilds using the in-tree eclasses
  * [cargo-edit](https://crates.io/crates/cargo-edit) — allows you to add and list dependencies by reading/writing to your Cargo.toml file from the command line
  * [cargo-generate](https://github.com/cargo-generate/cargo-generate) A generator of a rust project by leveraging a pre-existing git repository as a template.
  * [cargo-graph](https://crates.io/crates/cargo-graph) — updated fork of `cargo-dot` with additional features. Unmaintained, see `cargo-deps`
  * [cargo-info](https://crates.io/crates/cargo-info) — queries crates.io for crates details from command line
  * [cargo-license](https://crates.io/crates/cargo-license) — A cargo subcommand to quickly view the licenses of all dependencies.
  * [cargo-limit](https://crates.io/crates/cargo-limit) — Cargo with less noise: warnings are skipped until errors are fixed, Neovim integration, etc. [![build badge](https://github.com/cargo-limit//cargo-limit/actions/workflows/rust.yml/badge.svg)](https://github.com/cargo-limit//cargo-limit/actions)
  * [cargo-make](https://crates.io/crates/cargo-make) — Task runner and build tool. [![build badge](https://github.com/sagiegurari/cargo-make/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/cargo-make/actions)
  * [cargo-modules](https://crates.io/crates/cargo-modules) — A cargo plugin for showing a tree-like overview of a crate's modules.
  * [cargo-multi](https://crates.io/crates/cargo-multi) — runs specified cargo command on multiple crates
  * [cargo-outdated](https://crates.io/crates/cargo-outdated) — displays when newer versions of Rust dependencies are available, or out of date
  * [cargo-rdme](https://github.com/orium/cargo-rdme) [[cargo-rdme](https://crates.io/crates/cargo-rdme)] — Cargo subcommand to create your README from your crate’s documentation. [![build badge](https://github.com/orium/cargo-rdme/workflows/CI/badge.svg)](https://github.com/orium/cargo-rdme/actions?query=workflow%3ACI)
  * [cargo-release](https://crates.io/crates/cargo-release) — tool for releasing git-managed cargo project, build, tag, publish, doc and push [![Rust](https://github.com/crate-ci/cargo-release/actions/workflows/ci.yml/badge.svg)](https://github.com/crate-ci/cargo-release/actions/workflows/rust.yml)
  * [cargo-script](https://crates.io/crates/cargo-script) — lets people quickly and easily run Rust "scripts" which can make use of Cargo's package ecosystem
  * [cargo-udeps](https://github.com/est31/cargo-udeps) [[cargo-udeps](https://crates.io/crates/cargo-udeps)] — find unused dependencies
  * [cargo-update](https://crates.io/crates/cargo-update) — cargo subcommand for checking and applying updates to installed executables
  * [cargo-watch](https://crates.io/crates/cargo-watch) — utility for cargo to compile projects when sources change
  * [dtolnay/cargo-expand](https://github.com/dtolnay/cargo-expand) — Expand macros in your source code
* CMake
  * [Devolutions/CMakeRust](https://github.com/Devolutions/CMakeRust) — useful for integrating a Rust library into a CMake project
  * [SiegeLord/RustCMake](https://github.com/SiegeLord/RustCMake) — an example project showing usage of CMake with Rust
* [Fleet](https://github.com/dimensionhq/fleet) [[fleet-rs](https://crates.io/crates/fleet-rs)] - The blazing fast build tool for Rust.
* GitHub actions
  * [Mirlahiji/rust-action](https://github.com/Mirlahiji/rust-action) — rust github action
  * [peaceiris/actions-mdbook](https://github.com/peaceiris/actions-mdbook) — GitHub Actions for mdBook
* [Nix](https://nixos.org/)
  * [nix-community/fenix](https://github.com/nix-community/fenix) — Rust toolchains and rust analyzer nightly for nix [![build-badge](https://github.com/nix-community/fenix/actions/workflows/ci.yml/badge.svg)](https://github.com/nix-community/fenix/actions/workflows/ci.yml)

### Debugging

* GDB
  * [gdbgui](https://github.com/cs01/gdbgui) — Browser based frontend for gdb to debug C, C++, Rust, and go.
* LLDB
  * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) — A LLDB extension for [Visual Studio Code](https://code.visualstudio.com/).

### Deployment

* Docker
  * [emk/rust-musl-builder](https://github.com/emk/rust-musl-builder) — Docker images for compiling static Rust binaries using musl-libc and musl-gcc, with static versions of useful C libraries
  * [kpcyrd/mini-docker-rust](https://github.com/kpcyrd/mini-docker-rust) — An example project for very small rust docker images
  * [liuchong/docker-rustup](https://github.com/liuchong/docker-rustup) — A multiple version (with musl tools) Rust Docker image
  * [LukeMathWalker/cargo-chef](https://github.com/LukeMathWalker/cargo-chef) - A tool and pre-built images for caching compiling remote dependencies between Docker builds.
  * [rust-cross/rust-musl-cross](https://github.com/rust-cross/rust-musl-cross) — Docker images for compiling static Rust binaries using musl-cross [![Build](https://github.com/rust-cross/rust-musl-cross/workflows/Build/badge.svg)](https://github.com/rust-cross/rust-musl-cross/actions?query=workflow%3ABuild)
  * [rust-lang-nursery/docker-rust](https://github.com/rust-lang/docker-rust) — the official Rust Docker image
  * [Stavrospanakakis/is_ready](https://github.com/Stavrospanakakis/is_ready) - Wait for multiple services to become available ![Build](https://github.com/Stavrospanakakis/is_ready/actions/workflows/release.yml/badge.svg)
* Heroku
  * [emk/heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust) — A buildpack for Rust applications on Heroku
* [MarcoIeni/release-plz](https://github.com/MarcoIeni/release-plz) [[release-plz](https://crates.io/crates/release-plz)] — Release crates from CI, with changelog generation and semver check. [![build badge](https://github.com/MarcoIeni/release-plz/workflows/CI/badge.svg)](https://github.com/MarcoIeni/release-plz/actions)

### Embedded

[Rust Embedded](https://rust-embedded.org/) focuses on improving the end-to-end experience of using Rust in resource-constrained environments and non-traditional platforms. See [awesome-embedded-rust](https://github.com/rust-embedded/awesome-embedded-rust) for a curated, and more extended list of embedded Rust resources.

* Arduino
  * [avr-rust/ruduino](https://github.com/avr-rust/ruduino) Reusable components for the Arduino Uno.
* Cross compiling
  * [japaric/rust-cross](https://github.com/japaric/rust-cross) — everything you need to know about cross compiling Rust programs
  * [japaric/xargo](https://github.com/japaric/xargo) — effortless cross compilation of Rust programs to custom bare-metal targets like ARM Cortex-M
* Espressif
  * [esp-rs](https://github.com/esp-rs) home to a number of community projects enabling the use of the Rust programming language on various SoCs and modules produced by Espressif Systems.
* Firmware
  * [oreboot/oreboot](https://github.com/oreboot/oreboot) — oreboot is a fork of coreboot, with C removed, written in Rust
* nRF
  * [nrf-rs/nrf-hal](https://github.com/nrf-rs/nrf-hal) — A Rust HAL for the nRF family of devices

### FFI

See also [Foreign Function Interface](https://doc.rust-lang.org/book/first-edition/ffi.html), [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) (a collection of examples of using code written in Rust from other languages) and [FFI examples written in Rust](https://github.com/alexcrichton/rust-ffi-examples).

* C
  * [mozilla/cbindgen](https://github.com/mozilla/cbindgen) — generates C header files from Rust source files. Used in Gecko for WebRender
  * [Sean1708/rusty-cheddar](https://github.com/Sean1708/rusty-cheddar) — generates C header files from Rust source files
* C#
  * [csbindgen](https://github.com/Cysharp/csbindgen) - generates C# bindings for Rust source files
* C++
  * [dtolnay/cxx](https://github.com/dtolnay/cxx) — Safe interop between Rust and C++ [![build badge](https://img.shields.io/badge/github-dtolnay/cxx-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dtolnay/cxx)
  * [rust-cpp](https://crates.io/crates/cpp) - Embed C++ code directly in Rust. [![Build status](https://ci.appveyor.com/api/projects/status/uu76vmcrwnjqra0u/branch/master?svg=true)](https://ci.appveyor.com/project/mystor/rust-cpp/branch/master)
  * [rust-lang/rust-bindgen](https://github.com/rust-lang/rust-bindgen) — A Rust bindings generator
* Erlang
  * [rusterlium/rustler](https://github.com/rusterlium/rustler) — safe Rust bridge for creating Erlang NIF functions
* Java
  * [bennettanderson/rjni](https://github.com/benanders/rjni) — use Java from Rust
  * [drrb/java-rust-example](https://github.com/drrb/java-rust-example) — use Rust from Java
  * [j4rs](https://crates.io/crates/j4rs) — use Java from Rust
  * [jni](https://crates.io/crates/jni) — use Rust from Java
  * [jni-sys](https://crates.io/crates/jni-sys) — Rust definitions corresponding to jni.h
  * [rucaja](https://crates.io/crates/rucaja) — use Java from Rust
* Lua
  * [jcmoyer/rust-lua53](https://github.com/jcmoyer/rust-lua53) — Lua 5.3 bindings for Rust
  * [lilyball/rust-lua](https://github.com/lilyball/rust-lua) — Safe Rust bindings to Lua 5.1
  * [mlua-rs/mlua](https://github.com/mlua-rs/mlua) — High level Lua 5.4/5.3/5.2/5.1 (including LuaJIT) and Roblox Luau bindings to Rust with async/await support [![build badge](https://github.com/mlua-rs/mlua/workflows/CI/badge.svg)](https://github.com/mlua-rs/mlua/actions)
  * [tickbh/td_rlua](https://github.com/tickbh/td_rlua) [[td_rlua](https://crates.io/crates/td_rlua)] — Zero-cost high-level lua 5.3 wrapper for Rust
  * [tomaka/hlua](https://github.com/tomaka/hlua) — Rust library to interface with Lua
* mruby
  * [anima-engine/mrusty](https://github.com/anima-engine/mrusty) — mruby safe bindings for Rust
* Node.js
  * [infinyon/node-bindgen](https://github.com/infinyon/node-bindgen) - Easy way to generate nodejs module using Rust
  * [neon-bindings/neon](https://github.com/neon-bindings/neon) — Rust bindings for writing safe and fast native Node.js modules
  * [zhangyuang/node-ffi-rs](https://github.com/zhangyuang/node-ffi-rs) — A module written in Rust and N-API provides interface (FFI) features for Node.js
* Objective-C
  * [SSheldon/rust-objc](https://github.com/SSheldon/rust-objc) — Objective-C Runtime bindings and wrapper for Rust
* PHP
  * [phper-framework/phper](https://github.com/phper-framework/phper) — The framework that allows us to write PHP extensions using pure and safe Rust whenever possible
* Prolog
  * [mthom/scryer-prolog](https://github.com/mthom/scryer-prolog/) — Scryer Prolog is a free software ISO Prolog system written in Rust
* Python
  * [dgrunwald/rust-cpython](https://github.com/dgrunwald/rust-cpython) — Python bindings
  * [getsentry/milksnake](https://github.com/getsentry/milksnake) — extension for python setuptools that allows you to distribute dynamic linked libraries in Python wheels in the most portable way imaginable.
  * [PyO3/PyO3](https://github.com/PyO3/PyO3) — Rust bindings for the Python interpreter
  * [RustPython](https://github.com/RustPython/RustPython) — A Python Interpreter written in Rust [![Build Status](https://github.com/RustPython/RustPython/workflows/CI/badge.svg)](https://github.com/RustPython/RustPython/actions?query=workflow%3ACI)
* Ruby
  * [d-unsed/ruru](https://github.com/d-unsed/ruru) — native Ruby extensions written in Rust
  * [danielpclark/rutie](https://github.com/danielpclark/rutie) — native Ruby extensions written in Rust and vice versa
* Web Assembly
  * [rhysd/wain](https://github.com/rhysd/wain) - wain: WebAssembly INterpreter from scratch in Safe Rust with zero dependency [![build badge](https://github.com/rhysd/wain/workflows/CI/badge.svg?branch=master&event=push)](https://github.com/rhysd/wain/actions?query=workflow%3ACI+branch%3Amaster+event%3Apush)
  * [rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) — A project for facilitating high-level interactions between wasm modules and JS.
  * [rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack) — :package: :sparkles: pack up the wasm and publish it to npm!

### Formatters

* [dprint](https://github.com/dprint/dprint) — A pluggable and configurable code formatting platform [![build badge](https://github.com/dprint/dprint/workflows/CI/badge.svg)](https://github.com/dprint/dprint/actions?query=workflow%3ACI)
* [Prettier Rust](https://github.com/jinxdash/prettier-plugin-rust) — An opinionated Rust code formatter that autofixes bad syntax ([Prettier](https://prettier.io/) community plugin)
* [rustfmt](https://github.com/rust-lang/rustfmt) — Rust code formatter maintained by the Rust team and included in cargo

### IDEs

See also [Are we (I)DE yet?](https://areweideyet.com/) and [Rust Tools](https://www.rust-lang.org/tools).

  * [Eclipse](https://www.eclipse.org/)
    * [Eclipse Corrosion](https://github.com/eclipse-corrosion/corrosion)
  * [Emacs](https://www.gnu.org/software/emacs/)
    * [emacs-racer](https://github.com/racer-rust/emacs-racer) — Autocompletion (see also [company](https://company-mode.github.io) and [auto-complete](https://github.com/auto-complete/auto-complete))
    * [flycheck-rust](https://github.com/flycheck/flycheck-rust) — Rust support for [Flycheck](https://github.com/flycheck/flycheck)
    * [rust-mode](https://github.com/rust-lang/rust-mode) — Rust Major Mode
    * [rustic](https://github.com/brotzeit/rustic) - Rust development environment for Emacs [![build badge](https://github.com/brotzeit/rustic/workflows/CI/badge.svg)](https://github.com/brotzeit/rustic/actions?query=workflow%3ACI)
  * [gitpod.io](https://gitpod.io) — Online IDE with full Rust support based on Rust Language Server
  * [gnome-builder](https://wiki.gnome.org/Apps/Builder) native support for rust and cargo since Version 3.22.2
  * [IntelliJ](https://www.jetbrains.com/idea/)
    * [intellij-rust/intellij-rust](https://github.com/intellij-rust/intellij-rust) —
  * [Kakoune](http://kakoune.org/)
    * [kakoune-lsp](https://github.com/kakoune-lsp/kakoune-lsp/) — [LSP](https://microsoft.github.io/language-server-protocol/) client. Implemented in Rust and supports rls out of the box.
  * [lapce](https://github.com/lapce/lapce) — Lightning-fast and Powerful Code Editor written in Rust. [![build badge](https://github.com/lapce/lapce/actions/workflows/release.yml/badge.svg)](https://github.com/lapce/lapce/actions/workflows/release.yml)
  * [Ride](https://github.com/madeso/ride) —
  * [Sublime Text](https://www.sublimetext.com/)
    * [rust-lang/rust-enhanced](https://github.com/rust-lang/rust-enhanced) — official Rust package
  * [Vim](https://vim.sourceforge.io/) — the ubiquitous text editor
    * [autozimu/LanguageClient-neovim](https://github.com/autozimu/LanguageClient-neovim) — [LSP](https://microsoft.github.io/language-server-protocol/) client. Implemented in Rust and supports rls out of the box.
    * [crates.nvim](https://github.com/Saecki/crates.nvim) - plugin that helps to managing crates.io dependencies.
    * [rust.vim](https://github.com/rust-lang/rust.vim) — provides file detection, syntax highlighting, formatting, Syntastic integration, and more.
    * [vim-racer](https://github.com/racer-rust/vim-racer) — allows vim to use [Racer](https://github.com/racer-rust/racer) for Rust code completion and navigation.
  * Visual Studio
    * [dgriffen/rls-vs2017](https://github.com/ZoeyR/rls-vs2017) — Rust support for Visual Studio 2017 Preview [![build badge](https://ci.appveyor.com/api/projects/status/d2lxlincwninhsng?svg=true)](https://ci.appveyor.com/project/dgriffen/rls-vs2017)
    * [PistonDevelopers/VisualRust](https://github.com/PistonDevelopers/VisualRust) — A Visual Studio extension for Rust [![Build status](https://ci.appveyor.com/api/projects/status/5nw5no10jj0y4p3f?svg=true)](https://ci.appveyor.com/project/vosen/visualrust)
  * [Visual Studio Code](https://code.visualstudio.com/)
    * [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - TOML support in vscode
    * [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) — A LLDB extension
    * [crates](https://github.com/serayuzgur/crates) — crates is an extension for crates.io dependencies. [![build badge](https://img.shields.io/vscode-marketplace/v/serayuzgur.crates.svg)](https://github.com/serayuzgur/crates)
    * [Prettier - Code formatter (Rust)](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust) — Opinionated Rust code formatter that autofixes bad syntax ([Prettier](https://prettier.io/) community plugin)
    * [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) — An alternative rust language server to the RLS

### Profiling

* [Bencher](https://github.com/bencherdev/bencher) - A suite of continuous benchmarking tools designed to catch performance regressions in CI
* [bheisler/criterion.rs](https://github.com/bheisler/criterion.rs) — Statistics-driven benchmarking library
* [Bytehound](https://github.com/koute/bytehound) — A memory profiler for Linux
* [Divan](https://github.com/nvzqz/divan) — Simple yet powerful benchmarking library with allocation profiling
* [ellisonch/rust-stopwatch](https://github.com/ellisonch/rust-stopwatch) — A stopwatch library
* FlameGraphs
  * [llogiq/flame](https://github.com/llogiq/flame) —
  * [mrhooray/torch](https://github.com/mrhooray/torch) — generates FlameGraphs based on DWARF Debug Info
* [sharkdp/hyperfine](https://github.com/sharkdp/hyperfine) — A command-line benchmarking tool

### Services

* [deps.rs](https://github.com/deps-rs/deps.rs) — Detect outdated or insecure dependencies
* [docs.rs](https://docs.rs) — Automatic documentation generation of crates

### Static analysis

[[assert](https://crates.io/keywords/assert), [static](https://crates.io/keywords/static)]

* [facebookexperimental/MIRAI](https://github.com/facebookexperimental/mirai) — an abstract interpreter operating on Rust's mid-level intermediate representation (MIR) [![Continuous Integration](https://github.com/facebookexperimental/mirai/actions/workflows/rust.yml/badge.svg)](https://github.com/facebookexperimental/mirai/actions/workflows/rust.yml)
* [static_assertions](https://crates.io/crates/static_assertions) — Compile-time assertions to ensure that invariants are met

### Testing

[[test](https://crates.io/keywords/test), [testing](https://crates.io/keywords/testing)]

* Code Coverage
  * [tarpaulin](https://crates.io/crates/cargo-tarpaulin) — A code coverage tool
* Continuous Integration
  * [trust](https://github.com/japaric/trust) — A Travis CI and AppVeyor template to test your Rust crate on 5 architectures and publish binary releases of it for Linux, macOS and Windows
* Frameworks and Runners
  * [AlKass/polish](https://github.com/AlKass/polish) — Mini Testing/Test-Driven Framework [![Crates Package Status](https://img.shields.io/crates/v/polish.svg)](https://crates.io/crates/polish)
  * [cargo-dinghy](https://crates.io/crates/cargo-dinghy/) - A cargo extension to simplify running library tests and benches on smartphones and other small processor devices.
  * [cucumber](https://crates.io/crates/cucumber) [![Latest Version](https://img.shields.io/crates/v/cucumber.svg)](https://crates.io/crates/cucumber) — An implementation of the Cucumber testing framework for Rust. Fully native, no external test runners or dependencies. [![Build Status](https://github.com/cucumber-rs/cucumber/workflows/CI/badge.svg?branch=master)](https://github.com/cucumber-rs/cucumber)
  * [d-e-s-o/test-log](https://github.com/d-e-s-o/test-log) [[test-log](https://crates.io/crates/test-log)] — A replacement of the `#[test]` attribute that initializes logging and/or tracing infrastructure before running tests. [![GitHub Workflow Status](https://github.com/d-e-s-o/test-log/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/d-e-s-o/test-log/actions/workflows/test.yml)
  * [demonstrate](https://crates.io/crates/demonstrate) — Declarative Testing Framework [![Build Status](https://github.com/aubaugh/demonstrate/workflows/Continuous%20Integration/badge.svg?branch=master)](https://github.com/aubaugh/demonstrate)
  * [GoogleTest Rust](https://crates.io/crates/googletest) — Powerful test assertion framework based on the C++ test library GoogleTest [![Build Status](https://github.com/google/googletest-rust/workflows/CI/badge.svg)](https://github.com/google/googletest-rust/actions?query=workflow%3ACI+branch%3Amain)
  * [rlt](https://github.com/wfxr/rlt) — A universal load testing framework, with real-time tui support.
  * [rstest](https://crates.io/crates/rstest) — Fixture-based test framework [![Build Status](https://github.com/la10736/rstest/workflows/Test/badge.svg?branch=master)](https://github.com/la10736/rstest/actions)
  * [speculate](https://crates.io/crates/speculate) — An RSpec inspired minimal testing framework
* Mocking and Test Data
  * [asomers/mockall](https://github.com/asomers/mockall) [[mockall](https://crates.io/crates/mockall)] — A powerful mock object library. [![Cirrus Build Status](https://api.cirrus-ci.com/github/asomers/mockall.svg)](https://cirrus-ci.com/github/asomers/mockall)
  * [fake-rs](https://github.com/cksac/fake-rs) —  A library for generating fake data
  * [goldenfile](https://github.com/calder/rust-goldenfile) [[goldenfile](https://crates.io/crates/goldenfile)] - A library providing a simple API for goldenfile testing.
  * [httpmock](https://github.com/alexliesenfeld/httpmock) — HTTP mocking [![build badge](https://dev.azure.com/alexliesenfeld/httpmock/_apis/build/status/alexliesenfeld.httpmock?branchName=master)](https://dev.azure.com/alexliesenfeld/httpmock/_build/latest?definitionId=2&branchName=master)
  * [mockiato](https://crates.io/crates/mockiato) — A strict, yet friendly mocking library for unstable Rust 2018
  * [mockito](https://crates.io/crates/mockito) — HTTP mocking
  * [nrxus/faux](https://github.com/nrxus/faux/) [![Latest Version](https://img.shields.io/crates/v/faux.svg)](https://crates.io/crates/faux) — A library to create mocks out of structs. ![build](https://github.com/nrxus/faux/workflows/test/badge.svg?branch=master)
  * [synth](https://github.com/shuttle-hq/synth/) — Generate database data declaratively. [![build](https://github.com/shuttle-hq/synth/actions/workflows/synth-test.yml/badge.svg)](https://github.com/shuttle-hq/synth)
* Mutation Testing
  * [cargo-mutants](https://github.com/sourcefrog/cargo-mutants) [[cargo-mutants](https://crates.io/crates/cargo-mutants)] - Finds inadequately tested code by injecting mutations, no source changes required. [![build badge](https://github.com/sourcefrog/cargo-mutants/actions/workflows/tests.yml/badge.svg?branch=main&event=push)](https://github.com/sourcefrog/cargo-mutants/actions/workflows/tests.yml?query=branch%3Amain)
  * [mutagen](https://github.com/llogiq/mutagen) [[mutagen](https://crates.io/crates/mutagen)] — A source-level mutation testing framework (nightly only)
* Property Testing and Fuzzing
  * [proptest](https://crates.io/crates/proptest) — property testing framework inspired by the [Hypothesis](https://hypothesis.works/) framework for Python
  * [quickcheck](https://crates.io/crates/quickcheck) — A Rust implementation of [QuickCheck](https://wiki.haskell.org/Introduction_to_QuickCheck1)
  * [rust-fuzz/afl.rs](https://github.com/rust-fuzz/afl.rs) — A Rust fuzzer, using [AFL](https://lcamtuf.coredump.cx/afl/)

### Transpiling

* [BayesWitnesses/m2cgen](https://github.com/BayesWitnesses/m2cgen) — A CLI tool to transpile trained classic machine learning models into a native Rust code with zero dependencies. [![GitHub Actions Status](https://github.com/BayesWitnesses/m2cgen/workflows/GitHub%20Actions/badge.svg?branch=master)](https://github.com/BayesWitnesses/m2cgen/actions)
* [immunant/c2rust](https://github.com/immunant/c2rust) — C to Rust translator and cross checker built atop Clang/LLVM.
* [jameysharp/corrode](https://github.com/jameysharp/corrode) — A C to Rust translator written in Haskell.

## Libraries

* [perf-monitor-rs](https://github.com/larksuite/perf-monitor-rs) — A toolkit designed to be a foundation for applications to monitor their performance. [![crates.io](https://img.shields.io/crates/v/perf_monitor.svg)](https://crates.io/crates/perf_monitor)

### Artificial Intelligence

#### Genetic algorithms

* [innoave/genevo](https://github.com/innoave/genevo) — Execute genetic algorithm (GA) simulations in a customizable and extensible way.
* [m-decoster/RsGenetic](https://github.com/m-decoster/RsGenetic) — Genetic Algorithm library. In maintenance mode.
* [Martin1887/oxigen](https://github.com/Martin1887/oxigen) — Fast, parallel, extensible and adaptable genetic algorithm library. A example using this library solves the N Queens problem for N = 255 in only few seconds and using less than 1 MB of RAM.
* [pkalivas/radiate](https://github.com/pkalivas/radiate) — A customizable parallel genetic programming engine capable of evolving solutions for supervised, unsupervised, and reinforcement learning problems. Comes with complete and customizable implementation of NEAT and Evtree.![Crates.io](https://img.shields.io/crates/v/radiate)
* [willi-kappler/darwin-rs](https://github.com/willi-kappler/darwin-rs) — Evolutionary algorithms

#### Machine learning

See [[Machine learning](https://crates.io/keywords/machine-learning)]

See also [About Rust’s Machine Learning Community](https://medium.com/@autumn_eng/about-rust-s-machine-learning-community-4cda5ec8a790#.hvkp56j3f) and [Are we learning yet?](https://www.arewelearningyet.com).

* [autumnai/leaf](https://github.com/autumnai/leaf) — Open Machine Intelligence framework.. Abandoned project. The most updated fork is [juice](https://github.com/fff-rs/juice).
* [burn](https://github.com/tracel-ai/burn) - A Flexible and Comprehensive Deep Learning Framework.
* [coreylowman/dfdx](https://github.com/coreylowman/dfdx) — CUDA accelerated machine learning framework that leverages many of Rust's unique features. ![Crates.io](https://img.shields.io/crates/v/dfdx)
* [guillaume-be/rust-bert](https://github.com/guillaume-be/rust-bert) [[rust_bert](https://crates.io/crates/rust_bert)] — Ready-to-use NLP pipelines and language models
* [huggingface/candle](https://github.com/huggingface/candle) [[candle-core](https://crates.io/crates/candle-core)]- a minimalist ML framework with a focus on easiness of use and on performance (including GPU support)
* [huggingface/tokenizers](https://github.com/huggingface/tokenizers) - Hugging Face's tokenizers for modern NLP pipelines (original implementation) with bindings for Python. [![Build Status](https://github.com/huggingface/tokenizers/workflows/Rust/badge.svg?branch=master)](https://github.com/huggingface/tokenizers/actions)
* [LaurentMazare/tch-rs](https://github.com/LaurentMazare/tch-rs) — Bindings for PyTorch.
* [maciejkula/rustlearn](https://github.com/maciejkula/rustlearn) — Machine learning library. [![Circle CI](https://circleci.com/gh/maciejkula/rustlearn.svg?style=svg)](https://app.circleci.com/pipelines/github/maciejkula/rustlearn)
* [rust-ml/linfa](https://github.com/rust-ml/linfa) — Machine learning framework.
* [smartcorelib/smartcore](https://github.com/smartcorelib/smartcore) — Machine Learning Library [![Build Status](https://img.shields.io/circleci/build/github/smartcorelib/smartcore)](https://smartcorelib.org/)
* [tensorflow/rust](https://github.com/tensorflow/rust) — Bindings for TensorFlow.

#### OpenAI

* [64bit/async-openai](https://github.com/64bit/async-openai) [[async-openai](https://crates.io/crates/async-openai)] — Ergonomic Rust bindings for OpenAI API based on OpenAPI spec.
* [zurawiki/tiktoken-rs](https://github.com/zurawiki/tiktoken-rs) [[tiktoken-rs](https://crates.io/crates/tiktoken-rs)] — Library for tokenizing text with OpenAI models using tiktoken. [![CI](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/zurawiki/tiktoken-rs/actions/workflows/ci.yml)

### Astronomy

[[astronomy](https://crates.io/keywords/astronomy)]

* [cds-astro/aladin-lite](https://github.com/cds-astro/aladin-lite) - Web application for visualizing spatial and planetary image surveys in different projections
* [fitsio](https://crates.io/crates/fitsio) — fits interface library wrapping cfitsio
* [flosse/rust-sun](https://github.com/flosse/rust-sun) [[sun](https://crates.io/crates/sun)] — A rust port of the JS library suncalc
* [saurvs/astro-rust](https://github.com/saurvs/astro-rust) — astronomy

### Asynchronous

* [async-std](https://async.rs/) [[async-std](https://crates.io/crates/async-std)] - Async version of the Rust standard library [![CI](https://github.com/async-rs/async-std/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/async-rs/async-std/actions/workflows/ci.yml)
* [dpc/mioco](https://github.com/dpc/mioco) — Scalable, coroutine-based, asynchronous IO handling library
* [mio](https://github.com/tokio-rs/mio) — MIO is a lightweight IO library, with a focus on adding as little overhead as possible over the OS abstractions
* [rust-lang/futures-rs](https://github.com/rust-lang/futures-rs) — Zero-cost futures
* [t3hmrman/async-dropper](https://github.com/t3hmrman/async-dropper) [[async-dropper](https://crates.io/crates/async-dropper)] - Implementation of `AsyncDrop`
* [TeaEntityLab/fpRust](https://github.com/TeaEntityLab/fpRust) — Monad/MonadIO, Handler, Coroutine/doNotation, Functional Programming features for Rust
* [Xudong-Huang/may](https://github.com/Xudong-Huang/may) — Stackful coroutine library
* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) — A coroutine I/O library with a working-stealing scheduler

### Audio and Music

[[audio](https://crates.io/keywords/audio)]

* [hound](https://crates.io/crates/hound) — A WAV encoding and decoding library
* [insomnimus/nodi](https://github.com/insomnimus/nodi) [[nodi](https://crates.io/crates/nodi)] — A library for playback and abstraction of MIDI files. [![build badge](https://github.com/insomnimus/nodi/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/insomnimus/nodi/actions)
* [jhasse/ears](https://github.com/jhasse/ears) — A simple library to play Sounds and Musics, on top of OpenAL and libsndfile
* [musitdev/portmidi-rs](https://github.com/musitdev/portmidi-rs) — [PortMidi](https://portmedia.sourceforge.net/portmidi/) bindings
* [ozankasikci/rust-music-theory](https://github.com/ozankasikci/rust-music-theory) — Music theory library
* [pdeljanov/Symphonia](https://github.com/pdeljanov/Symphonia) — Audio decoding and media demuxing library supporting AAC, FLAC, MP3, MP4, OGG, Vorbis, and WAV.
* [RustAudio](https://github.com/RustAudio)
  * [RustAudio/cpal](https://github.com/RustAudio/cpal) - Low-level cross-platform audio I/O library. [![Actions Status](https://github.com/RustAudio/cpal/workflows/cpal/badge.svg?branch=master)](https://github.com/RustAudio/cpal/actions)
  * [RustAudio/rodio](https://github.com/RustAudio/rodio) — Audio playback library
  * [RustAudio/rust-portaudio](https://github.com/RustAudio/rust-portaudio) — PortAudio bindings
* [Serial-ATA/lofty-rs](https://github.com/Serial-ATA/lofty-rs) [[lofty](https://crates.io/crates/lofty)] — A library for reading and editing the metadata of various audio formats [![build badge](https://github.com/Serial-ATA/lofty-rs/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/Serial-ATA/lofty-rs/actions)

### Authentication

* [constantoine/totp-rs](https://github.com/constantoine/totp-rs) [[totp-rs](https://crates.io/crates/totp-rs)] — 2fa library to generate and verify TOTP-based tokens ![Build Status](https://github.com/constantoine/totp-rs/workflows/Rust/badge.svg)
* [Keats/jsonwebtoken](https://github.com/Keats/jsonwebtoken) — [JSON Web Token](https://en.wikipedia.org/wiki/JSON_Web_Token) library
* [oauth2](https://github.com/ramosbugs/oauth2-rs) — Extensible, strongly-typed OAuth2 client library
* [oxide-auth](https://github.com/HeroicKatora/oxide-auth) — A OAuth2 server library, for use in combination with actix or other frontends, featuring a set of configurable and pluggable backends [![Build Status](https://api.cirrus-ci.com/github/HeroicKatora/oxide-auth.svg?branch=master)](https://cirrus-ci.com/github/HeroicKatora/oxide-auth)
* [sgrust01/jwtvault](https://github.com/sgrust01/jwtvault) — Async library to manage and orchestrate JWT workflow
* [yup-oauth2](https://github.com/dermesser/yup-oauth2) — An oauth2 client implementation providing the Device, Installed and Service Account flows

### Automotive

* [idletea/tokio-socketcan](https://github.com/idletea/tokio-socketcan) [[tokio-socketcan](https://crates.io/crates/tokio-socketcan)] — Linux SocketCAN support for tokio based on the socketcan crate
* [marcelbuesing/can-dbc](https://github.com/marcelbuesing/can-dbc) [[can-dbc](https://crates.io/crates/can-dbc)] — A parser for the DBC format
* [marcelbuesing/tokio-socketcan-bcm](https://github.com/marcelbuesing/tokio-socketcan-bcm) [[tokio-socketcan-bcm](https://crates.io/crates/tokio-socketcan-bcm)] — Linux SocketCAN BCM support for tokio
* [mbr/socketcan](https://github.com/socketcan-rs/socketcan-rs) [[socketcan](https://crates.io/crates/socketcan)] — Linux SocketCAN library
* [Sensirion/lin-bus](https://github.com/Sensirion/lin-bus-rs) [[lin-bus](https://crates.io/crates/lin-bus)] — LIN bus driver traits and protocol implementation [![build badge](https://circleci.com/gh/Sensirion/lin-bus-rs.svg?style=svg)](https://app.circleci.com/pipelines/github/Sensirion/lin-bus-rs)

### Bioinformatics

* [Rust-Bio](https://github.com/rust-bio) — bioinformatics libraries.

### Caching

* [06chaynes/http-cache](https://github.com/06chaynes/http-cache) [[http-cache](https://crates.io/crates/http-cache)] - A caching middleware that follows HTTP caching rules [![build badge](https://github.com/06chaynes/http-cache/workflows/http-cache/badge.svg)](https://github.com/06chaynes/http-cache/actions/workflows/http-cache.yml)
* [aisk/rust-memcache](https://github.com/aisk/rust-memcache) — Memcached client library
* [al8n/stretto](https://github.com/al8n/stretto) - A high performance thread-safe memory-bound cache [![build badge](https://github.com/al8n/stretto/actions/workflows/ci.yml/badge.svg)](https://github.com/al8n/stretto/actions/workflows/ci.yml)
* [jaemk/cached](https://github.com/jaemk/cached) — Simple function caching/memoization
* [moka-rs/moka](https://github.com/moka-rs/moka) - A high performance concurrent caching library inspired by the Caffeine library for Java [![build badge](https://github.com/moka-rs/moka/workflows/CI/badge.svg)](https://github.com/moka-rs/moka/actions/workflows/CI.yml)
* [mozilla/sccache](https://github.com/mozilla/sccache/) - Shared Compilation Cache, great compilation
* [zkat/cacache-rs](https://github.com/zkat/cacache-rs) - A high-performance, concurrent, content-addressable disk cache, optimized for async APIs [![build badge](https://github.com/zkat/cacache-rs/workflows/CI/badge.svg)](https://github.com/zkat/cacache-rs/actions/workflows/ci.yml)

### Cloud

* AWS [[aws](https://crates.io/keywords/aws)]
  * [awslabs/aws-lambda-rust-runtime](https://github.com/awslabs/aws-lambda-rust-runtime) [[lambda_runtime](https://crates.io/crates/lambda_runtime)] — Runtime for AWS Lambda [![build badge](https://github.com/awslabs/aws-lambda-rust-runtime/workflows/Rust/badge.svg)](https://github.com/awslabs/aws-lambda-rust-runtime/actions)
  * [awslabs/aws-sdk-rust](https://github.com/awslabs/aws-sdk-rust) - The new AWS SDK
  * [rusoto/rusoto](https://github.com/rusoto/rusoto) —
* Load Balancer
  * [Convey](https://github.com/bparli/convey) - Layer 4 Load Balancer with dynamic configuration loading.
* Multi Cloud
  * [Qovery/engine](https://github.com/Qovery/engine) - Abstraction layer library that turns easy application deployment on Cloud providers in just a few minutes

### Command-line

* Argument parsing
  * [clap-rs](https://github.com/clap-rs/clap) [[clap](https://crates.io/crates/clap)] — A simple to use, full featured command-line argument parser
  * [cliparser](https://crates.io/crates/cliparser) — Simple command line parser. [![build badge](https://github.com/sagiegurari/cliparser/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/cliparser/actions)
  * [docopt/docopt.rs](https://github.com/docopt/docopt.rs) [[docopt](https://crates.io/crates/docopt)] — Implementation of [DocOpt](http://docopt.org)
  * [google/argh](https://github.com/google/argh) [[argh](https://crates.io/crates/argh)] — An opinionated Derive-based argument parser optimized for code size [![build badge](https://github.com/google/argh/workflows/Argh/badge.svg?branch=master)](https://github.com/google/argh/actions)
  * [killercup/quicli](https://github.com/killercup/quicli) [[quicli](https://crates.io/crates/quicli)] — quickly build cool CLI apps
  * [ksk001100/seahorse](https://github.com/ksk001100/seahorse) [[seahorse](https://crates.io/crates/seahorse)] — A minimal CLI framework [![Build status](https://github.com/ksk001100/seahorse/workflows/CI/badge.svg?branch=master)](https://github.com/ksk001100/seahorse/actions)
  * [TeXitoi/structopt](https://github.com/TeXitoi/structopt) [[structopt](https://crates.io/crates/structopt)] — parse command line argument by defining a struct
* Data visualization
  * [nukesor/comfy-table](https://github.com/nukesor/comfy-table) [[comfy-table](https://crates.io/crates/comfy-table)] — Beautiful dynamic tables for your cli tools. [![Build status](https://github.com/Nukesor/comfy-table/workflows/Tests/badge.svg?branch=master)](https://github.com/nukesor/comfy-table/actions)
  * [zhiburt/tabled](https://github.com/zhiburt/tabled) [[tabled](https://crates.io/crates/tabled)] — An easy to use library for pretty print tables of structs and enums. [![Build Status](https://github.com/zhiburt/tabled/actions/workflows/ci.yml/badge.svg)](https://github.com/zhiburt/tabled/actions)
* Human-centered design
  * [rust-cli/human-panic](https://github.com/rust-cli/human-panic) [[human-panic](https://crates.io/crates/human-panic)] — panic messages for humans
* Line editor
  * [kkawakam/rustyline](https://github.com/kkawakam/rustyline) [[rustyline](https://crates.io/crates/rustyline)] — readline implementation
  * [MovingtoMars/liner](https://github.com/MovingtoMars/liner) [[liner](https://crates.io/crates/liner)] — A library offering readline-like functionality
  * [murarth/linefeed](https://github.com/murarth/linefeed) [[linefeed](https://crates.io/crates/linefeed)] — Configurable, extensible, interactive line reader
  * [srijs/rust-copperline](https://github.com/srijs/rust-copperline) [[copperline](https://crates.io/crates/copperline)] — command line editing library
* Other
  * [mgrachev/update-informer](https://github.com/mgrachev/update-informer) [[update-informer](https://crates.io/crates/update-informer)] — Update informer for CLI applications. It checks for a new version on Crates.io and GitHub [![build badge](https://github.com/mgrachev/update-informer/workflows/CI/badge.svg)](https://github.com/mgrachev/update-informer/actions)
* Pipeline
  * [hniksic/rust-subprocess](https://github.com/hniksic/rust-subprocess) [[subprocess](https://crates.io/crates/subprocess)] — facilities for interaction with external pipelines
  * [imp/pager-rs](https://gitlab.com/imp/pager-rs) [[pager](https://crates.io/crates/pager)] — pipe your output through an external pager
  * [oconnor663/duct.rs](https://github.com/oconnor663/duct.rs) [[duct](https://crates.io/crates/duct)] — A builder for subprocess pipelines and IO redirection
  * [rust-cli/rexpect](https://github.com/rust-cli/rexpect) [[rexpect](https://crates.io/crates/rexpect)] — automate interactive applications such as ssh, ftp, passwd, etc [![CI](https://github.com/rust-cli/rexpect/actions/workflows/ci.yml/badge.svg)](https://github.com/rust-cli/rexpect/actions/workflows/ci.yml)
  * [zhiburt/expectrl](https://github.com/zhiburt/expectrl) [[expectrl](https://crates.io/crates/expectrl)] — A library for controlling interactive programs in a pseudo-terminal [![build badge](https://github.com/zhiburt/expectrl/actions/workflows/ci.yml/badge.svg)](https://github.com/zhiburt/expectrl/actions/workflows/ci.yml)
* Progress
  * [a8m/pb](https://github.com/a8m/pb) [[pbr](https://crates.io/crates/pbr)] — console progress bar
  * [console-rs/indicatif](https://github.com/console-rs/indicatif) [[indicatif](https://crates.io/crates/indicatif)] — indicate progress to users
  * [etienne-napoleone/spinach](https://github.com/etienne-napoleone/spinach) [[spinach](https://crates.io/crates/spinach)] — Practical spinner. [![CI](https://github.com/etienne-napoleone/spinach/actions/workflows/ci.yml/badge.svg)](https://github.com/etienne-napoleone/spinach/actions/workflows/ci.yml)
  * [FGRibreau/spinners](https://github.com/FGRibreau/spinners) [[spinners](https://crates.io/crates/spinners)] — 60+ elegant terminal spinners
* Prompt
  * [hashmismatch/terminal_cli.rs](https://github.com/hashmismatch/terminal_cli.rs) [[terminal_cli](https://crates.io/crates/terminal_cli)]  — build an interactive command prompt
  * [mikaelmello/inquire](https://github.com/mikaelmello/inquire) [[inquire](https://crates.io/crates/inquire)] — A library for building interactive prompts on terminals. [![Build status](https://github.com/mikaelmello/inquire/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/mikaelmello/inquire/actions)
  * [starship/starship](https://starship.rs/) [[starship](https://crates.io/crates/starship)]  — A minimal, blazing fast, and extremely customizable prompt for any shell [![Build status](https://github.com/starship/starship/workflows/Main%20workflow/badge.svg?branch=master)](https://github.com/starship/starship/actions)
  * [ynqa/promkit](https://github.com/ynqa/promkit) [[promkit](https://crates.io/crates/promkit)]  — A toolkit for building interactive command-line tools [![ci](https://github.com/ynqa/promkit/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/ynqa/promkit/actions/workflows/ci.yml)
* Style
  * [colored](https://github.com/colored-rs/colored) [[colored](https://crates.io/crates/colored)] — Coloring terminal so simple, you already know how to do it!
  * [console-rs/dialoguer](https://github.com/console-rs/dialoguer) [[dialoguer](https://crates.io/crates/dialoguer)] — Library for command line prompts and similar things.
  * [LukasKalbertodt/bunt](https://github.com/LukasKalbertodt/bunt) [[bunt](https://crates.io/crates/bunt)] — cross-platform terminal colors and styling with macros [![Build status](https://github.com/LukasKalbertodt/bunt/actions/workflows/ci.yml/badge.svg)](https://github.com/LukasKalbertodt/bunt/actions?query=workflow%3ACI+branch%3Amaster)
  * [LukasKalbertodt/term-painter](https://github.com/LukasKalbertodt/term-painter) [[term-painter](https://crates.io/crates/term-painter)] — cross-platform styled terminal output
  * [ogham/rust-ansi-term](https://github.com/ogham/rust-ansi-term) [[ansi_term](https://crates.io/crates/ansi_term)] — control colours and formatting on ANSI terminals
  * [SergioBenitez/yansi](https://github.com/SergioBenitez/yansi) [[yansi](https://crates.io/crates/yansi)] — A dead simple ANSI terminal color painting library
* TUI
  * BearLibTerminal
    * [cfyzium/bearlibterminal](https://github.com/nabijaczleweli/BearLibTerminal.rs) [[bear-lib-terminal](https://crates.io/crates/bear-lib-terminal)] — [BearLibTerminal](https://github.com/tommyettinger/BearLibTerminal) bindings
  * [gyscos/Cursive](https://github.com/gyscos/Cursive) [[cursive](https://crates.io/crates/cursive)] — build rich TUI applications
  * [ivanceras/titik](https://github.com/ivanceras/titik) - a crossplatform TUI widget library with the goal of providing interactive widgets
  * ncurses
    * [ihalila/pancurses](https://github.com/ihalila/pancurses) [[pancurses](https://crates.io/crates/pancurses)] — curses library, supports linux and windows
    * [jeaye/ncurses-rs](https://github.com/jeaye/ncurses-rs) [[ncurses](https://crates.io/crates/ncurses)] — [ncurses](https://www.gnu.org/software/ncurses/) bindings
  * [ogham/rust-term-grid](https://github.com/ogham/rust-term-grid) [[term_grid](https://crates.io/crates/term_grid)] — Library for putting things in a grid
  * [ratatui-org/ratatui](https://github.com/ratatui-org/ratatui) [[ratatui](https://crates.io/crates/ratatui)] — Library that's all about cooking up terminal user interfaces (TUIs)
  * [redox-os/termion](https://github.com/redox-os/termion) [[termion](https://crates.io/crates/termion)] — bindless library for controlling terminals/TTY
  * [ruterm](https://crates.io/crates/ruterm) - tiny & simple library for working with TTY
  * Termbox
    * [gchp/rustbox](https://github.com/gchp/rustbox) [[rustbox](https://crates.io/crates/rustbox)] — bindings to [Termbox](https://github.com/nsf/termbox)
  * [TimonPost/crossterm](https://github.com/crossterm-rs/crossterm) [[crossterm](https://crates.io/crates/crossterm)] — crossplatform terminal library

### Compression

* [7z](https://7-zip.org/7z.html)
  * [dyz1990/sevenz-rust](https://github.com/dyz1990/sevenz-rust) [[sevenz-rust](https://crates.io/crates/sevenz-rust)] — A 7z decompressor/compressor written in pure rust. [![Rust](https://github.com/dyz1990/sevenz-rust/workflows/Rust/badge.svg?branch=main)](https://github.com/dyz1990/sevenz-rust/actions)
* [Brotli](https://opensource.googleblog.com/2015/09/introducing-brotli-new-compression.html)
  * [dropbox/rust-brotli](https://github.com/dropbox/rust-brotli) — Brotli decompressor that optionally avoids the stdlib
  * [ende76/brotli-rs](https://github.com/ende76/brotli-rs) — implementation of Brotli compression
* bzip2
  * [alexcrichton/bzip2-rs](https://github.com/alexcrichton/bzip2-rs) — [libbz2](https://www.sourceware.org/bzip2/) bindings
* gzip
  * [zopfli](https://github.com/zopfli-rs/zopfli) [[zopfli](https://crates.io/crates/zopfli)] — implementation of the Zopfli compression algorithm for higher quality deflate or zlib compression
* gzp
  * [sstadick/gzp](https://github.com/sstadick/gzp/) - multi-threaded encoding and decoding of deflate formats and snappy
* miniz
  * [rust-lang/flate2-rs](https://github.com/rust-lang/flate2-rs) — [miniz](https://code.google.com/archive/p/miniz) bindings [![build badge](https://github.com/rust-lang/flate2-rs/workflows/CI/badge.svg?branch=master)](https://github.com/rust-lang/flate2-rs/actions)
* tar
  * [alexcrichton/tar-rs](https://github.com/alexcrichton/tar-rs) — tar archive reading/writing
* zip
  * [zip-rs/zip2](https://github.com/zip-rs/zip2) [[zip](https://crates.io/crates/zip)] — read and write  ZIP archives
* zstd
  * [gyscos/zstd-rs](https://github.com/gyscos/zstd-rs) — rust binding for the zstd compression library

### Computation

* [argmin-rs/argmin](https://github.com/argmin-rs/argmin) [[argmin](https://crates.io/crates/argmin)] — Optimization library
* [BLAS](https://en.wikipedia.org/wiki/Basic_Linear_Algebra_Subprograms) [[blas](https://crates.io/keywords/blas)]
  * [mikkyang/rust-blas](https://github.com/mikkyang/rust-blas) — BLAS bindings
* [calebwin/emu](https://github.com/calebwin/emu) — A language for GPGPU numerical computing
* [dimforge/nalgebra](https://github.com/dimforge/nalgebra) — low-dimensional linear algebra library
* [faer-rs](https://github.com/sarah-ek/faer-rs) [[faer](https://crates.io/crates/faer)] — Linear algebra foundation for Rust
* [GSL](http://www.gnu.org/software/gsl/)
  * [GuillaumeGomez/rust-GSL](https://github.com/GuillaumeGomez/rust-GSL) — GSL bindings
* [LAPACK](https://en.wikipedia.org/wiki/LAPACK)
  * [stainless-steel/lapack](https://github.com/blas-lapack-rs/lapack) — LAPACK bindings
* Parallel
  * [arrayfire/arrayfire-rust](https://github.com/arrayfire/arrayfire-rust) — [Arrayfire](https://github.com/arrayfire) bindings
  * [autumnai/collenchyma](https://github.com/autumnai/collenchyma) — An extensible, pluggable, backend-agnostic framework for parallel, high-performance computations on CUDA, OpenCL and common host CPU.
  * [luqmana/rust-opencl](https://github.com/luqmana/rust-opencl) — [OpenCL](https://www.khronos.org/opencl/) bindings
* Science
  * [cpmech/russell](https://github.com/cpmech/russell) — Rust Scientific Library for numerical mathematics, ordinary differential equations, special math functions, high-performance (sparse) linear algebra
  * [indigits/scirust](https://github.com/indigits/scirust) — Scientific computing library
* Statrs
  * [statrs-dev/statrs](https://github.com/statrs-dev/statrs) — Robust statistical computation library

### Concurrency

* [crossbeam-rs/crossbeam](https://github.com/crossbeam-rs/crossbeam) – Support for parallelism and low-level concurrency
* [orium/archery](https://github.com/orium/archery) [[archery](https://crates.io/crates/archery)] — Library to abstract from `Rc`/`Arc` pointer types. [![build badge](https://github.com/orium/archery/workflows/CI/badge.svg)](https://github.com/orium/archery/actions?query=workflow%3ACI)
* [Rayon](https://github.com/rayon-rs/rayon) – A data parallelism library
* [rustcc/coroutine-rs](https://github.com/rustcc/coroutine-rs) – Coroutine Library
* [zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs) – Coroutine I/O

### Configuration

* [andoriyu/uclicious](https://github.com/andoriyu/uclicious) [[uclicious](https://crates.io/crates/uclicious)] — [libUCL](https://github.com/vstakhov/libucl) based feature-rich configuration library. [![CircleCI](https://circleci.com/gh/vstakhov/libucl.svg?style=svg)](https://app.circleci.com/pipelines/github/vstakhov/libucl)
* [Kixunil/configure_me](https://github.com/Kixunil/configure_me) [[configure_me](https://crates.io/crates/configure_me)] — library for processing application configuration easily
* [mehcode/config-rs](https://github.com/mehcode/config-rs) [[config](https://crates.io/crates/config)] — Layered configuration system (with strong support for 12-factor applications).
* [SergioBenitez/Figment](https://github.com/SergioBenitez/Figment) [[figment](https://crates.io/crates/figment)] — A configuration library so con-free, it's unreal.
* [softprops/envy](https://github.com/softprops/envy) - deserialize env vars into typesafe structs [![Main](https://github.com/softprops/envy/actions/workflows/main.yml/badge.svg)](https://github.com/softprops/envy/actions/workflows/main.yml)

### Cryptography

[[crypto](https://crates.io/keywords/crypto), [cryptography](https://crates.io/keywords/cryptography)]

* [arkworks-rs/circom-compat](https://github.com/arkworks-rs/circom-compat) - Arkworks bindings to Circom's R1CS, for Groth16 Proof and Witness generation.
* [briansmith/ring](https://github.com/briansmith/ring) — Safe, fast, small crypto using Rust and BoringSSL's cryptography primitives.
* [briansmith/webpki](https://github.com/briansmith/webpki) — Web PKI TLS X.509 certificate validation.
* [conradkleinespel/rooster](https://github.com/conradkleinespel/rooster) [[rooster](https://crates.io/crates/rooster)] — Simple password manager to use in your terminal
* [cossacklabs/themis](https://github.com/cossacklabs/themis) [[themis](https://crates.io/crates/themis)] — a high-level cryptographic library for solving typical data security tasks, best fit for multi-platform apps. [![build badge](https://circleci.com/gh/cossacklabs/themis/tree/master.svg?style=shield)](https://app.circleci.com/pipelines/github/cossacklabs/themis)
* [DaGenix/rust-crypto](https://github.com/DaGenix/rust-crypto) — cryptographic algorithms
* [dalek-cryptography/curve25519-dalek](https://github.com/dalek-cryptography/curve25519-dalek) — Curve25519 operations
* [dalek-cryptography/ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) — Ed25519 digital signatures
* [dalek-cryptography/x25519-dalek](https://github.com/dalek-cryptography/x25519-dalek) — X25519 key exchange
* [debris/tiny-keccak](https://github.com/debris/tiny-keccak) — Keccak family (SHA3)
* [exonum/exonum](https://github.com/exonum/exonum) [[exonum](https://crates.io/crates/exonum)] — extensible framework for blockchain projects
* [facebook/opaque-ke](https://github.com/facebook/opaque-ke) — Implementation of the recent [OPAQUE](https://datatracker.ietf.org/doc/draft-krawczyk-cfrg-opaque/) password-authenticated key exchange. [![build badge](https://github.com/facebook/opaque-ke/workflows/Rust%20CI/badge.svg?branch=master)](https://github.com/facebook/opaque-ke)
* [iddm/randomorg](https://github.com/iddm/randomorg) - A random.org client library. [![Crates badge](https://img.shields.io/crates/v/randomorg.svg)](https://crates.io/crates/randomorg)
* [klutzy/suruga](https://github.com/klutzy/suruga) — Implementation of [TLS 1.2](https://datatracker.ietf.org/doc/html/rfc5246)
* [kornelski/rust-security-framework](https://github.com/kornelski/rust-security-framework) — Bindings for Security Framework (OSX native)
* [libOctavo/octavo](https://github.com/libOctavo/octavo) — Modular hash and crypto library
* [orion-rs/orion](https://github.com/orion-rs/orion) — This library aims to provide easy and usable crypto. 'Usable' meaning exposing high-level API's that are easy to use and hard to misuse. [![Tests](https://github.com/orion-rs/orion/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/orion-rs/orion/actions/workflows/test.yml)
* [racum/rust-djangohashers](https://github.com/racum/rust-djangohashers) [[djangohashers](https://crates.io/crates/djangohashers)] — Port of the password primitives used in the Django Project. It doesn't require Django, only hashes and validates passwords according to its style.
* [RustCrypto/hashes](https://github.com/RustCrypto/hashes) — Collection of cryptographic hash functions
* [rustls/rustls](https://github.com/rustls/rustls) — Implementation of TLS
* [sfackler/rust-native-tls](https://github.com/sfackler/rust-native-tls) — Bindings for native TLS libraries
* [sfackler/rust-openssl](https://github.com/sfackler/rust-openssl) — [OpenSSL](https://www.openssl.org/) bindings
* [sorairolake/abcrypt](https://github.com/sorairolake/abcrypt) [[abcrypt](https://crates.io/crates/abcrypt)] — A simple, modern and secure file encryption library. [![CI](https://github.com/sorairolake/abcrypt/workflows/CI/badge.svg?branch=develop)](https://github.com/sorairolake/abcrypt/actions?query=workflow%3ACI)
* [sorairolake/scryptenc-rs](https://github.com/sorairolake/scryptenc-rs) [[scryptenc](https://crates.io/crates/scryptenc)] — An implementation of the scrypt encrypted data format. [![CI](https://github.com/sorairolake/scryptenc-rs/workflows/CI/badge.svg?branch=develop)](https://github.com/sorairolake/scryptenc-rs/actions?query=workflow%3ACI)
* [w3f/schnorrkel](https://github.com/w3f/schnorrkel) - Schnorr VRFs and signatures on the Ristretto group

### Data processing

* [amv-dev/yata](https://github.com/amv-dev/yata) — high performance technical analysis library [![Build Status](https://img.shields.io/github/workflow/status/amv-dev/yata/Rust?branch=master)](https://github.com/amv-dev/yata/actions?query=workflow%3ARust)
* [bluss/ndarray](https://github.com/rust-ndarray/ndarray) — N-dimensional array with array views, multidimensional slicing, and efficient operations
* [kernelmachine/utah](https://github.com/kernelmachine/utah) — Dataframe structure and operations
* [pg_analytics](https://github.com/paradedb/paradedb/tree/dev/pg_analytics) - PostgreSQL extension that accelerates analytical query processing inside Postgres to a performance level comparable to dedicated OLAP databases.
* [pg_lakehouse](https://github.com/paradedb/paradedb/tree/dev/pg_lakehouse) - PostgreSQL extension that transforms Postgres into an analytical query engine over object stores like AWS S3/GCS and table formats like Delta Lake/Iceberg.
* [pola-rs/polars](https://github.com/pola-rs/polars) - Fast feature complete DataFrame library ![Build and test](https://github.com/pola-rs/polars/workflows/Build%20and%20test/badge.svg?branch=master)
* [weld-project/weld](https://github.com/weld-project/weld) — High-performance runtime for data analytics applications

### Data streaming

* [ArroyoSystems/arroyo](https://github.com/ArroyoSystems/arroyo) - High-performance real-time analytics in Rust and SQL [![CI](https://github.com/ArroyoSystems/arroyo/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/ArroyoSystems/arroyo/actions)
* [iggy-rs/iggy](https://github.com/iggy-rs/iggy) [[iggy](https://crates.io/crates/iggy)] — Persistent message streaming platform, supporting QUIC, TCP and HTTP transport protocols [![CI](https://github.com/iggy-rs/iggy/actions/workflows/test.yml/badge.svg)](https://github.com/iggy-rs/iggy/actions/workflows/test.yml)
* [infinyon/fluvio](https://github.com/infinyon/fluvio) - Programmable data streaming platform [![CI](https://github.com/infinyon/fluvio/workflows/CI/badge.svg?branch=stable)](https://github.com/infinyon/fluvio/actions)

### Data structures

* [ashvardanian/simsimd](https://github.com/ashvardanian/SimSIMD) - SIMD-accelerated vector distances and similarity functions for x86 AVX2 & AVX-512, and Arm NEON [![crates.io](https://img.shields.io/crates/v/simsimd.svg)](https://crates.io/crates/simsimd)
* [becheran/grid](https://github.com/becheran/grid) [[grid](https://crates.io/crates/grid)] —  Provide a two dimensional data structure that is easy to use and fast. [![build status](https://github.com/becheran/grid/actions/workflows/rust.yml/badge.svg)](https://github.com/becheran/grid/actions)
* [billyevans/tst](https://github.com/billyevans/tst) [[tst](https://crates.io/crates/tst)] — Ternary search tree collection
* [contain-rs](https://github.com/contain-rs) — Extension of Rust's std::collections
* [danielpclark/array_tool](https://github.com/danielpclark/array_tool) — Array helpers. Some of the most common methods you would use on Arrays made available on Vectors. Polymorphic implementations for handling most of your use cases.
* [fizyk20/generic-array](https://github.com/fizyk20/generic-array) – a hack to allow for arrays sized by typenums
* [garro95/priority-queue](https://github.com/garro95/priority-queue)[[priority-queue](https://crates.io/crates/priority-queue)] — A priority queue that implements priority changes.
* [greyblake/nutype](https://github.com/greyblake/nutype) [[nutype](https://crates.io/crates/nutype)] — define newtype structures with validation constraints. [![build status](https://github.com/greyblake/nutype/actions/workflows/ci.yml/badge.svg)](https://github.com/greyblake/nutype/actions)
* [mrhooray/kdtree-rs](https://github.com/mrhooray/kdtree-rs) — K-dimensional tree for fast geospatial indexing and nearest neighbors lookup
* [orium/rpds](https://github.com/orium/rpds) [[rpds](https://crates.io/crates/rpds)] — Persistent data structures. [![build badge](https://github.com/orium/rpds/workflows/CI/badge.svg)](https://github.com/orium/rpds/actions?query=workflow%3ACI)
* [RoaringBitmap/roaring-rs](https://github.com/RoaringBitmap/roaring-rs) – Roaring Bitmaps
* [rust-itertools/itertools](https://github.com/rust-itertools/itertools) — Extra iterator adaptors, functions and macros
* [tnballo/scapegoat](https://github.com/tnballo/scapegoat) [[scapegoat](https://crates.io/crates/scapegoat)] — Safe, fallible, stack-only alternative to `BTreeSet` and `BTreeMap`. [![GitHub Actions](https://github.com/tnballo/scapegoat/workflows/test/badge.svg?branch=master)](https://github.com/tnballo/scapegoat/actions)
* [xfix/enum-map](https://codeberg.org/xfix/enum-map) [[enum-map](https://crates.io/crates/enum-map)] — An optimized map implementation for enums using an array to store values.
* [yamafaktory/hypergraph](https://github.com/yamafaktory/hypergraph) [[hypergraph](https://crates.io/crates/hypergraph)] — Hypergraph is a data structure library to generate directed hypergraphs. [![ci](https://github.com/yamafaktory/hypergraph/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/yamafaktory/hypergraph/actions/workflows/ci.yml)

### Data visualization

* [blitzarx1/egui_graphs](https://github.com/blitzarx1/egui_graphs) - [[egui_graphs](https://crates.io/crates/egui_graphs)] - Interactive graph visualization widget powered by egui and petgraph. [![Crates.io](https://img.shields.io/crates/v/egui_graphs)](https://crates.io/crates/egui_graphs) [![docs.rs](https://img.shields.io/docsrs/egui_graphs)](https://docs.rs/egui_graphs)
* [djduque/pgfplots](https://github.com/djduque/pgfplots) [[pgfplots](https://crates.io/crates/pgfplots)] — Library to generate publication-quality figures. [![build](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml/badge.svg)](https://github.com/DJDuque/pgfplots/actions/workflows/rust.yml)
* [mazznoer/colorgrad-rs](https://github.com/mazznoer/colorgrad-rs) [[colorgrad](https://crates.io/crates/colorgrad)] — Color scales library for data visualization, charts, games, maps, generative art and others.
* [milliams/plotlib](https://github.com/milliams/plotlib)
* [plotly](https://github.com/plotly/plotly.rs) — Plotly for Rust.
* [plotpy](https://github.com/cpmech/plotpy) [[plotpy](https://crates.io/crates/plotpy)] — Rust plotting library using Python (Matplotlib)
* [plotters](https://github.com/plotters-rs/plotters) — [![build badge](https://github.com/plotters-rs/plotters/workflows/CI/badge.svg)](https://github.com/plotters-rs/plotters/actions)
* [rerun](https://github.com/rerun-io/rerun) — [[rerun](https://crates.io/crates/rerun)] — An SDK for logging computer vision and robotics data (tensors, point clouds, etc) paired with a visualizer for exploring that data over time.
* [saresend/gust](https://github.com/saresend/Gust) —

### Database

[[database](https://crates.io/keywords/database)]

* NoSQL [[nosql](https://crates.io/keywords/nosql)]

  * [ArangoDB](https://arangodb.com)
    * [Aragog](https://gitlab.com/qonfucius/aragog) [[aragog](https://crates.io/crates/aragog)] - A Lightweight ArangoDB Object document, relational and graph mapper [![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/-/commits/master)
    * [Arangors](https://github.com/fMeow/arangors) [[arangors](https://crates.io/crates/arangors)] - An ArangoDB driver
  * [Cassandra](https://cassandra.apache.org/_/index.html) [[cassandra](https://crates.io/keywords/cassandra), [cql](https://crates.io/keywords/cql)]
    * [AlexPikalov/cdrs](https://github.com/AlexPikalov/cdrs) [[cdrs](https://crates.io/crates/cdrs)] — native client
    * [krojew/cdrs-tokio](https://github.com/krojew/cdrs-tokio) [![build badge](https://github.com/krojew/cdrs-tokio/actions/workflows/rust.yml/badge.svg)](https://github.com/krojew/cdrs-tokio/actions)
      * [[cassandra-protocol](https://crates.io/crates/cassandra-protocol)] - Cassandra protocol implementation.
      * [[cdrs-tokio](https://crates.io/crates/cdrs-tokio)] - production-ready async Apache Cassandra driver
    * [Metaswitch/cassandra-rs](https://github.com/Metaswitch/cassandra-rs) —  bindings to the DataStax C/C++ client
  * CouchDB [[couchdb](https://crates.io/keywords/couchdb)]
    * [chill-rs/chill](https://github.com/chill-rs/chill) [[couchdb](https://crates.io/crates/chill)] — Client for the CouchDB REST API
  * [DynamoDB](https://aws.amazon.com/dynamodb/) [[dynamodb](https://crates.io/keywords/dynamodb)]
    * [softprops/dynomite](https://github.com/softprops/dynomite) - A library for strongly-typed and convenient interaction with `rusoto_dynamodb` [![build badge](https://github.com/softprops/dynomite/workflows/Main/badge.svg?branch=master)](https://github.com/softprops/dynomite/actions)
  * Elasticsearch [[elasticsearch](https://crates.io/keywords/elasticsearch)]
    * [benashford/rs-es](https://github.com/benashford/rs-es) [[rs-es](https://crates.io/crates/rs-es)] — Client for the [Elastic](https://www.elastic.co/) REST API
    * [elastic-rs/elastic](https://github.com/elastic-rs/elastic) [[elastic](https://crates.io/crates/elastic)] — elastic is an efficient, modular API client for Elasticsearch written in Rust [![build badge](https://ci.appveyor.com/api/projects/status/csa78tcumdpnbur2?svg=true)](https://ci.appveyor.com/project/KodrAus/elastic)
  * etcd
    * [jimmycuadra/rust-etcd](https://github.com/jimmycuadra/rust-etcd) [[etcd](https://crates.io/crates/etcd)] — A client library for CoreOS's etcd.
  * [InfluxDB](https://www.influxdata.com/)
    * [driftluo/InfluxDBClient-rs](https://github.com/driftluo/InfluxDBClient-rs) — Synchronization interface
  * LevelDB
    * [skade/leveldb](https://github.com/skade/leveldb) — [LevelDB](https://github.com/google/leveldb) bindings
  * LMDB [[lmdb](https://crates.io/keywords/lmdb)]
    * [vhbit/lmdb-rs](https://github.com/vhbit/lmdb-rs) [[lmdb-rs](https://crates.io/crates/lmdb-rs)] — [LMDB](https://www.symas.com/symas-embedded-database-lmdb) bindings
  * MongoDB [[mongodb](https://crates.io/keywords/mongodb)]
    * [mongodb/mongo-rust-driver](https://github.com/mongodb/mongo-rust-driver) [[mongodb](https://crates.io/crates/mongodb)] — [MongoDB](https://www.mongodb.com/) bindings
  * [PickleDB](https://pythonhosted.org/pickleDB/)
    * [seladb/pickledb-rs](https://github.com/seladb/pickledb-rs) — a lightweight and simple key-value store, heavily inspired by Python's PickleDB.
  * [PoloDB](https://www.polodb.org/)
    * [PoloDB](https://github.com/PoloDB/PoloDB) - An embedded JSON-based database has API similar to MongoDB. ![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PoloDB/PoloDB/rust.yml)
  * [Redb](https://www.redb.org/)
    * [Redb](https://github.com/cberner/redb) - An embedded key-value database. It provides a similar interface to other embedded key-value stores such as rocksdb and lmdb. ![GitHub Workflow Status](https://github.com/cberner/redb/actions/workflows/ci.yml/badge.svg)
  * Redis [[redis](https://crates.io/keywords/redis)]
    * [aembke/fred](https://github.com/aembke/fred.rs) [[fred](https://crates.io/crates/fred)] - A high level async [Redis](https://redis.io/) client for Rust with Tokio. [![CircleCI](https://circleci.com/gh/aembke/fred.rs/tree/main.svg?style=svg)]([https://circleci.com/gh/aembke/fred.rs/tree/main](https://app.circleci.com/pipelines/github/aembke/fred.rs?branch=main))
    * [redis-rs](https://github.com/redis-rs/redis-rs) — [Redis](https://redis.io/) library [![Rust](https://github.com/redis-rs/redis-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/redis-rs/redis-rs/actions/workflows/rust.yml)
  * [RocksDB](https://rocksdb.org/)
    * [rust-rocksdb/rust-rocksdb](https://github.com/rust-rocksdb/rust-rocksdb) — RocksDB bindings [![RocksDB CI](https://github.com/rust-rocksdb/rust-rocksdb/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/rust-rocksdb/rust-rocksdb/actions/workflows/rust.yml)
  * [SurrealDB](https://surrealdb.com/)
    * [surrealdb/surrealdb](https://github.com/surrealdb/surrealdb) — SurrealDB embedded document-graph database
  * [UnQLite](https://github.com/symisc/unqlite)
    * [zitsen/unqlite.rs](https://github.com/zitsen/unqlite.rs) — UnQLite bindings
  * [ZooKeeper](https://zookeeper.apache.org/)
    * [bonifaido/rust-zookeeper](https://github.com/bonifaido/rust-zookeeper) [[zookeeper](https://crates.io/crates/zookeeper)] — A client library for Apache ZooKeeper.
    * [krojew/rust-zookeeper](https://github.com/krojew/rust-zookeeper) [[zookeeper-async](https://crates.io/crates/zookeeper-async)] - Async Zookeeper client, based on tokio.  ![build status](https://github.com/krojew/rust-zookeeper/actions/workflows/rust.yml/badge.svg)
* OGM [[ogm](https://crates.io/keywords/ogm)]
    * [Aragog](https://gitlab.com/qonfucius/aragog) [[aragog](https://crates.io/crates/aragog)] - A Lightweight ArangoDB Object document, relational and graph mapper [![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/-/commits/master)
* ORM [[orm](https://crates.io/keywords/orm)]
  * [Brendonovich/prisma-client-rust](https://github.com/Brendonovich/prisma-client-rust) — An autogenerated query builder that provides simple and fully type-safe database access using the Prisma ecosystem. [![Test Status](https://img.shields.io/github/workflow/status/Brendonovich/prisma-client-rust/CI?label=tests&style=flat-square)](https://github.com/Brendonovich/prisma-client-rust/actions)
  * [diesel-rs/diesel](https://github.com/diesel-rs/diesel) — an ORM and Query builder
  * [ivanceras/rustorm](https://github.com/ivanceras/rustorm) — an ORM
  * [njord-rs/njord](https://github.com/njord-rs/njord) - ⛵ A lightweight ORM library for Rust [![build status](https://github.com/njord-rs/njord/actions/workflows/ci.yml/badge.svg)](https://github.com/njord-rs/njord/actions/workflows/ci.yml) ![crates.io](https://img.shields.io/crates/v/njord.svg)
  * [rbatis/rbatis](https://github.com/rbatis/rbatis) — ORM Framework High Performance(JSON based)
  * [SeaQL/sea-orm](https://github.com/SeaQL/sea-orm) — 🐚 An async & dynamic ORM  [![crate](https://img.shields.io/crates/v/sea-orm.svg)](https://crates.io/crates/sea-orm) [![docs](https://img.shields.io/docsrs/sea-orm/latest)](https://docs.rs/sea-orm) [![build status](https://github.com/SeaQL/sea-orm/actions/workflows/rust.yml/badge.svg)](https://github.com/SeaQL/sea-orm/actions/workflows/rust.yml)
  * [SeaQL/seaography](https://github.com/SeaQL/seaography) — 🧭 GraphQL framework for SeaORM [![crate](https://img.shields.io/crates/v/seaography.svg)](https://crates.io/crates/seaography) [![docs](https://img.shields.io/docsrs/seaography/latest)](https://docs.rs/seaography) [![build status](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml/badge.svg)](https://github.com/SeaQL/seaography/actions/workflows/tests.yaml)
* [sfackler/r2d2](https://github.com/sfackler/r2d2) — generic connection pool
* SQL [[sql](https://crates.io/keywords/sql)]
  * Generic
    * [launchbadge/sqlx](https://github.com/launchbadge/sqlx) - async PostgreSQL/MySQL/SQLite connection pool with strong typing support [![build badge](https://img.shields.io/github/workflow/status/launchbadge/sqlx/Rust/master?style=flat-square)](https://github.com/launchbadge/sqlx)
    * [SeaQL/sea-query](https://github.com/SeaQL/sea-query) - 🔱 A dynamic SQL query builder for MySQL, Postgres and SQLite [![crate](https://img.shields.io/crates/v/sea-query.svg)](https://crates.io/crates/sea-query) [![docs](https://img.shields.io/docsrs/sea-query/latest)](https://docs.rs/sea-query) [![build status](https://github.com/SeaQL/sea-query/actions/workflows/rust.yml/badge.svg)](https://github.com/SeaQL/sea-query/actions/workflows/rust.yml)
    * [SeaQL/sea-schema](https://github.com/SeaQL/sea-schema) - 🌿 SQL schema definition and discovery [![crate](https://img.shields.io/crates/v/sea-schema.svg)](https://crates.io/crates/sea-schema) [![docs](https://img.shields.io/docsrs/sea-schema/latest)](https://docs.rs/sea-schema) [![build status](https://github.com/SeaQL/sea-schema/actions/workflows/rust.yml/badge.svg)](https://github.com/SeaQL/sea-schema/actions/workflows/rust.yml)
  * Microsoft SQL
    * [prisma/tiberius](https://github.com/prisma/tiberius) — [![Cargo tests](https://github.com/prisma/tiberius/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/prisma/tiberius/actions/workflows/test.yml)
  * MySql [[mysql](https://crates.io/keywords/mysql)]
    * [AgilData/mysql-proxy-rs](https://github.com/AgilData/mysql-proxy-rs) — A MySQL Proxy [![CircleCI](https://circleci.com/gh/AgilData/mysql-proxy-rs/tree/master.svg?style=svg)](https://app.circleci.com/pipelines/github/AgilData/mysql-proxy-rs?branch=master)
    * [blackbeam/mysql_async](https://github.com/blackbeam/mysql_async) [[mysql_async](https://crates.io/crates/mysql_async)] — asynchronous Mysql driver based on Tokio. [![CircleCI](https://circleci.com/gh/blackbeam/mysql_async/tree/master.svg?style=shield)](https://app.circleci.com/pipelines/github/blackbeam/mysql_async?branch=master)
    * [blackbeam/rust-mysql-simple](https://github.com/blackbeam/rust-mysql-simple) [[mysql](https://crates.io/crates/mysql)] — A native MySql client
  * Oracle
    * [kubo/rust-oracle](https://github.com/kubo/rust-oracle) [[oracle](https://crates.io/crates/oracle)] — Oracle driver [![build badge](https://github.com/kubo/rust-oracle/actions/workflows/run-tests.yml/badge.svg?branch=master)](https://github.com/kubo/rust-oracle/actions/workflows/run-tests.yml)
  * PostgreSql [[postgres](https://crates.io/keywords/postgres), [postgresql](https://crates.io/keywords/postgresql)]
    * [sfackler/rust-postgres](https://github.com/sfackler/rust-postgres) [[postgres](https://crates.io/crates/postgres)] — A native [PostgreSQL](https://www.postgresql.org/) client
  * Sqlite [[sqlite](https://crates.io/keywords/sqlite)]
    * [rusqlite](https://github.com/rusqlite/rusqlite) — [Sqlite3](https://www.sqlite.org/index.html) bindings

### Date and time

[[date](https://crates.io/keywords/date), [time](https://crates.io/keywords/time)]

* [chronotope/chrono](https://github.com/chronotope/chrono) —
* [Mnwa/ms](https://github.com/Mnwa/ms) [[ms-converter](https://crates.io/crates/ms-converter)] — it's a library for converting human-like times to milliseconds [![build badge](https://github.com/Mnwa/ms/workflows/build/badge.svg?branch=master)](https://github.com/Mnwa/ms/actions?query=workflow%3Abuild)
* [sorairolake/nt-time](https://github.com/sorairolake/nt-time) [[nt-time](https://crates.io/crates/nt-time)] — A Windows file time library. [![CI](https://github.com/sorairolake/nt-time/workflows/CI/badge.svg?branch=develop)](https://github.com/sorairolake/nt-time/actions?query=workflow%3ACI)
* [time-rs/time](https://github.com/time-rs/time) — [![build badge](https://github.com/time-rs/time/workflows/Build/badge.svg)](https://github.com/time-rs/time/actions)

### Distributed systems

* Antimony
  * [antimonyproject/antimony](https://github.com/antimonyproject/antimony) [[antimony](https://crates.io/crates/antimony)] — stream processing / distributed computation platform
* Apache Kafka
  * [fede1024/rust-rdkafka](https://github.com/fede1024/rust-rdkafka) [[rdkafka](https://crates.io/crates/rdkafka)] — [librdkafka](https://github.com/confluentinc/librdkafka) bindings
  * [gklijs/schema_registry_converter](https://github.com/gklijs/schema_registry_converter) [[schema_registry_converter](https://crates.io/crates/schema_registry_converter)] — to integrate with [confluent schema registry](https://www.confluent.io/product/confluent-platform/data-compatibility/)
  * [kafka-rust/kafka-rust](https://github.com/kafka-rust/kafka-rust) —
* HDFS
  * [hyunsik/hdfs-rs](https://github.com/hyunsik/hdfs-rs) [[hdfs](https://crates.io/crates/hdfs)] — libhdfs bindings
* Other
  * [build-trust/ockam](https://github.com/build-trust/ockam) [[ockam](https://crates.io/crates/ockam)] - End-to-End Encryption, Mutual Authentication, and ABAC for distributed applications [![build badge](https://github.com/build-trust/ockam/workflows/Rust/badge.svg)](https://github.com/build-trust/ockam)

### Domain driven design

  * [serverlesstechnology/cqrs](https://github.com/serverlesstechnology/cqrs) [[cqrs-es](https://crates.io/crates/cqrs-es)] — A framework for CQRS and event sourcing with [user guide](https://doc.rust-cqrs.org/)

### eBPF

* [aya/aya-rs](https://github.com/aya-rs/aya) — Built with a focus on developer experience and operability.
* [libbpf/libbpf-rs](https://github.com/libbpf/libbpf-rs) — A minimal and opinionated eBPF tooling.

### Email

[[email](https://crates.io/keywords/email), [imap](https://crates.io/keywords/imap), [smtp](https://crates.io/keywords/smtp)]

* [duesee/imap-codec](https://github.com/duesee/imap-codec) [[imap-codec](https://crates.io/crates/imap-codec)] — Rock-solid and complete codec for IMAP [![Build & Test](https://github.com/duesee/imap-codec/actions/workflows/build_and_test.yml/badge.svg)](https://github.com/duesee/imap-codec/actions/workflows/build_and_test.yml)
* [gsquire/sendgrid-rs](https://github.com/gsquire/sendgrid-rs) — Library for SendGrid API
* [jdrouet/catapulte](https://github.com/jdrouet/catapulte) - A microservice to send emails using [MRML](https://github.com/jdrouet/mrml) templates.
* [jdrouet/jolimail](https://github.com/jdrouet/jolimail) - A web application to build [MRML](https://github.com/jdrouet/mrml) templates.
* [jdrouet/mrml](https://github.com/jdrouet/mrml) - A library to generate nice email templates working on any mail client.
* [lettre/lettre](https://github.com/lettre/lettre) — an SMTP-library [![CI](https://github.com/lettre/lettre/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/lettre/lettre/actions/workflows/test.yml)
* [mailtutan/mailtutan](https://github.com/mailtutan/mailtutan) An SMTP server for test and development environment.
* [meli/meli](https://github.com/meli/meli) - 🐝 terminal mail client
* [staktrace/mailparse](https://github.com/staktrace/mailparse) [[mailparse](https://crates.io/crates/mailparse)] — A library for parsing real-world email files
* [stalwartlabs/mail-auth](https://github.com/stalwartlabs/mail-auth) [[mail-auth](https://crates.io/crates/mail-auth)] - DKIM, ARC, SPF and DMARC message authentication library [![build badge](https://github.com/stalwartlabs/mail-auth/actions/workflows/rust.yml/badge.svg)](https://github.com/stalwartlabs/mail-auth/actions/workflows/rust.yml)
* [stalwartlabs/mail-parser](https://github.com/stalwartlabs/mail-parser) [[mail-parser](https://crates.io/crates/mail-parser)] - A fast and robust e-mail parsing library with full MIME support [![build badge](https://github.com/stalwartlabs/mail-parser/actions/workflows/rust.yml/badge.svg)](https://github.com/stalwartlabs/mail-parser/actions/workflows/rust.yml)
* [stalwartlabs/mail-send](https://github.com/stalwartlabs/mail-send) [[mail-send](https://crates.io/crates/mail-send)] - E-mail builder and SMTP client library with DKIM support [![build badge](https://github.com/stalwartlabs/mail-send/actions/workflows/rust.yml/badge.svg)](https://github.com/stalwartlabs/mail-send/actions/workflows/rust.yml)
* [tweedegolf/mailcrab](https://github.com/tweedegolf/mailcrab) — Email test server for development.

### Encoding

[[encoding](https://crates.io/keywords/encoding)]

* ASN.1
  * [alex/rust-asn1](https://github.com/alex/rust-asn1) — ASN.1 (DER) serializer
* Binary
  * [bincode-org/bincode](https://github.com/bincode-org/bincode) — A binary encoder/decoder [![CI](https://github.com/bincode-org/bincode/actions/workflows/rust.yml/badge.svg?branch=trunk)](https://github.com/bincode-org/bincode/actions/workflows/rust.yml)
  * [jamesmunns/postcard](https://github.com/jamesmunns/postcard) [[postcard](https://crates.io/crates/postcard)] — Postcard is a #![no_std] focused serializer and deserializer for Serde.
  * [m4b/goblin](https://github.com/m4b/goblin) [[goblin](https://crates.io/crates/goblin)] —  cross-platform, zero-copy, and endian-aware binary parsing
* BSON
  * [mongodb/bson-rust](https://github.com/mongodb/bson-rust) — Encoding and decoding support for BSON
* Byte swapping
  * [BurntSushi/byteorder](https://github.com/BurntSushi/byteorder) — Supports big-endian, little-endian and native byte orders
* Cap'n Proto
  * [capnproto/capnproto-rust](https://github.com/capnproto/capnproto-rust) —
* CBOR
  * [serde_cbor](https://crates.io/crates/serde_cbor) — CBOR support for serde
* Character Encoding
  * [hsivonen/encoding_rs](https://github.com/hsivonen/encoding_rs) [[encoding_rs](https://crates.io/crates/encoding_rs)] — A Gecko-oriented implementation of the Encoding Standard
  * [lifthrasiir/rust-encoding](https://github.com/lifthrasiir/rust-encoding) —
* CRC
  * [mrhooray/crc-rs](https://github.com/mrhooray/crc-rs) —
* CSV
  * [BurntSushi/rust-csv](https://github.com/BurntSushi/rust-csv) — A fast and flexible CSV reader and writer, with support for Serde
* EDN
  * [edn-rs](https://github.com/edn-rs/edn-rs) [[edn-rs](https://crates.io/crates/edn-rs)] — crate to parse and emit EDN format into Rust types.
* [FlatBuffers](https://flatbuffers.dev/)
  * [frol/flatc-rust](https://github.com/frol/flatc-rust) — FlatBuffers compiler (flatc) integration for Cargo build scripts
* HAR
  * [mandrean/har-rs](https://github.com/mandrean/har-rs) [[har](https://crates.io/crates/har)] — A HTTP Archive Format (HAR) serialization & deserialization library
* HTML
  * [servo/html5ever](https://github.com/servo/html5ever) — High-performance browser-grade HTML5 parser
* JSON
  * [importcjj/rust-ajson](https://github.com/importcjj/rust-ajson) [[ajson](https://crates.io/crates/ajson)] —  Get JSON values quickly
  * [maciejhirsz/json-rust](https://github.com/maciejhirsz/json-rust) [[json](https://crates.io/crates/json)] — JSON implementation
  * [pikkr/pikkr](https://github.com/pikkr/pikkr) [[pikkr](https://crates.io/crates/pikkr)] — JSON parser which picks up values directly without performing tokenization
  * [serde-rs/json](https://github.com/serde-rs/json) [[serde\_json](https://crates.io/crates/serde_json)] — JSON support for [Serde](https://github.com/serde-rs/serde) framework
  * [simd-lite/simd-json](https://github.com/simd-lite/simd-json) [[simd-json](https://crates.io/crates/simd-json)] — High performance JSON parser based on a port of simdjson
* MsgPack
  * [3Hren/msgpack-rust](https://github.com/3Hren/msgpack-rust) — Low/high level MessagePack implementation
* NetCDF
  * [georust/netcdf](https://github.com/georust/netcdf) [[netcdf](https://crates.io/crates/netcdf)] — Medium-level netCDF bindings, allowing easy reading and writing of array-like structures to a file.
* PEM
  * [jcreekmore/pem-rs](https://github.com/jcreekmore/pem-rs) [[pem](https://crates.io/crates/pem)] — Parse and encode PEM-encoded data
* ProtocolBuffers
  * [stepancheg/rust-protobuf](https://github.com/stepancheg/rust-protobuf) —
  * [tokio-rs/prost](https://github.com/tokio-rs/prost) — [![continuous integration](https://github.com/tokio-rs/prost/workflows/continuous%20integration/badge.svg?branch=master)](https://github.com/tokio-rs/prost/actions)
* rkyv
  * [rkyv/rkyv](https://github.com/rkyv/rkyv) [[rkyv](https://crates.io/crates/rkyv)] — rkyv (archive) is a zero-copy deserialization framework
* RON (Rusty Object Notation)
  * [https://github.com/ron-rs/ron](https://github.com/ron-rs/ron) —
* Serde
  * [iddm/serde-aux](https://github.com/iddm/serde-aux/) - additional tools for using with the serde library. [![CI](https://github.com/iddm/serde-aux/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/serde-aux/actions/workflows/ci.yml) [![Crates badge](https://img.shields.io/crates/v/serde-aux.svg)](https://crates.io/crates/serde-aux)
* TOML
  * [tamasfe/taplo](https://github.com/tamasfe/taplo) [[taplo](https://crates.io/crates/taplo)] — A TOML toolkit [![CI](https://github.com/tamasfe/taplo/workflows/Continuous%20integration/badge.svg)](https://github.com/tamasfe/taplo/actions?query=workflow%3A%22Continuous+integration%22)
  * [toml-rs/toml](https://github.com/toml-rs/toml) — [![CI](https://github.com/toml-rs/toml/actions/workflows/ci.yml/badge.svg)](https://github.com/toml-rs/toml/actions/workflows/ci.yml)
* XML
  * [Florob/RustyXML](https://github.com/Florob/RustyXML) — an XML parser
  * [media-io/yaserde](https://github.com/media-io/yaserde) — Yet Another Serializer/Deserializer specialized for XML
  * [netvl/xml-rs](https://github.com/netvl/xml-rs) — A streaming XML library
  * [shepmaster/sxd-document](https://github.com/shepmaster/sxd-document) — An XML library
  * [shepmaster/sxd-xpath](https://github.com/shepmaster/sxd-xpath) — An XPath library
  * [tafia/quick-xml](https://github.com/tafia/quick-xml) — High performance XML pull reader/writer
* YAML
  * [chyh1990/yaml-rust](https://github.com/chyh1990/yaml-rust) — The missing YAML 1.2 implementation.
  * [dtolnay/serde-yaml](https://github.com/dtolnay/serde-yaml) [[serde\_yaml](https://crates.io/crates/serde_yaml)] — YAML support for [Serde](https://github.com/serde-rs/serde) framework [![build](https://img.shields.io/github/workflow/status/dtolnay/serde-yaml/CI/master)](https://github.com/dtolnay/serde-yaml/actions?query=branch%3Amaster)
  * [vitiral/stfu8](https://github.com/vitiral/stfu8) [[stfu8](https://crates.io/crates/stfu8)] — Sorta Text Format in UTF-8

### Filesystem

[[filesystem](https://crates.io/keywords/filesystem)]
* Operations
  * [Camino](https://github.com/camino-rs/camino) [[camino](https://crates.io/crates/camino)] - Like Rust's std::path::Path, but UTF-8.
  * [ParthJadhav/Rust_Search](https://github.com/ParthJadhav/Rust_Search) [[rust_search](https://crates.io/crates/rust_search)] - Blazingly fast file search library.
  * [pop-os/dbus-udisks2](https://github.com/pop-os/dbus-udisks2) [[dbus-udisks2](https://crates.io/crates/dbus-udisks2)] - UDisks2 DBus API
  * [pop-os/sys-mount](https://github.com/pop-os/sys-mount) [[sys-mount](https://crates.io/crates/sys-mount)] — High level abstraction for the `mount` / `umount2` system calls.
  * [vitiral/path_abs](https://github.com/vitiral/path_abs) [[path_abs](https://crates.io/crates/path_abs)] — Absolute serializable path types and associated methods.
  * [webdesus/fs_extra](https://github.com/webdesus/fs_extra) — expanding opportunities standard library std::fs and std::io
* Temporary Files
  * [Stebalien/tempfile](https://github.com/Stebalien/tempfile) — temporary file library
  * [Stebalien/xattr](https://github.com/Stebalien/xattr) [[xattr](https://crates.io/crates/xattr)] — list and manipulate unix extended file attributes
  * [zboxfs/zbox](https://github.com/zboxfs/zbox) [[zbox](https://crates.io/crates/zbox)] — Zero-details, privacy-focused embeddable file system.

### Finance

* [avhz/RustQuant](https://github.com/avhz/RustQuant) [[RustQuant](https://crates.io/crates/RustQuant)] — A quantitative finance library. ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/avhz/RustQuant/build.yml)
* [d-e-s-o/apca](https://github.com/d-e-s-o/apca) [[apca](https://crates.io/crates/apca)] — Opinionated and comprehensive bindings to the [Alpaca API](https://alpaca.markets/) for stock trading and more. ![GitHub Workflow Status](https://github.com/d-e-s-o/apca/actions/workflows/test.yml/badge.svg?branch=main)

### Functional Programming

[[functional programming](https://crates.io/keywords/fp)]
* Prelude
  * [JasonShin/fp-core.rs](https://github.com/JasonShin/fp-core.rs) — A library for functional programming
  * [myrrlyn/tap](https://github.com/myrrlyn/tap) - Suffix-Position Pipeline Behavior

### Game development

See also [Are we game yet?](https://arewegameyet.rs)
* Allegro
  * [SiegeLord/RustAllegro](https://github.com/SiegeLord/RustAllegro) — [Allegro 5](https://liballeg.org/) bindings
* [Awesome Quads](https://github.com/ozkriff/awesome-quads) — A curated list of links to miniquad/macroquad-related code & resources
* [Awesome wgpu](https://github.com/rofrol/awesome-wgpu) — A curated list of wgpu code and resources
* bracket-lib (previously RLTK)
  * [bracket-lib](https://github.com/amethyst/bracket-lib) [[bracket-lib](https://crates.io/crates/bracket-lib)] - The Roguelike Toolkit (RLTK). [![Rust](https://github.com/amethyst/bracket-lib/actions/workflows/rust.yml/badge.svg)](https://github.com/amethyst/bracket-lib/actions/workflows/rust.yml)
* Challonge
  * [iddm/challonge-rs](https://github.com/iddm/challonge-rs) [[challonge](https://crates.io/crates/challonge)] — Client library for the Challonge REST API. Helps to organize tournaments. [![CI](https://github.com/iddm/challonge-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/challonge-rs/actions/workflows/ci.yml)
* Entity-Component Systems (ECS)
  * [amethyst/specs](https://github.com/amethyst/specs) — Specs Parallel ECS
  * [legion](https://github.com/amethyst/legion) — A feature rich high performance ECS library with minimal boilerplate [![build badge](https://github.com/amethyst/legion/workflows/CI/badge.svg?branch=master)](https://github.com/amethyst/legion/actions)
* Game Engines
  * [Bevy](https://github.com/bevyengine/bevy) is a refreshingly simple data-driven game engine. - [![Crates.io](https://img.shields.io/crates/v/bevy.svg)](https://crates.io/crates/bevy)
  [![Crates.io](https://img.shields.io/crates/d/bevy.svg)](https://crates.io/crates/bevy)
  * [Fyrox](https://fyrox.rs/) — Game engine 3D [![Crates.io](https://img.shields.io/crates/v/fyrox.svg)](https://crates.io/crates/fyrox) [![license](https://img.shields.io/crates/l/fyrox.svg)](https://github.com/FyroxEngine/Fyrox/blob/master/LICENSE.md) [![Crates.io](https://img.shields.io/crates/d/fyrox.svg)](https://crates.io/crates/fyrox)
  * [ggez](https://github.com/ggez/ggez) — A lightweight game framework for making 2D games with minimum friction - [![Crates.io](https://img.shields.io/crates/v/ggez.svg)](https://crates.io/crates/ggez) [![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/ggez/ggez/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/d/ggez.svg)](https://crates.io/crates/ggez)
  * [Kiss3d](http://kiss3d.org) — A Keep It Simple, Stupid 3d graphics engine [![Crates.io](https://img.shields.io/crates/d/kiss3d.svg)](https://crates.io/crates/kiss3d)
  * [oxidator](https://github.com/Ruddle/oxidator) — A real time strategy game/engine supporting WebGPU
  * [Piston](https://www.piston.rs/) — [![Crates.io](https://img.shields.io/crates/v/piston.svg?style=flat-square)](https://crates.io/crates/piston) [![Crates.io](https://img.shields.io/crates/l/piston.svg)](https://github.com/PistonDevelopers/piston/blob/master/LICENSE) [![Crates.io](https://img.shields.io/crates/d/piston.svg)](https://crates.io/crates/piston)
  * [Unrust](https://github.com/unrust/unrust) — Webgl 2.0 / native game engine
* [Godot](https://godotengine.org/)
  * [godot-rust/gdnative](https://github.com/godot-rust/gdnative) [[gdnative](https://crates.io/crates/gdnative)] - Bindings to the Godot game engine [![CI](https://github.com/godot-rust/gdnative/actions/workflows/full-ci.yml/badge.svg)](https://github.com/godot-rust/gdnative/actions/workflows/full-ci.yml)
* [Raylib](https://www.raylib.com/)
  * [deltaphc/raylib-rs](https://github.com/deltaphc/raylib-rs) [[raylib](https://crates.io/crates/raylib)] — Bindings for raylib
* [SDL](http://www.libsdl.org/) [[sdl](https://crates.io/keywords/sdl)]
  * [brson/rust-sdl](https://github.com/brson/rust-sdl) — SDL1 bindings
  * [Rust-SDL2/rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) — SDL2 bindings
* SFML
  * [jeremyletang/rust-sfml](https://github.com/jeremyletang/rust-sfml) — [SFML](https://www.sfml-dev.org/) bindings
* Skillratings
  * [atomflunder/skillratings](https://github.com/atomflunder/skillratings) [[skillratings](https://crates.io/crates/skillratings)] - Collection of skill rating algorithms for multiplayer games like Elo, Glicko-2, TrueSkill etc. [![crates.io badge](https://img.shields.io/crates/v/skillratings)](https://crates.io/crates/skillratings) [![CI](https://github.com/atomflunder/skillratings/actions/workflows/ci.yml/badge.svg)](https://github.com/atomflunder/skillratings/actions/workflows/ci.yml)
* Tcod-rs
  * [tomassedovic/tcod-rs](https://github.com/tomassedovic/tcod-rs) — Libtcod bindings.
  * Warning: Not maintained anymore
* Toornament-rs
  * [iddm/toornament-rs](https://github.com/iddm/toornament-rs) - Toornament.com API bindings. [![CI](https://github.com/iddm/toornament-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/toornament-rs/actions/workflows/ci.yml) [![Crates badge](https://img.shields.io/crates/v/toornament.svg)](https://crates.io/crates/toornament)
* Victorem
  * [VictoremWinbringer/Victorem](https://github.com/VictoremWinbringer/Victorem) [[Victorem](https://crates.io/crates/Victorem)] — Easy UDP Game Server and UDP Client framework for creating simple 2D and 3D online game prototype

### Geospatial

[[geo](https://crates.io/keywords/geo), [gis](https://crates.io/keywords/gis)]

* [DaveKram/coord_transforms](https://github.com/DaveKram/coord_transforms) [[coord_transforms](https://crates.io/crates/coord_transforms)] — coordinate transformations (2-d, 3-d, and geospatial)
* [Georust](https://github.com/georust) — geospatial tools and libraries written
* [MapLibre/Martin](https://github.com/maplibre/martin) — Map tile server with PostGIS, MBTiles, PMTiles, and sprites support. [![CI build](https://github.com/maplibre/martin/actions/workflows/ci.yml/badge.svg)](https://github.com/maplibre/martin/actions)[![crates.io version](https://img.shields.io/crates/v/martin.svg)](https://crates.io/crates/martin)[![Book](https://img.shields.io/badge/docs-Book-informational)](https://maplibre.org/martin/)
* [rust-reverse-geocoder](https://github.com/gx0r/rrgeo) — A fast, offline reverse geocoder, inspired by [thampiman/reverse-geocoder](https://github.com/thampiman/reverse-geocoder)
* [vlopes11/geomorph](https://github.com/vlopes11/geomorph) [[geomorph](https://crates.io/crates/geomorph)] — conversion between UTM, LatLon and MGRS coordinates

### Graph algorithms

* [neo4j-labs/graph](https://github.com/neo4j-labs/graph) - A library for high-performant graph algorithms [![graph CI status](https://img.shields.io/github/workflow/status/neo4j-labs/graph/CI/main?label=CI)](https://github.com/neo4j-labs/graph/actions/workflows/rust.yml)
* [petgraph/petgraph](https://github.com/petgraph/petgraph) - Graph data structure library. [![graph CI status](https://github.com/petgraph/petgraph/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/petgraph/petgraph/actions/workflows/ci.yml)

### Graphics

[[graphics](https://crates.io/keywords/graphics)]

* Font
  * [RazrFalcon/rustybuzz](https://github.com/RazrFalcon/rustybuzz) - An incremental harfbuzz port
  * [redox-os/rusttype](https://github.com/redox-os/rusttype) — Alternative to libraries like FreeType
* [gfx-rs/gfx](https://github.com/gfx-rs/gfx) — A high-performance, bindless graphics API.
* [gfx-rs/wgpu](https://github.com/gfx-rs/wgpu) - Native WebGPU implementation based on gfx-hal. [![build badge](https://github.com/gfx-rs/wgpu/workflows/CI/badge.svg?branch=master)](https://github.com/gfx-rs/wgpu/actions)
* OpenGL [[opengl](https://crates.io/keywords/opengl)]
  * [brendanzab/gl-rs](https://github.com/brendanzab/gl-rs) —
  * [glium/glium](https://github.com/glium/glium) — safe OpenGL wrapper.
  * [glutin](https://crates.io/crates/glutin) — Alternative to [GLFW](https://www.glfw.org/)
  * [Kiss3d](http://kiss3d.org) — draw simple geometric figures and play with them with one-liners
  * [PistonDevelopers/glfw-rs](https://github.com/PistonDevelopers/glfw-rs) —
* PDF
  * [bastibense/libharu_ng](https://github.com/bastibense/libharu_ng) [[libharu_ng](https://crates.io/crates/libharu_ng)] - Easily generate PDFs from your Rust app.
  * [fschutt/printpdf](https://github.com/fschutt/printpdf) — PDF writing library
  * [J-F-Liu/lopdf](https://github.com/J-F-Liu/lopdf) — PDF document manipulation
  * [kaj/rust-pdf](https://github.com/kaj/rust-pdf) —
* [Vulkan](https://www.vulkan.org/) [[vulkan](https://crates.io/keywords/vulkan)]
  * [erupt](https://gitlab.com/Friz64/erupt) [[erupt](https://crates.io/crates/erupt)] — [![build badge](https://gitlab.com/Friz64/erupt/badges/main/pipeline.svg)](https://gitlab.com/Friz64/erupt/-/pipelines)
  * [vulkano](https://github.com/vulkano-rs/vulkano) [[vulkano](https://crates.io/crates/vulkano)] —

### GUI

[[gui](https://crates.io/keywords/gui)]

* [autopilot-rs/autopilot-rs](https://github.com/autopilot-rs/autopilot-rs) — A simple, cross-platform GUI automation library.
* Cocoa
  * [servo/core-foundation-rs](https://github.com/servo/core-foundation-rs) —
* [DioxusLabs/dioxus](https://github.com/dioxuslabs/dioxus) - a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. ![rust ci](https://github.com/dioxuslabs/dioxus/actions/workflows/main.yml/badge.svg)
* [emilk/egui](https://github.com/emilk/egui) - Simple, fast, and highly portable immediate mode GUI library. egui runs on the web, natively, and in your favorite game engine. [![Build Status](https://github.com/emilk/egui/workflows/CI/badge.svg)](https://github.com/emilk/egui/actions?workflow=CI)
* [emoon/rust_minifb](https://github.com/emoon/rust_minifb) — minifb is a cross-platform window setup with optional bitmap rendering. It also comes with easy mouse and keyboard input. Primarily designed for prototyping
* [FLTK](https://www.fltk.org/)
  * [fltk-rs](https://github.com/fltk-rs/fltk-rs) — FLTK bindings [![Build](https://github.com/fltk-rs/fltk-rs/workflows/Build/badge.svg?branch=master)](https://github.com/fltk-rs/fltk-rs/actions)
* [Flutter](https://flutter.dev/)
  * [cunarist/rinf](https://github.com/cunarist/rinf) — Rust as your Flutter backend, Flutter as your Rust frontend [![Build Test](https://github.com/cunarist/rinf/actions/workflows/build_test.yaml/badge.svg)](https://github.com/cunarist/rinf/actions/workflows/build_test.yaml?query=branch%3Amain)
  * [flutter-rs](https://github.com/flutter-rs/flutter-rs) — Build flutter desktop app in dart & rust.
  * [fzyzcjy/flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge) — High-level memory-safe binding generator for Flutter/Dart <-> Rust
* [fschutt/azul](https://github.com/fschutt/azul) — A free, functional, IMGUI-oriented GUI framework for rapid development of desktop applications written in Rust, supported by the Mozilla WebRender rendering engine.
* [GTK+](https://www.gtk.org/) [[gtk](https://crates.io/keywords/gtk)]
  * [gtk-rs/gtk4-rs](https://github.com/gtk-rs/gtk4-rs) - GTK4 binding ![CI](https://github.com/gtk-rs/gtk4-rs/workflows/CI/badge.svg)
  * [relm](https://github.com/antoyo/relm) — Asynchronous, GTK+-based, GUI library, inspired by Elm
* [iced-rs/iced](https://github.com/iced-rs/iced) [[iced](https://crates.io/crates/iced)] — A cross-platform GUI library, focused on simplicity and type-safety. Inspired by Elm.
* [ImGui](https://github.com/ocornut/imgui)
  * [imgui-rs](https://github.com/imgui-rs/imgui-rs) — Bindings for ImGui [![Build Status](https://github.com/imgui-rs/imgui-rs/workflows/ci/badge.svg?branch=master)](https://github.com/imgui-rs/imgui-rs/actions)
* [IUP](http://webserver2.tecgraf.puc-rio.br/iup/)
  * [Kiss-ui](https://github.com/KISS-UI/kiss-ui) — A simple UI framework built on IUP
* [ivanceras/sauron-native](https://github.com/ivanceras/sauron-native) - A truly native and cross platform GUI library. One unified code can be run as native GUI, Html Web and TUI.
* [libui](https://github.com/andlabs/libui)
  * [rust-native-ui/libui-rs](https://github.com/rust-native-ui/libui-rs) — libui bindings.
* [makepad/makepad](https://github.com/makepad/makepad) [[makepad-widgets](https://crates.io/crates/makepad-widgets)] — Makepad is a creative software development platform that compiles to wasm/webGL, osx/metal, windows/dx11 linux/opengl.
* [Nuklear](https://github.com/Immediate-Mode-UI/Nuklear)
  * [nuklear-rust](https://github.com/snuk182/nuklear-rust) — Bindings for Nuklear
* [OrbTk](https://github.com/redox-os/orbtk) — The Orbital Widget Toolkit is a multi platform (G)UI toolkit using SDL2 [![Build and test](https://github.com/redox-os/orbtk/workflows/build/badge.svg?branch=develop)](https://github.com/redox-os/orbtk/actions)
* [PistonDevelopers/conrod](https://github.com/PistonDevelopers/conrod/) — An easy-to-use, immediate-mode, 2D GUI library
* [Qt](https://doc.qt.io)
  * [cyndis/qmlrs](https://github.com/cyndis/qmlrs) — QtQuick bindings
  * [rust-qt](https://github.com/rust-qt)
  * [woboq/qmetaobject-rs](https://github.com/woboq/qmetaobject-rs) — Integrate Qml and Rust by building the QMetaObject at compile time.
* [rise-ui](https://github.com/rise-ui/rise) — Simple component-based cross-Platform GUI Toolkit for developing beautiful and user-friendly interfaces.
* [saurvs/nfd-rs](https://github.com/saurvs/nfd-rs) — [nativefiledialog](https://github.com/mlabbe/nativefiledialog) bindings
* [Sciter](https://sciter.com/)
  * [sciter-sdk/rust-sciter](https://github.com/sciter-sdk/rust-sciter) — Sciter bindings [![build badge](https://ci.appveyor.com/api/projects/status/github/sciter-sdk/rust-sciter?svg=true)](https://ci.appveyor.com/project/sciter-sdk/rust-sciter)
* [slint-ui/slint](https://github.com/slint-ui/slint) [slint](https://crates.io/crates/slint) — [Slint](https://slint.dev/) is a toolkit to efficiently develop fluid graphical user interfaces for embedded devices and desktop applications. [![Build Status](https://github.com/slint-ui/slint/workflows/CI/badge.svg?branch=master)](https://github.com/slint-ui/slint/actions?query=workflow%3ACI)
* [tauri-apps/tauri](https://github.com/tauri-apps/tauri) — Build smaller, faster, and more secure desktop applications with a web frontend, powered by [WRY](https://github.com/tauri-apps/wry). [![test library](https://img.shields.io/github/workflow/status/tauri-apps/tauri/test%20library?label=test%20library)](https://github.com/tauri-apps/tauri/actions?query=workflow%3A%22test+library%22)
* [tauri-apps/wry](https://github.com/tauri-apps/wry) - Webview Rendering librarY.
* [xilem](https://github.com/linebender/xilem) — Successor of the data-first UI design toolkit [druid](https://github.com/linebender/druid).

### Image processing

* [abonander/img_hash](https://github.com/abonander/img_hash) — Perceptual image hashing and comparison for equality and similarity.
* [image-rs/image](https://github.com/image-rs/image) — Basic imaging processing functions and methods for converting to and from image formats
* [image-rs/imageproc](https://github.com/image-rs/imageproc) — An image processing library, based on the `image` library.
* [marekm4/dominant_color](https://github.com/marekm4/dominant_color) [[dominant_color](https://crates.io/crates/dominant_color)] — Dominant color extractor ![build badge](https://github.com/marekm4/dominant_color/actions/workflows/rust.yml/badge.svg?branch=master)
* [rust-cv/cv](https://github.com/rust-cv/cv) — Implement computer vision algorithms, abstractions, and systems. `#[no_std]` is supported where possible. ![build badge](https://github.com/rust-cv/cv/workflows/tests/badge.svg)
* [teovoinea/steganography](https://github.com/teovoinea/steganography) [[steganography](https://crates.io/crates/steganography)] — A simple steganography library
* [twistedfall/opencv-rust](https://github.com/twistedfall/opencv-rust) — Bindings for OpenCV

### Language specification

* [shnewto/bnf](https://github.com/shnewto/bnf) — A library for parsing Backus–Naur form context-free grammars.

### Logging

[[log](https://crates.io/keywords/log)]

* [estk/log4rs](https://github.com/estk/log4rs) — highly configurable logging framework modeled after Java's Logback and log4j libraries [![CircleCI](https://circleci.com/gh/estk/log4rs.svg?style=shield)](https://app.circleci.com/pipelines/github/estk/log4rs)
* [rbatis/fast_log](https://github.com/rbatis/fast_log) — Async log High-performance asynchronous logging
* [rust-lang/log](https://github.com/rust-lang/log) — Logging implementation
* [seanmonstar/pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger) — A pretty, easy-to-use logger.
* [slog-rs/slog](https://github.com/slog-rs/slog) — Structured, composable logging
* [tokio-rs/tracing](https://github.com/tokio-rs/tracing) — An application level tracing framework for async-aware structured logging, error handling, metrics, and more [![Build Status](https://github.com/tokio-rs/tracing/workflows/CI/badge.svg?branch=master)](https://github.com/tokio-rs/tracing/actions?query=workflow%3ACI)

### Macro

* cute
  * [mattgathu/cute](https://github.com/mattgathu/cute) — Macro for Python-esque list comprehensions.
* [Linq-in-Rust](https://github.com/StardustDL/Linq-in-Rust) - Macro and methods for C#-LINQ-like expressions. [![CI](https://github.com/StardustDL/Linq-in-Rust/workflows/CI/badge.svg?branch=master)](https://github.com/StardustDL/Linq-in-Rust/actions?query=workflow%3ACI)

### Markup language

* CommonMark
  * [pulldown-cmark/pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) — [CommonMark](https://commonmark.org/) parser
* [insomnimus/tidier](https://github.com/insomnimus/tidier) [[tidier](https://crates.io/crates/tidier)] - A library to format HTML, XHTML and XML documents. [![build badge](https://github.com/insomnimus/tidier/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/insomnimus/tidier/actions)

### Mobile

* Android / iOS
  * [ivnsch/rust_android_ios](https://github.com/ivnsch/rust_android_ios) — An example of using a shared lib for Android and iOS using rust-swig and cbindgen respectively.
* Generic
  * [Geal/rust_on_mobile](https://github.com/Geal/rust_on_mobile)
  * [redbadger/crux](https://github.com/redbadger/crux) [[crux_core](https://crates.io/crates/crux_core)] — Cross-platform app development. Crux helps you share your app's business logic and behavior across mobile (iOS/Android) and web — as a single reusable core. [![Build status](https://img.shields.io/github/actions/workflow/status/redbadger/crux/build.yaml)](https://github.com/redbadger/crux/actions)
* iOS
  * [TimNN/cargo-lipo](https://github.com/TimNN/cargo-lipo) — A cargo lipo subcommand which automatically creates a universal library for use with your iOS application.

### Network programming

* Bluetooth
  * [bluez/bluer](https://github.com/bluez/bluer) [[bluer](https://crates.io/crates/bluer)] — Official BlueZ bindings. [![build badge](https://github.com/bluez/bluer/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/bluez/bluer/actions/workflows/rust.yml)
* CoAP
  * [Covertness/coap-rs](https://github.com/Covertness/coap-rs) — A [Constrained Application Protocol(CoAP)](https://datatracker.ietf.org/doc/html/rfc7252) library.
* Docker
  * [fussybeaver/bollard](https://github.com/fussybeaver/bollard) — Docker daemon API
* FTP
  * [mattnenterprise/rust-ftp](https://github.com/mattnenterprise/rust-ftp) — an [FTP](https://en.wikipedia.org/wiki/File_Transfer_Protocol) client
* gRPC
  * [hyperium/tonic](https://github.com/hyperium/tonic) — A native gRPC client & server implementation with async/await support [![Crates.io](https://img.shields.io/crates/v/tonic)](https://crates.io/crates/tonic)
  * [tikv/grpc-rs](https://github.com/tikv/grpc-rs) — The gRPC library built on C Core library and futures
* HTTP
  * [Hurl](https://github.com/Orange-OpenSource/hurl) — Run and test HTTP requests with plain text and libcurl [![CI](https://github.com/Orange-OpenSource/hurl/workflows/CI/badge.svg)](https://github.com/Orange-OpenSource/hurl/actions)
* IPNetwork
  * [achanda/ipnetwork](https://github.com/achanda/ipnetwork) — A library to work with IP networks
  * [candrew/netsim](https://github.com/canndrew/netsim) — A library for network simulation and testing
* Low level
  * [actix/actix](https://github.com/actix/actix) — Actor library
  * [dylanmckay/protocol](https://github.com/dylanmckay/protocol) — Custom TCP/UDP protocol definitions
  * [libpnet/libpnet](https://github.com/libpnet/libpnet) — A cross-platform, low level networking
  * [smoltcp-rs/smoltcp](https://github.com/smoltcp-rs/smoltcp) — A standalone, event-driven TCP/IP stack that is designed for bare-metal, real-time systems
  * [tokio-rs/tokio](https://github.com/tokio-rs/tokio) — A network application framework for rapid development and highly scalable production deployments of clients and servers.
* message-io
  * [lemunozm/message-io](https://github.com/lemunozm/message-io) — Event-driven message library to build network applications easy and fast. Supports TCP, UDP and WebSockets. [![build badge](https://img.shields.io/github/workflow/status/lemunozm/message-io/message-io%20ci)](https://github.com/lemunozm/message-io/actions?query=workflow%3A%22message-io+ci%22)
* MQTT
  * [bytebeamio/rumqtt](https://github.com/bytebeamio/rumqtt) - A library for developers to build applications that communicate with the [MQTT protocol](https://mqtt.org) over TCP and WebSockets, with or without TLS. [![Build and Test](https://github.com/bytebeamio/rumqtt/actions/workflows/build.yml/badge.svg)](https://github.com/bytebeamio/rumqtt/actions/workflows/build.yml)
* NanoMsg
  * [thehydroimpulse/nanomsg.rs](https://github.com/thehydroimpulse/nanomsg.rs) — [nanomsg](https://nanomsg.org/) bindings
* NATS
  * [nats-io/nats.rs](https://github.com/nats-io/nats.rs) — Client for NATS, the cloud native messaging system. [![Build Status](https://github.com/nats-io/nats.rs/workflows/Rust/badge.svg?branch=master)](https://github.com/nats-io/nats.rs/actions)
* Nng
  * [neachdainn/nng-rs](https://gitlab.com/neachdainn/nng-rs) [[Nng](https://crates.io/crates/nng)] — [Nng (nanomsg v2)](https://nng.nanomsg.org/index.html) bindings [![build badge](https://gitlab.com/neachdainn/nng-rs/badges/master/pipeline.svg)](https://gitlab.com/neachdainn/nng-rs/-/pipelines)
* NNTP
  * [mattnenterprise/rust-nntp](https://github.com/mattnenterprise/rust-nntp) [[nntp](https://crates.io/crates/nntp)] — an [NNTP](https://en.wikipedia.org/wiki/Network_News_Transfer_Protocol) client
* P2P
  * [libp2p/rust-libp2p](https://github.com/libp2p/rust-libp2p) — Implementation of libp2p networking stack. [![Circle CI](https://circleci.com/gh/libp2p/rust-libp2p.svg?style=svg)](https://app.circleci.com/pipelines/github/libp2p/rust-libp2p)
* POP3
  * [mattnenterprise/rust-pop3](https://github.com/mattnenterprise/rust-pop3) [[pop3](https://crates.io/crates/pop3)] — A [POP3](https://en.wikipedia.org/wiki/Post_Office_Protocol) client
* QUIC
  * [aws/s2n-quic](https://github.com/aws/s2n-quic) - An implementation of the IETF QUIC protocol ![ci](https://img.shields.io/github/actions/workflow/status/aws/s2n-quic/ci.yml?branch=main)
  * [cloudflare/quiche](https://github.com/cloudflare/quiche) — cloudflare implementation of the QUIC transport protocol and HTTP/3 ![build](https://img.shields.io/github/actions/workflow/status/cloudflare/quiche/stable.yml?branch=master)
  * [mozilla/neqo](https://github.com/mozilla/neqo) — an Implementation of QUIC
  * [quinn-rs/quinn](https://github.com/quinn-rs/quinn) — Futures-based QUIC implementation [![build badge](https://dev.azure.com/dochtman/Projects/_apis/build/status/Quinn?branchName=master)](https://dev.azure.com/dochtman/Projects/_build)
  * [tencent/tquic](https://github.com/Tencent/tquic) - A high-performance, lightweight, and cross-platform QUIC library [![Build Status](https://img.shields.io/github/actions/workflow/status/tencent/tquic/rust.yml)](https://github.com/Tencent/tquic/actions/workflows/rust.yml)
* Raknet
  * [b23r0/rust-raknet](https://github.com/b23r0/rust-raknet) — RakNet Protocol implementation [![Build Status](https://img.shields.io/github/workflow/status/b23r0/rust-raknet/Rust)](https://github.com/b23r0/rust-raknet/actions/workflows/rust.yml)
* RPC
  * [ENQT-GmbH/remoc](https://github.com/ENQT-GmbH/remoc) [[remoc](https://crates.io/crates/remoc)] - Remoc provides channels (broadcast, mpsc, oneshot, watch) similar to Tokio's and trait calling over any remote transport. [![build badge](https://github.com/ENQT-GmbH/remoc/actions/workflows/rust.yml/badge.svg?branch=master)](https://github.com/ENQT-GmbH/remoc/actions/workflows/rust.yml)
  * [smallnest/rpcx-rs](https://github.com/smallnest/rpcx-rs) — A RPC library for developing microservices in easy and simple way.
* Socket.io
  * [1c3t3a/rust-socketio](https://github.com/1c3t3a/rust-socketio) [[rust_socketio](https://crates.io/crates/rust_socketio)] — an implementation of a [socket.io](https://socket.io) client written in Rust. [![build badge](https://github.com/1c3t3a/rust-socketio/actions/workflows/build.yml/badge.svg)](https://github.com/1c3t3a/rust-socketio/actions/workflows/build.yml)
* SSH
  * [alexcrichton/ssh2-rs](https://github.com/alexcrichton/ssh2-rs) — [libssh2](https://libssh2.org/) bindings
  * [Thrussh](https://pijul.org/thrussh) [[thrussh](https://crates.io/crates/thrussh)] — an SSH library, backed by [libsodium](https://doc.libsodium.org/)
* Stomp
  * [zslayton/stomp-rs](https://github.com/zslayton/stomp-rs) — A [STOMP 1.2](http://stomp.github.io/stomp-specification-1.2.html) client implementation
* VPN
  * [defguard/wireguard-rs](https://github.com/DefGuard/wireguard-rs) — A multi-platform library providing a unified high-level API for managing WireGuard interfaces using native OS kernel and userspace WireGuard protocol implementations
* ZeroMQ
  * [erickt/rust-zmq](https://github.com/erickt/rust-zmq) — [ZeroMQ](https://zeromq.org/) bindings

### Parsing

  * [comex/rust-shlex](https://github.com/comex/rust-shlex) [[shlex](https://crates.io/crates/shlex)] — Split a string into shell words, like Python's shlex. [![build badge](https://github.com/comex/rust-shlex/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/comex/rust-shlex/actions/workflows/test.yml)
  * [Folyd/robotstxt](https://github.com/Folyd/robotstxt) - Port of Google's robots.txt parser and matcher C++ library
  * [freestrings/jsonpath](https://github.com/freestrings/jsonpath) — [JsonPath](https://goessner.net/articles/JsonPath/) engine. Webassembly and Javascript support too
  * [hmeyer/stl_io](https://crates.io/crates/stl_io) - A parser for STL (STereoLithography) files
  * [igumnoff/shiva](https://github.com/igumnoff/shiva) - Shiva library: Implementation in Rust of a parser and generator for documents of any type (Plain text, Markdown, HTML, PDF and etc)
  * [kevinmehall/rust-peg](https://github.com/kevinmehall/rust-peg) — Parsing Expression Grammar (PEG) parser generator
  * [lalrpop/lalrpop](https://github.com/lalrpop/lalrpop) — LR(1) parser generator
  * [m4rw3r/chomp](https://github.com/m4rw3r/chomp) – A fast monadic-style parser combinator
  * [Marwes/combine](https://github.com/Marwes/combine) — parser combinator library
  * [nrc/zero](https://github.com/nrc/zero) [[zero](https://crates.io/crates/zero/)] — zero-allocation parsing of binary data
  * [pest-parser/pest](https://github.com/pest-parser/pest) — The Elegant Parser
  * [ptal/oak](https://github.com/ptal/oak) — A typed PEG parser generator (compiler plugin)
  * [replicadse/wavefront_rs](https://github.com/replicadse/wavefront_rs) — A parser for the Wavefront OBJ format. [![crates.io](https://img.shields.io/crates/v/wavefront_rs.svg)](https://crates.io/crates/wavefront_rs) [![crates.io](https://img.shields.io/crates/d/wavefront_rs?label=crates.io%20downloads)](https://crates.io/crates/wavefront_rs) [![build badge](https://github.com/replicadse/wavefront_rs/workflows/pipeline/badge.svg?branch=master)](https://github.com/replicadse/wavefront_rs/actions)
  * [rust-bakery/nom](https://github.com/rust-bakery/nom) — parser combinator library
  * [s-panferov/queryst](https://github.com/s-panferov/queryst) — A query string parsing library inspired by [gs](https://github.com/ljharb/qs#readme)
  * [softdevteam/grmtools](https://github.com/softdevteam/grmtools/) - A LR parser with better error correction
  * [tree-sitter/tree-sitter](https://github.com/tree-sitter/tree-sitter) - A parser generator tool and an incremental parsing library geared towards programming tools

### Peripherals

* Fingerprint reader
  * [alvaroparker/libfprint-rs](https://github.com/alvaroparker/libfprint-rs) [[libfprint-rs](https://crates.io/crates/libfprint-rs)] - Libfprint-rs provides a wrapper around the Linux libfprint library.
* Serial Port
  * [serialport/serialport-rs](https://github.com/serialport/serialport-rs) [[serialport](https://crates.io/crates/serialport)] — A cross-platform library that provides access to a serial port

### Platform specific

* Cross-platform
  * [iddm/thread-priority](https://github.com/iddm/thread-priority/) - Simple, crossplatform thread priority management. [![CI](https://github.com/iddm/thread-priority/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/thread-priority/actions/workflows/ci.yml) [![Crates badge](https://img.shields.io/crates/v/thread-priority.svg)](https://crates.io/crates/thread-priority)
  * [svartalf/rust-battery](https://crates.io/crates/battery) — Cross-platform information about the notebook batteries
* FreeBSD
  * [fubarnetes/libjail-rs](https://github.com/fubarnetes/libjail-rs/) [[jail](https://crates.io/crates/jail)] — FreeBSD jail library
* Linux
  * [hannobraun/inotify-rs](https://github.com/hannobraun/inotify-rs) — [inotify](https://en.wikipedia.org/wiki/Inotify) bindings [![Rust](https://github.com/hannobraun/inotify-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hannobraun/inotify-rs/actions/workflows/rust.yml)
  * [pop-os/distinst](https://github.com/pop-os/distinst/) — Linux distribution installer
  * [yaa110/rust-iptables](https://github.com/yaa110/rust-iptables) [[iptables](https://crates.io/crates/iptables)] — [iptables](https://www.netfilter.org/projects/iptables/index.html) bindings
* Unix-like
  * [nix-rust/nix](https://github.com/nix-rust/nix) — Unix-like API bindings [![Cirrus Build Status](https://api.cirrus-ci.com/github/nix-rust/nix.svg)](https://cirrus-ci.com/github/nix-rust/nix)
  * [rustix](https://github.com/bytecodealliance/rustix) — Safe bindings to POSIX/Unix/Linux/Winsock2 syscalls [![Actions Status](https://github.com/bytecodealliance/rustix/workflows/CI/badge.svg)](https://github.com/bytecodealliance/rustix/actions?query=workflow%3ACI)
  * [zargony/fuse-rs](https://github.com/zargony/fuse-rs) — [FUSE](https://github.com/libfuse/libfuse) bindings
* Windows
  * [microsoft/windows-rs](https://github.com/microsoft/windows-rs) — Rust for Windows [![Actions Status](https://github.com/microsoft/windows-rs/workflows/CI/badge.svg)](https://github.com/microsoft/windows-rs/actions)
  * [retep998/winapi-rs](https://github.com/retep998/winapi-rs) — Windows API bindings [![Rust](https://github.com/retep998/winapi-rs/actions/workflows/rust.yml/badge.svg?branch=dev)](https://github.com/retep998/winapi-rs/actions/workflows/rust.yml)

### Scripting

[[scripting](https://crates.io/keywords/scripting)]

* [3body-lang](https://github.com/rustq/3body-lang) - The Three Body Language
* [duckscript](https://crates.io/crates/duckscript) — [Simple, extendable and embeddable scripting language.](https://github.com/sagiegurari/duckscript) [![build badge](https://github.com/sagiegurari/duckscript/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/duckscript/actions)
* [fleabitdev/gamelisp](https://github.com/fleabitdev/glsp) — A Lisp-like scripting language for game development
* [gluon-lang/gluon](https://github.com/gluon-lang/gluon) —  A small, statically-typed, functional programming language
* [kcl](https://github.com/kcl-lang/kcl) - A constraint-based record & functional language mainly used in configuration and policy scenarios.
* [metacall/core](https://github.com/metacall/core) [[metacall](https://crates.io/crates/metacall)] — Cross-platform Polyglot Runtime which supports NodeJS, JavaScript, TypeScript, Python, Ruby, C#, Wasm, Java, Cobol and more. [![build badge](https://gitlab.com/metacall/core/badges/master/pipeline.svg)](https://gitlab.com/metacall/core)
* [mun](https://github.com/mun-lang/mun) — A compiled, statically-typed scripting language with first class hot reloading support
* [murarth/ketos](https://github.com/murarth/ketos) — A Lisp dialect functional programming language serving as a scripting and extension language for rust
* [PistonDevelopers/dyon](https://github.com/PistonDevelopers/dyon) — A rusty dynamically typed scripting language
* [rhaiscript/rhai](https://github.com/rhaiscript/rhai) — A tiny and fast embedded scripting language resembling a combination of JavaScript and Rust [![build badge](https://github.com/rhaiscript/rhai/workflows/Build/badge.svg)](https://github.com/rhaiscript/rhai/actions)
* [rune-rs/rune](https://github.com/rune-rs/rune) — An embeddable dynamic programming language

### Simulation

[[simulation](https://crates.io/keywords/simulation)]

* [nyx-space](https://crates.io/crates/nyx-space) - High fidelity, fast, reliable and validated astrodynamical toolkit library, used for spacecraft mission design and orbit determination [![Build Status](https://gitlab.com/nyx-space/nyx/badges/master/pipeline.svg)](https://gitlab.com/nyx-space/nyx/-/pipelines)

### System

* [ardaku/whoami](https://github.com/ardaku/whoami) [[whoami](https://crates.io/crates/whoami)] — crate to get the current user and environment. [![build badge](https://github.com/ardaku/whoami/actions/workflows/ci.yml/badge.svg?branch=stable)](https://github.com/ardaku/whoami/actions/workflows/ci.yml)
* [GuillaumeGomez/sysinfo](https://github.com/GuillaumeGomez/sysinfo) [[sysinfo](https://crates.io/crates/sysinfo)] — Cross-platform library to fetch system information [![build badge](https://github.com/GuillaumeGomez/sysinfo/actions/workflows/CI.yml/badge.svg?branch=master)](https://github.com/GuillaumeGomez/sysinfo/actions/workflows/CI.yml)
* [Phate6660/nixinfo](https://github.com/Phate6660/nixinfo) [[nixinfo](https://crates.io/crates/nixinfo)] — A lib crate for gathering system info such as cpu, distro, environment, kernel, etc.
* [sorairolake/sysexits-rs](https://github.com/sorairolake/sysexits-rs) [[sysexits](https://crates.io/crates/sysexits)] — The system exit codes as defined by [`<sysexits.h>`](https://manpages.ubuntu.com/manpages/lunar/man3/sysexits.h.3head.html). [![CI](https://github.com/sorairolake/sysexits-rs/workflows/CI/badge.svg?branch=develop)](https://github.com/sorairolake/sysexits-rs/actions?query=workflow%3ACI)

### Task scheduling

* [delay-timer](https://github.com/BinChengZhao/delay-timer) — Time-manager of delayed tasks. Like crontab, but asynchronous tasks are possible.
[![Build](https://github.com/BinChengZhao/delay-timer/actions/workflows/rust.yml/badge.svg)](
https://github.com/BinChengZhao/delay-timer/actions)

### Template engine

* Handlebars
  * [sunng87/handlebars-rust](https://github.com/sunng87/handlebars-rust) — Handlebars template engine with inheritance, custom helper support.
  * [zzau13/yarte](https://github.com/zzau13/yarte) — Yarte stands for **Y**et **A**nother **R**ust **T**emplate **E**ngine, is the fastest template engine.
* HTML
  * [djc/askama](https://github.com/djc/askama) — template rendering engine based on Jinja
  * [kaj/ructe](https://github.com/kaj/ructe) — HTML template system
  * [Keats/tera](https://github.com/Keats/tera) — template engine based on Jinja2 and the Django template language. [![Actions Status](https://github.com/Keats/tera/workflows/ci/badge.svg?branch=master)](https://github.com/Keats/tera/actions)
  * [lambda-fairy/maud](https://github.com/lambda-fairy/maud) — compile-time HTML templates
  * [Stebalien/horrorshow-rs](https://github.com/Stebalien/horrorshow-rs) — compile-time HTML templates
* Mustache
  * [rustache/rustache](https://github.com/rustache/rustache) —

### Text processing

* [becheran/wildmatch](https://github.com/becheran/wildmatch) [[wildmatch](https://crates.io/crates/wildmatch)] — Simple string matching with questionmark- and star-wildcard operator [![Actions Status](https://github.com/becheran/wildmatch/workflows/Build/badge.svg?branch=master)](https://github.com/becheran/wildmatch/actions)
* [BurntSushi/suffix](https://github.com/BurntSushi/suffix) — Linear time suffix array construction (with Unicode support)
* [BurntSushi/tabwriter](https://github.com/BurntSushi/tabwriter) — Elastic tab stops (i.e., text column alignment)
* [cpc](https://github.com/probablykasper/cpc) - Parses and calculates strings of math with support for units and unit conversion, from `1+2` to `1% of round(1 lightyear / 14!s to km/h)`.
* [Daniel-Liu-c0deb0t/triple_accel](https://github.com/Daniel-Liu-c0deb0t/triple_accel) [[triple_accel](https://crates.io/crates/triple_accel)] - Rust edit distance routines accelerated using SIMD; supports fast Hamming, Levenshtein, restricted Damerau-Levenshtein, etc. distance calculations and string search [![build badge](https://github.com/Daniel-Liu-c0deb0t/triple_accel/workflows/Test/badge.svg?branch=master)](https://github.com/Daniel-Liu-c0deb0t/triple_accel/actions)
* [fancy-regex/fancy-regex](https://github.com/fancy-regex/fancy-regex) [[fancy-regex](https://crates.io/crates/fancy-regex)] - Regular expressions implementation designed to support a relatively rich set of features such as look-around and backtracking. [![crates](https://img.shields.io/crates/v/fancy-regex.svg)](https://crates.io/crates/fancy-regex) [![build badge](https://github.com/fancy-regex/fancy-regex/workflows/ci/badge.svg)](https://github.com/fancy-regex/fancy-regex/actions/workflows/ci.yml)
* [greyblake/whatlang-rs](https://github.com/greyblake/whatlang-rs) — Natural language detection library based on trigrams
* [Lucretiel/joinery](https://github.com/Lucretiel/joinery) [[joinery](https://crates.io/crates/joinery)] – Generic string + iterable joining
* [mgeisler/textwrap](https://github.com/mgeisler/textwrap) [[textwrap](https://crates.io/crates/textwrap)] — Word wrap text (with support for hyphenation)
* [null8626/decancer](https://github.com/null8626/decancer) [[decancer](https://crates.io/crates/decancer)] — A tiny package that removes common unicode confusables/homoglyphs from strings. [![crates](https://img.shields.io/crates/v/decancer.svg)](https://crates.io/crates/decancer) [![build badge](https://github.com/null8626/decancer/workflows/CI/badge.svg)](https://github.com/null8626/decancer/actions/workflows/CI.yml)
* [ps1dr3x/easy_reader](https://github.com/ps1dr3x/easy_reader) — A reader that allows forwards, backwards and random navigations through the lines of huge files without consuming iterators
* [pwoolcoc/ngrams](https://github.com/pwoolcoc/ngrams) [[ngrams](https://crates.io/crates/ngrams)] — Construct [n-grams](https://en.wikipedia.org/wiki/N-gram) from arbitrary iterators
* [rust-lang/regex](https://github.com/rust-lang/regex) — Regular expressions (RE2 style)
* [strsim-rs](https://crates.io/crates/strsim) — String similarity metrics
* [yaa110/rake-rs](https://github.com/yaa110/rake-rs) [[rake](https://crates.io/crates/rake)] — Multilingual implementation of RAKE algorithm for Rust

### Text search

* [andylokandy/simsearch-rs](https://github.com/andylokandy/simsearch-rs) [[simsearch](https://crates.io/crates/simsearch)] — A simple and lightweight fuzzy search engine that works in memory, searching for similar strings
* [BurntSushi/fst](https://github.com/BurntSushi/fst) [[fst](https://crates.io/crates/fst)] —
* [CurrySoftware/perlin](https://github.com/CurrySoftware/perlin) [[perlin](https://crates.io/crates/perlin)]
* [meilisearch/MeiliSearch](https://github.com/meilisearch/MeiliSearch) — Ultra relevant, instant and typo-tolerant full-text search API. [![Build Status](https://github.com/meilisearch/MeiliSearch/workflows/Cargo%20test/badge.svg?branch=master)](https://github.com/meilisearch/MeiliSearch/actions)
* [pg_search](https://github.com/paradedb/paradedb/tree/dev/pg_search) - PostgreSQL extension that enables full-text search over SQL tables using the BM25 algorithm, the state-of-the-art ranking function for full-text search.
* [tantivy](https://github.com/quickwit-oss/tantivy) [[tantivy](https://crates.io/crates/tantivy)] — A horse-speed full-text search engine library written in Rust. [![Build Status](https://github.com/quickwit-oss/tantivy/actions/workflows/test.yml/badge.svg)](https://github.com/quickwit-oss/tantivy/actions/workflows/test.yml)

### Unsafe

* [zerocopy](https://crates.io/crates/zerocopy) — Utilities for safely reinterpreting arbitrary byte sequences as native Rust types

### Video

* [ffmpeg-sidecar](https://github.com/nathanbabcock/ffmpeg-sidecar) — Wrap a standalone FFmpeg binary in an intuitive Iterator interface. [![Build Status](https://github.com/nathanbabcock/ffmpeg-sidecar/actions/workflows/rust.yml/badge.svg)](https://github.com/nathanbabcock/ffmpeg-sidecar/actions/workflows/rust.yml)

### Virtualization

* [beneills/quantum](https://github.com/beneills/quantum) — Advanced quantum computer simulator
* [bytecodealliance/wasmtime](https://github.com/bytecodealliance/wasmtime) — A standalone runtime for WebAssembly [![Build Status](https://github.com/bytecodealliance/wasmtime/workflows/CI/badge.svg)](https://github.com/bytecodealliance/wasmtime/actions?query=workflow%3ACI)
* [chromium/chromiumos/platform/crosvm](https://chromium.googlesource.com/chromiumos/platform/crosvm/) CrOSVM — Enables Chrome OS to run Linux apps inside a fast, secure virtualized environment
* [oxidecomputer/propolis](https://github.com/oxidecomputer/propolis) - Userspace program for illumos bhyve kernel modules
* [saurvs/hypervisor-rs](https://github.com/saurvs/hypervisor-rs) — Hardware-accelerated virtualization on OS X

### Web programming

See also [Are we web yet?](https://www.arewewebyet.org) and [Rust web framework comparison](https://github.com/flosse/rust-web-framework-comparison).

* Client-side / WASM
  * [cargo-web](https://crates.io/crates/cargo-web) — A Cargo subcommand for the client-side Web
  * [leptos](https://github.com/leptos-rs/leptos) — Leptos is a full-stack, isomorphic web framework leveraging fine-grained reactivity to build declarative user interfaces.[![crate](https://img.shields.io/crates/v/create-rust-app.svg)](https://crates.io/crates/leptos)
  * [sauron](https://github.com/ivanceras/sauron) - Client side web framework which closely adheres to The Elm Architecture.
  * [seed](https://github.com/seed-rs/seed) — A framework for creating web apps
  * [stdweb](https://crates.io/crates/stdweb) — A standard library for the client-side Web
  * [yew](https://crates.io/crates/yew) — A framework for making client web apps
* HTTP Client
  * [alexcrichton/curl-rust](https://github.com/alexcrichton/curl-rust) — [libcurl](https://curl.se/libcurl/) bindings
  * [async-graphql](https://github.com/async-graphql/async-graphql) - A GraphQL server library [![Build Status](https://dev.azure.com/graphql-rust/GraphQL%20Rust/_apis/build/status/graphql-rust.juniper)](https://dev.azure.com/graphql-rust/GraphQL%20Rust/_build/latest?definitionId=1)
  * [DoumanAsh/yukikaze](https://gitlab.com/Douman/yukikaze) [[yukikaze](https://crates.io/crates/yukikaze)] — Beautiful and elegant Yukikaze is little HTTP client library based on hyper. [![build badge](https://gitlab.com/Douman/yukikaze/badges/master/pipeline.svg)](https://gitlab.com/Douman/yukikaze)
  * [ducaale/xh](https://github.com/ducaale/xh) - Friendly and fast tool for sending HTTP requests [![crate](https://img.shields.io/crates/v/create-rust-app.svg)](https://crates.io/crates/xh) [![GitHub actions Status](https://github.com/ducaale/xh/workflows/CI/badge.svg?branch=master)](https://github.com/ducaale/xh/actions)
  * [graphql-client](https://github.com/graphql-rust/graphql-client) — Typed, correct GraphQL requests and responses. [![GitHub actions Status](https://github.com/graphql-rust/graphql-client/workflows/CI/badge.svg?branch=master)](https://github.com/graphql-rust/graphql-client/actions)
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [![CI](https://github.com/hyperium/hyper/workflows/CI/badge.svg?branch=master)](https://github.com/hyperium/hyper/actions?query=workflow%3ACI)
  * [seanmonstar/reqwest](https://github.com/seanmonstar/reqwest) — an ergonomic HTTP Client.
* HTTP Server
  * [actix/actix-web](https://github.com/actix/actix-web) — A lightweight async web framework with websocket support
  * [Anansi](https://github.com/saru-tora/anansi) — A simple full-stack web framework
  * [branca](https://crates.io/crates/branca) — Implementation of Branca for Authenticated and Encrypted API tokens.
  * [carllerche/tower-web](https://github.com/carllerche/tower-web) [[tower-web](https://crates.io/crates/tower-web)] — A fast, boilerplate free, web framework
  * [danclive/sincere](https://github.com/danclive/sincere) — A micro web framework based on hyper and multithreading.
  * [GildedHonour/frank_jwt](https://github.com/GildedHonour/frank_jwt) — JSON Web Token implementation.
  * [Gotham](https://github.com/gotham-rs/gotham) — A flexible web framework that does not sacrifice safety, security or speed.
  * [Graphul](https://github.com/graphul-rs/graphul) — An Express-inspired web framework. [![crate](https://img.shields.io/crates/v/create-rust-app.svg)](https://crates.io/crates/graphul)
  * [handlebars-rust](https://github.com/sunng87/handlebars-rust) — an Iron web framework middleware.
  * [hyperium/hyper](https://github.com/hyperium/hyper) — an HTTP implementation [![CI](https://github.com/hyperium/hyper/workflows/CI/badge.svg?branch=master)](https://github.com/hyperium/hyper/actions?query=workflow%3ACI)
  * [Iron](https://github.com/iron/iron) — A middleware-based server framework
  * [Juniper](https://github.com/graphql-rust/juniper) — GraphQL server library
  * [miketang84/sapper](https://github.com/miketang84/sapper) — A lightweight web framework built on async hyper.
  * [Nickel](https://github.com/nickel-org/nickel.rs/) — inspired by [Express](http://expressjs.com/)
  * [poem-web/poem](https://github.com/poem-web/poem) - A full-featured and easy-to-use web framework. [![CI](https://github.com/poem-web/poem/actions/workflows/ci.yml/badge.svg)](https://github.com/poem-web/poem/actions/workflows/ci.yml)
  * [Rocket](https://github.com/rwf2/Rocket) — Rocket is a web framework with a focus on ease-of-use, expressability, and speed
  * [Rustless](https://github.com/rustless/rustless) — A REST-like API micro-framework inspired by [Grape](https://github.com/ruby-grape/grape) and [Hyper](https://github.com/hyperium/hyper)
  * [Salvo](https://github.com/salvo-rs/salvo) — an easy to use webframework base on hyper and tokio. [![build build](https://github.com/salvo-rs/salvo/workflows/CI%20(Linux)/badge.svg?branch=master&event=push)](https://github.com/salvo-rs/salvo/actions)
  * [Saphir](https://github.com/richerarc/saphir) — A progressive web framework with low-level control, without the pain.
  * [seanmonstar/warp](https://github.com/seanmonstar/warp) — A super-easy, composable, web server framework for warp speeds. [![crate](https://img.shields.io/crates/v/create-rust-app.svg)](https://crates.io/crates/warp)
  * [tiny-http](https://github.com/tiny-http/tiny-http) — Low level HTTP server library
  * [tokio/axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework built with Tokio, Tower, and Hyper [![Build badge](https://github.com/tokio-rs/axum/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/tokio-rs/axum/actions/workflows/CI.yml)
  * [tomaka/rouille](https://github.com/tomaka/rouille) — Web framework
  * [Zino](https://github.com/zino-rs/zino) — Next-generation framework for composable applications
* Miscellaneous
  * [cargonauts](https://github.com/cargonauts-rs/cargonauts) — A web framework intended for building maintainable, well-factored web apps.
  * [causal-agent/scraper](https://github.com/causal-agent/scraper) [[scraper](https://crates.io/crates/scraper)] - HTML parsing and querying with CSS selectors. [![Build Status](https://github.com/causal-agent/scraper/actions/workflows/test.yml/badge.svg?branch=master)](https://github.com/causal-agent/scraper/actions)
  * [hominee/dyer](https://github.com/hominee/dyer) [[dyer](https://crates.io/crates/dyer)] - dyer is designed for reliable, flexible and fast Request-Response based service, including data processing, web-crawling and so on, providing some friendly, flexible, comprehensive features without compromising speed.
  * [juhaku/utoipa](https://github.com/juhaku/utoipa) - Simple, Fast, Code first and Compile time generated OpenAPI documentation [![crates.io](https://img.shields.io/crates/v/utoipa.svg?label=crates.io&color=orange&logo=rust)](https://crates.io/crates/utoipa) [![Utoipa build](https://github.com/juhaku/utoipa/actions/workflows/build.yaml/badge.svg)](https://github.com/juhaku/utoipa/actions/workflows/build.yaml)
  * [osohq/oso](https://github.com/osohq/oso) [[oso](https://crates.io/crates/oso)] - A policy engine for authorization that's embedded in your application. [![Build Status](https://github.com/osohq/oso/workflows/Development/badge.svg?branch=main)](https://github.com/osohq/oso/actions?query=branch%3Amain+workflow%3ADevelopment)
  * [pwoolcoc/soup](https://gitlab.com/pwoolcoc/soup) [[soup](https://crates.io/crates/soup)] — A library similar to Python's BeautifulSoup, designed to enable quick and easy manipulation and querying of HTML documents. [![Build Status](https://gitlab.com/pwoolcoc/soup/badges/master/pipeline.svg)](https://gitlab.com/pwoolcoc/soup/badges/master/pipeline.svg)
  * [pyrossh/rust-embed](https://github.com/pyrossh/rust-embed) — A macro to embed static assets into the rust binary
  * [rookie](https://github.com/thewh1teagle/rookie) — Load cookies from any browser on any platform. ![crates.io](https://img.shields.io/crates/v/rookie.svg)
  * [serenity-rs/serenity](https://github.com/serenity-rs/serenity) [[serenity](https://crates.io/crates/serenity)] - A library for the Discord API
  * [softprops/openapi](https://github.com/softprops/openapi) — A library for processing openapi spec files
  * [svix/svix-webhooks](https://github.com/svix/svix-webhooks) [[svix](https://crates.io/crates/svix)]- A library for sending webhooks and verifying signatures.
  * [tbot](https://gitlab.com/SnejUgal/tbot) [[tbot](https://crates.io/crates/tbot)] - Make cool Telegram bots easily [![pipeline status](https://gitlab.com/SnejUgal/tbot/badges/master/pipeline.svg)](https://gitlab.com/SnejUgal/tbot/-/commits/master)
  * [teloxide/teloxide](https://github.com/teloxide/teloxide/) - An elegant Telegram bots framework [![Build Status](https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg?branch=master)](https://github.com/teloxide/teloxide/actions)
  * [tu6ge/valitron](https://github.com/tu6ge/valitron) [[valitron](https://crates.io/crates/valitron)] - An ergonomic, functional and configurable validator
  * [utkarshkukreti/select.rs](https://github.com/utkarshkukreti/select.rs) [[select](https://crates.io/crates/select)] — A library to extract useful data from HTML documents, suitable for web scraping.
* Reverse Proxy
  * [sozu-proxy/sozu](https://github.com/sozu-proxy/sozu) [[sozu](https://crates.io/crates/sozu)] — A HTTP reverse proxy. [![CI](https://github.com/sozu-proxy/sozu/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/sozu-proxy/sozu/actions/workflows/ci.yml)
* Static Site Generators
  * [cobalt-org/cobalt.rs](https://github.com/cobalt-org/cobalt.rs) — Static site generator [![Build Status](https://dev.azure.com/cobalt-org/cobalt-org/_apis/build/status/cobalt.rs?branchName=master)](https://dev.azure.com/cobalt-org/cobalt-org/_build?definitionId=2)
  * [FuGangqiang/mdblog.rs](https://github.com/FuGangqiang/mdblog.rs) [[mdblog](https://crates.io/crates/mdblog)] — Static site generator from markdown files.
  * [getzola/zola](https://github.com/getzola/zola) [[zola](https://www.getzola.org/)] — An opinionated static site generator with everything built-in. [![Build Status](https://dev.azure.com/getzola/zola/_apis/build/status/getzola.zola?branchName=master)](https://dev.azure.com/getzola/zola/_build)
  * [grego/blades](https://github.com/grego/blades) [[blades](https://getblades.org/)] — Blazing fast dead simple static site generator.
  * [leven-the-blog/leven](https://github.com/leven-the-blog/leven) [[leven](https://crates.io/crates/leven)] — A simple, parallelized blog generator.
* [WebSocket](https://datatracker.ietf.org/doc/rfc6455/)
  * [housleyjk/ws-rs](https://github.com/housleyjk/ws-rs) — lightweight, event-driven WebSockets
  * [iddm/urlshortener-rs](https://github.com/iddm/urlshortener-rs) — A very simple urlshortener library. [![CI](https://github.com/iddm/urlshortener-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/iddm/urlshortener-rs/actions/workflows/ci.yml) [![Crates badge](https://img.shields.io/crates/v/urlshortener.svg)](https://crates.io/crates/urlshortener)
  * [rust-websocket](https://github.com/websockets-rs/rust-websocket) — A framework for dealing with WebSocket connections (both clients and servers)
  * [snapview/tungstenite-rs](https://github.com/snapview/tungstenite-rs) — Lightweight stream-based WebSocket implementation.
  * [vi/websocat](https://github.com/vi/websocat) — CLI for interacting with WebSockets, with functionality of Netcat, Curl and Socat.

## Registries

A registry allows you to publish your Rust libraries as crate packages, to share them with others publicly and privately.

* [Cloudsmith :heavy_dollar_sign:](https://cloudsmith.com/product/formats/cargo-registry) — A fully managed package management SaaS, with first-class support for public and private Cargo/Rust registries (plus many others). Has a generous free-tier and is also completely free for open-source.
* [Crates](https://crates.io) — The official public registry for Rust/Cargo.
* [w4/chartered](https://github.com/w4/chartered) - A private, authenticated, permissioned Cargo registry [![CI](https://github.com/w4/chartered/actions/workflows/ci.yml/badge.svg)](https://github.com/w4/chartered/actions/workflows/ci.yml)

## Resources

* Benchmarks
  * [TeXitoi/benchmarksgame-rs](https://github.com/TeXitoi/benchmarksgame-rs) — Implementations for the [The Computer Language Benchmarks Game](https://benchmarksgame-team.pages.debian.net/benchmarksgame/)
* Decks & Presentations
  * [Learning systems programming with Rust](https://speakerdeck.com/jvns/learning-systems-programming-with-rust) — Presented by [Julia Evans](https://twitter.com/@b0rk) @ Rustconf 2016.
  * [Rust: Hack Without Fear!](https://www.youtube.com/watch?v=lO1z-7cuRYI) — Presented by [Nicholas Matsakis](https://github.com/nikomatsakis) @ C++Now 2018
  * [Shipping a Solid Rust Crate](https://www.youtube.com/watch?v=t4CyEKb-ywA) — Presented by [Michael Gattozzi](https://github.com/mgattozzi) @ RustConf 2017
* [Discover Rust Libraries & Code Snippets](https://kandi.openweaver.com/explorelibrary/rust) - A curated list of libraries, authors, kits, tutorials & learning resources on kandi
* Learning
  * [Aquascope](https://github.com/cognitive-engineering-lab/aquascope) - Interactive visualizations of Rust at compile-time and run-time
  * [Awesome Rust Streaming](https://github.com/jamesmunns/awesome-rust-streaming) - A community curated list of livestreams.
  * [awesome-rust-mentors](https://rustbeginners.github.io/awesome-rust-mentors/) — A list of helpful mentors willing to take mentees and educate them about Rust and programming.
  * [Build a language VM](https://blog.subnetzero.io/post/building-language-vm-part-00/)
  * [CodeCrafters.io](https://app.codecrafters.io/tracks/rust) — Build your own Redis, Git, Docker, or SQLite
  * [Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/) — A 3-day course on Rust Fundamentals plus 1-day courses on Android, Bare-metal Rust, and Concurrency. Available in English, [Brazilian Portuguese](https://google.github.io/comprehensive-rust/pt-BR/), and [Korean](https://google.github.io/comprehensive-rust/ko/).
  * [Easy Rust](https://github.com/Dhghomon/easy_rust) - Learn Rust in easy English.
  * [exercism.org](https://exercism.org/tracks/rust) — programming exercises that help you learn new concepts in Rust.
  * [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/) - A hands-on guide to learning Rust by making games - by [Herbert Wolverson](https://github.com/thebracket/) (paid)
  * [Idiomatic Rust](https://github.com/mre/idiomatic-rust) —  A peer-reviewed collection of articles/talks/repos which teach idiomatic Rust.
  * [Learn Rust by 500 lines code](https://github.com/cuppar/rtd) — Learn Rust by 500 lines code, build a Todo Cli Application from scratch.
  * [Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/) — in-depth exploration of Rust's memory management rules, through implementing a few different types of list structures.
  * [Little Book of Rust Books](https://lborb.github.io/book/) - Curated list of rust books and how-tos.
  * [Programming Community Curated Resources for Learning Rust](https://hackr.io/tutorials/learn-rust) — A list of recommended resources voted by the programming community.
  * [Refactoring to Rust](https://www.manning.com/books/refactoring-to-rust) - A book that introduces to Rust language.
  * [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
  * [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/) — A collection of simple examples that demonstrate good practices to accomplish common programming tasks, using the crates of the Rust ecosystem.
  * [Rust Flashcards](https://github.com/ad-si/Rust-Flashcards) - Over 550 flashcards to learn Rust from first principles.
  * [Rust for professionals](https://overexact.com/rust-for-professionals/) — A quick introduction to Rust for experienced software developers.
  * [Rust Gym](https://github.com/warycat/rustgym) - A big collection of coding interview problems solved in Rust.
  * [Rust in Action](https://www.manning.com/books/rust-in-action) — A hands-on guide to systems programming with Rust by [Tim McNamara](https://github.com/timClicks) (paid)
  * [Rust in Motion](https://www.manning.com/livevideo/rust-in-motion?a_aid=cnichols&a_bid=6a993c2e) — A video series by [Carol Nichols](https://github.com/carols10cents) and [Jake Goulding](https://github.com/shepmaster) (paid)
  * [Rust Language Cheat Sheet](https://cheats.rs/)
  * [Rust Tiếng Việt](https://rust-tieng-viet.github.io/) - Learn Rust in Vietnamese.
  * [rust-how-do-i-start](https://github.com/jondot/rust-how-do-i-start) - A repo dedicated to answering the question: "So, Rust. How do I _start_?". A beginner only hand-picked resources and learning track.
  * [rust-learning](https://github.com/ctjhoa/rust-learning) — A collection of useful resources to learn Rust
  * [Rustlings](https://github.com/rust-lang/rustlings) — small exercises to get you used to reading and writing Rust code
  * [Rusty CS](https://github.com/AbdesamedBendjeddou/Rusty-CS) - A Computer Science Curriculum that helps practice the acquired academic knowledge in Rust
  * [stdx](https://github.com/brson/stdx) — Learn these crates first as an extension to std
  * [Take your first steps with Rust](https://learn.microsoft.com/en-us/training/paths/rust-first-steps/) - Lay the foundation of knowledge you need to build fast and effective programs in Rust.
  * [Tour of Rust](https://tourofrust.com) - This is meant to be an interactive step by step guide through the features of the Rust programming language.
  * [University of Pennsylvania's Comp Sci Rust Programming Course](http://cis198-2016s.github.io/schedule/)
* Podcasts
  * [New Rustacean](https://newrustacean.com) — A podcast about learning Rust
  * [Rustacean Station](https://rustacean-station.org/) — A community project for creating podcast content for Rust
* [Rust Design Patterns](https://github.com/rust-unofficial/patterns)
* [Rust Guidelines](http://aturon.github.io/)
* [Rust Servers, Services and Apps - MEAP](https://www.manning.com/books/rust-servers-services-and-apps) - Build backend servers, services, and front-ends in Rust to get fast, reliable, and maintainable applications.
* [Rust Subreddit](https://www.reddit.com/r/rust/) — A subreddit(forum) where rust related questions, articles and resources are posted and discussed
* [RustBooks](https://github.com/sger/RustBooks) — list of RustBooks
* [RustCamp 2015 Talks](https://www.youtube.com/playlist?list=PLE7tQUdRKcybdIw61JpCoo89i4pWU5f_t)
* [RustViz](https://github.com/rustviz/rustviz) — generates visualizations from simple Rust programs to assist users in better understanding the Rust Lifetime and Borrowing mechanism.
* [Watch Jon Gjengset Implement BitTorrent in Rust](https://www.youtube.com/watch?v=jf_ddGnum_4)

## License

[![CC0](https://licensebuttons.net/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)
