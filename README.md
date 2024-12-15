# 🎧 hyperx-volume-wheel-fix -> Rust version.
**This is the newer, Rust version**

The main daemon works but it still lacks some functionality from the python version.

Until then, use my [python version](https://github.com/foelkdavid/hyperx-volume-wheel-fix/edit/main/README.md).

## Whats the issue?
The volume wheel of HyperX headsets like the "HyperX_Cloud_III_Wireless" wont work on linux.

## Whats the solution?
A small daemon that checks /dev/input for the corresponding events and runs commands based on the input.


## Installing:
Just clone this repo, cd into it and run:
```bash
cargo build -r
```
you will find your binary under `target/release`.

## Usage:
If you use wireplumber, just autostart the binary with your System and you are good to go.

Otherwise modify the `adjust_volume()` function inside the source code.

## Will this work for other headsets?
With adjustments, yes!

Try to adjust the device filter and the Keycodes, this way the majority of headsets wheels/buttons/whatever should work.

This way you can make this work with any type of input, not only Headsets.
</br>-> As long as there is a device for it somewhere under `/proc/bus/input/devices`.

You could also change the commands it executes and do completely different things with it. :)



## Current Functionality:
#### Note: All of this already works in the [python version](https://github.com/foelkdavid/hyperx-volume-wheel-fix/edit/main/README.md).
- [x] The volume wheel works and can successfully control the volume of audio sinks/etc.
- [ ] Automatic detection of input/event path. -> no further config needed
- [ ] Daemon runs in different modes (waiting and control)
    - this allows for hotplugging etc.
- [ ] Desktop notifications based status of daemon:
  - Daemon is waiting for headset/adapter
  - Daemon connected to headset/adapter
  - Daemon disconnected from headset/adapter
