# sys_stat utility tool

This cmd line tool provides easy access for system statistics to be displayed to user. Using the `sysinfo` crate, the app should be supported across all major OSs (Linux/Windows/Mac/OBSD). Currently tested only on Linux.

Supported metrics:

  * CPU Info
  * CPU Stats (freq/usage)
  * Ram/Swap Stats
  * Network Stats (Up/Down)
  * Hardware Temperature
  * Disk Info

Example usage linux:
```SHELL
$ cargo run -- --help

Simple program to monitor system statistics Can be used with `watch` for periodic statistics

USAGE:
    sys_stat [OPTIONS]

OPTIONS:
    -c, --cpu-info         CPU info
    -d, --disk-info        Disk stats
    -f, --freq-cpu         CPU stats (freq/usage)
    -h, --hw-temps         Hardware temps
        --help             Print help information
    -n, --network-info     
    -o, --os-info          OS info
    -r, --ram-info         Ram/Swap stats
    -V, --version          Print version information
    -w, --watch <WATCH>    Watch for specified seconds [default: 0]
```
To see cpu info and current stats
```SHELL
$ cargo run -- -c -f

CPU Statistics:
Core 0: 2199Hz, 0.00%
Core 1: 3400Hz, 0.00%
Core 2: 3400Hz, 0.00%
Core 3: 2200Hz, 0.00%
Core 4: 3400Hz, 0.00%
Core 5: 3400Hz, 0.00%
Core 6: 2200Hz, 0.00%
Core 7: 2310Hz, 0.00%
Core 8: 3752Hz, 0.00%
Core 9: 3400Hz, 0.00%
Core 10: 3400Hz, 0.00%
Core 11: 2200Hz, 0.00%
Core 12: 3400Hz, 0.00%
Core 13: 3523Hz, 0.00%
Core 14: 3423Hz, 0.00%
Core 15: 4699Hz, 0.00%
Core 16: 3400Hz, 0.00%
Core 17: 2199Hz, 0.00%
Core 18: 2199Hz, 0.00%
Core 19: 2604Hz, 0.00%
Core 20: 2484Hz, 0.00%
Core 21: 2199Hz, 0.00%
Core 22: 2200Hz, 0.00%
Core 23: 3400Hz, 0.00%
Core 24: 3759Hz, 0.00%
Core 25: 3760Hz, 0.00%
Core 26: 3760Hz, 0.00%
Core 27: 3759Hz, 0.00%
Core 28: 3614Hz, 0.00%
Core 29: 3400Hz, 0.00%
Core 30: 2200Hz, 0.00%
Core 31: 3400Hz, 0.00%
Load Average: one minute: 0.64%, five minutes: 0.54%, fifteen minutes: 0.49%

CPU info:
Name      - cpu
Brand     - AMD Ryzen 9 5950X 16-Core Processor
Vendor ID - AuthenticAMD
Cores     - 16
```
This tool can be used for **continuous monitoring.** With the option **[-w SECS]**
```SHELL
cargo run -- -w 2 -h

Component		current temp,		max temp,		crit temp
amdgpu edge		52        °C,		52        °C,		100       °C
amdgpu junction		55        °C,		56        °C,		110       °C
amdgpu mem		64        °C,		64        °C,		100       °C
asusec Chipset		65        °C,		65        °C,		0         °C
asusec CPU		40        °C,		40        °C,		0         °C
asusec Motherboard		39        °C,		39        °C,		0         °C
asusec T_Sensor		-40       °C,		-40       °C,		0         °C
asusec VRM		51        °C,		51        °C,		0         °C
asusec Water_In		-40       °C,		-40       °C,		0         °C
asusec Water_Out		-40       °C,		-40       °C,		0         °C
iwlwifi_1 temp1		42        °C,		42        °C,		0         °C
k10temp Tccd1		33.5      °C,		35.75     °C,		0         °C
k10temp Tccd2		31.25     °C,		34.75     °C,		0         °C
k10temp Tctl		36.75     °C,		37.25     °C,		0         °C
nvme Composite WDS200T1X0E-00AFY0 temp1		56.85     °C,		56.85     °C,		87.85     °C
