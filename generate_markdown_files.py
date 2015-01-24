# -*- coding: utf-8 -*-
"""
Generates the files `README.md` and `UNSTABLE.md`.
It helps keep the content in a sane format and also allows for entries
to be easily switched between stable/unstable status.
"""


def md_link(name, url):
    """
    Returns a web link in Markdown format
    """
    return '[%s](%s)' % (name, url)


"""
        '': {
            "descr": md_link('', '') + ' bindings',
            "unstable": False
        },
"""
DATA = \
    {
        'mattnenterprise/rust-nntp': {
            "descr": 'A ' + md_link('NNTP', 'http://en.wikipedia.org/wiki/Network_News_Transfer_Protocol') + ' client for Rust',
            "unstable": False
        },
        'mattnenterprise/rust-pop3': {
            "descr": 'A ' + md_link('POP3', 'http://en.wikipedia.org/wiki/Post_Office_Protocol') + ' client for Rust',
            "unstable": False
        },
        'mattnenterprise/rust-ftp': {
            "descr": 'An ' + md_link('FTP', 'http://en.wikipedia.org/wiki/File_Transfer_Protocol') + ' client for Rust',
            "unstable": False
        },
        'AngryLawyer/rust-sdl2': {
            'descr': "[SDL2](http://www.libsdl.org/) bindings",
            'unstable': False,
        },
        'Arcterus/game-of-life': {
            'unstable': True,
        },
        'Arcterus/rust-snake': {
            'unstable': True,
        },
        'BurntSushi/quickcheck': {
            'descr': "a Rust implementation of [QuickCheck](http://www.haskell.org/haskellwiki/Introduction_to_QuickCheck1)",
            'unstable': False,
        },
        'BurntSushi/rust-csv': {
            'unstable': False,
        },
        'Cargo': {
            'descr': "the Rust package manager",
            'unstable': False,
            'travis_url': None,
            'url': 'http://crates.io/',
        },
        'Coeuvre/rust-2048': {
            'travis_url': None,
            'unstable': False,
        },
        'Coeuvre/rust-pong': {
            'unstable': True,
        },
        'DaGenix/rust-crypto': {
            'descr': "cryptographic algorithms in Rust",
            'unstable': False,
        },
        'Florob/RustyXML': {
            'descr': "an XML parser written in Rust",
            'unstable': False,
        },
        'FrozenCow/rust-airhockey': {
            'unstable': True,
        },
        'Geal/rust-csv': {
            'unstable': True,
        },
        'Iron': {
            'descr': "a middleware-based server framework",
            'travis_url': 'https://travis-ci.org/iron/iron',
            'travis_badge': 'https://travis-ci.org/iron/iron.svg?branch=master',
            'unstable': False,
            'url': 'http://ironframework.io/',
        },
        'JeremyLetang/ears': {
            'descr': "a simple library to play Sounds and Musics, on top of OpenAL and libsndfile",
            'travis_url': None,
            'unstable': False,
        },
        'JeremyLetang/rgtk': {
            'descr': "[Gtk+](http://www.gtk.org) bindings",
            'travis_url': None,
            'unstable': False,
        },
        'JeremyLetang/rust-portaudio': {
            'descr': "[PortAudio](http://www.portaudio.com/) bindings",
            'travis_url': None,
            'unstable': False,
        },
        'JeremyLetang/rustenstein3D': {
            'descr': "a raycasting engine in rust",
            'unstable': True,
        },
        'Nickel': {
            'descr': "inspired by [Express](http://expressjs.com/)",
            'travis_badge': 'https://travis-ci.org/nickel-org/nickel.rs.svg?branch=master',
            'travis_url': 'https://travis-ci.org/nickel-org/nickel.rs',
            'unstable': False,
            'url': 'http://nickel.rs/',
        },
        'Ogeon/rustful': {
            'descr': "a RESTful web framework for Rust ",
            'unstable': False,
        },
        'PistonDevelopers/image': {
            'descr': "Basic imaging processing functions and methods for converting to and from image formats",
            'unstable': False,
        },
        'PistonDevelopers/piston': {
            'unstable': False,
        },
        'Rust CI': {
            'descr': "a [Travis CI](https://travis-ci.com) dashboard for Rust projects",
            'travis_url': None,
            'unstable': False,
            'url': 'http://www.rust-ci.org/',
        },
        'Rust Guidelines': {
            'travis_url': None,
            'unstable': False,
            'url': 'http://aturon.github.io/',
        },
        'Rust by Example': {
            'travis_url': None,
            'unstable': False,
            'url': 'http://rustbyexample.com/',
        },
        'Rustless': {
            'descr': "a REST-like API micro-framework inspired by [Grape](https://github.com/intridea/grape) and [Hyper](https://github.com/hyperium/hyper)",
            'travis_badge': 'https://travis-ci.org/rustless/rustless.svg?branch=master',
            'travis_url': 'https://travis-ci.org/rustless/rustless',
            'unstable': False,
            'url': 'http://rustless.org/',
        },
        'Servo': {
            'descr': "a prototype web browser engine written in Rust",
            'unstable': False,
            'travis_url': None,
            'url': 'https://github.com/servo/servo',
        },
        'SiegeLord/RustAllegro': {
            'descr': "[Allegro 5](http://liballeg.org/) bindings",
            'unstable': False,
        },
        'SiegeLord/RustCMake': {
            'descr': "an example project showing usage of CMake with Rust",
            'unstable': False,
        },
        'Teepee': {
            'unstable': True,
        },
        'TyOverby/bincode': {
            'descr': "a binary encoder/decoder in Rust",
            'unstable': False,
        },
        'Ygg01/xml-air': {
            'descr': "A hybrid pull, DOM parser written in pure Rust",
            'unstable': True,
        },
        'alexcrichton/bzip2-rs': {
            'descr': "[libbz2](http://www.bzip.org) bindings",
            'unstable': False,
        },
        'alexcrichton/flate2-rs': {
            'descr': "[miniz](https://code.google.com/p/miniz/) bindings",
            'unstable': False,
        },
        'alexcrichton/ssh2-rs': {
            'descr': "[libssh2](http://www.libssh2.org/) bindings",
            'unstable': False,
        },
        'alexcrichton/tar-rs': {
            'descr': "tar archive reading/writing in Rust",
            'unstable': False,
        },
        'alexcrichton/toml-rs': {
            'unstable': False,
        },
        'arjantop/rust-bencode': {
            'descr': "[Bencode](http://en.wikipedia.org/wiki/Bencode) implementation in Rust",
            'unstable': False,
        },
        'bachm/rusty-tetris': {
            'unstable': True,
        },
        'bbodi/rust-voxlap': {
            'descr': "[Voxlap](http://advsys.net/ken/voxlap.htm) bindings",
            'travis_url': None,
            'unstable': False,
        },
        'bjz/bullet-rs': {
            'unstable': True,
        },
        'bjz/gl-rs': {
            'unstable': False,
        },
        'bjz/glfw-rs': {
            'unstable': False,
        },
        'bjz/openal-rs': {
            'descr': "[OpenAL 1.1](http://www.openal.org/) bindings",
            'unstable': False,
        },
        'blackbeam/rust-mysql-simple': {
            'descr': "a native MySql client",
            'unstable': False,
        },
        'brson/rust-sdl': {
            'descr': "[SDL1](http://www.libsdl.org/) bindings",
            'unstable': False,
        },
        'bvssvni/rust-snake': {
            'unstable': True,
        },
        'carllerche/curl-rust': {
            'descr': "[libcurl](http://curl.haxx.se/libcurl/) bindings",
            'unstable': False,
        },
        'carllerche/nix-rust': {
            'descr': "Linux API bindings",
            'unstable': False,
        },
        'chris-morgan/rust-http': {
            'descr': "will be replaced by [Teepee](http://teepee.rs/)",
            'unstable': False,
        },
        'cyndis/qmlrs': {
            'descr': "[QtQuick](http://doc.qt.io) bindings",
            'unstable': False,
        },
        'dnaq/sodiumoxide': {
            'descr': "[libsodium](https://github.com/jedisct1/libsodium) bindings",
            'unstable': False,
        },
        'docopt/docopt.rs': {
            'descr': "a Rust implementation of [DocOpt](http://docopt.org)",
            'unstable': False,
        },
        'dpc/rustyhex': {
            'unstable': True,
        },
        'dwrensha/capnproto-rust': {
            'unstable': False,
        },
        'erickt/rust-mongrel2': {
            'descr': "[Mongrel2](http://mongrel2.org) bindings",
            'unstable': True,
        },
        'erickt/rust-mustache': {
            'unstable': True,
        },
        'erickt/rust-tnetstring': {
            'unstable': False,
        },
        'erickt/rust-zmq': {
            'descr': "[ZeroMQ](http://zeromq.org) bindings",
            'unstable': True,
        },
        'farcaller/shiny': {
            'descr': "a fancy syntax similar to Ruby's Rspec or Objective-C' kiwi",
            'unstable': False,
        },
        'gchp/iota': {
            'descr': "a simple text editor written in Rust",
            'unstable': False,
        },
        'gchp/rustbox': {
            'descr': "a Rust implementation of [termbox](http://github.com/nsf/termbox)",
            'unstable': False,
        },
        'hyperium/hyper': {
            'descr': "an HTTP implementation",
            'unstable': False,
        },
        'jeaye/ncurses-rs': {
            'unstable': False,
        },
        'jeaye/q3': {
            'unstable': True,
        },
        'jeremyletang/rust-sfml': {
            'descr': "[SFML](http://www.sfml-dev.org/) bindings",
            'unstable': False,
        },
        'kenz-gelsoft/wxRust': {
            'descr': "[wxWidgets](http://www.wxwidgets.org/) bindings",
            'unstable': True,
        },
        'kimhyunkang/libyaml-rust': {
            'descr': "[libyaml](http://pyyaml.org/wiki/LibYAML) bindings",
            'unstable': False,
        },
        'klutzy/suruga': {
            'descr': "a Rust implementation of [TLS 1.2](http://tools.ietf.org/html/rfc5246)",
            'travis_url': None,
            'unstable': False,
        },
        'libpnet/libpnet': {
            'descr': "Cross-platform, low level networking",
            'unstable': False,
        },
        'lifthrasiir/angolmois-rust': {
            'descr': "a minimalistic music video game which supports the BMS format",
            'unstable': False,
        },
        'lifthrasiir/rust-chrono': {
            'descr': "",
            'unstable': False,
        },
        'lifthrasiir/rust-encoding': {
            'unstable': False,
        },
        'lifthrasiir/rust-zip': {
            'descr': "read and write ZIP archives",
            'unstable': False,
        },
        'linuxfood/rustsqlite': {
            'descr': "[Sqlite3](http://www.sqlite.org/) bindings",
            'unstable': False,
        },
        'luqmana/rust-opencl': {
            'descr': "[OpenCL](https://www.khronos.org/opencl/) bindings",
            'unstable': False,
        },
        'mitsuhiko/redis-rs': {
            'descr': "[Redis](http://redis.io) library in Rust",
            'unstable': False,
        },
        'mneumann/rust-msgpack': {
            'unstable': False,
        },
        'mozilla-servo/rust-cocoa': {
            'travis_url': None,
            'unstable': False,
        },
        'mynery/xxo': {
            'unstable': True,
        },
        'netvl/xml-rs': {
            'descr': "a streaming XML library",
            'unstable': False,
        },
        'ozkriff/marauder': {
            'unstable': True,
        },
        'rlane/cubeland': {
            'unstable': True,
        },
        'rust-lang/time': {
            'descr': "",
            'unstable': False,
        },
        'rustache/rustache': {
            'unstable': False,
        },
        'samdoshi/portmidi-rs': {
            'descr': "[PortMidi](http://portmedia.sourceforge.net/portmidi/) bindings",
            'unstable': False,
        },
        'schickling/rust-beanstalkd': {
            'descr': "[Beanstalkd](https://github.com/kr/beanstalkd) bindings",
            'unstable': False,
        },
        'seb-m/common.rs': {
            'descr': "Common Rust crypto utilities",
            'unstable': False,
        },
        'sebcrozet/kiss3d': {
            'unstable': True,
        },
        'sebcrozet/ncollide': {
            'unstable': True,
        },
        'sebcrozet/nphysics': {
            'unstable': True,
        },
        'servo/html5ever': {
            'descr': "High-performance browser-grade HTML5 parser",
            'unstable': False,
        },
        'servo/rust-glut': {
            'travis_url': None,
            'unstable': False,
        },
        'servo/rust-opengles': {
            'unstable': True,
        },
        'sfackler/rust-openssl': {
            'descr': "[OpenSSL](https://www.openssl.org/) bindings",
            'unstable': False,
        },
        'sfackler/rust-postgres': {
            'descr': "a native [PostgreSQL](http://www.postgresql.org) client",
            'unstable': False,
        },
        'stepancheg/rust-protobuf': {
            'unstable': False,
        },
        'thehydroimpulse/nanomsg.rs': {
            'descr': "a modern messaging library that is the successor to ZeroMQ",
            'unstable': False,
        },
        'thestinger/rust-gmp': {
            'descr': "[libgmp](https://gmplib.org/) bindings",
            'unstable': False,
        },
        'tomaka/android-rs-glue': {
            'descr': "glue between Rust and Android",
            'unstable': False,
        },
        'tomaka/glutin': {
            'descr': "Rust alternative to [GLFW](http://www.glfw.org/)",
            'unstable': False,
        },
        'tomjakubowski/rethinkdb-rs': {
            'descr': "[RethinkDB](http://www.rethinkdb.com) bindings",
            'unstable': True,
        },
        'uutils/coreutils': {
            'descr': "cross-platform Rust rewrite of the GNU coreutils",
            'unstable': False,
        },
        'vhbit/ObjCrust': {
            'descr': "using Rust to create an iOS static library",
            'unstable': False,
        },
        'vhbit/curl-rs': {
            'descr': "[libcurl](http://curl.haxx.se/libcurl/) bindings",
            'unstable': False,
        },
        'wycats/hammer.rs': {
            'unstable': True,
        },
        'zeromq/zmq.rs': {
            'descr': "Rust implementation of the [ZeroMQ](http://zeromq.org)protocol",
            'unstable': True,
        },
        'zokier/pong-rs': {
            'unstable': True,
        },
        'zslayton/stomp-rs': {
            'descr': "[STOMP 1.2](http://stomp.github.io/stomp-specification-1.2.html) client implementation in Rust",
            'unstable': False,
        },
    }


