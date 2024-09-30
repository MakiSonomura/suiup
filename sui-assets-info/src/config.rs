pub const fn local_environment() -> &'static str {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    let os_suffix = "macos-arm64";
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    let os_suffix = "macos-x86_64";
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let os_suffix = "ubuntu-x86_64";
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    let os_suffix = "ubuntu-aarch64";
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    let os_suffix = "windows-x86_64";
    os_suffix
}
