# tmux-sysinfo

Measures CPU pressure by reading `/proc/pressure/cpu`

temperature read from `/sys/devices/platform/coretemp.0/hwmon/hwmon4/temp1_input`.

use env `SYSINFO_CPU_TEMP` to specify the temperature file location 
if `/sys/devices/platform/coretemp.0/hwmon/hwmon4/temp1_input` does not exits

RAM usage read from `/proc/meminfo` (Used, MemTotal). 
