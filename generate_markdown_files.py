# -*- coding: utf-8 -*-

def md_link(name, url):
  "Returns a web link in Markdown format"
  return '[%s](%s)' % (name, url)

DATA = \
    {
      "bjz/openal-rs": {
          "url": "https://github.com/bjz/openal-rs/",
          "descr": md_link('OpenAL 1.1', 'http://www.openal.org/') + " bindings"
        },
      "Cargo": {
          "url": "http://crates.io",
          "descr": "the Rust package manager"
        },
      "klutzy/suruga": {
          "url": 'https://github.com/klutzy/suruga',
          "descr": 'a Rust implementation of ' + md_link('TLS 1.2', 'http://tools.ietf.org/html/rfc5246')
        },
      "JeremyLetang/ears": {
          "url": "https://github.com/JeremyLetang/ears",
          "descr": "a simple library to play Sounds and Musics, on top of OpenAL and libsndfile",
          "travis_url": "https://travis-ci.org/JeremyLetang/ears",
          "travis_badge": "https://travis-ci.org/jeremyletang/ears.svg?branch=master"
        },
      "JeremyLetang/rust-portaudio": {
          "url": "https://github.com/JeremyLetang/rust-portaudio",
          "descr": md_link('PortAudio', 'http://www.portaudio.com/') + " bindings",
          "travis_url": "https://travis-ci.org/JeremyLetang/rust-portaudio",
          "travis_badge": "https://travis-ci.org/jeremyletang/rust-portaudio.svg?branch=master"
        },
      "musitdev/rust-portmidi": {
          "url": "https://github.com/musitdev/rust-portmidi",
          "descr": md_link('PortMidi', 'http://portmedia.sourceforge.net/portmidi/') + " bindings",
          "travis_url": "https://travis-ci.org/musitdev/rust-portmidi",
          "travis_badge": "https://travis-ci.org/musitdev/rust-portmidi.svg?branch=master"
        },
      "PistonDevelopers/image": {
          "url": "https://github.com/PistonDevelopers/image",
          "descr": 'Basic imaging processing functions and methods for converting to and from image formats',
          "travis_url": "https://travis-ci.org/PistonDevelopers/image",
          "travis_badge": "https://travis-ci.org/PistonDevelopers/image.svg?branch=master"
        },
      "seb-m/common.rs": {
          "url": 'https://github.com/klutzy/suruga',
          "descr": 'Common Rust crypto utilities'
        },
      "SiegeLord/RustCMake": {
          "url": "https://github.com/SiegeLord/RustCMake",
          "descr": "an example project showing usage of CMake with Rust",
          "travis_url": "https://travis-ci.org/SiegeLord/RustCMake",
          "travis_badge": "https://travis-ci.org/SiegeLord/RustCMake.svg?branch=master"
        },
      "PistonDevelopers/rust-empty": {
          "url": "https://github.com/PistonDevelopers/rust-empty",
          "descr": "a Makefile to get started with Rust"
        },
      "docopt/docopt.rs": {
          "url": "https://github.com/docopt/docopt.rs",
          "descr": "a Rust implementation of " + md_link('DocOpt', 'http://docopt.org')
        },
      "wycats/hammer.rs": {
          "url": "https://github.com/wycats/hammer.rs"
        },
      "lifthrasiir/rust-zip": {
          "url": "https://github.com/lifthrasiir/rust-zip",
          "descr": "read and write ZIP archives",
          "travis_url": "https://travis-ci.org/lifthrasiir/rust-zip",
          "travis_badge": "https://travis-ci.org/lifthrasiir/rust-zip.svg?branch=master"
        },
      "eholk/rust-opencl": {
          "url": "https://github.com/eholk/rust-opencl",
          "descr": md_link('OpenCL', 'https://www.khronos.org/opencl/') + " bindings",
          "travis_url": "https://travis-ci.org/eholk/rust-opencl",
          "travis_badge": "https://travis-ci.org/eholk/rust-opencl.svg?branch=master"
        },
      "DaGenix/rust-crypto": {
          "url": "https://github.com/DaGenix/rust-crypto",
          "descr": "cryptographic algorithms in Rust",
          "travis_url": "https://travis-ci.org/DaGenix/rust-crypto",
          "travis_badge": "https://travis-ci.org/DaGenix/rust-crypto.svg?branch=master"
        },
      "dnaq/sodiumoxide": {
          "url": "https://github.com/dnaq/sodiumoxide",
          "descr": md_link('libsodium', 'https://github.com/jedisct1/libsodium') + " bindings"
        },
      "sfackler/rust-openssl": {
          "url": "https://github.com/sfackler/rust-openssl",
          "descr": "OpenSSL bindings",
          "travis_url": "https://travis-ci.org/sfackler/rust-openssl",
          "travis_badge": "https://travis-ci.org/sfackler/rust-openssl.svg?branch=master"
        },
      "blackbeam/rust-mysql-simple": {
          "url": "https://github.com/blackbeam/rust-mysql-simple",
          "descr": "a native MySql client",
          "travis_url": "https://travis-ci.org/blackbeam/rust-mysql-simple",
          "travis_badge": "https://travis-ci.org/blackbeam/rust-mysql-simple.svg?branch=master"
        },
      "sfackler/rust-postgres": {
          "url": "https://github.com/sfackler/rust-postgres",
          "descr": "a native " + md_link('PostgreSQL', 'http://www.postgresql.org') + " client",
          "travis_url": "https://travis-ci.org/sfackler/rust-postgres",
          "travis_badge": "https://travis-ci.org/sfackler/rust-postgres.svg?branch=master"
        },
      "linuxfood/rustsqlite": {
          "url": "https://github.com/linuxfood/rustsqlite",
          "descr": md_link('Sqlite3', 'http://www.sqlite.org/') + " bindings",
        },
      "dwrensha/capnproto-rust": {
          "url": "https://github.com/dwrensha/capnproto-rust",
          "travis_url": "https://travis-ci.org/dwrensha/capnproto-rust",
          "travis_badge": "https://travis-ci.org/dwrensha/capnproto-rust.svg?branch=master"
        },
      "lifthrasiir/rust-encoding": {
          "url": "https://github.com/lifthrasiir/rust-encoding",
          "travis_url": "https://travis-ci.org/lifthrasiir/rust-encoding",
          "travis_badge": "https://travis-ci.org/lifthrasiir/rust-encoding.svg?branch=master"
        },
      "BurntSushi/rust-csv": {
          "url": "https://github.com/BurntSushi/rust-csv",
          "travis_url": "https://travis-ci.org/BurntSushi/rust-csv",
          "travis_badge": "https://api.travis-ci.org/BurntSushi/rust-csv.svg?branch=master"
        },
      "Geal/rust-csv": {
          "url": "https://github.com/Geal/rust-csv",
          "travis_url": "https://travis-ci.org/Geal/rust-csv",
          "travis_badge": "https://travis-ci.org/Geal/rust-csv.svg?branch=master",
          "unstable": True
        },
      "mneumann/rust-msgpack": {
          "url": "https://github.com/mneumann/rust-msgpack",
          "travis_url": "https://travis-ci.org/mneumann/rust-msgpack",
          "travis_badge": "https://travis-ci.org/mneumann/rust-msgpack.svg?branch=master"
        },
      "stepancheg/rust-protobuf": {
          "url": "https://github.com/stepancheg/rust-protobuf",
          "travis_url": "https://travis-ci.org/stepancheg/rust-protobuf",
          "travis_badge": "https://travis-ci.org/stepancheg/rust-protobuf.svg?branch=master"
        },
      "alexcrichton/toml-rs": {
          "url": "https://github.com/alexcrichton/toml-rs"
        },
      "erickt/rust-tnetstring": {
          "url": "https://github.com/erickt/rust-tnetstring",
          "travis_url": "https://travis-ci.org/erickt/rust-tnetstring",
          "travis_badge": "https://travis-ci.org/erickt/rust-tnetstring.svg?branch=master",
          "unstable": True
        },
      "bjz/sax-rs": {
          "url": "https://github.com/bjz/sax-rs",
          "descr": "bindings to libxml2's SAX parser",
          "travis_url": "https://travis-ci.org/bjz/sax-rs",
          "travis_badge": "https://travis-ci.org/bjz/sax-rs.svg?branch=master"
        },
      "Ygg01/xml-air": {
          "url": "https://github.com/Ygg01/xml-air",
          "descr": "A hybrid pull, DOM parser written in pure Rust",
          "travis_url": "https://travis-ci.org/Ygg01/xml-air",
          "travis_badge": "https://travis-ci.org/Ygg01/xml-air.svg?branch=master"
        },
      "Florob/RustyXML": {
          "url": "https://github.com/Florob/RustyXML",
          "descr": "an XML parser written in Rust",
          "travis_url": "https://travis-ci.org/Florob/RustyXM",
          "travis_badge": "https://travis-ci.org/Florob/RustyXML.svg?branch=master"
        },
      "netvl/rust-xml": {
          "url": "https://github.com/netvl/rust-xml",
          "descr": "a streaming XML library",
          "travis_url": "https://travis-ci.org/netvl/rust-xml",
          "travis_badge": "https://travis-ci.org/netvl/rust-xml.svg?branch=master"
        },
      "JeremyLetang/rustenstein3D": {
          "url": "https://github.com/JeremyLetang/rustenstein3D/",
          "descr": "a raycasting engine in rust",
          "travis_url": "",
          "travis_badge": "",
          'unstable': True
        },
      "lifthrasiir/angolmois-rust": {
          "url": "https://github.com/lifthrasiir/angolmois-rust",
          "descr": "a minimalistic music video game which supports the BMS format",
          "travis_url": "https://travis-ci.org/lifthrasiir/angolmois-rust",
          "travis_badge": "https://travis-ci.org/lifthrasiir/angolmois-rust.svg?branch=master"
        },
      "mozilla-servo/rust-cocoa": {
          "url": "https://github.com/mozilla-servo/rust-cocoa"
        },
      "JeremyLetang/rgtk": {
          "url": "https://github.com/JeremyLetang/rgtk",
          "descr": md_link('Gtk+', 'http://www.gtk.org') + " bindings",
          "travis_url": "https://travis-ci.org/jeremyletang/rgtk",
          "travis_badge": "https://travis-ci.org/jeremyletang/rgtk.svg?branch=master"
        },
      "jeaye/ncurses-rs": {
          "url": "https://github.com/jeaye/ncurses-rs",
          "travis_url": "https://travis-ci.org/jeaye/ncurses-rs",
          "travis_badge": "https://travis-ci.org/jeaye/ncurses-rs.svg?branch=master"
        },
      "AngryLawyer/rust-sdl2": {
          "url": "https://github.com/AngryLawyer/rust-sdl2",
          "descr": md_link('SDL2', 'http://www.libsdl.org/') + " bindings",
          "travis_url": "https://travis-ci.org/AngryLawyer/rust-sdl2",
          "travis_badge": "https://travis-ci.org/AngryLawyer/rust-sdl2.svg?branch=master"
        },
      "brson/rust-sdl": {
          "url": "https://github.com/brson/rust-sdl",
          "descr": md_link('SDL1', 'http://www.libsdl.org/') + " bindings",
          "travis_url": "https://travis-ci.org/brson/rust-sdl",
          "travis_badge": "https://travis-ci.org/brson/rust-sdl.svg?branch=master"
        },
      "gchp/rustbox": {
          "url": "https://github.com/gchp/rustbox",
          "descr": "a Rust implementation of " + md_link('termbox', 'http://github.com/nsf/termbox')
        },
      "erickt/rust-zmq": {
          "url": "https://github.com/erickt/rust-zmq",
          "descr": md_link('ZeroMQ', 'http://zeromq.org') + " bindings",
          "travis_url": "https://travis-ci.org/erickt/rust-zmq",
          "travis_badge": "https://travis-ci.org/erickt/rust-zmq.svg?branch=master"
        },
      "zeromq/zmq.rs": {
            "url": "https://github.com/zeromq/zmq.rs",
            "descr": "Rust implementation of the " + md_link('ZeroMQ', 'http://zeromq.org') + "protocol",
            "travis_url": "https://travis-ci.org/zeromq/zmq.rs",
            "travis_badge": "https://travis-ci.org/zeromq/zmq.rs.svg?branch=master",
            "unstable": True
        },
      "erickt/rust-mustache": {
          "url": "https://github.com/erickt/rust-mustache",
          "travis_url": "https://travis-ci.org/erickt/rust-mustache",
          "travis_badge": "https://travis-ci.org/erickt/rust-mustache.svg?branch=master"
        },
      "BurntSushi/quickcheck": {
          "url": "https://github.com/BurntSushi/quickcheck",
          "descr": 'a Rust implementation of ' + md_link('QuickCheck', 'http://www.haskell.org/haskellwiki/Introduction_to_QuickCheck1'),
          "travis_url": "https://travis-ci.org/BurntSushi/quickcheck",
          "travis_badge": "https://travis-ci.org/BurntSushi/quickcheck.svg?branch=master"
        },
      "farcaller/shiny": {
          "url": "https://github.com/farcaller/shiny",
          "descr": "a fancy syntax similar to ruby's rspec or Objective-C's kiwi"
        },
      "chris-morgan/rust-http": {
          "url": "https://github.com/chris-morgan/rust-http",
          "descr": "will be replaced by " + md_link('Teepee', 'http://teepee.rs/'),
          "travis_url": "https://travis-ci.org/chris-morgan/rust-http",
          "travis_badge": "https://travis-ci.org/chris-morgan/rust-http.svg?branch=master"
        },
      "hyperium/hyper": {
          "url": "https://github.com/hyperium/hyper",
          "travis_url": "https://travis-ci.org/hyperium/hyper",
          "travis_badge": "https://travis-ci.org/hyperium/hyper.svg?branch=master"
        },
      "carllerche/curl-rust": {
          "url": "https://github.com/carllerche/curl-rust",
          "descr": md_link('libcurl', 'http://curl.haxx.se/libcurl/') + " bindings"
        },
      "vhbit/curl-rs": {
          "url": "https://github.com/vhbit/curl-rs",
          "descr": md_link('libcurl', 'http://curl.haxx.se/libcurl/') + " bindings",
          "travis_url": "https://travis-ci.org/vhbit/curl-rs.svg?branch=master",
          "travis_badge": "https://travis-ci.org/vhbit/curl-rs"
        },
      "erickt/rust-mongrel2": {
          "url": "https://github.com/erickt/rust-mongrel2",
          "descr": md_link('Mongrel2', 'http://mongrel2.org') + " bindings",
          "travis_url": "https://travis-ci.org/erickt/rust-mongrel2.svg?branch=master",
          "travis_badge": "https://travis-ci.org/erickt/rust-mongrel2"
        },
      "Iron": {
          "url": "http://ironframework.io/",
          "descr": "inspired by " + md_link('Express', 'http://expressjs.com/'),
          "travis_url": "https://travis-ci.org/iron/iron.svg?branch=master",
          "travis_badge": "https://travis-ci.org/iron/iron"
        },
      "Nickel": {
          "url": "http://nickel.rs/",
          "descr": "inspired by " + md_link('Express', 'http://expressjs.com/')
        },
      "Rust by Example": {
          "url": "http://rustbyexample.com/",
        },
      "Rust CI": {
          "url": "http://www.rust-ci.org",
          "descr": "a " + md_link('Travis CI', 'https://travis-ci.com') + " dashboard for Rust projects",
        },
      "Rust Guidelines": {
          "url": "http://aturon.github.io",
        },
      "kenz-gelsoft/wxRust": {
          "url": "https://github.com/kenz-gelsoft/wxRust",
          "descr": md_link('wxWidgets', 'http://www.wxwidgets.org/') + " bindings",
          "travis_url": "https://travis-ci.org/kenz-gelsoft/wxRust",
          "travis_badge": "https://travis-ci.org/kenz-gelsoft/wxRust.svg?branch=master",
          "unstable": True
        },
      "Arcterus/game-of-life": {
          "url": "https://github.com/Arcterus/game-of-life",
          "unstable": True
        },
      "Arcterus/rust-snake": {
          "url": "https://github.com/Arcterus/rust-snake",
          "unstable": True
        },
      "bachm/rusty-tetris": {
          "url": "https://github.com/bachm/rusty-tetris",
          "unstable": True
        },
      "bvssvni/rust-snake": {
          "url": "https://github.com/bvssvni/rust-snake",
          "unstable": True
        },
      "Coeuvre/rust-2048": {
          "url": "https://github.com/Coeuvre/rust-2048",
          "unstable": True
        },
      "Coeuvre/rust-pong": {
          "url": "https://github.com/Coeuvre/rust-pong",
          "unstable": True
        },
      "dpc/rustyhex": {
          "url": "https://github.com/dpc/rustyhex",
          "unstable": True
        },
      "FrozenCow/rust-airhockey": {
          "url": "https://github.com/FrozenCow/rust-airhockey",
          "unstable": True
        },
      "jeaye/q3": {
          "url": "https://github.com/jeaye/q3",
          "unstable": True
        },
      "mynery/xxo": {
          "url": "https://github.com/mynery/xxo",
          "unstable": True
        },
      "ozkriff/marauder": {
          "url": "https://github.com/ozkriff/marauder",
          "unstable": True
        },
      "rlane/cubeland": {
          "url": "https://github.com/rlane/cubeland",
          "unstable": True
        },
      "zokier/pong-rs": {
          "url": "https://github.com/zokier/pong-rs",
          "unstable": True
        },
      "bjz/gl-rs": {
          "url": "https://github.com/bjz/gl-rs",
          "unstable": True
        },
      "servo/rust-opengles": {
          "url": "https://github.com/servo/rust-opengles",
          "unstable": True
        },
      "PistonDevelopers/piston": {
          "url": "https://github.com/pistondevelopers/piston",
          "travis_url": "https://travis-ci.org/PistonDevelopers/piston",
          "travis_badge": "https://travis-ci.org/PistonDevelopers/piston.svg?branch=master",
        },
      "sebcrozet/kiss3d": {
          "url": "https://github.com/sebcrozet/kiss3d",
          "unstable": True
        },
      "servo/rust-glut": {
          "url": "https://github.com/mozilla-servo/rust-glut",
          "unstable": True
        },
      "bjz/glfw-rs": {
          "url": "https://github.com/bjz/glfw-rs",
          "unstable": True
        },
      "jeremyletang/rust-sfml": {
          "url": "https://github.com/JeremyLetang/rust-sfml",
          "travis_url": "https://travis-ci.org/jeremyLetang/rust-sfml",
          "travis_badge": "https://travis-ci.org/jeremyletang/rust-sfml.svg?branch=master",
        },
      "SiegeLord/RustAllegro": {
          "url": "https://github.com/SiegeLord/RustAllegro",
          "unstable": True
        },
      "bjz/bullet-rs": {
          "url": "https://github.com/bjz/bullet-rs",
          "unstable": True
        },
      "sebcrozet/ncollide": {
          "url": "https://github.com/sebcrozet/ncollide",
          "unstable": True
        },
      "sebcrozet/nphysics": {
          "url": "https://github.com/sebcrozet/nphysics",
          "unstable": True
        },
    }


