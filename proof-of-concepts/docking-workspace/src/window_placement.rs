//! Tiny Windows-only monitor placement guard for the docking experiment.
//!
//! Positions in this module are **physical desktop pixels**, including negative
//! coordinates for monitors to the left of the Windows primary display.
//! This is not a general-purpose window manager.

/// Keep at least part of the title bar visible in the selected monitor's
/// *working area* (excluding the taskbar). Coordinates are physical pixels.
fn clamp_title_bar(position: [i32; 2], work: [i32; 4]) -> [i32; 2] {
    const GRABBABLE_WIDTH: i32 = 120;
    const GRABBABLE_HEIGHT: i32 = 36;

    let rightmost_x = work[2].saturating_sub(GRABBABLE_WIDTH).max(work[0]);
    let bottommost_y = work[3].saturating_sub(GRABBABLE_HEIGHT).max(work[1]);
    [
        position[0].clamp(work[0], rightmost_x),
        position[1].clamp(work[1], bottommost_y),
    ]
}

/// Return a window position guaranteed to leave a draggable part of its
/// title bar on a real, currently connected monitor. Returns None rather
/// than restoring an unverified coordinate if the Windows API fails.
#[cfg(windows)]
pub fn safe_restore_position(position: [i32; 2]) -> Option<[i32; 2]> {
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromRect,
    };

    // Find the monitor nearest the SAVED location. MONITOR_DEFAULTTONEAREST
    // also recovers placements on monitors which have been disconnected.
    // A small title-bar rectangle is more meaningful than a single point.
    let title_bar = RECT {
        left: position[0],
        top: position[1],
        right: position[0].saturating_add(120),
        bottom: position[1].saturating_add(36),
    };
    // SAFETY: title_bar is a live RECT and the monitor handle is only used
    // immediately in GetMonitorInfoW. No OS-owned memory is retained.
    let monitor = unsafe { MonitorFromRect(&title_bar, MONITOR_DEFAULTTONEAREST) };
    if monitor.is_null() {
        return None;
    }

    let mut info = MONITORINFO {
        cbSize: std::mem::size_of::<MONITORINFO>() as u32,
        ..Default::default()
    };
    // SAFETY: the OS fills our correctly sized, mutable MONITORINFO struct.
    if unsafe { GetMonitorInfoW(monitor, &mut info) } == 0 {
        return None;
    }

    let rc = info.rcWork;
    if rc.left >= rc.right || rc.top >= rc.bottom {
        return None;
    }
    Some(clamp_title_bar(
        position,
        [rc.left, rc.top, rc.right, rc.bottom],
    ))
}

/// No window coordinates should be force-restored on non-Windows targets.
/// Windows 10+ is the scope of this particular proof of concept.
#[cfg(not(windows))]
pub fn safe_restore_position(_position: [i32; 2]) -> Option<[i32; 2]> {
    None
}

#[cfg(test)]
mod tests {
    use super::clamp_title_bar;

    #[test]
    fn leaves_a_valid_right_monitor_position_untouched() {
        assert_eq!(
            clamp_title_bar([4102, 568], [3840, 0, 5280, 2560]),
            [4102, 568]
        );
    }

    #[test]
    fn supports_negative_coordinates_on_left_monitor() {
        assert_eq!(
            clamp_title_bar([-600, 500], [-1440, 0, 0, 2560]),
            [-600, 500]
        );
    }

    #[test]
    fn rescues_a_window_far_off_the_right_and_bottom() {
        assert_eq!(
            clamp_title_bar([6153, 9000], [3840, 0, 5280, 2560]),
            [5160, 2524]
        );
    }

    #[test]
    fn rescues_a_window_off_the_left_and_top() {
        assert_eq!(
            clamp_title_bar([-9999, -800], [-1440, -50, 0, 2510]),
            [-1440, -50]
        );
    }
}
