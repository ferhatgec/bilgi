// MIT License
//
// Copyright (c) 2021 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub struct CPUInfos {
    pub model_name: String,
    pub cores     : u32
}

impl CPUInfos {
    pub fn init(&mut self) {
        self.get_linux_cpu_infos("/proc/cpuinfo");
    }

    pub fn get_linux_cpu_infos(&mut self, file: &str) {
        if std::path::Path::new(file).exists() {
            if let Ok(lines) =
            crate::helpers::helpers::read_lines(file) {
                for line in lines {
                    if let Ok(ip) = line {
                        if ip.starts_with('c') {
                            if ip.contains("cpu cores") {
                                let mut ip = ip
                                    .replace("cpu cores", "")
                                    .replace(":", "")
                                    .trim_start().to_string();

                                if !ip.is_empty() {
                                    self.cores = ip.parse::<u32>().unwrap();
                                }
                            }

                            continue;
                        }

                        if ip.starts_with('m') {
                            if ip.contains("model name") {
                                self.model_name = ip
                                    .replace("model name", "")
                                    .replace(":", "")
                                    .trim_start().to_string();
                            }

                            continue;
                        }
                    }
                }
            }
        }
    }
}