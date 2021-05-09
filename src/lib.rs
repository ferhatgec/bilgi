// Bilgi - Cross-platform system information crate in Rust.
//
// Bilgi can take these informations (work-in-progress):
// * OS
// * Kernel
// * Username
// * Hostname
//
// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod system;
pub mod helpers;

#[cfg(test)]
mod tests {
    #[test]
    fn system_infos() {
        let infos = crate::system::sys::init();

        println!(
            "\
            OS      : {}\n\
            Kernel  : {}\n\
            Username: {}\n\
            Hostname: {}",

            infos.os_name,
            infos.kernel,
            infos.username,
            infos.hostname
        );
    }
}
