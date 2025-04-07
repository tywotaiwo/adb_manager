use std::process::Command;

pub struct AdbManager;

impl AdbManager {
    pub fn new() -> Self {
        Self
    }

    pub fn run_adb_command(&self, args: &[&str]) -> Result<String, String> {
        let output = Command::new("adb")
            .args(args)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            String::from_utf8(output.stdout)
                .map_err(|e| e.to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).into_owned())
        }
    }

    pub fn get_devices(&self) -> Result<Vec<String>, String> {
        let output = self.run_adb_command(&["devices"])?;
        let mut devices = Vec::new();
        
        // Skip the first line which is "List of devices attached"
        for line in output.lines().skip(1) {
            if line.trim().is_empty() {
                continue;
            }
            
            if let Some(device_id) = line.split_whitespace().next() {
                devices.push(device_id.to_string());
            }
        }
        
        Ok(devices)
    }
} 