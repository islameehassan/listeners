use std::ffi::c_uchar;
use std::os::raw::c_ulong;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(super) struct Tcp6Table {
    pub(super) rows_count: c_ulong,
    pub(super) rows: [Tcp6Row; 1],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(super) struct Tcp6Row {
    pub(super) local_addr: [c_uchar; 16],
    local_scope_id: c_ulong,
    pub(super) local_port: c_ulong,
    remote_addr: [c_uchar; 16],
    remote_scope_id: c_ulong,
    remote_port: c_ulong,
    pub(super) state: c_ulong,
    pub(super) owning_pid: c_ulong,
}
