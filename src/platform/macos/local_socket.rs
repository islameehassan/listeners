use crate::platform::macos::c_socket_fd_info::CSocketFdInfo;
use crate::platform::macos::libproc::proc_pidfdinfo;
use crate::platform::macos::pid::Pid;
use crate::platform::macos::socket_fd::SocketFd;
use crate::platform::macos::statics::PROC_PID_FD_SOCKET_INFO;
use std::ffi::{c_int, c_void};
use std::mem;
use std::mem::MaybeUninit;
use std::net::{IpAddr, SocketAddr};

#[derive(Debug)]
pub(super) struct LocalSocket(SocketAddr);

impl LocalSocket {
    pub(super) fn new(addr: IpAddr, port: u16) -> Self {
        LocalSocket(SocketAddr::new(addr, port))
    }

    pub(super) fn socket_addr(&self) -> SocketAddr {
        self.0
    }

    pub(super) fn from_pid_fd(pid: Pid, fd: &SocketFd) -> crate::Result<Self> {
        let mut sinfo: MaybeUninit<CSocketFdInfo> = MaybeUninit::uninit();

        let return_code = unsafe {
            proc_pidfdinfo(
                pid.as_c_int(),
                fd.fd(),
                PROC_PID_FD_SOCKET_INFO,
                sinfo.as_mut_ptr().cast::<c_void>(),
                c_int::try_from(mem::size_of::<CSocketFdInfo>())?,
            )
        };

        if return_code < 0 {
            return Err("Failed to get file descriptor information".into());
        }

        let c_socket_fd_info = unsafe { sinfo.assume_init() };
        c_socket_fd_info.to_local_socket()
    }
}
