use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead};
use std::mem;
use std::path::Path;
use std::process::Command;
use std::{thread::sleep, time::Duration};

/// Keycode to increase volume.
const VOLUME_UP_KEY: u16 = 115;

/// Keycode to decrease volume.
const VOLUME_DOWN_KEY: u16 = 114;

/// Increment (in percent) used when raising volume.
const INCREMENT: u8 = 1;

/// Filter string used when waiting/finding the correct device id.<br>
/// To check this, take a look at your /proc/bus/input/devices.
const DEVICE_FILTER_STRING: &str = "HyperX Cloud III Wireless Consumer Control";

/**
 ** <p> Reference for this: </p>
 ** <https://github.com/torvalds/linux/blob/master/include/uapi/linux/input.h>
 **/
struct InputEvent {
    _seconds: i64,      // should correspond ~ to timeval.tv_sec (8 bytes)
    _microseconds: i64, // should correspond ~ timeval.tv_usec (8 bytes)
    event_type: u16,
    event_code: u16,
    _value: i32,
}

/**
 ** <p> Either <b>Up</b> or <b>Down</b> </p>
 **/
enum VolumeDirection {
    Up,
    Down,
}

/**
 ** <h3>adjust_volume()</h3>
 ** <p>Adjusts volume on default sink using wpctl.</p>
 **/
// TODO -> make changing sink and adjust_command more adjustable somewhere more convenient
fn adjust_volume(direction: VolumeDirection, increment: u8) {
    let symbol = match direction {
        VolumeDirection::Up => "+",
        VolumeDirection::Down => "-",
    };

    let adjust_command = format!("{}%{}", increment, symbol);

    Command::new("wpctl")
        .arg("set-volume")
        .arg("@DEFAULT_AUDIO_SINK@")
        .arg(&adjust_command)
        .status()
        .expect("Failed to adjust volume"); // TODO -> look into error handling etc.
                                            // TODO -> inform the user somehow.
}

/**
 ** <h3>parse_input_event</h3>
 ** <p>This just manually slices the buffer and writes the bytes into the corresponding fields.</p>
 ** <p>Option will return None if it fails which will make it less error prone.</p>
 **/
fn parse_input_event(buffer: &[u8]) -> Option<InputEvent> {
    if buffer.len() != mem::size_of::<InputEvent>() {
        return None;
    }

    let _seconds = i64::from_ne_bytes(buffer[0..8].try_into().ok()?);
    let _microseconds = i64::from_ne_bytes(buffer[8..16].try_into().ok()?);
    let event_type = u16::from_ne_bytes(buffer[16..18].try_into().ok()?);
    let event_code = u16::from_ne_bytes(buffer[18..20].try_into().ok()?);
    let _value = i32::from_ne_bytes(buffer[20..24].try_into().ok()?);

    Some(InputEvent {
        _seconds,
        _microseconds,
        event_type,
        event_code,
        _value,
    })
}

/**
 ** <h3>send_notification()</h3>
 ** <p>sends a notification via notify-send</p>
 **/
fn send_notification(message: &str) {
    Command::new("notify-send")
        .arg("-a")
        .arg("hyperx-daemon")
        .arg("HyperX Daemon:")
        .arg(message)
        .status()
        .expect("Failed to send Notification");
}


/**
 ** <h3>get_device_event_id()</h3>
 ** <p>Takes a filter string to search for in the /proc/bus/input/devices file.</p>
 ** <p>If a match is found, it returns the corresponding event ID found under /dev/input/event.</p>
 **/
fn get_device_event_id(device_filter_string: &str) -> String {
    let path = Path::new("/proc/bus/input/devices");
    let file = File::open(path).expect("Failed to open /proc/bus/input/devices");
    let reader = io::BufReader::new(file);
    let mut capture = false;

    for line in reader.lines().flatten() {
        if line.starts_with("N: Name=") && line.contains(device_filter_string) {
            capture = true;
        }

        if capture {
            if line.starts_with("H: Handlers=") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for part in parts {
                    if part.starts_with("event") {
                        return part.to_string();
                    }
                }
            }
            if line.trim().is_empty() {
                capture = false;
            }
        }
    }
    return "?".to_string();
}


/**
 ** <h3>volume_daemon()</h3>
 ** <p>checks for events on the device and adjusts volume accordingly.</p>
 **/
fn volume_daemon(device_path: String) {
    let event_size = mem::size_of::<InputEvent>();
    let mut file = std::fs::File::open(device_path).expect("Failed to open input file");

    loop {
        let mut buffer = vec![0u8; event_size];
        match file.read_exact(&mut buffer) {
            Ok(_) => {
                if let Some(event) = parse_input_event(&buffer) {
                    if event.event_type == 1 {
                        let volume_direction = match event.event_code {
                            VOLUME_UP_KEY => VolumeDirection::Up,
                            VOLUME_DOWN_KEY => VolumeDirection::Down,
                            _ => continue, // Skip unrecognized codes
                        };
                        adjust_volume(volume_direction, INCREMENT);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading event: {}", e);
                break;
            }
        }
    }
}

/**
 ** <h3>wait_for_event_id()</h3>
 ** <p>Waits for the device to appear, returns the device_input_id when found.</p>
 **/
fn wait_for_event_id() -> String {
    let mut disconnected_switch = false;
    loop {
        let event_id = get_device_event_id(DEVICE_FILTER_STRING);
        if event_id != "?"{
            send_notification("󰋎 Device connected.");
            return event_id;
        } else {
            if ! disconnected_switch {
                send_notification("󰋐 No device found. Waiting...");
                disconnected_switch = true;
            }
            sleep(Duration::new(2, 0));
        }
    }
}

/**
 ** <h3>main</h3>
 ** <p>1. Opens event file.</p>
 ** <p>2. Parses new event into some InputEvent struct using "parse_input_event()."</p>
 ** <p>3. Matches (if event_type is 1) the volume key direction.</p>
 ** <p>4. Runs "adjust_volume()."</p>
 **/
fn main() {

    loop{
        let event_id = wait_for_event_id();
        let device_path = format!("{}{}", "/dev/input/", event_id);
        volume_daemon(device_path);

    }
}
