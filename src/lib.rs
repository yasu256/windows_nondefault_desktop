//! Utility for applications that only work properly on the active Windows desktop.

use std::{
    mem,
    process::exit,
    ptr::{null, null_mut},
};

use windows_sys::Win32::System::{
    StationsAndDesktops::{
        GetThreadDesktop, GetUserObjectInformationA, OpenInputDesktop, HDESK, UOI_NAME,
    },
    Threading::{
        CreateProcessA, GetCurrentThreadId, WaitForSingleObject, INFINITE, PROCESS_INFORMATION,
        STARTUPINFOA,
    },
};

extern "C" {
    static _pgmptr: *const i8;
}

/// Re-runs the application on the active desktop if the desktop of the current process is not active.
pub fn assume_active_desktop() {
    let h_desk_thread = unsafe { GetThreadDesktop(GetCurrentThreadId()) };
    let desk_name_thread = name_from_h_desk(h_desk_thread);

    let h_desk_input = unsafe { OpenInputDesktop(0, 0, 0) };
    let desk_name_input = name_from_h_desk(h_desk_input);

    if desk_name_thread == desk_name_input {
        return;
    }

    // The window is not visible to the user because the process belongs to a different desktop than the desktop
    // currently receiving user input.

    let mut si: STARTUPINFOA = unsafe { mem::zeroed() };

    si.cb = mem::size_of::<STARTUPINFOA>() as _;
    si.lpDesktop = desk_name_input.as_ptr() as *mut _;

    let mut pi: PROCESS_INFORMATION = unsafe { mem::zeroed() };

    let ret = unsafe {
        CreateProcessA(
            _pgmptr as _,
            null_mut(),
            null(),
            null(),
            0,
            0,
            null(),
            null(),
            &si,
            &mut pi,
        )
    };

    let exitcode = if ret != 0 {
        // process creation succeeded
        unsafe {
            WaitForSingleObject(pi.hProcess, INFINITE);
        }
        0
    } else {
        // process creation failed
        1
    };

    exit(exitcode);
}

/// Gets desktop name from handle.
fn name_from_h_desk(h_desk: HDESK) -> Vec<u8> {
    let mut n_length_needed = 0;

    unsafe {
        GetUserObjectInformationA(h_desk, UOI_NAME, null_mut(), 0, &mut n_length_needed);
    }

    let mut buff: Vec<u8> = vec![0; n_length_needed as usize];

    unsafe {
        GetUserObjectInformationA(
            h_desk,
            UOI_NAME,
            buff.as_mut_ptr() as *mut _,
            n_length_needed,
            null_mut(),
        );
    }

    buff
}
