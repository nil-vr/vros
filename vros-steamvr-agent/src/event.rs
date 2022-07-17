//! autocxx cannot handle unions so just include the bindgen struct here.
#![allow(non_camel_case_types, non_snake_case)]

use ovr_overlay_sys::{EVREventType, TrackedDeviceIndex_t};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VREvent_t {
    pub eventType: EVREventType,
    pub trackedDeviceIndex: TrackedDeviceIndex_t,
    pub eventAgeSeconds: f32,
    pub data: VREvent_Data_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union VREvent_Data_t {
    pub reserved: VREvent_Reserved_t,
    pub controller: VREvent_Controller_t,
    pub mouse: VREvent_Mouse_t,
    pub scroll: VREvent_Scroll_t,
    pub process: VREvent_Process_t,
    pub notification: VREvent_Notification_t,
    pub overlay: VREvent_Overlay_t,
    pub status: VREvent_Status_t,
    pub keyboard: VREvent_Keyboard_t,
    pub ipd: VREvent_Ipd_t,
    pub chaperone: VREvent_Chaperone_t,
    pub performanceTest: VREvent_PerformanceTest_t,
    pub touchPadMove: VREvent_TouchPadMove_t,
    pub seatedZeroPoseReset: VREvent_SeatedZeroPoseReset_t,
    pub screenshot: VREvent_Screenshot_t,
    pub screenshotProgress: VREvent_ScreenshotProgress_t,
    pub applicationLaunch: VREvent_ApplicationLaunch_t,
    pub cameraSurface: VREvent_EditingCameraSurface_t,
    pub messageOverlay: VREvent_MessageOverlay_t,
    pub property: VREvent_Property_t,
    pub hapticVibration: VREvent_HapticVibration_t,
    pub webConsole: VREvent_WebConsole_t,
    pub inputBinding: VREvent_InputBindingLoad_t,
    pub actionManifest: VREvent_InputActionManifestLoad_t,
    pub spatialAnchor: VREvent_SpatialAnchor_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Reserved_t {
    pub reserved0: u64,
    pub reserved1: u64,
    pub reserved2: u64,
    pub reserved3: u64,
    pub reserved4: u64,
    pub reserved5: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Controller_t {
    pub button: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Mouse_t {
    pub x: f32,
    pub y: f32,
    pub button: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Scroll_t {
    pub xdelta: f32,
    pub ydelta: f32,
    pub unused: u32,
    pub viewportscale: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Process_t {
    pub pid: u32,
    pub oldPid: u32,
    pub bForced: bool,
    pub bConnectionLost: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Notification_t {
    pub ulUserValue: u64,
    pub notificationId: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Overlay_t {
    pub overlayHandle: u64,
    pub devicePath: u64,
    pub memoryBlockId: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Status_t {
    pub statusState: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Keyboard_t {
    pub cNewInput: [::std::os::raw::c_char; 8usize],
    pub uUserValue: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Ipd_t {
    pub ipdMeters: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Chaperone_t {
    pub m_nPreviousUniverse: u64,
    pub m_nCurrentUniverse: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_PerformanceTest_t {
    pub m_nFidelityLevel: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_TouchPadMove_t {
    pub bFingerDown: bool,
    pub flSecondsFingerDown: f32,
    pub fValueXFirst: f32,
    pub fValueYFirst: f32,
    pub fValueXRaw: f32,
    pub fValueYRaw: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_SeatedZeroPoseReset_t {
    pub bResetBySystemMenu: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Screenshot_t {
    pub handle: u32,
    pub type_: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_ScreenshotProgress_t {
    pub progress: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_ApplicationLaunch_t {
    pub pid: u32,
    pub unArgsHandle: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_EditingCameraSurface_t {
    pub overlayHandle: u64,
    pub nVisualMode: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_MessageOverlay_t {
    pub unVRMessageOverlayResponse: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_Property_t {
    pub container: PropertyContainerHandle_t,
    pub prop: ETrackedDeviceProperty,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_HapticVibration_t {
    pub containerHandle: u64,
    pub componentHandle: u64,
    pub fDurationSeconds: f32,
    pub fFrequency: f32,
    pub fAmplitude: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_WebConsole_t {
    pub webConsoleHandle: WebConsoleHandle_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_InputBindingLoad_t {
    pub ulAppContainer: PropertyContainerHandle_t,
    pub pathMessage: u64,
    pub pathUrl: u64,
    pub pathControllerType: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_InputActionManifestLoad_t {
    pub pathAppKey: u64,
    pub pathMessage: u64,
    pub pathMessageParam: u64,
    pub pathManifestPath: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VREvent_SpatialAnchor_t {
    pub unHandle: SpatialAnchorHandle_t,
}

pub type PropertyContainerHandle_t = u64;
pub type ETrackedDeviceProperty = ::std::os::raw::c_int;
pub type WebConsoleHandle_t = u64;
pub type SpatialAnchorHandle_t = u32;
