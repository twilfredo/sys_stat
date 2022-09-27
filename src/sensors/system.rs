use crate::human_bytes;
use colored::Colorize;
use sysinfo::{ComponentExt, CpuExt, NetworkExt, System, SystemExt};

pub struct Sensors {
    sys: sysinfo::System,
}

impl Sensors {
    pub fn new() -> Self {
        Sensors {
            sys: System::new_all(),
        }
    }

    pub fn refresh_sens(&mut self) {
        self.sys.refresh_all();
    }

    pub fn show_temps(&self) {
        println!("");
        println!(
            "{}",
            format!("Component\t\tcurrent temp,\t\tmax temp,\t\tcrit temp")
                .purple()
                .on_black()
        );
        for component in self.sys.components() {
            println!(
                "{0: <10}\t\t{1: <10}°C,\t\t{2: <10}°C,\t\t{3: <10}°C",
                component.label(),
                component.temperature(),
                component.max(),
                match component.critical() {
                    Some(crit_temp) => crit_temp,
                    None => 0.0,
                },
            );
        }
    }

    pub fn show_os_info(&self) {
        println!("");
        println!("OS info:");
        println!(
            "System name:             {}",
            self.sys.name().unwrap_or("N/A".to_string())
        );
        println!(
            "System kernel version:   {}",
            self.sys.kernel_version().unwrap_or("N/A".to_string())
        );
        println!(
            "System OS version:       {}",
            self.sys.os_version().unwrap_or("N/A".to_string())
        );
        println!(
            "System host name:        {}",
            self.sys.host_name().unwrap_or("N/A".to_string())
        );

        let uptime = self.sys.uptime();
        let seconds = uptime % 60;
        let minutes = (uptime / 60) % 60;
        let hours = (uptime / 60) / 60;
        println!(
            "System Uptime:           {}hrs:{}mns:{}s",
            hours, minutes, seconds
        );

        let load_avg = self.sys.load_average();
        println!(
            "load_average:            one minute: {}%, five minutes: {}%, fifteen minutes: {}%",
            load_avg.one, load_avg.five, load_avg.fifteen,
        );
    }

    pub fn show_disk_info(&self) {
        println!("");
        println!("Disk info:");
        println!("=> disks:");
        for disk in self.sys.disks() {
            println!("{:?}", disk);
        }
    }

    pub fn show_ram_info(&self) {
        println!("");
        // RAM and swap information:
        println!("Ram/Swap info:");
        println!(
            "total memory: {}",
            human_bytes((self.sys.total_memory()) as f64)
        );
        println!(
            "used memory : {}",
            human_bytes((self.sys.used_memory()) as f64)
        );
        println!(
            "free memory : {}",
            human_bytes((self.sys.available_memory()) as f64)
        );
        println!(
            "Ram Usage   : {:.2}%",
            (self.sys.used_memory() as f64 / self.sys.total_memory() as f64) * 100.00
        );
        println!(
            "total swap  : {}",
            human_bytes((self.sys.total_swap()) as f64)
        );
        println!(
            "used swap   : {}",
            human_bytes((self.sys.used_swap()) as f64)
        );
    }

    pub fn show_cpu_info(&self) {
        println!("");
        println!("CPU info:");
        let cpu = self.sys.global_cpu_info();
        // TODO: crate bug name/brand might not always be accurate.
        println!("Name      - {}", cpu.name());
        println!("Brand     - {}", cpu.brand());
        println!("Vendor ID - {}", cpu.vendor_id());
        println!(
            "Cores     - {}",
            self.sys.physical_core_count().unwrap_or(0)
        );
    }

    pub fn show_cpu_stats(&self) {
        println!("");
        println!("CPU Statistics:");
        let mut core_count = 0;
        for proc in self.sys.cpus() {
            println!(
                "Core {}: {}Hz, {:.2}%",
                core_count,
                proc.frequency(),
                proc.cpu_usage(),
            );
            core_count += 1;
        }
        println!(
            "Load Average: one minute: {}%, five minutes: {}%, fifteen minutes: {}%",
            self.sys.load_average().one,
            self.sys.load_average().five,
            self.sys.load_average().fifteen,
        );
    }

    pub fn show_network_stats(&self) {
        println!("");
        println!("Network Stats:");
        println!("=> networks:");
        for (interface_name, data) in self.sys.networks() {
            println!(
                "{}: {}/{}",
                interface_name,
                human_bytes(data.received() as f64),
                human_bytes(data.transmitted() as f64)
            );
        }
    }
}
