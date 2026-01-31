//! Platform-related code
use daedalus::minecraft::{Os, OsRule};
use regex::Regex;

// OS detection
pub trait OsExt {
    /// Get the OS of the current system
    fn native() -> Self;

    /// Gets the OS + Arch of the current system
    fn native_arch(java_arch: &str) -> Self;

    /// Gets the OS from an OS + Arch
    fn get_os(&self) -> Self;
}

impl OsExt for Os {
    fn native() -> Self {
        Self::Linux
    }

    fn native_arch(java_arch: &str) -> Self {
        if java_arch == "aarch64" {
            Os::LinuxArm64
        } else if java_arch == "arm" {
            Os::LinuxArm32
        } else {
            Os::Linux
        }
    }

    fn get_os(&self) -> Self {
        match self {
            Os::LinuxArm32 => Os::Linux,
            Os::LinuxArm64 => Os::Linux,
            _ => self.clone(),
        }
    }
}

// Bit width
#[cfg(target_pointer_width = "64")]
pub const ARCH_WIDTH: &str = "64";

#[cfg(target_pointer_width = "32")]
pub const ARCH_WIDTH: &str = "32";

// Platform rule handling
pub fn os_rule(
    rule: &OsRule,
    java_arch: &str,
    // Minecraft updated over 1.18.2 (supports MacOS Natively)
    minecraft_updated: bool,
) -> bool {
    let mut rule_match = true;

    if let Some(ref arch) = rule.arch {
        rule_match &= !matches!(arch.as_str(), "x86" | "arm");
    }

    if let Some(name) = &rule.name {
        if minecraft_updated
            && (name != &Os::LinuxArm64 || name != &Os::LinuxArm32)
        {
            rule_match &= Os::native() == name.get_os()
                || &Os::native_arch(java_arch) == name;
        } else {
            rule_match &= &Os::native_arch(java_arch) == name;
        }
    }

    if let Some(version) = &rule.version {
        if let Ok(regex) = Regex::new(version.as_str()) {
            rule_match &=
                regex.is_match(&sys_info::os_release().unwrap_or_default());
        }
    }

    rule_match
}

pub fn classpath_separator(_java_arch: &str) -> &'static str {
    ":"
}
