mod args;
mod pacmd;
mod parser;
use pacmd::PACMD;

fn main() {
        let dbg = args::is_debug();
        let cards = PACMD::get_card_list();
        let bluez_card = parser::get_bt_card_info(&cards);

        if let Some(card) = bluez_card {
                let pacmd = PACMD {
                        mac: card.0,
                        sink: card.1,
                        dbg: dbg,
                };
                let success = pacmd.set_a2dp_sink();
                let w = if success {
                        "Successfully set"
                } else {
                        "Failed to set"
                };
                println!("{} A2DP sink for {}", w, pacmd.mac);
        }
}
