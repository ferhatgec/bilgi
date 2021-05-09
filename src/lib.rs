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
pub mod cpu;

#[cfg(test)]
mod tests {
    #[test]
    fn system_infos() {
        let infos  = crate::system::sys::init();
        let cpu_infos= crate::cpu::cpu::init();

        println!(
            "\
            OS      : {}\n\
            Kernel  : {}\n\
            Username: {}\n\
            Hostname: {}\n\
            --------\n\
            CPU     : {}",

            infos.os_name,
            infos.kernel,
            infos.username,
            infos.hostname,

            cpu_infos.model_name
        );
    }
}