HEADER_STABLE = \
"""# Awesome Rust

A curated list of awesome Rust code and resources. Inspired by the other [awesome lists](https://github.com/bayandin/awesome-awesomeness).

Only projects that are stable and useful to users are added. Projects that do not compile with Rust-nightly for a longer time are moved to `UNSTABLE.md`."""

HEADER_UNSTABLE = \
"""# Unstable

A list of awesome but unstable/experimental Rust projects which some day hopefully will be migrated to `README.md`."""

TOC = \
"""
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
"""

def add( value, rows_stable=None, rows_unstable=None ):

    if rows_stable:
        rows_stable.append( value )
    if rows_unstable:
        rows_unstable.append( value )

#end def add

def entry( prefix, name, rows_stable=None, rows_unstable=None ):

    entry = DATA.get(name)

    if entry is None:
        print '"%s" not found' % name
        return

    url = entry.get('url')
    description = entry.get('descr', '')

    if description:
        description = ' ' + description

    travis_badge = entry.get('travis_badge')
    travis_url = entry.get('travis_url')

    if travis_badge and travis_url:
        travis = ' ' + md_link( '<img src="%s">' % travis_badge, travis_url )
    else:
        travis = ''

    if description or travis:
        separator = ' —'
    else:
        separator = ''

    unstable = entry.get('unstable')

    if unstable:
        rows = rows_unstable
    else:
        rows = rows_stable

    rows.append('%s[%s](%s)%s%s%s' % (prefix, name, url, separator, description, travis))