def pretty_print():
    """
    """

    projects_names = DATA.keys()
    projects_names.sort()

    print '    {'

    for projects_name in projects_names:

        project_data = DATA[projects_name]
        project_data.setdefault('unstable', False)

        print "        '%s': {" % projects_name

        keys = project_data.keys()
        keys.sort()

        for key in keys:

            value = project_data[key]

            if type(value) == str:
                value_format = '"%s"' % value
            else:
                value_format = "%s" % value

            print "            '%s': %s," % (key, value_format)

        print "        },"

    print '    }'

# end def pretty_print


HEADER_STABLE = \
"""# Awesome Rust

A curated list of awesome Rust code and resources. Inspired by the other [awesome lists](https://github.com/bayandin/awesome-awesomeness).

Only projects that are stable and useful to users are added. Projects that do not compile with Rust-nightly for a longer time are moved to [UNSTABLE.md](UNSTABLE.md)."""

HEADER_UNSTABLE = \
"""# Unstable

A list of awesome but unstable/experimental Rust projects which some day hopefully will be migrated to `README.md`."""

TOC = \
"""
- [Awesome Rust](#awesome-rust)
  - [Applications](#applications)
    - [Games](#games)
  - [Frameworks](#frameworks)
    - [Audio](#audio)
    - [Build system](#build-system)
    - [Command-line argument parsing](#command-line-argument-parsing)
    - [Compression](#compression)
    - [Computation](#computation)
    - [Cryptography](#cryptography)
    - [Database](#database)
    - [Date and time](#date-and-time)
    - [Encoding](#encoding)
    - [Game development](#game-development)
    - [Games](#games)
    - [GUI](#gui)
    - [Image processing](#image-processing)
    - [Mobile](#mobile)
    - [Network programming](#network-programming)
    - [Platform specific](#platform-specific)
    - [Template engine](#template-engine)
    - [Testing](#testing)
    - [Web programming](#web-programming)
  - [Resources](#resources)
"""


