# vros

Kind of like ROS, Robot Operating System, vros works by combining modules. Modules are typically smaller than a standalone application to do the same thing.

Unlike downloading executable files from the [VRChat OSC Resources page], vros modules are JavaScript code that can be executed in a secure sandbox. However, the sandbox currently allows everything for all modules, negating the security benefits.

[VRChat OSC Resources page]: https://docs.vrchat.com/docs/osc-resources

## Possible use cases

### Image metadata

Similar to [VRChat-Exif-Writer], a vros script can collect information about what is happening and add it to the image metadata. A partial implementation of this is included in `apps/vrchat-camera-meta.ts`.

[VRChat-Exif-Writer]: https://github.com/m-hayabusa/VRChat-Exif-Writer