#end def entry


# Generate Markdown files.
rows_stable = [HEADER_STABLE, TOC]
rows_unstable = [HEADER_UNSTABLE, TOC]

add(  '## Code\n', rows_stable, rows_unstable )

add(  '\n### Audio\n', rows_stable, rows_unstable )
entry( '* ', 'bjz/openal-rs', rows_stable, rows_unstable )
entry( '* ', 'JeremyLetang/ears', rows_stable, rows_unstable )
entry( '* ', 'JeremyLetang/rust-portaudio', rows_stable, rows_unstable )
entry( '* ', 'musitdev/rust-portmidi', rows_stable, rows_unstable )

add(   '\n### Build system\n', rows_stable, rows_unstable )
entry( '* ', 'Cargo', rows_stable, rows_unstable )
add(   '* CMake', rows_stable, rows_unstable )
entry( '  * ', 'SiegeLord/RustCMake', rows_stable, rows_unstable )
add(   '* Make', rows_stable, rows_unstable )
entry( '  * ', 'PistonDevelopers/rust-empty', rows_stable, rows_unstable )

add(   '\n### Command-line argument parsing\n', rows_stable, rows_unstable )
entry( '* ', 'docopt/docopt.rs', rows_stable, rows_unstable )
entry( '* ', 'wycats/hammer.rs', rows_stable, rows_unstable )

