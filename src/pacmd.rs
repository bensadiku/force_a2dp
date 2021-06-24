use std::process::Command;
use std::{thread, time};
pub struct PACMD {
    /// Mac Address e.g. AA:11:2B:3F:44:55
    pub mac: String,
    /// Sink Card e.g. bluez_card.AA_11_2B_3F_44_55
    pub sink: String,
    /// Is debug config enabled
    pub dbg: bool,
}

impl PACMD {
    /// Get all cards listed by pacmd
    pub fn get_card_list() -> String {
        let output = Command::new("pacmd")
            .arg("list-cards")
            .output()
            .expect("Failed to list cards");

        return String::from_utf8_lossy(&output.stdout).to_string();
    }

    /// Restart bluetooth connection
    pub fn restart(&self) -> bool {
        let out = Command::new("bluetoothctl")
            .arg("disconnect")
            .arg(&self.mac)
            .output()
            .expect("Failed to disconnect Bluetooth");

        let _disconnected = String::from_utf8_lossy(&out.stdout).contains("Successful disconnected");


        self.sleep(7);
        Command::new("bluetoothctl")
            .arg("connect")
            .arg(&self.mac)
            .spawn()
            .expect("Failed to connect Bluetooth");
        self.sleep(3);
        let connected = String::from_utf8_lossy(&out.stdout).contains("Connection successful");

        return connected;
    }

    fn sleep(&self, seconds: u64) {
        let ten_seconds = time::Duration::from_secs(seconds);
        thread::sleep(ten_seconds);
    }

    /// Force A2DP sink
    pub fn set_a2dp_sink(&self) -> bool {
        let out = Command::new("pacmd")
            .arg("set-card-profile")
            .arg(&self.sink)
            .arg("a2dp_sink")
            .output();

        match out {
            Ok(output) => {
                let success = output.status.success();
                if !success {
                    println!("Failed to update A2DP, restarting Bluetooth");
                    let success = self.restart();
                    if !success {
                        println!(
                            "Failed to restart {}",
                            String::from_utf8_lossy(&output.stderr)
                        );
                    } else {
                        return self.set_a2dp_sink();
                    }
                }
                success
            }
            Err(e) => {
                let success = self.restart();
                if !success {
                    println!("Failed to restart {}", e);
                } else {
                    return self.set_a2dp_sink();
                }
                success
            }
        }
    }
}
