use winapi::{ctypes::c_void, shared::{minwindef::{DWORD, FALSE}, windef::HWND__}, um::{processthreadsapi::OpenProcess, psapi::GetProcessImageFileNameA, winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ}, winuser::{GetForegroundWindow, GetWindowThreadProcessId}}};

#[cfg(windows)]
use crate::core::process::Process;

#[cfg(windows)] extern crate winapi;

unsafe fn get_current_active_window_handle() -> *mut HWND__{
    
    unsafe{
        let handle = GetForegroundWindow();
        return handle;
    }
}
unsafe fn get_process_exec_name(handle:*mut c_void) -> String{

    const BUFFER_SIZE:usize = 256;
    let mut buffer: [i8;BUFFER_SIZE] =[0;BUFFER_SIZE];
    unsafe{
        let name_length = GetProcessImageFileNameA(handle, buffer.as_mut_ptr(), BUFFER_SIZE.try_into().unwrap());
        let name = String::from_utf8_lossy(&buffer[..name_length.try_into().unwrap()].iter().map(|&c| c as u8 ).collect::<Vec<u8>>()).into_owned();
        return name;
    }
    
}

unsafe fn current_active_window_handle_to_process_handle(window_handle:*mut HWND__) -> *mut c_void{
    unsafe{
        let mut dw_pid: DWORD = 0;
        GetWindowThreadProcessId(window_handle, &mut dw_pid);
        OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, dw_pid)

    }
}

fn parse_process_exec_name(name:&str)->Option<String>{

    name.rsplit('\\').next().map(|name|{
        name.strip_suffix(".exe").unwrap_or(name).to_string()
    })
}

unsafe fn get_process_title_name(window_handle: *mut HWND__) -> String{
    use winapi::um::winuser::GetWindowTextW;
    const BUFFER_SIZE:usize = 256;
    let mut buffer:[u16;BUFFER_SIZE] = [0;BUFFER_SIZE];

    unsafe{
        let title_length = GetWindowTextW(window_handle,buffer.as_mut_ptr(),BUFFER_SIZE.try_into().unwrap());
        let title = String::from_utf16_lossy(&buffer[..title_length.try_into().unwrap()].iter().map(|&c| c as u16 ).collect::<Vec<u16>>()).to_owned();
        title
    }
}

#[cfg(windows)]
pub unsafe  fn get_current_active_process() -> Process{
    unsafe{
        let window_handle = get_current_active_window_handle();
        let title_name = get_process_title_name(window_handle);

        let process_handle = current_active_window_handle_to_process_handle(window_handle);
        let exec_name = get_process_exec_name(process_handle);
        
        Process { title_name: title_name, exec_name: parse_process_exec_name(&exec_name).unwrap_or(exec_name) }
    }
}