def add(value, rows_stable=None, rows_unstable=None):
    if rows_stable:
        rows_stable.append(value)
    if rows_unstable:
        rows_unstable.append(value)


# end def add


def entry(prefix, project_name, rows_stable=None, rows_unstable=None):

    # Get the project data.
    project_data = DATA.get(project_name)

    if project_data is None:
        print '"%s" not found' % project_name
        return

    url = project_data.get('url', 'https://github.com/%s' % project_name)
    description = project_data.get('descr', '')

    if description:
        description = ' ' + description


    if 'travis_badge' in project_data:
        travis_badge = project_data.get('travis_badge')
    else:
        travis_badge = 'https://travis-ci.org/%s.svg?branch=master' % project_name


    if 'travis_url' in project_data:
        travis_url = project_data.get('travis_url')
    else:
        travis_url = 'https://travis-ci.org/%s' % project_name


    if travis_badge and travis_url:
        travis = ' ' + md_link('<img src="%s">' % travis_badge, travis_url)
    else:
        travis = ''


    if description or travis:
        separator = ' —'
    else:
        separator = ''

    unstable = project_data.get('unstable')

    if unstable:
        rows = rows_unstable
    else:
        rows = rows_stable

    rows.append('%s[%s](%s)%s%s%s' % (prefix, project_name, url, separator, description, travis))

