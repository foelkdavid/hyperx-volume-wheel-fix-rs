# 🎧 hyperx-volume-wheel-fix -> Rust version.
**This is the newer, Rust version**

The main daemon works but it still lacks some functionality from the python version.

Until then, use my [python version](https://github.com/foelkdavid/hyperx-volume-wheel-fix/edit/main/README.md).

## Whats the issue?
The volume wheel of HyperX headsets like the "HyperX_Cloud_III_Wireless" wont work on linux.

## Whats the solution?
A small daemon that checks /dev/input for the corresponding events and runs commands based on the input.

## How do i use this?
If you use wireplumber, just autostart this script with your System and you are good to go.
Otherwise just adjust the command inside the script.

## Will this work for other headsets?
Maybe?
Try to adjust the device filter and the Keycodes, then it should work.

This way you can make this work with any type of input.

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
