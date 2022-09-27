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
```

```SHELL
Simple program to monitor system statistics Can be used with `watch` for periodic statistics

USAGE:
    sys_stat [OPTIONS]

OPTIONS:
    -a, --all              Show all
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
$ cargo run -- -w 2 -h
```
```SHELL
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
```

See all metrics, updated periodically with a 2 second polling delay
```SHELL
$ cargo run -- -w 2 -a
```
```
OS info:
System name:             Fedora Linux
System kernel version:   5.19.10-200.fc36.x86_64
System OS version:       36
System host name:        fedora
System Uptime:           2hrs:59mns:45s
load_average:            one minute: 0.82%, five minutes: 0.66%, fifteen minutes: 0.52%

Component		current temp,		max temp,		crit temp
amdgpu edge		53        °C,		53        °C,		100       °C
amdgpu junction		56        °C,		56        °C,		110       °C
amdgpu mem		64        °C,		64        °C,		100       °C
asusec Chipset		65        °C,		65        °C,		0         °C
asusec CPU		41        °C,		41        °C,		0         °C
asusec Motherboard		39        °C,		39        °C,		0         °C
asusec T_Sensor		-40       °C,		-40       °C,		0         °C
asusec VRM		52        °C,		52        °C,		0         °C
asusec Water_In		-40       °C,		-40       °C,		0         °C
asusec Water_Out		-40       °C,		-40       °C,		0         °C
iwlwifi_1 temp1		44        °C,		44        °C,		0         °C
k10temp Tccd1		34.75     °C,		35        °C,		0         °C
k10temp Tccd2		32.25     °C,		32.25     °C,		0         °C
k10temp Tctl		39.5      °C,		39.625    °C,		0         °C
nvme Composite WDS200T1X0E-00AFY0 temp1		57.85     °C,		57.85     °C,		87.85     °C

CPU Statistics:
Core 0: 2860Hz, 1.46%
Core 1: 3400Hz, 4.88%
Core 2: 3238Hz, 2.90%
Core 3: 3400Hz, 2.90%
Core 4: 3253Hz, 0.97%
Core 5: 2843Hz, 0.49%
Core 6: 2200Hz, 2.90%
Core 7: 3400Hz, 1.46%
Core 8: 4667Hz, 14.42%
Core 9: 3189Hz, 0.97%
Core 10: 3523Hz, 0.98%
Core 11: 4095Hz, 2.43%
Core 12: 3843Hz, 0.98%
Core 13: 4164Hz, 4.02%
Core 14: 2200Hz, 0.97%
Core 15: 3912Hz, 0.97%
Core 16: 2200Hz, 1.94%
Core 17: 3033Hz, 0.49%
Core 18: 3044Hz, 0.97%
Core 19: 2913Hz, 0.98%
Core 20: 3222Hz, 0.98%
Core 21: 2200Hz, 0.49%
Core 22: 3212Hz, 0.97%
Core 23: 3032Hz, 2.97%
Core 24: 4674Hz, 0.97%
Core 25: 3740Hz, 0.49%
Core 26: 3311Hz, 0.97%
Core 27: 3651Hz, 0.49%
Core 28: 3400Hz, 0.49%
Core 29: 3400Hz, 1.46%
Core 30: 3739Hz, 0.97%
Core 31: 3400Hz, 1.44%
Load Average: one minute: 0.82%, five minutes: 0.66%, fifteen minutes: 0.52%

Disk info:
=> disks:
Disk("/dev/nvme0n1p3")[FS: ['b', 't', 'r', 'f', 's']][Type: SSD][removable: no] mounted on "/": 1954497675264/1998694907904 B
Disk("/dev/nvme0n1p3")[FS: ['b', 't', 'r', 'f', 's']][Type: SSD][removable: no] mounted on "/home": 1954497675264/1998694907904 B
Disk("/dev/nvme0n1p2")[FS: ['e', 'x', 't', '4']][Type: SSD][removable: no] mounted on "/boot": 628498432/1020702720 B
Disk("/dev/nvme0n1p1")[FS: ['v', 'f', 'a', 't']][Type: SSD][removable: no] mounted on "/boot/efi": 613273600/627900416 B

CPU info:
Name      - cpu
Brand     - AMD Ryzen 9 5950X 16-Core Processor
Vendor ID - AuthenticAMD
Cores     - 16

Ram/Swap info:
total memory: 125.7 GiB
used memory : 5.6 GiB
free memory : 118.7 GiB
Ram Usage   : 4.43%
total swap  : 8 GiB
used swap   : 0 B

Network Stats:
=> networks:
enp4s0: 0 B/0 B
wlp6s0: 6.2 KiB/4.8 KiB
enp5s0: 0 B/0 B
lo: 0 B/0 B


```

