use std::process::Command;
use std::io::Read;
use std::mem;



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
 ** <h3>adjust_volume</h3>
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
 ** <h3>main</h3>
 ** <p>1. Opens event file.</p>
 ** <p>2. Parses new event into some InputEvent struct using "parse_input_event()."</p>
 ** <p>3. Matches (if event_type is 1) the volume key direction.</p>
 ** <p>4. Runs "adjust_volume()."</p>
 **/
fn main() {
    const INCREMENT: u8 = 1;
    const VOLUME_UP_KEY: u16 = 115;
    const VOLUME_DOWN_KEY: u16 = 114;
    const EVENT_PATH: &str = "/dev/input/event18";

    let event_size = mem::size_of::<InputEvent>();
    let mut file = std::fs::File::open(EVENT_PATH).expect("Failed to open input file");

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
