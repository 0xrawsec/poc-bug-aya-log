#![no_std]
#![no_main]

use aya_bpf::{helpers::bpf_get_prandom_u32, macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::{error, info};

#[repr(C)]
enum Error {
    Foo,
    Bar,
}

impl Error {
    const fn into_str(self) -> &'static str {
        match self {
            Self::Foo => "foo",
            Self::Bar => "bar",
        }
    }
}

#[kprobe]
pub fn poc_bug_aya_log(ctx: ProbeContext) -> u32 {
    match try_poc_bug_aya_log(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_poc_bug_aya_log(ctx: ProbeContext) -> Result<u32, u32> {
    info!(&ctx, "function schedule called");
    let err = {
        if unsafe { bpf_get_prandom_u32() } % 2 == 0 {
            Error::Bar
        } else {
            Error::Foo
        }
    };
    error!(&ctx, "{}", err.into_str());
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
