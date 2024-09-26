use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;
use std::thread;
use std::time::Duration;
use sysinfo::Process;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    display_system_info(&sys);
    display_memory_info(&sys);
    display_cpu_info(&mut sys);
    display_network_info();
    display_gpu_stats().unwrap();
    display_top_cpu_processes(&sys);
}

fn display_system_info(sys: &System) {
    println!("=> System Information:");
    println!("System name:             {:?}", System::name());
    println!("System kernel version:   {:?}", System::kernel_version());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());
    println!("Number of CPUs: {}", sys.cpus().len());
}

fn display_memory_info(sys: &System) {
    println!("=> Memory Information:");
    println!("Total memory: {} bytes", sys.total_memory());
    println!("Used memory : {} bytes", sys.used_memory());
    println!("Total swap  : {} bytes", sys.total_swap());
    println!("Used swap   : {} bytes", sys.used_swap());
}

fn display_cpu_info(sys: &mut System) {
    println!("=> CPU Information:");

    let mut cpu_usage_sum = 0.0;
    let cpu_count = sys.cpus().len();

    sys.refresh_cpu_usage();
    thread::sleep(Duration::from_millis(1000));
    sys.refresh_cpu_usage();

    for cpu in sys.cpus() {
        cpu_usage_sum += cpu.cpu_usage();
        println!(
            "[CPU] Name: {:?}, Usage: {:?}%, Brand: {:?}, Frequency: {:?} MHz, Vendor ID: {:?}",
            cpu.name(),
            cpu.cpu_usage(),
            cpu.brand(),
            cpu.frequency(),
            cpu.vendor_id()
        );
    }

    println!(
        "[CPU] Average Usage: {:.2}%",
        cpu_usage_sum / (cpu_count as f32)
    );
    println!("[CPU] Uptime: {:?}", format_uptime(System::uptime()));
}

fn format_uptime(uptime: u64) -> String {
    let days = uptime / 86400;
    let hours = (uptime % 86400) / 3600;
    let minutes = (uptime % 3600) / 60;
    format!("{} days, {} hours, {} minutes", days, hours, minutes)
}

fn display_network_info() {
    println!("=> Network Information:");
    let mut networks = sysinfo::Networks::new_with_refreshed_list();

    // Initial measurement
    networks.refresh();
    let initial_data: Vec<(String, u64, u64)> = networks
        .iter()
        .map(|(interface_name, data)| {
            (
                interface_name.clone(),
                data.total_received(),
                data.total_transmitted(),
            )
        })
        .collect();

    // Wait for 1 second
    thread::sleep(Duration::from_secs(1));

    // Final measurement
    networks.refresh();
    let final_data: Vec<(String, u64, u64)> = networks
        .iter()
        .map(|(interface_name, data)| {
            (
                interface_name.clone(),
                data.total_received(),
                data.total_transmitted(),
            )
        })
        .collect();

    // Calculate and display data per second
    for (
        (initial_name, initial_received, initial_transmitted),
        (final_name, final_received, final_transmitted),
    ) in initial_data.into_iter().zip(final_data.into_iter())
    {
        if initial_name == final_name {
            let received_per_second = final_received - initial_received;
            let transmitted_per_second = final_transmitted - initial_transmitted;
            println!(
                "{}: {} B/s (down) / {} B/s (up)",
                initial_name, received_per_second, transmitted_per_second
            );
        }
    }
}

fn display_gpu_stats() -> Result<(), NvmlError> {
    println!("=> GPU Information:");
    let nvml = Nvml::init()?;
    let device = nvml.device_by_index(0)?;

    let utilization = device.utilization_rates()?;
    let memory = device.memory_info()?;
    let encoder_utilization = device.encoder_utilization()?;
    let decoder_utilization = device.decoder_utilization()?;

    println!("GPU Utilization: {}%", utilization.gpu);
    println!(
        "Memory Utilization: {} / {} MiB",
        memory.used / 1024 / 1024,
        memory.total / 1024 / 1024
    );
    println!("Encoder Utilization: {}%", encoder_utilization.utilization);
    println!("Decoder Utilization: {}%", decoder_utilization.utilization);

    Ok(())
}

fn display_top_cpu_processes(sys: &System) {
    let mut processes: Vec<&Process> = sys.processes().values().collect();
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

    println!("=> Top CPU-Consuming Processes:");
    for process in processes.iter().take(10) {
        let name = process.name();
        let cpu_usage = process.cpu_usage();
        println!("{:?}: {:.2}%", name, cpu_usage);
    }
}