#end def entry


# Generate Markdown files.
rows_stable = [HEADER_STABLE, TOC]
rows_unstable = [HEADER_UNSTABLE, TOC]

add('## Applications\n', rows_stable, rows_unstable)
entry('* ', 'gchp/iota', rows_stable, rows_unstable)
entry('* ', 'uutils/coreutils', rows_stable, rows_unstable)
entry('* ', 'Servo', rows_stable, rows_unstable)

add('\n### Games\n', rows_stable, rows_unstable)
entry('* ', 'Arcterus/game-of-life', rows_stable, rows_unstable)
entry('* ', 'Arcterus/rust-snake', rows_stable, rows_unstable)
entry('* ', 'bachm/rusty-tetris', rows_stable, rows_unstable)
entry('* ', 'bvssvni/rust-snake', rows_stable, rows_unstable)
entry('* ', 'Coeuvre/rust-2048', rows_stable, rows_unstable)
entry('* ', 'Coeuvre/rust-pong', rows_stable, rows_unstable)
entry('* ', 'dpc/rustyhex', rows_stable, rows_unstable)
entry('* ', 'FrozenCow/rust-airhockey', rows_stable, rows_unstable)
entry('* ', 'jeaye/q3', rows_stable, rows_unstable)
entry('* ', 'lifthrasiir/angolmois-rust', rows_stable, rows_unstable)
entry('* ', 'mynery/xxo', rows_stable, rows_unstable)
entry('* ', 'ozkriff/marauder', rows_stable, rows_unstable)
entry('* ', 'rlane/cubeland', rows_stable, rows_unstable)
entry('* ', 'zokier/pong-rs', rows_stable, rows_unstable)

