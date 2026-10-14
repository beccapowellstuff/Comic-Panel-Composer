//! Experimental Windows-only workspace activation behaviour.
//!
//! When the MAIN app window becomes active, raise the native tool windows in
//! normal z-order WITHOUT taking keyboard focus away from the main window.
//! Unlike "always on top", these windows can sit behind other applications.
//! This is a narrowly scoped PoC: normal product code should identify windows
//! with strong handles rather than matching their titles.

#[cfg(windows)]
pub fn raise_tool_windows_without_focus() -> usize {
    use windows_sys::Win32::Foundation::{HWND, LPARAM};
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetWindowTextW, GetWindowThreadProcessId, HWND_TOP, IsWindowVisible,
        SWP_NOACTIVATE, SWP_NOMOVE, SWP_NOSIZE, SetWindowPos,
    };

    struct Found {
        handles: Vec<HWND>,
        process_id: u32,
    }

    unsafe extern "system" fn collect(hwnd: HWND, arg: LPARAM) -> i32 {
        // SAFETY: The pointer refers to a live stack value for the entire
        // synchronous EnumWindows call. No pointer escapes the callback.
        let found = unsafe { &mut *(arg as *mut Found) };
        let mut owner = 0_u32;
        // SAFETY: hwnd is supplied by Windows; owner is writable.
        unsafe { GetWindowThreadProcessId(hwnd, &mut owner) };
        if owner != found.process_id || unsafe { IsWindowVisible(hwnd) } == 0 {
            return 1;
        }

        let mut buffer = [0_u16; 128];
        // SAFETY: buffer is writable and length matches the allocated size.
        let n = unsafe { GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32) };
        if n > 0 {
            let title = String::from_utf16_lossy(&buffer[..n as usize]);
            if ["Pages", "Layers", "Inspector"].iter().any(|&name| {
                title.starts_with(name) && title.ends_with(" — native Windows window")
            }) {
                found.handles.push(hwnd);
            }
        }
        1
    }

    let mut found = Found {
        handles: Vec::new(),
        process_id: std::process::id(),
    };
    // SAFETY: collect only writes through a valid pointer to found during
    // this synchronous call; hwnd values are owned by this process.
    unsafe { EnumWindows(Some(collect), &mut found as *mut Found as LPARAM) };

    let count = found.handles.len();
    // Raise without activating each window, preserving main-window keyboard
    // focus and without changing the permanent always-on-top flag.
    for hwnd in found.handles.into_iter().rev() {
        // SAFETY: The HWND was supplied by EnumWindows for this process.
        // SetWindowPos may fail if a window closed meanwhile; that is benign.
        unsafe { SetWindowPos(hwnd, HWND_TOP, 0, 0, 0, 0, SWP_NOACTIVATE | SWP_NOMOVE | SWP_NOSIZE) };
    }
    count
}

#[cfg(not(windows))]
pub fn raise_tool_windows_without_focus() -> usize {
    0
}
