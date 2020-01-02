//! All subcommands in leetcode-cli
//! 
//! ```sh
//! SUBCOMMANDS:
//!     help    Prints this message or the help of the given subcommand(s)
//!     list    List problems
//! ```
use clap::{App, ArgMatches};

/// Abstract commands' traits.
pub trait Command {
    /// Usage of the spefic command
    fn usage<'a, 'c>() -> App<'a, 'c>;

    /// The handler will deal [args, options,...] from the command-line
    fn handler(m: &ArgMatches);
}

mod list;
mod pick;
mod stat;
mod cache;
pub use list::ListCommand;
pub use pick::PickCommand;
pub use stat::StatCommand;
pub use cache::CacheCommand;