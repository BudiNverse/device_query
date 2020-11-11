extern crate device_query;

use device_query::{DeviceQuery, DeviceState};

fn main() {
    let mut device_state = DeviceState::new();
    let mut prev_keys = vec![];
    let mut keycodes = vec![];
    loop {
        device_state.get_keys(&mut keycodes);
        if keycodes != prev_keys {
            println!("{:?}", &keycodes);
        }

        prev_keys.clear();
        prev_keys.extend_from_slice(&keycodes);
    }
}
