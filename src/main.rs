pub mod sensors;
use crate::sensors::system::*;
use clap::Parser;
use human_bytes::human_bytes;

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

fn run_stats(sys: &Sensors, opts: &Opts) {
    match opts.os_info {
        0 => {}
        _ => {
            sys.show_os_info();
        }
    }
    match opts.hw_temps {
        0 => {}
        _ => {
            sys.show_temps();
        }
    }
    match opts.freq_cpu {
        0 => {}
        _ => {
            sys.show_cpu_stats();
        }
    }
    match opts.disk_info {
        0 => {}
        _ => {
            sys.show_disk_info();
        }
    }

    match opts.cpu_info {
        0 => {}
        _ => {
            sys.show_cpu_info();
        }
    }

    match opts.ram_info {
        0 => {}
        _ => {
            sys.show_ram_info();
        }
    }

    match opts.network_info {
        0 => {}
        _ => {
            sys.show_network_stats();
        }
    }
}

fn main() {
    let mut sys = sensors::system::Sensors::new();

    let opts = Opts::parse();
    sys.refresh_sens();
    run_stats(&sys, &opts);
}
