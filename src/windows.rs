use std::iter::once;
use std::{ffi::OsStr, os::windows::ffi::OsStrExt, ptr::null_mut};
use winapi::um::consoleapi::{GetConsoleMode, SetConsoleMode};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING;
use winapi::um::{
	fileapi::{CreateFileW, OPEN_EXISTING},
	winnt::{FILE_SHARE_WRITE, GENERIC_READ, GENERIC_WRITE},
};

// I HATE WINDOWS

unsafe fn get_out_handle() -> *mut winapi::ctypes::c_void {
	unsafe {
		let out_name: Vec<u16> = OsStr::new("CONOUT$").encode_wide().chain(once(0)).collect();

		CreateFileW(
			out_name.as_ptr(),
			GENERIC_READ | GENERIC_WRITE,
			FILE_SHARE_WRITE,
			null_mut(),
			OPEN_EXISTING,
			0,
			null_mut(),
		)
	}
}

#[cfg(windows)]
pub fn enable_ansi_support() -> Result<(), u32> {
	unsafe {
		let h_stdout = get_out_handle();
		if h_stdout == INVALID_HANDLE_VALUE {
			return Err(GetLastError());
		}

		let mut console_mode: u32 = 0;
		if 0 == GetConsoleMode(h_stdout, &mut console_mode) {
			return Err(GetLastError());
		}

		if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0
			&& 0 == SetConsoleMode(h_stdout, console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING)
		{
			return Err(GetLastError());
		}
	}

	Ok(())
}
