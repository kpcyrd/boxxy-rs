# libautoboxxy

Exports a static constructor that is executed by the dynamic loader as soon as
the library is loaded. It then executes `$AUTOBOXXY` from the environment as a
boxxy command and terminates the process.

    cargo build && AUTOBOXXY="exec id" LD_PRELOAD=`pwd`/../target/debug/libautoboxxy.so date

## Usage with php

There's a ~~bug~~ feature in php that allows you to execute code even when
`shell_exec` and friends are disabled by php.ini. This uses autoboxxy under the
hood to take over the sendmail invocation to execute arbitrary code. See
`boxxy.php` to see how this is done.

This technique has been borrowed from [Chankro].

[Chankro]: https://github.com/TarlogicSecurity/Chankro
