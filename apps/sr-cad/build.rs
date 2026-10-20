//! Windows only: embed the app icon and version info into `sr-cad.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/sr-cad.ico");
    println!("cargo:rerun-if-env-changed=SR_CAD_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=SR_CAD_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/sr-cad.ico").exists() {
        res.set_icon("../../assets/app-icon/sr-cad.ico");
    }
    res.set("ProductName", "SR-Cad")
        .set("FileDescription", "SR-Cad computer-aided design")
        .set("CompanyName", "SR-Cad")
        .set("LegalCopyright", "Copyright (c) 2026 ArtCraft Team and the CADCraft contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "sr-cad.exe")
        .set("InternalName", "sr-cad");
    if let Err(e) = res.compile() {
        if std::env::var_os("SR_CAD_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=sr-cad.exe built without icon/version resources: {e}");
    }
}
