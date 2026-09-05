fn main() {
    // println!(r"cargo:rustc-link-search=native=C:\vcpkg\installed\x64-windows-static-md\lib");

    // println!("cargo:rustc-link-lib=static=zlib");
    // println!("cargo:rustc-link-lib=static=nghttp2");
    #[cfg(windows)]
    {
        vcpkg::find_package("nghttp2").unwrap();
        vcpkg::find_package("zlib").unwrap();
        // vcpkg::Config::new()
        //     .lib_name("zs")
        //     .find_package("zlib")
        //     .unwrap();
    }
    
}


// [target.x86_64-pc-windows-msvc]
// rustflags = [
//     "-L", "C:/vcpkg/installed/x64-windows-static-md/lib",
//     "-l", "static=zs",
//     "-l", "static=nghttp2",
// ]