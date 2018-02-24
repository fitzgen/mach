extern crate ctest;

#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug)]
struct Xcode(pub u32, pub u32);

impl Xcode {
    fn version() -> Xcode {
        use std::process::Command;
        let out = Command::new("/usr/bin/xcodebuild")
            .arg("-version").output().expect("failed to execute xcodebuild");
        let stdout = ::std::str::from_utf8(&out.stdout).expect("couldn't parse stdout as UTF8");
        let stderr = ::std::str::from_utf8(&out.stderr).expect("couldn't parse stderr as UTF8");

        if !out.status.success() {
            eprintln!("stdout: {}", stdout);
            eprintln!("stderr: {}", stderr);
            panic!("xcodebuild -version failed");
        }

        // xcodebuild -version output looks like:
        //
        // Xcode 9.2
        // Build version 9C40b
        let mut iter = stdout.split(|c: char| c.is_whitespace() || c == '.').skip(1).map(
            |c| c.parse().expect("failed to parse Xcode version into number")
        );
        let major: u32 = iter.next().expect("failed to parse Xcode major version");
        let minor: u32 = iter.next().expect("failed to parse Xcode minor version");

        Xcode(major, minor)
    }
}

fn main() {
    let xcode = Xcode::version();
    // kept on purpose for debugging:
    // println!("cargo:warning=\"Xcode version: {:?}\"", xcode);

    let mut cfg = ctest::TestGenerator::new();

    // Include the header files where the C APIs are defined
    cfg.header("mach/mach.h")
        .header("mach/clock_types.h")
        .header("mach/i386/boolean.h")
        .header("mach/kern_return.h")
        .header("mach/i386/kern_return.h")
        .header("mach/mach_port.h")
        .header("mach/memory_object_types.h")
        .header("mach/message.h")
        .header("mach/port.h")
        .header("mach/i386/_structs.h")
    // .header("mach/task.defs")
     .header("mach/task.h")
        .header("mach/task_info.h")
    //.header("mach/thread_act.defs")
        .header("mach/thread_act.h")
        .header("mach/thread_status.h")
        .header("mach/mach_traps.h")
        .header("mach/mach_types.h")
    //.header("mach/mach_vm.defs.")
        .header("mach/mach_vm.h")
        .header("mach/vm_attributes.h")
        .header("mach/vm_behavior.h")
        .header("mach/vm_inherit.h")
        .header("mach/vm_page_size.h")
        .header("mach/vm_prot.h")
        .header("mach/vm_purgable.h")
        .header("mach/vm_region.h")
        .header("mach/vm_statistics.h")
        .header("mach/vm_sync.h")
        .header("mach/i386/vm_types.h")
        .header("bootstrap.h");

    cfg.skip_struct(|s| {
        match s {
            // TODO: this type is a bitfield and must be verified by hand
            "mach_msg_port_descriptor_t" |

            // TODO: this type is a bitfield and must be verified by hand
            "mach_msg_ool_descriptor_t" |

            // TODO: this type is a bitfield and must be verified by hand
            "mach_msg_ool_ports_descriptor_t" |

            // FIXME: this type is not exposed in /usr/include/mach
            // but seems to be exposed in
            // SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach
            "ipc_port" |

            // FIXME: multiple type mismatches in struct fields
            // FIXME: not #[repr(C)]
            "vm_statistics" |

            // FIXME: removed since MacOSX 10.3.9:
            // FIXME: not #[repr(C)]
            "pmap_statistics" |

            // FIXME: not #[repr(C)]
            "task_dyld_info" |

            // FIXME (x86_64): wrong type offsets, field types, etc.
            // * size: rust: 40 (0x28) != c 36 (0x24)
            // * align: rust: 8 (0x8) != c 4 (0x4)
            "vm_region_basic_info_64" |

            // FIXME: bad field offsets, types, etc
            // * i686: size: rust: 64 (0x40) != c 68 (0x44)
            // * x86_64:
            //   * size: rust: 72 (0x48) != c 68 (0x44)
            //   * align: rust: 8 (0x8) != c 4 (0x4)
            "vm_region_submap_info_64" |

            // FIXME (x86_64):
            // * size: rust: 56 (0x38) != c 48 (0x30)
            // * align: rust: 8 (0x8) != c 4 (0x4)
            "vm_region_submap_short_info_64" |

            // FIXME (x86_64): align: rust: 8 (0x8) != c 4 (0x4)
            "mach_vm_read_entry" |

            // FIXME: size: rust: 32 (0x20) != c 36 (0x24)
            "vm_region_extended_info"
            => true,
            _ => false,
        }
    });

    cfg.skip_type(|s| {
        match s {
            // FIXME: this type is not exposed in /usr/include/mach
            // but seems to be exposed in
            // SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach
            "ipc_port_t" |

            // FIXME: type duplicated multiple times inside the library.
            "task_name_t" |

            // FIXME: removed since MacOSX 10.3.9:
            "pmap_statistics_t" |

            // FIXME (i686): size: rust: 4 (0x4) != c 8 (0x8)
            "user_addr_t"|

            // FIXME (686): align: rust: 4 (0x4) != c 8 (0x8)
            "bad user_addr_t" |

            // FIXME (x86_64):
            // * size: rust: 40 (0x28) != c 36 (0x24)
            // * align: rust: 8 (0x8) != c 4 (0x4)
            "vm_region_basic_info_data_64_t" |

            // FIXME:
            // * i686: size: rust: 64 (0x40) != c 68 (0x44)
            // * x86_64:
            //   * size: rust: 72 (0x48) != c 68 (0x44)
            //   * align: rust: 8 (0x8) != c 4 (0x4)
            "vm_region_submap_info_data_64_t"|

            // FIXME: size: rust: 32 (0x20) != c 36 (0x24)
            "vm_region_extended_info_data_t" |

            // FIXME (x86_64): field types, offsets, etc.
            // * size: rust: 56 (0x38) != c 48 (0x30)
            // * align: rust: 8 (0x8) != c 4 (0x4)
            "vm_region_submap_short_info_data_64_t" |

            // FIXME (x86_64): align: rust: 8 (0x8) != c 4 (0x4)
            "mach_vm_read_entry_t" |

            // FIXME: size: rust: 48 (0x30) != c 60 (0x3c)
            "vm_statistics_data_t"

                => true,
            _ => false,
        }
    });

    cfg.skip_fn(|s| {
        match s {
            // FIXME: not found in /usr/include/mach but maybe in
            // SDKs/MacOSX.sdk/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach
            "mach_task_self" |

            // FIXME: incompatible function type:
            // 'int  (unsigned int, unsigned long long, unsigned long, unsigned int)'
            // vs
            // 'void (unsigned int, unsigned long long, unsigned long, unsigned int)'
            "mach_vm_write" |

            // FIXME: incompatible function type:
            // 'int (unsigned int, unsigned long long *, unsigned long long, unsigned long long, int, unsigned int, unsigned long long, unsigned int, int, int, unsigned int)'
            // vs
            // 'int (unsigned int, unsigned long long, unsigned long long, unsigned long long, int, unsigned int, unsigned long long, unsigned int, int, int, unsigned int)'
            "mach_vm_map" |
            // FIXME: incompatible function type:
            // 'int (unsigned int, unsigned long *, unsigned long, int, unsigned int *, unsigned int)'
            // 'int (*)(unsigned int, unsigned long long *, unsigned long long, int, unsigned int *, unsigned int)'
            "mach_make_memory_entry"

            => true,
            _ => false,
        }
    });

    cfg.skip_const(move |s| {
        match s {
            // FIXME: wrong value: byte 1: rust: 7 (0x7) != c 0 (0x0)
            "VM_VOLATILE_GROUP_DEFAULT" |
            // FIXME: wrong value:
            // * i686: byte 0: rust: 5 (0x5) != c 4 (0x4)
            "TASK_BASIC_INFO" |
            // FIXME: wrong value: byte 0: rust: 11 (0xb) != c 13 (0xd)
            "VM_REGION_EXTENDED_INFO"
            => true,
            // Added in MacOSX 10.11.0 (Xcode 7.3)
            "TASK_VM_INFO_PURGEABLE_ACCOUNT" | "TASK_FLAGS_INFO" | "TASK_DEBUG_INFO_INTERNAL"
                if xcode < Xcode(7, 3) => true,
            // Removed after MacOSX 10.6 (does not appear in MacOSX 10.7)
            "VM_PROT_TRUSTED" if xcode > Xcode(4, 3) => true,
            _ => false,
        }
    });

    cfg.fn_cname(|rust, _link_name| match rust {
        v => v.to_string(),
    });

    cfg.skip_signededness(|c| {
        // signededness test does not make sense for these:
        match c {
            // struct types:
            "mach_timespec_t" |
            "ipc_port_t"  |
            "vm_statistics_data_t" |

            // array types:
            "vm_region_info_data_t" |
            "mach_vm_read_entry_t" |

            // pointer types:
            "clock_attr_t" |
            "memory_object_fault_info_t" |
            "exception_port_arrary_t" |
            "exception_handler_array_t" |
            "thread_state_t" |
            "thread_array_t" |
            "thread_port_array_t" |
            "thread_act_array_t" |
            "thread_act_port_array_t" |
            "ledger_array_t" |
            "ledger_port_array_t" |
            "task_info_t" |
            "task_array_t" |
            "task_port_array_t" |
            "processor_array_t" |
            "processor_port_array_t" |
            "processor_set_array_t" |
            "processor_set_name_array_t" |
            "processor_set_name_port_array_t" |
            "vm_region_t" |
            "vm_region_info_t" |
            "vm_region_info_64_t" |
            "vm_region_recurse_info_t" |
            "vm_region_recurse_info_64_t" |
            "vm_region_basic_info_64_t" |
            "vm_region_basic_info_t" |
            "vm_region_basic_info_data_t" |
            "vm_region_basic_info_data_64_t" |
            "vm_region_extended_info_t" |
            "vm_region_extended_info_data_t" |
            "vm_region_top_info_t" |
            "vm_region_top_info_data_t" |
            "vm_region_submap_info_t" |
            "vm_region_submap_info_64_t" |
            "vm_region_submap_info_data_t" |
            "vm_region_submap_info_data_64_t" |
            "vm_region_submap_short_info_64_t" |
            "vm_region_submap_short_info_data_64_t" |
            "vm_page_info_t" |
            "vm_page_info_basic_t" |
            "vm_page_info_basic_data_t" |
            "vm_statistics_t"
                => true,
            _ => false,
        }
    });

    // struct foo in Rust should translate to foo in C:
    cfg.type_name(|ty, is_struct| match ty {
        "mach_timespec" => format!("{}_t", ty),
        "vm_region_basic_info_64"
        | "vm_region_basic_info"
        | "vm_region_extended_info"
        | "vm_region_top_info"
        | "vm_region_submap_info"
        | "vm_region_submap_info_64"
        | "vm_region_submap_short_info_64"
        | "vm_page_info_basic"
        | "vm_statistics"
        | "mach_vm_read_entry" => format!("struct {}", ty),
        _ if is_struct => format!("{}", ty),
        _ => ty.to_string(),
    });

    // Include the directory where the header files are defined
    cfg.include("/usr/include");

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}