add(   '\n### Compression\n', rows_stable, rows_unstable )
entry( '* ', 'lifthrasiir/rust-zip', rows_stable, rows_unstable )

add(   '\n### Computation\n', rows_stable, rows_unstable )
entry( '* ', 'eholk/rust-opencl', rows_stable, rows_unstable )

add(   '\n### Cryptography\n', rows_stable, rows_unstable )
entry( '* ', 'DaGenix/rust-crypto', rows_stable, rows_unstable )
entry( '* ', 'dnaq/sodiumoxide', rows_stable, rows_unstable )
entry( '* ', 'klutzy/suruga', rows_stable, rows_unstable )
entry( '* ', 'seb-m/common.rs', rows_stable, rows_unstable )
entry( '* ', 'sfackler/rust-openssl', rows_stable, rows_unstable )

add(   '\n### Database\n', rows_stable, rows_unstable )
add(   '* SQL', rows_stable, rows_unstable )
add(   '  * MySql', rows_stable, rows_unstable )
entry( '    * ', 'blackbeam/rust-mysql-simple', rows_stable, rows_unstable )
add(   '  * PostgreSql', rows_stable, rows_unstable )
entry( '    * ', 'sfackler/rust-postgres', rows_stable, rows_unstable )
add(   '  * Sqlite', rows_stable, rows_unstable )
entry( '    * ', 'linuxfood/rustsqlite', rows_stable, rows_unstable )

