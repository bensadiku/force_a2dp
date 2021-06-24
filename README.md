## force_a2dp

WIP workaround fix for [a pulse audio bug](https://bugs.launchpad.net/ubuntu/+source/pulseaudio/+bug/1838151), HSP/HFP profiles in Ubuntu 20.04 have terrible quality, this mini scrcipt will force A2DP sink for any BT device connected. Note that A2DP is unidirectional (BT limitation), so if you force this profile bt mics will not be active.

This wraps pacmd and bluetoothctl so if there's an error setting the A2DP profile, it will restart Bluetooth and try to set the profile again.


### Install
If you have Cargo installed:

`cargo install force_a2dp`

Or download from [release tab](https://github.com/bensadiku/force_a2dp/releases)

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Licensing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.