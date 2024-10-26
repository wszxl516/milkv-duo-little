#[allow(dead_code)]
#[repr(u32)]
pub enum FwResourceType {
    RscCarveout = 0,
    RscDevmem = 1,
    RscTrace = 2,
    RscVdev = 3,
    RscLast = 4,
}

#[repr(C)]
pub struct ResourceTableHeader<const N: usize> {
    pub ver: u32,
    pub num: u32,
    pub reserved: [u32; 2],
    pub offsets: [u32; N],
}

impl<const N: usize> ResourceTableHeader<N> {
    const fn new(offsets: [u32; N]) -> Self {
        ResourceTableHeader {
            ver: 1,
            num: N as u32,
            reserved: [0, 0],
            offsets,
        }
    }
}

#[repr(C)]
pub struct ResourceTable {
    pub header: ResourceTableHeader<0>,
}

#[link_section = ".resource_table"]
#[no_mangle]
#[used]
pub static RESOURCE_TABLE: ResourceTable = ResourceTable {
    header: ResourceTableHeader::new([]),
};
