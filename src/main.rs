use std::env;
use std::io;
use structopt::StructOpt;

mod opts;
mod uname;
mod writer;

use crate::opts::Opts;
use crate::writer::WriteReceiver;

fn main() {
    let opt = Opts::from_args();
    let stdout = io::stdout();
    let lock = stdout.lock();
    main_generic(opt, &mut io::LineWriter::new(lock));
}

fn main_generic<W: WriteReceiver>(mut opt: Opts, handle: &mut W) {
    if opt.all {
        let contents = uname::all_uname_infos();
        handle.write_with_separator(contents.as_str(), false);
        handle.write(b"\n").unwrap();
        return;
    }

    let infos = uname::uname().unwrap();
    if !(opt.kernel_name
        || opt.nodename
        || opt.kernel_release
        || opt.kernel_version
        || opt.machine
        || opt.processor)
    {
        opt.kernel_name = true
    }

    if opt.kernel_name {
        handle.write_with_separator(uname::to_str(&infos.sysname), true);
    }

    if opt.nodename {
        handle.write_with_separator(uname::to_str(&infos.nodename), true);
    }

    if opt.kernel_release {
        handle.write_with_separator(uname::to_str(&infos.release), true);
    }

    if opt.kernel_version {
        handle.write_with_separator(uname::to_str(&infos.version), true);
    }

    if opt.machine {
        handle.write_with_separator(uname::to_str(&infos.machine), true);
    }

    if opt.processor {
        handle.write_with_separator(env::consts::ARCH, true);
    }

    handle.write_with_separator("\n", false);
}