add(   '\n### Encoding\n', rows_stable, rows_unstable )
add(   '* Cap\'n Proto', rows_stable, rows_unstable )
entry( '  * ', 'dwrensha/capnproto-rust', rows_stable, rows_unstable )
add(   '* Character Encoding', rows_stable, rows_unstable )
entry( '  * ', 'lifthrasiir/rust-encoding', rows_stable, rows_unstable )
add(   '* CSV', rows_stable, rows_unstable )
entry( '  * ', 'BurntSushi/rust-csv', rows_stable, rows_unstable )
entry( '  * ', 'Geal/rust-csv', rows_stable, rows_unstable )
add(   '* MsgPck', rows_stable, rows_unstable )
entry( '  * ', 'mneumann/rust-msgpack', rows_stable, rows_unstable )
add(   '* ProtocolBuffers', rows_stable, rows_unstable )
entry( '  * ', 'stepancheg/rust-protobuf', rows_stable, rows_unstable )
add(   '* TOML', rows_stable, rows_unstable )
entry( '  * ', 'alexcrichton/toml-rs', rows_stable, rows_unstable )
add(   '* Tnetstring', rows_stable, rows_unstable )
entry( '  * ', 'erickt/rust-tnetstring', rows_stable, rows_unstable )
add(   '* XML', rows_stable, rows_unstable )
entry( '  * ', 'bjz/sax-rs', rows_stable, rows_unstable )
entry( '  * ', 'Florob/RustyXML', rows_stable, rows_unstable )
entry( '  * ', 'netvl/rust-xml', rows_stable, rows_unstable )
entry( '  * ', 'Ygg01/xml-air', rows_stable, rows_unstable )

