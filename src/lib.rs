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

        let mut cpu_infos = crate::cpu::CPUInfos {
            model_name: "".to_string(),
            cores     : 0
        };

        cpu_infos.init();

        println!(
            "\
            OS      : {}\n\
            Kernel  : {}\n\
            Username: {}\n\
            Hostname: {}\n\
            --------\n\
            CPU     : {}\n\
            Cores   : {}",

            infos.os_name,
            infos.kernel,
            infos.username,
            infos.hostname,

            cpu_infos.model_name,
            cpu_infos.cores
        );
    }
}