add('\n## Frameworks\n', rows_stable, rows_unstable)

add('\n### Audio\n', rows_stable, rows_unstable)
entry('* ', 'bjz/openal-rs', rows_stable, rows_unstable)
entry('* ', 'JeremyLetang/ears', rows_stable, rows_unstable)
entry('* ', 'JeremyLetang/rust-portaudio', rows_stable, rows_unstable)
entry('* ', 'samdoshi/portmidi-rs', rows_stable, rows_unstable)

add('\n### Build system\n', rows_stable, rows_unstable)
entry('* ', 'Cargo', rows_stable, rows_unstable)
add('* CMake', rows_stable, rows_unstable)
entry('  * ', 'SiegeLord/RustCMake', rows_stable, rows_unstable)

add('\n### Command-line argument parsing\n', rows_stable, rows_unstable)
entry('* ', 'docopt/docopt.rs', rows_stable, rows_unstable)
entry('* ', 'wycats/hammer.rs', rows_stable, rows_unstable)

add('\n### Compression\n', rows_stable, rows_unstable)
entry('* ', 'alexcrichton/bzip2-rs', rows_stable, rows_unstable)
entry('* ', 'alexcrichton/flate2-rs', rows_stable, rows_unstable)
entry('* ', 'alexcrichton/tar-rs', rows_stable, rows_unstable)
entry('* ', 'lifthrasiir/rust-zip', rows_stable, rows_unstable)