add(   '\n### Game development\n', rows_stable, rows_unstable )
entry( '* ', 'bjz/bullet-rs', rows_stable, rows_unstable )
entry( '* ', 'JeremyLetang/rustenstein3D', rows_stable, rows_unstable )
entry( '* ', 'PistonDevelopers/piston', rows_stable, rows_unstable )
entry( '* ', 'sebcrozet/kiss3d', rows_stable, rows_unstable )
entry( '* ', 'sebcrozet/ncollide', rows_stable, rows_unstable )
entry( '* ', 'sebcrozet/nphysics', rows_stable, rows_unstable )
entry( '* ', 'SiegeLord/RustAllegro', rows_stable, rows_unstable )

add(   '\n### Games\n', rows_stable, rows_unstable )
entry( '* ', 'Arcterus/game-of-life', rows_stable, rows_unstable )
entry( '* ', 'Arcterus/rust-snake', rows_stable, rows_unstable )
entry( '* ', 'bachm/rusty-tetris', rows_stable, rows_unstable )
entry( '* ', 'bvssvni/rust-snake', rows_stable, rows_unstable )
entry( '* ', 'Coeuvre/rust-2048', rows_stable, rows_unstable )
entry( '* ', 'Coeuvre/rust-pong', rows_stable, rows_unstable )
entry( '* ', 'dpc/rustyhex', rows_stable, rows_unstable )
entry( '* ', 'FrozenCow/rust-airhockey', rows_stable, rows_unstable )
entry( '* ', 'jeaye/q3', rows_stable, rows_unstable )
entry( '* ', 'lifthrasiir/angolmois-rust', rows_stable, rows_unstable )
entry( '* ', 'mynery/xxo', rows_stable, rows_unstable )
entry( '* ', 'ozkriff/marauder', rows_stable, rows_unstable )
entry( '* ', 'rlane/cubeland', rows_stable, rows_unstable )
entry( '* ', 'zokier/pong-rs', rows_stable, rows_unstable )

