use std::ffi::CStr;
use std::mem;
use std::str::from_utf8_unchecked;

pub fn uname() -> Result<libc::utsname, ()> {
    let mut uname = unsafe { mem::zeroed() };
    let res = unsafe { libc::uname(&mut uname) };
    if res < 0 {
        Err(())
    } else {
        Ok(uname)
    }
}

#[inline]
pub fn to_str(bytes: &[libc::c_char]) -> &str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(bytes.as_ptr()).to_bytes()) }
}

pub fn all_uname_infos() -> String {
    let uname = uname().unwrap();
    format!(
        "{} {} {} {} {}",
        to_str(&uname.sysname),
        to_str(&uname.nodename),
        to_str(&uname.release),
        to_str(&uname.version),
        to_str(&uname.machine),
    )
}
