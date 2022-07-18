# vros

Kind of like [ROS], Robot Operating System, vros works by combining modules.

Unlike downloading executable files from the [VRChat OSC Resources page], vros modules are JavaScript code that can be executed in a secure sandbox. However, the sandbox currently allows everything for all modules, negating the security benefits.

[ROS]: https://www.ros.org/
[VRChat OSC Resources page]: https://docs.vrchat.com/docs/osc-resources

## Possible use cases

### Image metadata

Similar to [VRChat-Exif-Writer], a vros script can collect information about what is happening and add it to the image metadata. A partial implementation of this is included in `apps/vrchat-camera-meta.ts`.

Additionally, images could be converted from PNG to a lossy format, and images taken in certain worlds could be automatically cropped.

[VRChat-Exif-Writer]: https://github.com/m-hayabusa/VRChat-Exif-Writer

### Watches and other simple OSC gimicks

Because of the module system, simple OSC gimicks take very little code. An example is included in `apps/internet-time.ts`.

Heart rate monitors are also simple, if you have a way to get the heart rate data. In practice, heart rate monitor modules will generally require some unsandboxed code to talk to the heart rate monitor. They will probably look something like the `vros-steamvr` module. However, [vrc-osc-miband-hrm]'s solution of connecting the device to a browser, connected to a service, connected to OSC, would be compatible with sandboxing.

[vrc-osc-miband-hrm]: https://github.com/vard88508/vrc-osc-miband-hrm/

## How does it work?

Every module is loaded into a separate [Deno] worker. All workers execute simultaneously. The workers are able to communicate with each other by using the [Broadcast Channel API].

vros only makes one small extension to the Deno API. `vros.expandPath` takes a path in the form `${LocalAppDataLow}\VRChat\VRChat` and converts it to `C:\Users\user\AppData\LocalLow\VRChat\VRChat`. It's not possible to do this perfectly using the standard Deno API without disabling the sandbox. It's important that this code be consistent with the code that would grant permissions.

vros comes with a few modules:

- vros-steamvr: This module provides information about the currently running OpenVR/SteamVR application. See `apps/steamvr-app-demo.ts` for an example.
- vros-vrchat-logs: This module provides information from VRChat log files. See `apps/vrchat-camera-meta.ts` and `apps/vrchat-video-demo.ts` for examples.
- vros-vrchat-osc-out: This module provides a simple OSC sender that can be used by other modules. See `apps/internet-time.ts` for an example.

[Deno]: https://deno.land/
[Broadcast Channel API]: https://developer.mozilla.org/en-US/docs/Web/API/Broadcast_Channel_API

## Why?

- Easier scripting. vros takes care of the basics.
- Better security. If the sandbox were enabled and there was a UI, users could more safely download scripts. Maybe scripts could even be loaded from internet sources.
- I wanted to try module loading and sandboxing with Deno.

## Building

You will need to have Rust, Microsoft's Visual C++ toolchain (rustup can install this), Clang, and the Deno CLI.

From a x64 Native Tools command prompt, run `deno task build` within the vros folder. Now you can start vros using `cargo run`.