add(   '\n### GUI\n', rows_stable, rows_unstable )
add(   '* Cocoa', rows_stable, rows_unstable )
entry( '  * ', 'mozilla-servo/rust-cocoa', rows_stable, rows_unstable )
add(   '* Gtk+', rows_stable, rows_unstable )
entry( '  * ', 'JeremyLetang/rgtk', rows_stable, rows_unstable )
add(   '* ncurses', rows_stable, rows_unstable )
entry( '  * ', 'jeaye/ncurses-rs', rows_stable, rows_unstable )
add(   '* OpenGL', rows_stable, rows_unstable )
entry( '  * ', 'bjz/gl-rs', rows_stable, rows_unstable )
entry( '  * ', 'bjz/glfw-rs', rows_stable, rows_unstable )
entry( '  * ', 'servo/rust-glut', rows_stable, rows_unstable )
entry( '  * ', 'servo/rust-opengles', rows_stable, rows_unstable )
add(   '* SDL', rows_stable, rows_unstable )
entry( '  * ', 'AngryLawyer/rust-sdl2', rows_stable, rows_unstable )
entry( '  * ', 'brson/rust-sdl', rows_stable, rows_unstable )
add(   '* SFML', rows_stable, rows_unstable )
entry( '  * ', 'jeremyletang/rust-sfml', rows_stable, rows_unstable )
add(   '* Termbox', rows_stable, rows_unstable )
entry( '  * ', 'gchp/rustbox', rows_stable, rows_unstable )
add(   '* wxWidgets', rows_stable, rows_unstable )
entry( '  * ', 'kenz-gelsoft/wxRust', rows_stable, rows_unstable )

