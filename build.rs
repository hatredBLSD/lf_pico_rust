fn main() {
    println!("cargo:rustc-link-search=native=/home/hatred/luckfox-pico/tools/linux/toolchain/arm-rockchip830-linux-uclibcgnueabihf/arm-rockchip830-linux-uclibcgnueabihf/sysroot/usr/lib");
    println!("cargo:rustc-link-search=native=/home/hatred/luckfox-pico/tools/linux/toolchain/arm-rockchip830-linux-uclibcgnueabihf/arm-rockchip830-linux-uclibcgnueabihf/sysroot/lib");
    println!("cargo:rustc-link-lib=c");
    println!("cargo:rustc-link-lib=gcc_s");
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=rockit");
    println!("cargo:rustc-link-lib=rockchip_mpp");
    println!("cargo:rustc-link-lib=rga");
}
