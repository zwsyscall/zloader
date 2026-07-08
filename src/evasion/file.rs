use std::os::windows::ffi::OsStrExt;
use std::{env, fs};

use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Storage::FileSystem::{
    CreateFileW, DELETE, FILE_ATTRIBUTE_NORMAL, FILE_DISPOSITION_INFO_EX,
    FILE_DISPOSITION_INFO_EX_FLAGS, FILE_FLAG_BACKUP_SEMANTICS, FILE_RENAME_INFO,
    FILE_SHARE_DELETE, FILE_SHARE_READ, FILE_SHARE_WRITE, FileDispositionInfoEx, FileRenameInfo,
    OPEN_EXISTING, SetFileInformationByHandle,
};
use windows::core::PCWSTR;

struct SafeHandle(HANDLE);
impl Drop for SafeHandle {
    fn drop(&mut self) {
        if !self.0.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.0);
            }
        }
    }
}

pub fn self_delete() -> Result<Vec<u8>, windows::core::Error> {
    let path = env::current_exe()?;
    let file_data = fs::read(&path)?;
    let mut path_wide: Vec<u16> = path.as_os_str().encode_wide().collect();
    path_wide.push(0);

    let open_file = |flags| unsafe {
        CreateFileW(
            PCWSTR(path_wide.as_ptr()),
            DELETE.0,
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            None,
            OPEN_EXISTING,
            flags,
            None,
        )
        .map(SafeHandle)
    };

    {
        let handle = open_file(FILE_FLAG_BACKUP_SEMANTICS)?;
        let stream_name: Vec<u16> = ":kurwa".encode_utf16().collect();
        let name_bytes = (stream_name.len() * size_of::<u16>()) as u32;
        let total_size = size_of::<FILE_RENAME_INFO>() + name_bytes as usize;

        let mut buffer = vec![0u64; total_size.div_ceil(8)];
        let rename_info = buffer.as_mut_ptr() as *mut FILE_RENAME_INFO;

        unsafe {
            (*rename_info).Anonymous.ReplaceIfExists = true.into();
            (*rename_info).FileNameLength = name_bytes;

            std::ptr::copy_nonoverlapping(
                stream_name.as_ptr(),
                (*rename_info).FileName.as_mut_ptr(),
                stream_name.len(),
            );

            SetFileInformationByHandle(
                handle.0,
                FileRenameInfo,
                buffer.as_ptr() as *const _,
                total_size as u32,
            )?;
        }
    }

    {
        let handle = open_file(FILE_ATTRIBUTE_NORMAL)?;
        let mut disp_info = FILE_DISPOSITION_INFO_EX {
            Flags: FILE_DISPOSITION_INFO_EX_FLAGS(0x1 | 0x2 | 0x10),
        };

        unsafe {
            SetFileInformationByHandle(
                handle.0,
                FileDispositionInfoEx,
                &mut disp_info as *mut _ as *const _,
                size_of::<FILE_DISPOSITION_INFO_EX>() as u32,
            )?;
        }
    }

    Ok(file_data)
}