add('\n### Computation\n', rows_stable, rows_unstable)
entry('* ', 'luqmana/rust-opencl', rows_stable, rows_unstable)
entry('* ', 'thestinger/rust-gmp', rows_stable, rows_unstable)

add('\n### Cryptography\n', rows_stable, rows_unstable)
entry('* ', 'DaGenix/rust-crypto', rows_stable, rows_unstable)
entry('* ', 'dnaq/sodiumoxide', rows_stable, rows_unstable)
entry('* ', 'klutzy/suruga', rows_stable, rows_unstable)
entry('* ', 'seb-m/common.rs', rows_stable, rows_unstable)
entry('* ', 'sfackler/rust-openssl', rows_stable, rows_unstable)

add('\n### Database\n', rows_stable, rows_unstable)
add('* NoSQL', rows_stable, rows_unstable)
add('  * Redis', rows_stable, rows_unstable)
entry('    * ', 'mitsuhiko/redis-rs', rows_stable, rows_unstable)
add('  * RethinkDB', rows_stable, rows_unstable)
entry('    * ', 'tomjakubowski/rethinkdb-rs', rows_stable, rows_unstable)
add('* SQL', rows_stable, rows_unstable)
add('  * MySql', rows_stable, rows_unstable)
entry('    * ', 'blackbeam/rust-mysql-simple', rows_stable, rows_unstable)
add('  * PostgreSql', rows_stable, rows_unstable)
entry('    * ', 'sfackler/rust-postgres', rows_stable, rows_unstable)
add('  * Sqlite', rows_stable, rows_unstable)
entry('    * ', 'linuxfood/rustsqlite', rows_stable, rows_unstable)

add('\n### Date and time\n', rows_stable, rows_unstable)
entry('* ', 'lifthrasiir/rust-chrono', rows_stable, rows_unstable)
entry('* ', 'rust-lang/time', rows_stable, rows_unstable)

