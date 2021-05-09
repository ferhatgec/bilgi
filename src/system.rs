// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub struct SystemInfos {
    pub os_name : String,
    pub kernel  : String,

    pub username: String,
    pub hostname: String,
    pub language: String
}

pub mod sys {
    use crate::system::SystemInfos;

    pub fn init() -> SystemInfos {
        SystemInfos {
            os_name : get_os      (),
            kernel  : get_kernel  (),

            username: get_username(),
            hostname: get_hostname(),
            language: get_language()
        }
    }

    pub fn get_os() -> String {
        #[cfg(target_os = "windows")]   return "Windows".to_string     ();
        #[cfg(target_os = "macos")]     return "macOS".to_string       ();
        #[cfg(target_os = "ios")]       return "iOS".to_string         ();
        #[cfg(target_os = "linux")]     return get_linux_distro("/etc/os-release");
        #[cfg(target_os = "android")]   return "Android".to_string     ();
        #[cfg(target_os = "freebsd")]   return "FreeBSD".to_string     ();
        #[cfg(target_os = "dragonfly")] return "DragonflyBSD".to_string();
        #[cfg(target_os = "openbsd")]   return "OpenBSD".to_string     ();
        #[cfg(target_os = "netbsd")]    return "NetBSD".to_string      ();
        #[cfg(target = "unix")]         return "Unix".to_string        ();
    }

    pub fn get_linux_distro(file: &str) -> String {
        if std::path::Path::new(file).exists() {
            if let Ok(lines) =
                crate::helpers::helpers::read_lines(file) {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.starts_with('P') {
                            if ip.contains("PRETTY_NAME=\"") {
                                return ip.
                                    replace("PRETTY_NAME=", "")
                                    .replace("\"", "");
                            }
                        }
                    }
                }
            }
        }

        "GNU/Linux".to_string()
    }

    pub fn get_kernel() -> String {
        #[cfg(target_os = "windows")]   return "NT".to_string     ();
        #[cfg(target_os = "macos")]     return "XNU".to_string    ();
        #[cfg(target_os = "ios")]       return "XNU".to_string    ();
        #[cfg(target_os = "linux")]     return "Linux".to_string  ();
        #[cfg(target_os = "android")]   return "Linux".to_string  ();
        #[cfg(target_os = "freebsd")]   return "FreeBSD".to_string();
        #[cfg(target_os = "dragonfly")] return "FreeBSD".to_string();
        #[cfg(target_os = "openbsd")]   return "FreeBSD".to_string();
        #[cfg(target_os = "netbsd")]    return "NetBSD".to_string ();
        #[cfg(target = "unix")]         return "Unix".to_string   ();
    }

    pub fn get_username() -> String {
        std::env::var(
            if cfg!(target_os = "linux") {
                "USER"
            } else {
                "USERNAME"
            }
        ).unwrap()
    }

    pub fn get_hostname() -> String {
        std::env::var(
            if cfg!(target_os = "linux") {
                "HOSTNAME"
            } else {
                "COMPUTERNAME"
            }
        ).unwrap()
    }

    pub fn get_language() -> String {
        std::env::var(
            if cfg!(target_os = "linux"    )
                || cfg!(target_os = "freebsd"  )
                || cfg!(target_os = "openbsd"  )
                || cfg!(target_os = "macos"    )
                || cfg!(target_os = "ios"      )
                || cfg!(target_os = "dragonfly")
                || cfg!(target_os = "netbsd"   ) {
                "LANG"
            } else { "" }
        ).unwrap()
    }
}

