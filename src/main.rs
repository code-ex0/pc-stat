use sysinfo::{ComponentExt, NetworkExt, ProcessExt, ProcessorExt, System, SystemExt};
use serde::{Serialize, Deserialize};
use std::{thread, time::Duration};
use reqwest::blocking::Client;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
struct Pc {
    id: usize,
    cpu_load: f32,
    cores_loads: Vec<f32>,
    cores_frequency: Vec<u64>,
    cores_temp: Vec<f32>
}

impl Pc {
    fn new() -> Pc {
        Pc {
            id: 0,
            cpu_load: 0.0,
            cores_loads: vec![],
            cores_frequency: vec![],
            cores_temp: vec![]
        }
    }
}

impl fmt::Display for Pc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}--{}-{:?}-{:?}-{:?}",self.id , self.cpu_load, self.cores_loads, self.cores_frequency, self.cores_temp)
    }
}

fn main() {
    let mut sys = System::new_all();
    loop {
        sys.refresh_all();
        thread::sleep(Duration::from_millis(1000));

        let mut c: Pc = Pc::new();
        c.cores_loads = sys.processors()
            .iter()
            .map(|x| x.cpu_usage())
            .collect::<Vec<f32>>();

        c.cores_frequency = sys.processors()
            .iter()
            .map(|x| x.frequency())
            .collect::<Vec<u64>>();

        c.cores_temp = sys.components()
            .iter()
            .filter(|x| x.label().contains("Core") )
            .map(|x| x.temperature())
            .collect::<Vec<f32>>();

        let mut result: f32 = 0.;
        for i in &c.cores_loads { result += i }
        c.cpu_load = result / c.cores_loads.len() as f32;
        match Client::new()
            .post("http://localhost:8000/api")
            .json(&c)
            .send()
        {
            Ok(_) => {
                #[cfg(debug_assertions)]
                println!("{}",c);
            }
            Err(_err) => {
                #[cfg(debug_assertions)]
                println!("{}", _err);
            }
        }

//         println!("=> components:");
//         for component in sys.components() {
//             println!("{:?}", component.temperature());
//         }
//
//         println!("total memory: {} KB", sys.total_memory());
//         println!("used memory : {} KB", sys.used_memory());
//         println!("total swap  : {} KB", sys.total_swap());
//         println!("used swap   : {} KB", sys.used_swap());
//
// // Display system information:
//         println!("System name:             {:?}", sys.name());
//         println!("System kernel version:   {:?}", sys.kernel_version());
//         println!("System OS version:       {:?}", sys.os_version());
//         println!("System host name:        {:?}", sys.host_name());
    }
}
