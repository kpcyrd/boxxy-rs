### Note: this is currently broken :/

There (used) to be a feature to securely back-connect to a remote listener with
tls. Instead of using the CA trust store the `revshell` command expects the
certificates fingerprint to be passed as argument.

Generate a tls key for the listener (none of these fields matter):

```
$ openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout tls.key -out tls.crt
Generating a RSA private key
....+++++
..........+++++
writing new private key to 'tls.key'
-----
You are about to be asked to enter information that will be incorporated
into your certificate request.
What you are about to enter is what is called a Distinguished Name or a DN.
There are quite a few fields but you can leave some blank
For some fields there will be a default value,
If you enter '.', the field will be left blank.
-----
Country Name (2 letter code) [AU]:.
State or Province Name (full name) [Some-State]:.
Locality Name (eg, city) []:.
Organization Name (eg, company) [Internet Widgits Pty Ltd]:.
Organizational Unit Name (eg, section) []:.
Common Name (e.g. server FQDN or YOUR name) []:example.com
Email Address []:.
$
```

Calculate the fingerprint of the newly generated certificate:

```
cargo run --example fingerprint tls.crt
```

Run the listener:

```
ncat -lvnp 1337 --ssl --ssl-cert tls.crt --ssl-key tls.key
```

Compile boxxy with the network features:

```
$ cargo run --all-features --example boxxy
   Compiling boxxy v0.12.1 (/home/user/repos/kpcyrd/boxxy-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 16.93s
     Running `target/debug/examples/boxxy`
 [%]> revshell 127.0.0.1:1337 SHA256-FytW1slP5zTMKIq7814yrWRbZqAypnjmgo3jl0CQTzI
[*] connecting to 127.0.0.1:1337...
[+] connected!
[+] established encrypted connection
[*] see you on the other side...
```

This currently crashes with:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: InvalidLastSymbol(42, 105)', src/crypto.rs:29:91
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
