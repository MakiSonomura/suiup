use crate::{config, Result, UpdaterError};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

macro_rules! parseerror {
    ($desc:expr) => {
        |_| UpdaterError::ParseAssetDescError { desc: $desc }
    };
}

macro_rules! literal_enum {
    (
        $ty: ident { $( $name:ident => $value:expr),* }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $ty {
            $($name, )*
            Unknown,
        }
        impl From<&str> for $ty {
            fn from(v: &str) -> $ty {
                match v {
                    $( $value => $ty::$name, )*
                    _ => $ty::Unknown
                }
            }
        }

        impl FromStr for $ty {
            type Err = UpdaterError;
            fn from_str(s: &str) -> Result<$ty> {
                let r = $ty::from(s);
                match r {
                    $ty::Unknown => Err(parseerror!("network")(())),
                    _ => Ok(r)
                }
            }
        }

        impl From<$ty> for &'static str {
            fn from(b: $ty) -> &'static str {
                match b {
                    $( $ty::$name => $value, )*
                    $ty::Unknown => "unknown"
                }
            }
        }

        impl Display for $ty {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $($ty::$name => write!(f, "{}", $value), )*
                    _ => write!(f, "unknown")
                }
            }
        }


    };
}

literal_enum! {
    Network {
        Mainnet => "mainnet",
        Testnet => "testnet",
        Devnet => "devnet"
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Version {
    const PREFIX: &'static str = "v";

    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromStr for Version {
    type Err = UpdaterError;

    fn from_str(s: &str) -> Result<Self> {
        let s: Vec<_> = s.split('.').collect();
        if s.len() != 3 {
            Err(parseerror!("version")(()))
        } else {
            let major: u8 = if s[0].starts_with(Self::PREFIX) {
                &s[0][Self::PREFIX.len()..]
            } else {
                s[0]
            }
            .parse()
            .map_err(parseerror!("major"))?;
            let minor: u8 = s[1].parse().map_err(parseerror!("minor"))?;
            let patch: u8 = s[2].parse().map_err(parseerror!("patch"))?;
            Ok(Self {
                major,
                minor,
                patch,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct SuiAssetDesc {
    network: Network,
    version: Version,
}

impl SuiAssetDesc {
    const DESC_PREFIX: &'static str = "sui";
    const DESC_SUFFIX: &'static str = config::local_environment();
    const FILE_EXTNSN: &'static str = ".tgz";

    pub fn new(network: Network, version: Version) -> Self {
        Self { network, version }
    }

    // pattern like "mainnet-v1.30.1"
    pub fn from_twins(s: &str) -> Result<Self> {
        let twins: Vec<_> = s.split('-').collect();
        if twins.len() != 2 {
            Err(parseerror!("twins")(()))
        } else {
            Ok(Self {
                network: twins[0].parse()?,
                version: twins[1].parse()?,
            })
        }
    }

    // pattern like "sui-mainnet-v1.30.1-ubuntu-x86_64.tgz"
    pub fn from_quints(s: &str) -> Result<Self> {
        if let Some(s) = s.strip_prefix(Self::DESC_PREFIX) {
            if let Some(s) = s.strip_suffix(Self::FILE_EXTNSN) {
                if let Some(s) = s.strip_suffix(Self::DESC_SUFFIX) {
                    if s.len() >= 2 {
                        let s = &s[1..s.len() - 1];
                        if let Ok(res) = Self::from_twins(s) {
                            return Ok(res);
                        }
                    }
                }
            }
        }
        Err(parseerror!("quints")(()))
    }

    pub fn network(&self) -> &Network {
        &self.network
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn desc(&self) -> String {
        format!("{}-{}", self.network, self.version)
    }
}
