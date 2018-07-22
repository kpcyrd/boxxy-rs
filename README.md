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
    boxxy = "0.*"

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

## Invoking from php

See [autoboxxy](autoboxxy/) for tooling to load boxxy from php, even if
`shell_exec` and friends are disabled by php.ini.

## Static binary

You may need to build a fully static binary, this is possible using the
`x86_64-unknown-linux-musl` target.

    cargo build --release --example boxxy --target x86_64-unknown-linux-musl
    strip target/x86_64-unknown-linux-musl/release/examples/boxxy

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

    sudo target/debug/ipc-listener /run/boxxy-foo.sock 'systemctl start ipc-boxxy@foo'

You can run arbitrary commands with `exec`:

    exec bash -i

## AWS lambda

The example folder contains a reimplementation of lambdash, it automatically
deploys boxxy as an aws lambda and allows you to execute commands on it. The
client supports cross account access, but needs a preconfigured role that the
lambda should use. You need to build a [static binary](#static-binary) first.

    cargo run --features=aws --example lambdash -- \
        --assume-role arn:aws:iam::133713371337:role/AdminRole \
        --role arn:aws:iam::133337133337:role/lambda-test-role
        eu-west-1 boxxy

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
