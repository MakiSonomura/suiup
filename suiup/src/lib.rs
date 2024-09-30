pub(crate) mod error;
pub(crate) mod process;
pub(crate) mod subcommand;
pub(crate) use error::{Error, Result};
pub use process::Process;
pub(crate) mod common;

use anstyle::{AnsiColor, Color, Style};
use clap::{builder::Styles, Args, Parser};
use subcommand::{config, default, list, update};
use sui_assets_info::prelude::{BackEnd, GithubBackend, Network};

const HEADER_COLOR: Option<Color> = Some(Color::Ansi(AnsiColor::BrightBlue));
const LITERAL_COLOR: Option<Color> = Some(Color::Ansi(AnsiColor::Green));
const STYLES: Styles = Styles::plain()
    .header(Style::new().bold().fg_color(HEADER_COLOR))
    .usage(Style::new().bold().fg_color(HEADER_COLOR))
    .literal(Style::new().bold().fg_color(LITERAL_COLOR));

#[derive(Debug, Parser)]
#[clap(name = "suiup",  styles = STYLES)]
pub struct CLI {
    #[clap(subcommand)]
    pub(crate) subcommand: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(about = "List all installed sui toolkits in the default root")]
    List,
    #[command(about = "Print the default config of suiup")]
    Config,
    #[command(about = "Set default toolkit")]
    Default {
        #[command(flatten)]
        opt: DefaultOpt,
    },

    #[command(about = "Install sui toolkit")]
    Install {
        #[command(flatten)]
        opt: UpdateOpt,
    },
    #[command(about = "Install the latest toolkit")]
    Latest {
        #[command(flatten)]
        opt: LatestOpt,
    },
}

#[derive(Debug, Args)]
struct DefaultOpt {
    #[arg(required = true)]
    desc: String,
}
#[derive(Debug, Args)]
struct UpdateOpt {
    #[arg(
        required = true,
        num_args = 1..,
    )]
    toolkit_desc: String,
    #[clap(
        short,
        long,
        default_value_t = BackEnd::GithubBackend(GithubBackend::new())
    )]
    backend: BackEnd,
}

#[derive(Debug, Args)]
struct LatestOpt {
    #[clap(
        long,
        short,
        default_value_t = Network::Mainnet
    )]
    network: Network,
    #[clap(
        short,
        long,
        default_value_t = BackEnd::GithubBackend(GithubBackend::new())
    )]
    backend: BackEnd,
}

impl Subcommand {
    pub(crate) async fn run(self, process: &Process<'_>) -> Result<()> {
        match self {
            Self::List => list::run(process),
            Self::Config => config::run(process),
            Self::Default { opt } => default::run(process, opt).await,
            Self::Install { opt } => update::run(process, opt).await,
            Self::Latest { opt } => update::run_latest(process, opt).await,
        }
    }
}

impl CLI {
    pub async fn run(self, process: &Process<'_>) -> Result<()> {
        self.subcommand.run(process).await
    }
}