add('\n### Encoding\n', rows_stable, rows_unstable)
entry('* ', 'TyOverby/bincode', rows_stable, rows_unstable)
add('* Bencode', rows_stable, rows_unstable)
entry('  * ', 'arjantop/rust-bencode', rows_stable, rows_unstable)
add('* Cap\'n Proto', rows_stable, rows_unstable)
entry('  * ', 'dwrensha/capnproto-rust', rows_stable, rows_unstable)
add('* Character Encoding', rows_stable, rows_unstable)
entry('  * ', 'lifthrasiir/rust-encoding', rows_stable, rows_unstable)
add('* CSV', rows_stable, rows_unstable)
entry('  * ', 'BurntSushi/rust-csv', rows_stable, rows_unstable)
entry('  * ', 'Geal/rust-csv', rows_stable, rows_unstable)
add('* HTML', rows_stable, rows_unstable)
entry('  * ', 'servo/html5ever', rows_stable, rows_unstable)
add('* MsgPck', rows_stable, rows_unstable)
entry('  * ', 'mneumann/rust-msgpack', rows_stable, rows_unstable)
add('* ProtocolBuffers', rows_stable, rows_unstable)
entry('  * ', 'stepancheg/rust-protobuf', rows_stable, rows_unstable)
add('* TOML', rows_stable, rows_unstable)
entry('  * ', 'alexcrichton/toml-rs', rows_stable, rows_unstable)
add('* Tnetstring', rows_stable, rows_unstable)
entry('  * ', 'erickt/rust-tnetstring', rows_stable, rows_unstable)
add('* XML', rows_stable, rows_unstable)
entry('  * ', 'Florob/RustyXML', rows_stable, rows_unstable)
entry('  * ', 'netvl/xml-rs', rows_stable, rows_unstable)
entry('  * ', 'Ygg01/xml-air', rows_stable, rows_unstable)
add('* YAML', rows_stable, rows_unstable)
entry('  * ', 'kimhyunkang/libyaml-rust', rows_stable, rows_unstable)

add('\n### Game development\n', rows_stable, rows_unstable)
entry('* ', 'bbodi/rust-voxlap', rows_stable, rows_unstable)
entry('* ', 'bjz/bullet-rs', rows_stable, rows_unstable)
entry('* ', 'JeremyLetang/rustenstein3D', rows_stable, rows_unstable)
entry('* ', 'PistonDevelopers/piston', rows_stable, rows_unstable)
entry('* ', 'sebcrozet/kiss3d', rows_stable, rows_unstable)
entry('* ', 'sebcrozet/ncollide', rows_stable, rows_unstable)
entry('* ', 'sebcrozet/nphysics', rows_stable, rows_unstable)
entry('* ', 'SiegeLord/RustAllegro', rows_stable, rows_unstable)

add('\n### GUI\n', rows_stable, rows_unstable)
add('* Cocoa', rows_stable, rows_unstable)
entry('  * ', 'mozilla-servo/rust-cocoa', rows_stable, rows_unstable)
add('* Gtk+', rows_stable, rows_unstable)
entry('  * ', 'JeremyLetang/rgtk', rows_stable, rows_unstable)
add('* ncurses', rows_stable, rows_unstable)
entry('  * ', 'jeaye/ncurses-rs', rows_stable, rows_unstable)
add('* OpenGL', rows_stable, rows_unstable)
entry('  * ', 'bjz/gl-rs', rows_stable, rows_unstable)
entry('  * ', 'bjz/glfw-rs', rows_stable, rows_unstable)
entry('  * ', 'servo/rust-glut', rows_stable, rows_unstable)
entry('  * ', 'servo/rust-opengles', rows_stable, rows_unstable)
entry('  * ', 'tomaka/glutin', rows_stable, rows_unstable)
add('* Qt', rows_stable, rows_unstable)
entry('  * ', 'cyndis/qmlrs', rows_stable, rows_unstable)
add('* SDL', rows_stable, rows_unstable)
entry('  * ', 'AngryLawyer/rust-sdl2', rows_stable, rows_unstable)
entry('  * ', 'brson/rust-sdl', rows_stable, rows_unstable)
add('* SFML', rows_stable, rows_unstable)
entry('  * ', 'jeremyletang/rust-sfml', rows_stable, rows_unstable)
add('* Termbox', rows_stable, rows_unstable)
entry('  * ', 'gchp/rustbox', rows_stable, rows_unstable)
add('* wxWidgets', rows_stable, rows_unstable)
entry('  * ', 'kenz-gelsoft/wxRust', rows_stable, rows_unstable)

add('\n### Image processing\n', rows_stable, rows_unstable)
entry('* ', 'PistonDevelopers/image', rows_stable, rows_unstable)

