#![allow(dead_code)]

use lazy_static::lazy_static;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::registers::{ReadWrite};
use tock_registers::{register_structs};
use crate::common::rwlock::RwLock;

pub const MAILBOX_MAX_NUM: usize = 8;
pub const MAILBOX_DONE_OFFSET: usize = 2;
pub const MAILBOX_CONTEXT_OFFSET: usize = 1024;
pub const RECEIVE_CPU: usize = 2;
pub const SEND_TO_CPU: usize = 1;
use crate::config::{MAILBOX_REG_BASE, MAILBOX_REG_BUFF};

#[repr(C,packed(8))]
pub struct CpuMboxInt {
    pub cpu_mbox_int_clr: ReadWrite<u32>,
    pub cpu_mbox_int_mask: ReadWrite<u32>,
    pub cpu_mbox_int_int: ReadWrite<u32>,
    pub cpu_mbox_int_raw: ReadWrite<u32>,
}


register_structs! {
    pub MailboxSetRegister {
        (0x00 => pub cpu_mbox_en:[ReadWrite<u32>; 4]),
        (0x10 => pub cpu_mbox_set: [CpuMboxInt; 4]),
        (0x50 => reserved1: [u32; 4]),
        (0x60 => pub mbox_set: ReadWrite<u32>),
        (0x64 => pub mbox_status: ReadWrite<u32>),
        (0x68 => reserved2: [u32; 2]),
        (0x70 => pub cpu_mbox_status: [u32; 4]),
        (0x80 => @END),
    }
}




#[repr(C, packed(8))]
#[derive(Copy, Clone, Debug, Default)]
pub struct ValidT {
	linux_valid: u8,
	rtos_valid: u8
}

#[repr(C, packed(8))]
#[derive(Copy, Clone, Debug, Default)]
pub struct CmdquT {
    pub ip_id: u8,
    pub cmd_id: u8,
    pub valid: ValidT,
    pub param_ptr: i32,
}


pub struct MailBox(usize);
impl MailBox{
    pub fn new(base: usize) -> Self{
        MailBox(base)
    }
    pub fn mbox_reg(&self) -> &mut MailboxSetRegister {
        unsafe {
            &mut *(self.0 as *mut MailboxSetRegister)
        }
    }

    pub fn mbox_content_reg(&self) -> *mut CmdquT {
        MAILBOX_REG_BUFF as *mut CmdquT
    }

    pub fn enable(&self, channel: usize){
        self.mbox_reg().cpu_mbox_set[SEND_TO_CPU].cpu_mbox_int_mask.set(self.mbox_reg().cpu_mbox_set[SEND_TO_CPU].cpu_mbox_int_mask.get() &!(1 << channel));
    }
    pub fn read (&self, channel: usize) -> CmdquT
    {

        let set_val = self.mbox_reg().cpu_mbox_set[RECEIVE_CPU].cpu_mbox_int_int.get();
        let mut data= Default::default();
        for i in 0..MAILBOX_MAX_NUM
        {
            let valid_val = set_val & (1 << i);
            if valid_val.ne(&0)
            {
                data = unsafe {self.mbox_content_reg().add(channel).read_volatile()};

            }
        }
        self.mbox_reg().cpu_mbox_set[RECEIVE_CPU].cpu_mbox_int_clr.set(1 << channel);
        self.mbox_reg().cpu_mbox_en[RECEIVE_CPU].set( self.mbox_reg().cpu_mbox_en[RECEIVE_CPU].get()&!(1 << channel));
        data
    }
    pub fn write (&self, channel: usize, data: CmdquT)
    {
        let mut data = data;
        data.valid.rtos_valid = 1;
        data.cmd_id = data.cmd_id << 1 | 1;
        unsafe {
            self.mbox_content_reg().add(channel).write_volatile(data);  
        }
        // clear mailbox
        self.mbox_reg().cpu_mbox_set[SEND_TO_CPU].cpu_mbox_int_clr.set(1 << channel);
        // trigger mailbox valid to rtos
        self.mbox_reg().cpu_mbox_en[SEND_TO_CPU].set(self.mbox_reg().cpu_mbox_en[SEND_TO_CPU].get()|(1 << channel));
        self.mbox_reg().mbox_set.set(1 << channel);
    }
}

lazy_static!{
    pub static ref MBOX_MSG: RwLock<[Option<CmdquT>;8]> = RwLock::new([None; 8]);
}
pub fn mail_box_handler(){
    let mbox = MailBox::new(MAILBOX_REG_BASE);
    let data = mbox.read(0);
    mbox.write(0, data);
    let mut mbox = MBOX_MSG.write();
    for i in 0..8 {
        match mbox[i]{
            None => {
                mbox[i].replace(data);
            }
            Some(_) => {}
        }
    }
}
pub fn mail_box_fetch() -> Option<CmdquT>{
    let mut mbox = MBOX_MSG.write();
    for i in 0..8 {
        match mbox[i]{
            None => {}
            Some(data) => {
                mbox[i] = None;
                return Some(data);
            }
        }
    }
    None
}