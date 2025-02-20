#[repr(C)]
#[derive(Debug)]
pub struct ResourceTable {
    pub ver: u32,
    pub num: u32,
    pub reserved: [u32; 2usize],
    pub offset: [u32; 0],
}

#[repr(C)]
#[derive(Debug)]
pub struct FwRscHdr {
    pub type_: u32,
    pub data: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FwRscCarveout {
    pub da: u32,
    pub pa: u32,
    pub len: u32,
    pub flags: u32,
    pub reserved: u32,
    pub name: [u8; 32usize],
}

#[repr(C)]
#[derive(Debug)]
pub struct RemoteResourceTable {
    pub resource_table: ResourceTable,
    pub offset: [u32; 1usize],
    pub carve_out: FwRscHdr,
    pub carve_out_data: FwRscCarveout,
}




#[link_section = ".resource_table"]
#[no_mangle]
#[used]
static RESOURCE_TABLE: RemoteResourceTable = RemoteResourceTable{
    resource_table: ResourceTable{ver: 1, num: 1, reserved: [0; 2], offset: [0; 0]},
    offset: [0x14;1 ],
    carve_out: FwRscHdr{type_: 0, data: [0;0]},
    carve_out_data: FwRscCarveout{da: 0x8fe00000, pa: 0x8fe00000, len: 0x200000,flags: 0x0, reserved: 0, name: 
        [116, 101, 120, 116,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},
};