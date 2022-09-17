pub mod sensors;
use crate::sensors::system::*;
use clap::Parser;
use human_bytes::human_bytes;
use std::{thread, time};

/// Simple program to monitor system statistics
/// Can be used with `watch` for periodic statistics
#[derive(Parser, Debug)]
#[clap(author = "Wilfred MK", version = "0.1.5", about, long_about = None)]
struct Opts {
    /// CPU info
    #[clap(short, long)]
    cpu_info: bool,

    /// CPU stats (freq/usage)
    #[clap(short, long)]
    freq_cpu: bool,

    /// Hardware temps
    #[clap(short, long)]
    hw_temps: bool,

    /// Ram/Swap stats
    #[clap(short, long)]
    ram_info: bool,

    /// Disk stats
    #[clap(short, long)]
    disk_info: bool,

    /// OS info
    #[clap(short, long)]
    os_info: bool,

    // Network stats
    #[clap(short, long)]
    network_info: bool,

    /// Watch for specified seconds
    #[clap(short, long, value_parser, default_value_t = 0)]
    watch: u8,
}

fn clear_term() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn usage() {
    println!(
        "USAGE:\n
        sys_stat [OPTIONS]"
    );
}

fn run_stats(sys: &Sensors, opts: &Opts) {
    if opts.os_info {
        sys.show_os_info();
    }
    if opts.hw_temps {
        sys.show_temps();
    }
    if opts.freq_cpu {
        sys.show_cpu_stats();
    }
    if opts.disk_info {
        sys.show_disk_info();
    }

    if opts.cpu_info {
        sys.show_cpu_info();
    }

    if opts.ram_info {
        sys.show_ram_info();
    }

    if opts.network_info {
        sys.show_network_stats();
    }
}

fn num_opts_matched(opts: &Opts) -> Result<usize, usize> {
    let mut opt_count = 0;
    if opts.os_info {
        opt_count += 1;
    }
    if opts.hw_temps {
        opt_count += 1;
    }
    if opts.freq_cpu {
        opt_count += 1;
    }
    if opts.disk_info {
        opt_count += 1;
    }

    if opts.cpu_info {
        opt_count += 1;
    }

    if opts.ram_info {
        opt_count += 1;
    }

    if opts.network_info {
        opt_count += 1;
    }

    // Incase we were asked to watch but no options were given to
    // display a particular metric.
    if opt_count == 0 {
        return Err(opt_count);
    }
    Ok(opt_count)
}

fn main() -> Result<(), i32> {
    let mut sys = sensors::system::Sensors::new();
    let opts = Opts::parse();
    // If no metric are specified
    if num_opts_matched(&opts).is_err() {
        println!("Must specify a metric to read");
        usage();
        return Err(-1);
    }

    clear_term();
    if opts.watch != 0 {
        let delay_s = time::Duration::from_secs(opts.watch as u64);
        loop {
            sys.refresh_sens();
            run_stats(&sys, &opts);
            thread::sleep(delay_s);
            clear_term();
        }
    } else {
        run_stats(&sys, &opts);
        Ok(())
    }
}
