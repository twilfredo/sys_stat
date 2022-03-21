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
$ ./sys_stat --help

Simple program to monitor system statistics Can be used with `watch` for periodic statistics

USAGE:
    sys_stat [OPTIONS]

OPTIONS:
    -c, --cpu-info        cpu info
    -d, --disk-info       disk stats
    -f, --freq-cpu        cpu stats (freq/usage)
    -h, --hw-temps        hardware temps
        --help            Print help information
    -n, --network-info
    -o, --os-info         OS info
    -r, --ram-info        ram/swap stats
    -V, --version         Print version information
```
To see cpu info and current stats
```SHELL
$ ./sys_stat -f -c

CPU Statistics:
Core 0: 4701Hz, 2.50%
Core 1: 4702Hz, 2.56%
Core 2: 4701Hz, 7.32%
Core 3: 4700Hz, 7.32%
Core 4: 4702Hz, 2.50%
Core 5: 4702Hz, 5.00%
Core 6: 4701Hz, 5.00%
Core 7: 4702Hz, 2.63%
Core 8: 4700Hz, 7.50%
Core 9: 4702Hz, 5.00%
Core 10: 4700Hz, 5.00%
Core 11: 4700Hz, 9.76%
Avg Usage 6.1983%

CPU info:
Name - cpu0
Brand - Intel(R) Core(TM) i5-10600K CPU @ 4.10GHz
Vendor ID - GenuineIntel
Cores - 12
```
This tool can be used with `watch` for **continuous monitoring.**
```SHELL
watch ./sys_stat -h

Every 2.0s: ./target/debug/sys_stat -h                                             ArchDesktop: Mon Mar 21 22:29:58 2022

Component               current temp,           max temp,               crit temp
Composite               43.85     °C,           83.85     °C,           83.85     °C
Composite               42.85     °C,           76.85     °C,           79.85     °C
Core 0                  45        °C,           80        °C,           100       °C
Core 1                  46        °C,           80        °C,           100       °C
Core 2                  47        °C,           80        °C,           100       °C
Core 3                  48        °C,           80        °C,           100       °C
Core 4                  48        °C,           80        °C,           100       °C
Core 5                  61        °C,           80        °C,           100       °C
edge                    54        °C,           54        °C,           100       °C
junction                57        °C,           57        °C,           110       °C
mem                     56        °C,           56        °C,           100       °C
Package id 0            61        °C,           80        °C,           100       °C
Sensor 1                43.85     °C,           65261.848 °C,           0         °C
Sensor 2                49.85     °C,           65261.848 °C,           0         °C
CPU                     27.8      °C,           27.8      °C,           0         °C

```
or even
```SHELL
watch ./sys_stat -f

Every 2.0s: ./sys_stat -f                                                          ArchDesktop: Fri Mar  4 14:35:19 2022

CPU Statistics:
Core 0: 4700Hz, 5.71%
Core 1: 4700Hz, 8.33%
Core 2: 4700Hz, 10.81%
Core 3: 4701Hz, 8.33%
Core 4: 4701Hz, 13.89%
Core 5: 4700Hz, 10.81%
Core 6: 4700Hz, 11.11%
Core 7: 4700Hz, 8.11%
Core 8: 4701Hz, 8.33%
Core 9: 4700Hz, 16.22%
Core 10: 4701Hz, 8.11%
Core 11: 4702Hz, 5.71%
Avg Usage 9.2166%
```