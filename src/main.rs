use std::{fs, io, env};
use std::fmt::{Display, Formatter};

const CPU_TEMP_ENV_KEY: &str = "SYSINFO_CPU_TEMP";

fn main() {
    let sys_info = SysInfo::init();

    print!("{}", sys_info)
}

struct SysInfo {
    temperature: f64,
    mem_used: f64,
    mem_total: f64,
    cpu_pressure: Option<procfs::CpuPressure>,
}

impl SysInfo {
    fn new() -> Self {
        Self{
            temperature: 0.0,
            mem_used: 0.0,
            mem_total: 0.0,
            cpu_pressure: None
        }
    }

    fn init() -> Self {
        let mut default = Self::new();
        default.cpu_pressure();
        default.cpu_temp();
        default.mem_info();
        default
    }

    fn cpu_pressure(&mut self) -> Result<(), procfs::ProcError> {
        /*
        let load = fs::read_to_string("/proc/loadavg").unwrap().split(' ').nth(0).unwrap().parse::<f64>().unwrap();
        let num_cpus = fs::read_dir("/sys/devices/system/cpu").unwrap()
            .map(|x| x.unwrap().file_name().into_string().unwrap())
            .filter(|x| x.starts_with("cpu") && x.chars().nth(3).unwrap().is_numeric())
            .count() as f64;
        */
        match procfs::CpuPressure::new() {
            Ok(pressure) => {
                self.cpu_pressure = Some(pressure);
                Ok(())
            },
            Err(err) => Err(err)
        }
    }

    fn cpu_temp(&mut self) -> io::Result<f64> {
        // /sys/devices/platform/coretemp.0/hwmon/hwmon4/temp1_input
        // /sys/class/hwmon/hwmon0/temp1_input
        let temp = fs::read_to_string("/sys/devices/platform/coretemp.0/hwmon/hwmon4/temp1_input")
            .or(fs::read_to_string("/sys/devices/platform/coretemp.0/hwmon/hwmon3/temp1_input"))
            .or_else(|_|{
                let key = CPU_TEMP_ENV_KEY;
                match env::var(key) {
                    Ok(val) => {
                        println!("{}: {:?}", key, val);
                        fs::read_to_string(val)
                    },
                    Err(e) => {
                        Err(io::Error::new(io::ErrorKind::Other, format!("couldn't interpret {}: {}", key, e)))
                    },
                }
            })
            .and_then(|data| match data.trim().parse::<f64>() {
                Ok(x) => {
                    Ok(x)
                },
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Could not parse float")),
            })
            .map(|num| num / 1000.0);

        match &temp {
            Ok(t) => self.temperature = *t,
            Err(err) => {
                println!("{}", err)
            }
        }
        temp
    }

    fn mem_info(&mut self) {
        let mem_info = procfs::Meminfo::new();
        match mem_info {
            Ok(info) => {
                let mut mem_used: u64 = info.mem_total - info.mem_free;
                mem_used += info.shmem.map_or(0, |v| v);
                mem_used -= info.buffers;
                mem_used -= info.cached;
                mem_used -= info.s_reclaimable.map_or(0, |v| v);
                self.mem_used = mem_used as f64;
                self.mem_total = info.mem_total as f64;
            }
            Err(_) => {}
        }

    }
}

impl Display for SysInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        const MI_B: f64 = 1024.0 * 1024.0;
        const GI_B: f64 = MI_B * 1024.0;
        // let _mem_usedavailable = (mem_info.mem_total - mem_info.mem_available.map_or(0, |v| v)) as f64 / GI_B;

        let mem_used_gb = self.mem_used / GI_B;

        let mem_total_gb = self.mem_total as f64 / GI_B;

        let mut avg10: f32 = 0.0;
        let mut avg60: f32 = 0.0;
        let mut avg300: f32 = 0.0;

        match &self.cpu_pressure {
            None => {}
            Some(p) => {
                 avg10 = p.some.avg10;
                 avg60 = p.some.avg60;
                 avg300 = p.some.avg300;
            }
        }

        // cpu pressure 10s, 60s, 300s, temp, mem used/total
        writeln!(f,
            "{:.2} {:.2} {:.2} {:.1}°C {:.2}/{:.2}GB",
            avg10,
            avg60,
            avg300,
            self.temperature,
            mem_used_gb,
            mem_total_gb,
        )
    }
}