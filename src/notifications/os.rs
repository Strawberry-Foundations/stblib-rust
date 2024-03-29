#[derive(PartialEq, Eq, Debug)]
pub enum OS {
    Undefined,
    Unknown,
    Linux,
    LinuxLibNotify,
    Windows,
    WindowsLegacy,
    MacOS,
}