extern crate bilgi;

fn main() {
    let infos  = bilgi::system::sys::init();

    let mut cpu_infos = bilgi::cpu::CPUInfos {
        model_name: "".to_string(),
        vendor_id : "".to_string(),
        cores     : 0
    };

    cpu_infos.init();

    println!(
        "\
            OS      : {}\n\
            Kernel  : {}\n\
            Username: {}\n\
            Hostname: {}\n\
            Language: {}\n\
            Term    : {}\n\
            --------\n\
            CPU     : {}\n\
            Cores   : {}\n\
            VendorID: {}",

        infos.os_name,
        infos.kernel,
        infos.username,
        infos.hostname,
        infos.language,

        (if !infos.emulator.1.is_empty() {
            infos.emulator.1
        } else { infos.emulator.0 }),

        cpu_infos.model_name,
        cpu_infos.cores,
        cpu_infos.vendor_id
    );
}
