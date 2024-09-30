pub(crate) mod config;

use crate::Result;
use home::env::{home_dir_with_env, Env, OS_ENV};
use std::borrow::Cow;
use std::ffi::OsString;
// use std::io::IsTerminal;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::{env, io};

#[derive(Clone, Debug)]
pub struct OsProcess<'p> {
    // pub(self) stderr_is_a_tty: bool,
    // pub(self) stdout_is_a_tty: bool,
    _phan: PhantomData<&'p ()>,
}

impl<'p> OsProcess<'p> {
    pub fn new() -> Self {
        OsProcess {
            // stderr_is_a_tty: io::stderr().is_terminal(),
            // stdout_is_a_tty: io::stdout().is_terminal(),
            _phan: PhantomData,
        }
    }
}

impl<'p> Default for OsProcess<'p> {
    fn default() -> Self {
        OsProcess::new()
    }
}

#[derive(Debug, Clone)]
pub enum Process<'p> {
    OsProcess(OsProcess<'p>),
}

impl<'p> Process<'p> {
    pub fn os() -> Self {
        Self::OsProcess(OsProcess::new())
    }

    pub fn var(&self, key: &str) -> Result<Cow<'p, str>> {
        match self {
            Process::OsProcess(_) => Ok(Cow::Owned(env::var(key)?)),
        }
    }

    fn suiup_home_with_cwd_env(&self, cwd: &Path) -> Result<PathBuf> {
        match self.var_os("SUIUP_HOME").filter(|h| !h.is_empty()) {
            Some(home) => {
                let home = PathBuf::from(home);
                if home.is_absolute() {
                    Ok(home)
                } else {
                    Ok(cwd.join(&home))
                }
            }
            _ => home_dir_with_env(self)
                .map(|d| d.join(".suiup"))
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::Other, "could not find suiup home dir").into()
                }),
        }
    }

    fn suiup_home_with_env(&self) -> Result<PathBuf> {
        let cwd = self.current_dir()?;
        self.suiup_home_with_cwd_env(&cwd)
    }

    pub fn name(&self) -> Option<Cow<'p, str>> {
        let arg0 = match self.var("SUIUP_FORCE_ARG0") {
            Ok(v) => v,
            _ => self.args().next()?,
        };

        Some(arg0)
    }

    pub fn current_dir(&self) -> Result<PathBuf> {
        match self {
            Self::OsProcess(_) => Ok(env::current_dir()?),
        }
    }

    // pub(crate) fn home_dir(&self) -> Option<PathBuf> {
    //     home::env::home_dir_with_env(self)
    // }

    pub(crate) fn suiup_home(&self) -> Result<PathBuf> {
        self.suiup_home_with_env()
    }

    pub(crate) fn args(&self) -> Box<dyn Iterator<Item = Cow<'p, str>>> {
        let args = env::args();
        match self {
            Process::OsProcess(_) => Box::new(args.into_iter().map(Cow::Owned)),
        }
    }

    pub(crate) fn var_os(&self, key: &str) -> Option<OsString> {
        match self {
            Process::OsProcess(_) => env::var_os(key),
        }
    }
}

impl<'p> Env for Process<'p> {
    fn home_dir(&self) -> Option<PathBuf> {
        match self {
            Process::OsProcess(_) => OS_ENV.home_dir(),
        }
    }

    fn current_dir(&self) -> io::Result<PathBuf> {
        match self {
            Process::OsProcess(_) => OS_ENV.current_dir(),
        }
    }

    fn var_os(&self, key: &str) -> Option<OsString> {
        self.var_os(key)
    }
}
