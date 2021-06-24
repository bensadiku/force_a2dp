// run with `cargo test -- --nocapture` for  the logs

#[cfg(test)]
mod tests {
        use force_a2dp::parser;
        #[test]
        fn test_actual_card() {
                let txt = r#" 3 card(s) available.
        index: 13
        name: <bluez_card.1A_11_11_1A_11_11>
        driver: <module-bluez5-device.c>
        owner module: 40
        properties:
                device.description = "Bose"
                device.string = "1A:11:11:1A:11:11"
                device.api = "bluez"
                device.class = "sound"
                device.bus = "bluetooth"
                device.form_factor = "headphone"
                bluez.path = "/org/bluez/hci0/dev_1A_11_11_1A_11_11"
                bluez.class = "0x240418"
                bluez.alias = "Bose"
                device.icon_name = "audio-headphones-bluetooth"
        profiles:
                headset_head_unit: Headset Head Unit (HSP/HFP) (priority 30, available: unknown)
                a2dp_sink: High Fidelity Playback (A2DP Sink) (priority 40, available: yes)
                off: Off (priority 0, available: yes)
        active profile: <a2dp_sink>
        sinks:
                bluez_sink.1A_11_11_1A_11_11.a2dp_sink/#28: Bose 
        sources:
                bluez_sink.1A_11_11_1A_11_11.a2dp_sink.monitor/#45: Monitor of Bose
        ports:
                headphone-output: Headphone (priority 0, latency offset 0 usec, available: yes)
                        properties:
                                
                headphone-input: Bluetooth Input (priority 0, latency offset 0 usec, available: unknown)
                        properties:

                "#;
                let card = parser::get_bt_card_info(txt).unwrap();
                println!("CARD: {:?}", card);

                let mac = card.0;
                let sink = card.1;

                assert_eq!("1A:11:11:1A:11:11", &mac);
                assert_eq!("bluez_card.1A_11_11_1A_11_11", &sink);
        }

        #[test]
        fn test_no_card() {
                let txt = r#"    index: 3
                name: <alsa_card.pci-xxxxx>
                driver: <module-alsa-card.c>
                owner module: xx
                properties:
                        alsa.card = "0"
                        alsa.card_name = "xxx"
                        alsa.long_card_name = "  xx  xx"
                        alsa.driver_name = "snd_hda_intel"
                        device.bus_path = "pci-xxxx:xx:xx.x"
                        sysfs.path = "/devices/pcixxxx:xx/xxxx:xx:xx.x/sound/cardx"
                        device.bus = "pci"
                        device.vendor.id = "xx"
                        device.vendor.name = "xx Corporation"
                        device.product.id = "xxxx"
                        device.form_factor = "internal"
                        device.string = "0"
                        device.description = "Built-in Audio"
                        module-udev-detect.discovered = "1"
                        device.icon_name = "audio-card-pci"
        "#;
                let card = parser::get_bt_card_info(txt);
                assert_eq!(None, card);
        }
}