add(   '\n### Image processing\n', rows_stable, rows_unstable )
entry( '* ', 'PistonDevelopers/image', rows_stable, rows_unstable )

add(   '\n### Network programming\n', rows_stable, rows_unstable )
add(   '* ZeroMQ', rows_stable, rows_unstable )
entry( '  * ', 'erickt/rust-zmq', rows_stable, rows_unstable )

add(   '\n### Template engine\n', rows_stable, rows_unstable )
add(   '* Mustache', rows_stable, rows_unstable )
entry( '  * ', 'erickt/rust-mustache', rows_stable, rows_unstable )

add(   '\n### Testing\n', rows_stable, rows_unstable )
entry( '* ', 'BurntSushi/quickcheck', rows_stable, rows_unstable )
entry( '* ', 'farcaller/shiny', rows_stable, rows_unstable )

add(   '\n### Web programming\n', rows_stable, rows_unstable )
add(   'See also ' + md_link('http://arewewebyet.com/', 'http://arewewebyet.com/') + '\n', rows_stable, rows_unstable )
add(   '* Core', rows_stable, rows_unstable )
entry( '  * ', 'chris-morgan/rust-http', rows_stable, rows_unstable )
entry( '  * ', 'hyperium/hyper', rows_stable, rows_unstable )
add(   '* Client', rows_stable, rows_unstable )
entry( '  * ', 'carllerche/curl-rust', rows_stable, rows_unstable )
entry( '  * ', 'vhbit/curl-rs', rows_stable, rows_unstable )
add(   '* Server', rows_stable, rows_unstable )
entry( '  * ', 'erickt/rust-mongrel2', rows_stable, rows_unstable )
entry( '  * ', 'Iron', rows_stable, rows_unstable )
entry( '  * ', 'Nickel', rows_stable, rows_unstable )

add(   '\n## Resources\n', rows_stable, rows_unstable )
entry( '* ', 'Rust by Example', rows_stable, rows_unstable )
entry( '* ', 'Rust CI', rows_stable, rows_unstable )
entry( '* ', 'Rust Guidelines', rows_stable, rows_unstable )

markdown_stable = '\n'.join(rows_stable)
markdown_unstable = '\n'.join(rows_unstable )

with open( 'README.md', 'wb' ) as f:
    f.write( markdown_stable )
with open( 'UNSTABLE.md', 'wb' ) as f:
    f.write( markdown_unstable )
