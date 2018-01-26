# boxxy-rs [![Build Status][travis-img]][travis] [![Build status][appveyor-img]][appveyor] [![crates.io][crates-img]][crates] [![docs.rs][docs-img]][docs]

[travis-img]:   https://travis-ci.org/kpcyrd/boxxy-rs.svg?branch=master
[travis]:       https://travis-ci.org/kpcyrd/boxxy-rs
[appveyor-img]: https://ci.appveyor.com/api/projects/status/yd8xlom2h9v4yi2s/branch/master?svg=true
[appveyor]:     https://ci.appveyor.com/project/kpcyrd/boxxy-rs/branch/master
[crates-img]:   https://img.shields.io/crates/v/boxxy.svg
[crates]:       https://crates.io/crates/boxxy
[docs-img]:     https://docs.rs/boxxy/badge.svg
[docs]:         https://docs.rs/boxxy

"_If you implement boundaries and nobody is around to push them, do they even
exist?_". Have you ever wondered how your sandbox looks like from the inside?
Tempted to test if you can escape it, if only you had a shell to give it a try?
boxxy is a library that can be linked into a debug build of an existing program
and drop you into an interactive shell. From there you can step through various
stages of your sandbox and verify it actually contains™.

## Development

    cargo run --example boxxy

## Linking with rust

Just put a dev-dependencies in your Cargo.toml and copy `examples/boxxy.rs` to
your `examples/` folder. Modify to include your sandbox.

    [dev-dependencies]
    boxxy = "*"

## Linking with C

There is an example program, check the Makefile to see how it's built.

    make cboxxy

## Calling into machinecode

     [%]> # just RET to prompt
     [%]> jit ww==
     [%]> # print ohai and exit
     [%]> jit 6xpeuAEAAABIice6BQAAAA8FuDwAAABIMf8PBejh////b2hhaQo=

You can use the `objdump` utility to generate shellcode from assembly:

    make sc/ohai && cargo run --example objdump sc/ohai

## Debugging systemd security

There is a special ipc binary that automatically swaps its stdio interface with
an unix domain socket so it can be used to debug security settings of a systemd
unit.

Prepare `ipc-boxxy`:

    cargo build --release --example ipc-boxxy
    install -Dm755 target/release/examples/ipc-boxxy /usr/local/bin/ipc-boxxy

Prepare systemd unit:

    sudo tee /etc/systemd/system/ipc-boxxy@.service <<EOF
    [Unit]
    Description=ipc boxxy debugger

    [Service]
    User=root
    ExecStart=/usr/local/bin/ipc-boxxy /run/boxxy-%i.sock

    NoNewPrivileges=yes
    ProtectSystem=strict
    ProtectHome=true
    PrivateTmp=true
    PrivateDevices=true
    ProtectKernelTunables=true
    ProtectKernelModules=true
    ProtectControlGroups=true
    RestrictAddressFamilies=AF_UNIX
    MemoryDenyWriteExecute=true
    CapabilityBoundingSet=
    InaccessiblePaths=-/etc/ssh

    EOF

Attach to shell:

    sudo nc -Ul /run/boxxy-foo.sock &
    sudo systemctl start ipc-boxxy@foo
    fg

You can run arbitrary commands with `exec`, but stdio is still attached to the
original process instead of your socket, to fix this setup a new listener in a
different terminal:

    sudo nc -Ul /run/boxxy-foo2.sock

And attach /bin/sh to this socket with ncat from your boxxy session:

    exec ncat -c sh\ -i\ 2>&1 -U /run/boxxy-foo2.sock

## Examples

There are vulnerable sandboxes (`examples/vuln-*`) as a challenge that can be
exploited using the boxxy shell (no need to compile any exploits).

**DO NOT POST SPOILERS**

Start a challenge using eg. `cargo run --example vuln-chroot`

## Warning

The shell is a basic interface for human input, do not write actual scripts,
there be dragons.

**Do not include boxxy in production builds.**

## License

This project is free software released under the LGPL3+ license.
