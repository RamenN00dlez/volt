/*
    Copyright 2021 Volt Contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

// Modules
mod classes;
mod commands;
mod model;
mod prompt;
mod utils;

// Std Imports
use std::sync::Arc;

// Library Imports
use anyhow::Result;
use colored::Colorize;

// Crate Level Imports
use commands::AppCommand;
use utils::{get_arguments, initialize};

// Constants
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("{} {}", "error".red().bold(), err);
        let err_chain = err.chain().skip(1);
        if err_chain.clone().next().is_some() {
            eprintln!("{}", "\nCaused by:".italic().truecolor(190, 190, 190));
        }
        err_chain.for_each(|cause| eprintln!(" - {}", cause.to_string().truecolor(190, 190, 190)));
        #[cfg(not(debug_assertions))]
        eprintln!(
            "\nIf the problem persists, please submit an issue on the Github repository.\n{}",
            "https://github.com/voltpkg/volt/issues/new".underline()
        );
        std::process::exit(1);
    }
}

async fn try_main() -> Result<()> {
    let (app, args) = initialize();

    let (flags, args) = get_arguments(&args);

    let app_cmd = AppCommand::current().unwrap_or(AppCommand::Help); // Default command is help
    let cmd = app_cmd.command();

    if flags.iter().any(|flag| flag == "--help") {
        println!("{}", cmd.help());
        return Ok(());
    }

    cmd.exec(Arc::new(app), args, flags).await
}
