use crate::{pr_info, pr_notice};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct mailboxmsg {
    pub data: *const u32,
    pub channel: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cpu_mailbox_info_offset {
    pub mbox_info: i8,
    pub reserved: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailbox_set_register {
    pub cpu_mbox_en: [cpu_mailbox_info_offset; 4],
    pub cpu_mbox_set: [cpu_mbox_int; 4],
    pub reserved: [i32; 4],
    pub mbox_set: mailbox_set,
    pub mbox_status: mailbox_status,
    pub reserved2: [i32; 2],
    pub cpu_mbox_status: [cpu_mailbox_status; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cpu_mailbox_status {
    pub mbox_status: u8,
    pub reserved: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mailbox_status {
    pub mbox_status: u8,
    pub reserved: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union mailbox_set {
    pub mbox_set: u8,
    pub reserved: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_mbox_int {
    pub cpu_mbox_int_clr: cpu_mailbox_int_clr_offset,
    pub cpu_mbox_int_mask: cpu_mailbox_int_mask_offset,
    pub cpu_mbox_int_int: cpu_mailbox_int_offset,
    pub cpu_mbox_int_raw: cpu_mailbox_int_raw_offset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cpu_mailbox_int_raw_offset {
    pub mbox_int_raw: i8,
    pub reserved: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cpu_mailbox_int_offset {
    pub mbox_int: i8,
    pub reserved: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cpu_mailbox_int_mask_offset {
    pub mbox_int_mask: i8,
    pub reserved: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union cpu_mailbox_int_clr_offset {
    pub mbox_int_clr: i8,
    pub reserved: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailbox_done_register {
    pub cpu_mbox_done_en: [cpu_mailbox_info_offset; 4],
    pub cpu_mbox_done: [cpu_mbox_int; 4],
}

pub struct MailBox{
    mbox_reg: *mut mailbox_set_register,
    mbox_done_reg: *mut mailbox_done_register,
    mailbox_context: *mut u32,
    callbacks: [Option::<fn(&mailboxmsg) -> ()>; 8]
}

impl MailBox {
    pub const fn new(base: usize) -> Self{
        let mbox_reg = base as *mut mailbox_set_register;
        let mbox_done_reg = (0x1900000 as i32 + 2 as i32)as *mut mailbox_done_register;
        let mailbox_context = (0x1900000 as i32 + 0x400 as i32) as *mut u32;
        Self{
            mbox_reg,
            mbox_done_reg,
            mailbox_context,
            callbacks: [None; 8]
        }
    }
    pub fn init(&self){
        for i in 0..8 {
            unsafe {
                core::ptr::write_volatile(
                    self.mailbox_context.offset(i as isize),
                    0u32,
                ); 
            }
        }
    }
    pub fn mailbox_register(
        &mut self,
        channel: i32,
        callback: fn(&mailboxmsg) -> (),
    ) {
        if channel >= 0 as i32 && channel < 8 as i32 {
            self.callbacks[channel as usize].replace(callback);
        }
    }
    pub fn mailbox_unregister(&mut self, channel: i32) {
        if channel >= 0 as i32 && channel < 8 as i32 {
            self.callbacks[channel as usize] = None;
        }
    }
    fn do_callback(&self, channel: i32, msg: &mailboxmsg) {
        match self.callbacks[channel as usize]{
            Some(f) => f(msg),
            None => pr_notice!("channel {} not registered!", channel)
        }
    }
    pub fn mailbox_disable_receive(&self, channel: i32) {
        unsafe {
            ::core::ptr::write_volatile(
                &mut (*self.mbox_reg)
                    .cpu_mbox_set[1 as i32 as usize]
                    .cpu_mbox_int_mask
                    .mbox_int_mask as *mut i8,
                (::core::ptr::read_volatile::<
                    i8,
                >(
                    &(*self.mbox_reg)
                        .cpu_mbox_set[1 as i32 as usize]
                        .cpu_mbox_int_mask
                        .mbox_int_mask as *const i8,
                ) as i32 | (1 as i32) << channel) as i8 as i8,
            );
        }

    }
    pub fn mailbox_enable_receive(&self, channel: i32) {
        unsafe {
            ::core::ptr::write_volatile(
                &mut (*self.mbox_reg)
                    .cpu_mbox_set[1 as i32 as usize]
                    .cpu_mbox_int_mask
                    .mbox_int_mask as *mut i8,
                (::core::ptr::read_volatile::<
                    i8,
                >(
                    &(*self.mbox_reg)
                        .cpu_mbox_set[1 as i32 as usize]
                        .cpu_mbox_int_mask
                        .mbox_int_mask as *const i8,
                ) as i32 & !((1 as i32) << channel)) as i8
                    as i8,
            );
        }

    }

    pub fn mailbox_read(&self, channel: i32, msg: &mut mailboxmsg) {
        msg.channel = channel;
        msg.data = unsafe {
            (self.mailbox_context).offset(channel as isize)
        };
        unsafe {
            ::core::ptr::write_volatile(
                &mut (*self.mbox_reg)
                    .cpu_mbox_set[2 as i32 as usize]
                    .cpu_mbox_int_clr
                    .mbox_int_clr as *mut i8,
                ((1 as i32) << channel) as i8,
            );
            ::core::ptr::write_volatile(
                &mut (*self.mbox_reg).cpu_mbox_en[2 as i32 as usize].mbox_info
                    as *mut i8,
                (::core::ptr::read_volatile::<
                    i8,
                >(
                    &(*self.mbox_reg).cpu_mbox_en[2 as i32 as usize].mbox_info
                        as *const i8,
                ) as i32 & !((1 as i32) << channel)) as i8
                    as i8,
            );
        }
    }
    pub fn mailbox_write(&self, msg: &mut mailboxmsg) {
        unsafe {
            ::core::ptr::write_volatile(
                &mut (*self.mbox_reg)
                    .cpu_mbox_set[1 as i32 as usize]
                    .cpu_mbox_int_clr
                    .mbox_int_clr as *mut i8,
                ((1 as i32) << msg.channel) as i8,
            );
            ::core::ptr::write_volatile(
                &mut (*self.mbox_reg).cpu_mbox_en[1 as i32 as usize].mbox_info
                    as *mut i8,
                (::core::ptr::read_volatile::<
                    i8,
                >(
                    &(*self.mbox_reg).cpu_mbox_en[1 as i32 as usize].mbox_info
                        as *const i8,
                ) as i32 | (1 as i32) << msg.channel) as i8
                    as i8,
            );
            ::core::ptr::write_volatile(
                &mut (*self.mbox_reg).mbox_set.mbox_set as *mut u8,
                ((1 as i32) << msg.channel) as u8,
            );
        }

    }
    pub fn mailbox_isr(&self) -> i32 {
        let mut msg: mailboxmsg = mailboxmsg {
            data: 0 as _,
            channel: 0,
        };
        let mut valid_val: u8;
        let set_val: u8;
        unsafe {
            set_val = (*self.mbox_reg)
            .cpu_mbox_set[2 as i32 as usize]
            .cpu_mbox_int_int
            .mbox_int as u8;
        }
        for i in 0..8 {
            valid_val = (set_val as i32 & (1 as i32) << i) as u8;
            if valid_val != 0 {
                self.mailbox_read(i, &mut msg);
                self.mailbox_write(&mut msg);
                self.do_callback(i, &msg);
            }
        }
        return 0;
    }
}

