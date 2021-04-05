use crate::Buffer;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive as _;

/// The build_version_command contains the min OS version on which this
/// binary was built to run for its platform.  The list of known platforms and
/// tool values following it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildVersion {
    /// sizeof(BuildVersion) + ntools * sizeof(BuildToolVersion)
    pub cmd_size: u32,
    pub platform: Platform,
    pub minos: Version,
    pub sdk: Version,
    pub ntools: u32,

    pub tools: Vec<BuildToolVersion>,
}

impl BuildVersion {
    pub fn parse(buf: &mut Buffer) -> Self {
        let cmd_size = buf.read_u32();
        let platform = Platform::parse(buf);
        let minos = Version::parse(buf);
        let sdk = Version::parse(buf);
        let ntools = buf.read_u32();

        let mut tools = Vec::with_capacity(ntools as usize);
        for _ in 0..ntools {
            tools.push(BuildToolVersion::parse(buf));
        }

        // 必ず8バイト境界にそろうためアライメントは不要

        BuildVersion {
            cmd_size,
            platform,
            minos,
            sdk,
            ntools,
            tools,
        }
    }
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
    fn parse(buf: &mut Buffer) -> Self {
        let platform_n = buf.read_u32();
        Platform::from_u32(platform_n)
            .unwrap_or_else(|| panic!("Invalid platform number {}", platform_n))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    major: u16,
    minor: u8,
    release: u8,
}

impl Version {
    fn parse(buf: &mut Buffer) -> Self {
        let n = buf.read_u32();
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
    tool: Tool,
    version: u32,
}

impl BuildToolVersion {
    fn parse(buf: &mut Buffer) -> Self {
        let tool = Tool::parse(buf);
        let version = buf.read_u32();

        BuildToolVersion { tool, version }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive)]
pub enum Tool {
    Clang = 1,
    Swift = 2,
    LD = 3,
}

impl Tool {
    fn parse(buf: &mut Buffer) -> Self {
        let tool_n = buf.read_u32();
        Tool::from_u32(tool_n).unwrap_or_else(|| panic!("Unsupported tool number {}", tool_n))
    }
}
