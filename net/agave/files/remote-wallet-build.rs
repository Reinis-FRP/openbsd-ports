fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("openbsd") {
        println!("cargo:rerun-if-changed=ledger_uhid.c");
        cc::Build::new().file("ledger_uhid.c").compile("agave_ledger_uhid");
        println!("cargo:rustc-link-lib=usbhid");
    }
}
