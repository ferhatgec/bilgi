extern crate bilgi;

fn main() {
    let infos  = bilgi::system::sys::init();

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
