use regex::Regex;

/// Return some info about the card we're using (if any)
/// Will return the standard mac address used by bluetoothctl
/// Will return the bluez mac address format used by pacmd
pub fn get_bt_card_info(output: &str) -> Option<(String, String)> {
    let re = Regex::new(r#"[\n\r].*device.string = \s*([^\n\r]*)"#).unwrap();
    let captures = re.captures_iter(&output);
    for capture in captures {
        if let Some(matched) = capture.get(1) {
            let card = matched
                .as_str()
                .replace("\"", "")
                .to_owned();
            if card.len() > 3 {
                let sink = format!("bluez_card.{}",card.replace(":", "_"));
                return Some((card, sink));
            }
        }
    }
    None
}
