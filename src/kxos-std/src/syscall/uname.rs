use crate::prelude::*;

use crate::{memory::write_val_to_user, syscall::SYS_UNAME};

use super::SyscallReturn;

// We don't use the real name and version of our os here. Instead, we pick up fake values witch is the same as the ones of linux.
// The values are used to fool glibc since glibc will check the version and os name.
lazy_static! {
    /// used to fool glibc
    static ref SYS_NAME: CString = CString::new("Linux").unwrap();
    static ref NODE_NAME: CString = CString::new("WHITLEY").unwrap();
    static ref RELEASE: CString = CString::new("5.13.0").unwrap();
    static ref VERSION: CString = CString::new("5.13.0").unwrap();
    static ref MACHINE: CString = CString::new("x86_64").unwrap();
    static ref DOMAIN_NAME: CString = CString::new("").unwrap();
    static ref UTS_NAME: UtsName = {
        let mut uts_name = UtsName::new();
        copy_cstring_to_u8_slice(&SYS_NAME, &mut uts_name.sysname);
        copy_cstring_to_u8_slice(&NODE_NAME, &mut uts_name.nodename);
        copy_cstring_to_u8_slice(&RELEASE, &mut uts_name.release);
        copy_cstring_to_u8_slice(&VERSION, &mut uts_name.version);
        copy_cstring_to_u8_slice(&MACHINE, &mut uts_name.machine);
        copy_cstring_to_u8_slice(&DOMAIN_NAME, &mut uts_name.domainname);
        uts_name
    };
}

const UTS_FIELD_LEN: usize = 65;

#[derive(Debug, Clone, Copy, Pod)]
#[repr(C)]
struct UtsName {
    sysname: [u8; UTS_FIELD_LEN],
    nodename: [u8; UTS_FIELD_LEN],
    release: [u8; UTS_FIELD_LEN],
    version: [u8; UTS_FIELD_LEN],
    machine: [u8; UTS_FIELD_LEN],
    domainname: [u8; UTS_FIELD_LEN],
}

impl UtsName {
    const fn new() -> Self {
        UtsName {
            sysname: [0; UTS_FIELD_LEN],
            nodename: [0; UTS_FIELD_LEN],
            release: [0; UTS_FIELD_LEN],
            version: [0; UTS_FIELD_LEN],
            machine: [0; UTS_FIELD_LEN],
            domainname: [0; UTS_FIELD_LEN],
        }
    }
}

fn copy_cstring_to_u8_slice(src: &CStr, dst: &mut [u8]) {
    let src = src.to_bytes_with_nul();
    let len = src.len().min(dst.len());
    dst[..len].copy_from_slice(&src[..len]);
}

pub fn sys_uname(old_uname_addr: u64) -> Result<SyscallReturn> {
    debug!("[syscall][id={}][SYS_UNAME]", SYS_UNAME);
    do_sys_uname(old_uname_addr as Vaddr)?;
    Ok(SyscallReturn::Return(0))
}

pub fn do_sys_uname(old_uname_addr: Vaddr) -> Result<usize> {
    debug!("old_uname_addr: 0x{:x}", old_uname_addr);
    debug!("uts name size: {}", core::mem::size_of::<UtsName>());
    debug!("uts name align: {}", core::mem::align_of::<UtsName>());

    write_val_to_user(old_uname_addr, &*UTS_NAME)?;
    Ok(0)
}
