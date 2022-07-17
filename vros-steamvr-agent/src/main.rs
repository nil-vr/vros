use std::{
    ffi::CStr,
    io::{self, StdoutLock, Write},
    mem::{self},
    pin::Pin,
    ptr,
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::{Duration, Instant},
};

use event::VREvent_t;
use ovr_overlay_sys::{
    k_unMaxApplicationKeyLength, EVRApplicationError, EVRApplicationProperty, EVRApplicationType,
    EVREventType, EVRInitError, IVRApplications, IVRSystem,
};
use vros_steamvr_core::{ApplicationName, FromAgent, InitializationError, ToAgent};

mod event;

struct System(Pin<&'static mut IVRSystem>);

impl System {
    fn new() -> Result<Self, InitializationError> {
        unsafe {
            let mut error = EVRInitError::VRInitError_None;
            let system = ovr_overlay_sys::VR_Init(
                &mut error,
                EVRApplicationType::VRApplication_Background,
                ptr::null(),
            );
            if error != EVRInitError::VRInitError_None {
                let ptr = ovr_overlay_sys::VR_GetVRInitErrorAsSymbol(error);
                let name = if ptr.is_null() {
                    "(null)".to_owned()
                } else {
                    String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
                };

                return Err(InitializationError {
                    name,
                    code: error as u32,
                });
            }

            Ok(System(Pin::new_unchecked(&mut *system)))
        }
    }
}

impl Drop for System {
    fn drop(&mut self) {
        unsafe {
            ovr_overlay_sys::VR_Shutdown();
        }
    }
}

fn get_scene_application_name(
    mut applications: Pin<&mut IVRApplications>,
) -> Result<Option<ApplicationName>, EVRApplicationError> {
    unsafe {
        let pid = applications.as_mut().GetCurrentSceneProcessId();
        if pid == 0 {
            return Ok(None);
        }
        let mut key = [0i8; k_unMaxApplicationKeyLength as usize];
        match applications.as_mut().GetApplicationKeyByProcessId(
            pid,
            key.as_mut_ptr(),
            mem::size_of_val(&key) as u32,
        ) {
            EVRApplicationError::VRApplicationError_None => {}
            EVRApplicationError::VRApplicationError_NoApplication => return Ok(None),
            other => return Err(other),
        }
        let key_len = key.iter().position(|v| *v == 0).unwrap_or(key.len());
        let key_str = String::from_utf8_lossy(mem::transmute(&key[..key_len]));
        let mut buffer = Vec::new();
        loop {
            let mut error = EVRApplicationError::VRApplicationError_None;
            let len = applications.as_mut().GetApplicationPropertyString(
                key.as_ptr(),
                EVRApplicationProperty::VRApplicationProperty_Name_String,
                buffer.as_mut_ptr() as *mut _ as *mut _,
                buffer.capacity() as u32,
                &mut error,
            );
            match error {
                EVRApplicationError::VRApplicationError_None
                    if len as usize <= buffer.capacity() =>
                {
                    buffer.set_len(len as usize);
                    return Ok(Some(ApplicationName {
                        key: key_str.into_owned(),
                        name: String::from_utf8_lossy(&buffer[..len as usize - 1]).into_owned(),
                    }));
                }
                EVRApplicationError::VRApplicationError_None
                | EVRApplicationError::VRApplicationError_BufferTooSmall => {
                    buffer.reserve(len as usize);
                }
                EVRApplicationError::VRApplicationError_NoApplication => return Ok(None),
                other => return Err(other),
            }
        }
    }
}

struct Sender<'a>(StdoutLock<'a>);

impl<'a> Sender<'a> {
    fn send(&mut self, message: &FromAgent) {
        let bytes = bincode::serialize(message).unwrap();
        self.0
            .write_all(&(bytes.len() as u32).to_le_bytes())
            .expect("Send failed");
        self.0.write_all(&bytes).expect("Send failed");
        self.0.flush().expect("Send failed");
    }
}

pub fn main() {
    let (stdin_send, stdin) = mpsc::sync_channel::<bincode::Result<ToAgent>>(0);
    let _stdin_thread = thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        loop {
            // weird clippy bug?
            #[allow(clippy::significant_drop_in_scrutinee)]
            match bincode::deserialize_from(&mut stdin) {
                Ok(message) => stdin_send.send(Ok(message)),
                Err(err) => stdin_send.send(Err(err)),
            }
            .expect("Thread send error");
        }
    });

    let mut stdout = Sender(io::stdout().lock());

    let mut system = match System::new() {
        Ok(system) => system,
        Err(err) => {
            stdout.send(&FromAgent::InitializationError(err));
            return;
        }
    };

    let mut applications = unsafe { Pin::new_unchecked(&mut *ovr_overlay_sys::VRApplications()) };

    stdout.send(&FromAgent::InitializationCompleted);

    let mut event: VREvent_t = unsafe { mem::zeroed() };
    let frame_target = Duration::from_secs(1) / 120;

    if let Ok(name) = get_scene_application_name(applications.as_mut()) {
        stdout.send(&FromAgent::ApplicationName(name));
    }

    'outer: loop {
        let frame_start = Instant::now();
        unsafe {
            while system.0.as_mut().PollNextEvent(
                &mut event as *mut _ as *mut _,
                mem::size_of_val(&event) as u32,
            ) {
                match dbg!(event.eventType) {
                    EVREventType::VREvent_SceneApplicationChanged => {
                        if let Ok(name) = get_scene_application_name(applications.as_mut()) {
                            stdout.send(&FromAgent::ApplicationName(name));
                        }
                    }
                    EVREventType::VREvent_EnterStandbyMode => {}
                    EVREventType::VREvent_LeaveStandbyMode => {}
                    EVREventType::VREvent_Quit => {
                        system.0.as_mut().AcknowledgeQuit_Exiting();
                        break 'outer;
                    }
                    _ => {}
                }
            }
        }
        loop {
            match stdin.recv_timeout(frame_target.saturating_sub(frame_start.elapsed())) {
                Ok(Ok(command)) => match command {},
                Ok(Err(err)) => panic!("I/O error: {:?}", err),
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) => return,
            }
        }
    }
}