add('\n### Mobile\n', rows_stable, rows_unstable)
entry('* ', 'tomaka/android-rs-glue', rows_stable, rows_unstable)
entry('* ', 'vhbit/ObjCrust', rows_stable, rows_unstable)

add('\n### Network programming\n', rows_stable, rows_unstable)
add('* Low level', rows_stable, rows_unstable)
entry('  * ', 'libpnet/libpnet', rows_stable, rows_unstable)
add('* Beanstalkd', rows_stable, rows_unstable)
entry('  * ', 'schickling/rust-beanstalkd', rows_stable, rows_unstable)
add('* FTP', rows_stable, rows_unstable)
entry('  * ', 'mattnenterprise/rust-ftp', rows_stable, rows_unstable)
add('* NanoMsg', rows_stable, rows_unstable)
entry('  * ', 'thehydroimpulse/nanomsg.rs', rows_stable, rows_unstable)
add('* NNTP', rows_stable, rows_unstable)
entry('  * ', 'mattnenterprise/rust-nntp', rows_stable, rows_unstable)
add('* POP3', rows_stable, rows_unstable)
entry('  * ', 'mattnenterprise/rust-pop3', rows_stable, rows_unstable)
add('* SSH', rows_stable, rows_unstable)
entry('  * ', 'alexcrichton/ssh2-rs', rows_stable, rows_unstable)
add('* Stomp', rows_stable, rows_unstable)
entry('  * ', 'zslayton/stomp-rs', rows_stable, rows_unstable)
add('* ZeroMQ', rows_stable, rows_unstable)
entry('  * ', 'erickt/rust-zmq', rows_stable, rows_unstable)

add('\n### Platform specific\n', rows_stable, rows_unstable)
add('* Linux', rows_stable, rows_unstable)
entry('  * ', 'carllerche/nix-rust', rows_stable, rows_unstable)

add('\n### Template engine\n', rows_stable, rows_unstable)
add('* Mustache', rows_stable, rows_unstable)
entry('  * ', 'erickt/rust-mustache', rows_stable, rows_unstable)
entry('  * ', 'rustache/rustache', rows_stable, rows_unstable)

add('\n### Testing\n', rows_stable, rows_unstable)
entry('* ', 'BurntSushi/quickcheck', rows_stable, rows_unstable)
entry('* ', 'farcaller/shiny', rows_stable, rows_unstable)

add('\n### Web programming\n', rows_stable, rows_unstable)
add('See also ' + md_link('http://arewewebyet.com/', 'http://arewewebyet.com/') + '\n', rows_stable, rows_unstable)
add('* Core', rows_stable, rows_unstable)
entry('  * ', 'chris-morgan/rust-http', rows_stable, rows_unstable)
entry('  * ', 'hyperium/hyper', rows_stable, rows_unstable)
entry('  * ', 'Teepee', rows_stable, rows_unstable)
add('* Client', rows_stable, rows_unstable)
entry('  * ', 'carllerche/curl-rust', rows_stable, rows_unstable)
entry('  * ', 'vhbit/curl-rs', rows_stable, rows_unstable)
add('* Server', rows_stable, rows_unstable)
entry('  * ', 'erickt/rust-mongrel2', rows_stable, rows_unstable)
entry('  * ', 'Iron', rows_stable, rows_unstable)
entry('  * ', 'Nickel', rows_stable, rows_unstable)
entry('  * ', 'Ogeon/rustful', rows_stable, rows_unstable)
entry('  * ', 'Rustless', rows_stable, rows_unstable)

add('\n## Resources\n', rows_stable, rows_unstable)
entry('* ', 'Rust by Example', rows_stable, rows_unstable)
entry('* ', 'Rust CI', rows_stable, rows_unstable)
entry('* ', 'Rust Guidelines', rows_stable, rows_unstable)

markdown_stable = '\n'.join(rows_stable)
markdown_unstable = '\n'.join(rows_unstable)

with open('README.md', 'wb') as f:
    f.write(markdown_stable)
with open('UNSTABLE.md', 'wb') as f:
    f.write(markdown_unstable)
