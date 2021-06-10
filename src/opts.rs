use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "unamers", about = "Print certain system information.")]
#[structopt(setting = structopt::clap::AppSettings::AllowLeadingHyphen)]
pub struct Opts {
    /// Print all information, in the following order.
    #[structopt(short, long)]
    pub all: bool,

    /// Print the kernel name.
    #[structopt(short = "s", long)]
    pub kernel_name: bool,

    /// Print the network node hostname.
    #[structopt(short, long)]
    pub nodename: bool,

    /// Print the kernel release.
    #[structopt(short = "r", long)]
    pub kernel_release: bool,

    /// Print the kernel version.
    #[structopt(short = "v", long)]
    pub kernel_version: bool,

    /// Print the machine hardware name.
    #[structopt(short = "m", long)]
    pub machine: bool,

    /// Print the operating system.
    #[structopt(short, long)]
    pub operating_system: bool,
}

// root@531763fbd5b0:/# uname --help
// Usage: uname [OPTION]...
// Print certain system information.  With no OPTION, same as -s.

//   -a, --all                print all information, in the following order,
//                              except omit -p and -i if unknown:
//   -s, --kernel-name        print the kernel name
//   -n, --nodename           print the network node hostname
//   -r, --kernel-release     print the kernel release
//   -v, --kernel-version     print the kernel version
//   -m, --machine            print the machine hardware name
//   -p, --processor          print the processor type (non-portable)
//   -i, --hardware-platform  print the hardware platform (non-portable)
//   -o, --operating-system   print the operating system
//       --help     display this help and exit
//       --version  output version information and exit

// GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
// Report uname translation bugs to <https://translationproject.org/team/>
// Full documentation at: <https://www.gnu.org/software/coreutils/uname>
// or available locally via: info '(coreutils) uname invocation'
