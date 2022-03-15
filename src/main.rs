use clap::Parser;
use colored::Colorize;
use human_bytes::human_bytes;
use std::{thread, time};
use sysinfo::{ComponentExt, NetworkExt, NetworksExt, ProcessExt, ProcessorExt, System, SystemExt};

/// Simple program to monitor system statistics
/// Can be used with `watch` for periodic statistics
#[derive(Parser, Debug)]
#[clap(author = "Wilfred MK", version = "0.1", about, long_about = None)]
struct Opts {
    /// cpu info
    #[clap(short, long, parse(from_occurrences))]
    cpu_info: u8,

    /// cpu stats (freq/usage)
    #[clap(short, long, parse(from_occurrences))]
    freq_cpu: u8,

    /// hardware temps
    #[clap(short, long, parse(from_occurrences))]
    hw_temps: u8,

    /// ram/swap stats
    #[clap(short, long, parse(from_occurrences))]
    ram_info: u8,

    /// disk stats
    #[clap(short, long, parse(from_occurrences))]
    disk_info: u8,

    /// OS info
    #[clap(short, long, parse(from_occurrences))]
    os_info: u8,

    // network stats
    #[clap(short, long, parse(from_occurrences))]
    network_info: u8,
}

fn show_temps(sys: &sysinfo::System) {
    println!(
        "{}",
        format!("Component\t\tcurrent temp,\t\tmax temp,\t\tcrit temp")
            .purple()
            .on_black()
    );
    for component in sys.components() {
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

fn show_os_info(sys: &sysinfo::System) {
    println!("OS info:");
    println!("System name:             {}", sys.name().unwrap());
    println!("System kernel version:   {}", sys.kernel_version().unwrap());
    println!("System OS version:       {}", sys.os_version().unwrap());
    println!("System host name:        {}", sys.host_name().unwrap());

    let uptime = sys.uptime();
    let seconds = uptime % 60;
    let minutes = (uptime / 60) % 60;
    let hours = (uptime / 60) / 60;
    println!(
        "System Uptime:           {}hrs:{}mns:{}s",
        hours, minutes, seconds
    );

    let load_avg = sys.load_average();
    println!(
        "load_average:            one minute: {}%, five minutes: {}%, fifteen minutes: {}%",
        load_avg.one, load_avg.five, load_avg.fifteen,
    );
}

fn show_disk_info(sys: &sysinfo::System) {
    println!("Disk info:");
    println!("=> disks:");
    for disk in sys.disks() {
        println!("{:?}", disk);
    }
}

fn show_ram_info(sys: &sysinfo::System) {
    // RAM and swap information:
    println!("Ram/Swap info:");
    println!(
        "total memory: {}",
        human_bytes((sys.total_memory() * 1024) as f64)
    );
    println!(
        "used memory : {}",
        human_bytes((sys.used_memory() * 1024) as f64)
    );
    println!(
        "free memory : {}",
        human_bytes((sys.available_memory() * 1024) as f64)
    );
    println!(
        "Ram Usage   : {:.2}%",
        (sys.used_memory() as f64 / sys.total_memory() as f64) * 100.00
    );
    println!(
        "total swap  : {}",
        human_bytes((sys.total_swap() * 1024) as f64)
    );
    println!(
        "used swap   : {}",
        human_bytes((sys.used_swap() * 1024) as f64)
    );
}

fn show_cpu_info(sys: &sysinfo::System) {
    println!("CPU info:");
    println!("Name - {}", sys.processors()[0].name());
    println!("Brand - {}", sys.processors()[0].brand());
    println!("Vendor ID - {}", sys.processors()[0].vendor_id());
    println!("Cores - {}", sys.processors().len());
}

fn show_cpu_stats(sys: &sysinfo::System) {
    println!("CPU Statistics:");
    for i in 0..sys.processors().len() {
        println!(
            "Core {}: {}Hz, {:.2}%",
            i,
            sys.processors()[i].frequency(),
            sys.processors()[i].cpu_usage()
        );
    }
    println!("Avg Usage {:.4}%", sys.global_processor_info().cpu_usage());
}

fn show_network_stats(sys: &sysinfo::System) {
    println!("Network Stats:");
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!(
            "{}: {}/{} B",
            interface_name,
            data.received(),
            data.transmitted()
        );
    }
}

fn run_stats(sys: &mut sysinfo::System, opts: &Opts) {
    match opts.os_info {
        0 => {}
        _ => {
            show_os_info(sys);
        }
    }
    match opts.hw_temps {
        0 => {}
        _ => {
            show_temps(sys);
        }
    }
    match opts.freq_cpu {
        0 => {}
        _ => {
            show_cpu_stats(sys);
        }
    }
    match opts.disk_info {
        0 => {}
        _ => {
            show_disk_info(sys);
        }
    }

    match opts.cpu_info {
        0 => {}
        _ => {
            show_cpu_info(sys);
        }
    }

    match opts.ram_info {
        0 => {}
        _ => {
            show_ram_info(sys);
        }
    }

    match opts.network_info {
        0 => {}
        _ => {
            show_network_stats(sys);
        }
    }
}

fn main() {
    let mut sys = System::new_all();
    let opts = Opts::parse();
    sys.refresh_all();
    run_stats(&mut sys, &opts);
}
