use std::fs;

fn main() {
    let cpu_pressure = procfs::CpuPressure::new().unwrap();

    /*
    let load = fs::read_to_string("/proc/loadavg").unwrap().split(' ').nth(0).unwrap().parse::<f64>().unwrap();
    let num_cpus = fs::read_dir("/sys/devices/system/cpu").unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .filter(|x| x.starts_with("cpu") && x.chars().nth(3).unwrap().is_numeric())
        .count() as f64;
    */
    // /sys/class/hwmon/hwmon0/temp1_input
    let temp = fs::read_to_string("/sys/devices/platform/coretemp.0/hwmon/hwmon4/temp1_input")
        .unwrap()
        .trim()
        .parse::<f64>()
        .unwrap()
        / 1000.0;

    let mem_info = procfs::Meminfo::new().unwrap();
    const MiB: f64 = 1024.0 * 1024.0;
    const GiB: f64 = MiB * 1024.0;
    let _mem_usedav = (mem_info.mem_total - mem_info.mem_available.map_or(0, |v| v)) as f64 / GiB;
    let mut mem_used: u64 = mem_info.mem_total - mem_info.mem_free;

    mem_used += mem_info.shmem.map_or(0, |v| v);

    mem_used -= mem_info.buffers;
    mem_used -= mem_info.cached;
    mem_used -= mem_info.s_reclaimable.map_or(0, |v| v);
    let mem_used = mem_used as f64 / GiB;

    let mem_total = mem_info.mem_total as f64 / GiB;
    // cpu pressure 10s, 60s, 300s, temp, mem used/total
    println!(
        "{:.2} {:.2} {:.2} {:.1}°C {:.2}/{:.2}GB",
        cpu_pressure.some.avg10,
        cpu_pressure.some.avg60,
        cpu_pressure.some.avg300,
        temp,
        mem_used,
        mem_total,
    );
}
