use core::slice;

use alloc::{string::String, vec::Vec};

use crate::{gsdata::GsData, process, MODULE_RESPONSE};

pub extern "C" fn main() -> ! {
    log::info!("root process started");

    let mut living_children: Vec<usize> = {
        let process = unsafe { GsData::process() }
            .ok()
            .flatten()
            .expect("root process gsdata.process is missing");

        MODULE_RESPONSE
            .modules()
            .iter()
            .filter(|file| file.path().starts_with(b"/bin/"))
            .map(|file| {
                let name = String::from_utf8_lossy(&file.path()[5..]);

                let addr = file.addr();
                let size = file.size() as usize;
                let slice = unsafe { slice::from_raw_parts(addr, size) };
                (name, slice)
            })
            .map(|(name, data)| process.create_user(&name, data))
            .collect()
    };

    loop {
        x86_64::instructions::hlt();
        {
            let process = unsafe { GsData::process() }
                .ok()
                .flatten()
                .expect("root process gsdata.process is missing");
            let children = &process.children;
            living_children.retain(|child_id| match children.get(*child_id) {
                Some(child) => match child.state {
                    process::SubprocessState::Alive => true,
                    process::SubprocessState::Dead(code) => {
                        log::info!("{} exited with code {code}", child.name);
                        false
                    }
                },
                None => false,
            });
        }
    }
}
