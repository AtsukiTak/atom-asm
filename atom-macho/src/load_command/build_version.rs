use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// The build_version_command contains the min OS version on which this
/// binary was built to run for its platform.  The list of known platforms and
/// tool values following it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildVersionCommand {
    pub cmd: u32,
    /// BuildVersion::SIZE + ntools * BuildToolVersion::SIZE
    pub cmdsize: u32,
    pub platform: Platform,
    /// X.Y.Z is encoded in nibbles xxxx.yy.zz
    pub minos: Version,
    /// X.Y.Z is encoded in nibbles xxxx.yy.zz
    pub sdk: Version,
    pub ntools: u32,
}

impl BuildVersionCommand {
    pub const TYPE: u32 = 0x32;

    pub const SIZE: u32 = 0x18; // 24
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Platform {
    MacOS = 1,
    IOS = 2,
    TvOS = 3,
    WatchOS = 4,
    BridgeOS = 5,
    MacCatalyst = 6,
    IOSSimulator = 7,
    TvOSSimulator = 8,
    WatchOSSimulator = 9,
    Driverkit = 10,
}

impl Platform {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap_or_else(|| panic!("Invalid platform number {}", n))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    pub major: u16,
    pub minor: u8,
    pub release: u8,
}

impl Version {
    pub fn from_u32(n: u32) -> Self {
        let major = (n >> 16) as u16;
        let minor = ((n >> 8) & 0xFF) as u8;
        let release = (n & 0xFF) as u8;
        Version {
            major,
            minor,
            release,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildToolVersion {
    pub tool: Tool,
    pub version: u32,
}

impl BuildToolVersion {
    pub const SIZE: u32 = 0x8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Tool {
    Clang = 1,
    Swift = 2,
    LD = 3,
}

impl Tool {
    pub fn from_u32(n: u32) -> Self {
        FromPrimitive::from_u32(n).unwrap_or_else(|| panic!("Unsupported tool number {}", n))
    }
}
