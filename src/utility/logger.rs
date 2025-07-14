use sysinfo::{ProcessExt, System, SystemExt, Pid};

pub struct Logger {
    system: System,
    this_pid: Pid
}

impl Logger {
    pub fn create() -> Self {
        let id = sysinfo::get_current_pid().unwrap();
        Self {
            system: System::new_all(),
            this_pid: id,
        }
    }

    pub fn log_window_info(&mut self) {
        self.system.refresh_all();

        if let Some(process) = self.system.process(Pid::from(self.this_pid)) {
            let memory_usage = (process.memory() / 1024) as f32;
            let cpu_usage = process.cpu_usage();

            print!("\x1B[2J\x1B[H");
            println!("{:.2} MB / {:.2}%", memory_usage / 1000.0, cpu_usage);
        }
    }

    pub fn log_info<T: std::fmt::Debug>(&self, name: &str, value: T) {
        if self.is_printable::<T>() {
            print!("\x1B[2J\x1B[H");
            println!("{}: {:?}", name, value);
        }
        else {
            println!("{} is not printable (Debug trait absent)", name);
        }
    }

    fn is_printable<T: std::fmt::Debug>(&self) -> bool {
        true
    }
}