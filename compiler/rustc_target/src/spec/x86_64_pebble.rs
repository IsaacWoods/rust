use crate::spec::{LinkerFlavor, LldFlavor, Target, TargetOptions};

/*
 * For Pebble userspace images, we need to force the linker to keep the `.caps` section, which it thinks is unused.
 * As far as I know, the only way to reliably do this is to use a custom linker script. Since we have to anyway, we
 * use it to lay out the segments nicely.
 */
const LINKER_SCRIPT: &str = include_str!("./x86_64_pebble.ld");

pub fn target() -> Target {
    let options = TargetOptions {
        cpu: "x86-64".to_string(),
        max_atomic_width: Some(64),
        /*
         * NOTE: this should be temporary. When the Pebble kernel can handle userspace tasks that use these
         * features, LLVM can be allowed to generate instructions that use them.
         */
        features: "-mmx,-sse,-sse2,-sse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float"
            .to_string(),

        os_family: None,
        os: "pebble".to_string(),
        executables: true,

        has_elf_tls: true,

        linker_flavor: LinkerFlavor::Lld(LldFlavor::Ld),
        linker: Some("rust-lld".to_owned()),
        lld_flavor: LldFlavor::Ld,
        linker_is_gnu: true,
        link_script: Some(LINKER_SCRIPT.to_string()),

        ..Default::default()
    };

    Target {
        llvm_target: "x86_64-unknown-none".to_string(),
        pointer_width: 64,
        data_layout: "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
            .to_string(),
        arch: "x86_64".to_string(),
        options,
    }
}
