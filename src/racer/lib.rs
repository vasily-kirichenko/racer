#[macro_use] extern crate log;

extern crate syntex_syntax;
extern crate syntex_errors;
extern crate toml;
extern crate env_logger;
extern crate typed_arena;

#[macro_use]
pub mod testutils;
pub mod core;
pub mod scopes;
pub mod ast;
pub mod typeinf;
pub mod nameres;
pub mod codeiter;
pub mod codecleaner;
pub mod util;
pub mod matchers;
pub mod snippets;
pub mod cargo;
