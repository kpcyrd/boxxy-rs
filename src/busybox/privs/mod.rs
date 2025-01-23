#[cfg(unix)]
import_cmd!(id);

#[cfg(target_os = "openbsd")]
import_cmd!(pledge);

#[cfg(unix)]
import_cmd!(setuid);

#[cfg(unix)]
import_cmd!(seteuid);

#[cfg(target_os = "linux")]
import_cmd!(setreuid);

#[cfg(target_os = "linux")]
import_cmd!(setresuid);

#[cfg(unix)]
import_cmd!(setgid);

#[cfg(target_os = "linux")]
import_cmd!(setresgid);

#[cfg(unix)]
import_cmd!(setgroups);

#[cfg(target_os = "linux")]
import_cmd!(caps);

#[cfg(target_os = "linux")]
import_cmd!(keepcaps);
