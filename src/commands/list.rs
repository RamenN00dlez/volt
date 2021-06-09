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

//! Compress node_modules into node_modules.pack.

// Std Imports
use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;
use colored::Colorize;

// Crate Level Imports
use crate::utils::App;
use crate::VERSION;

use walkdir::WalkDir;

// Super Imports
use super::Command;

pub struct List {}

#[async_trait]
impl Command for List {
    /// Display a help menu for the `volt list` command.
    fn help() -> String {
        format!(
            r#"volt {}
    
List dependency tree from node_modules.
Usage: {} {} {} {}
Options: 
    
  {} {} Output verbose messages on internal operations."#,
            VERSION.bright_green().bold(),
            "volt".bright_green().bold(),
            "clone".bright_purple(),
            "[repository]".white(),
            "[flags]".white(),
            "--verbose".blue(),
            "(-v)".yellow(),
        )
    }

    /// Execute the `volt list` command
    ///
    /// List node_modules into node_modules.pack.
    /// ## Arguments
    /// * `app` - Instance of the command (`Arc<App>`)
    /// ## Examples
    /// ```
    /// // List node_modules into node_modules.pack
    /// // .exec() is an async call so you need to await it
    /// Add.exec(app).await;
    /// ```
    /// ## Returns
    /// * `Result<()>`
    async fn exec(_app: Arc<App>) -> Result<()> {
        let dirs = WalkDir::new("node_modules");

        let dependency_paths: Vec<_> = dirs
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_dir() || entry.file_type().is_symlink())
            .filter_map(|entry| Some(entry))
            .collect();

        let mut dependencies: Vec<String> = vec![];

        for dep in dependency_paths {
            let dep_path = dep.path().to_str().unwrap();
            let dep_path_split: Vec<&str> = dep_path.split("\\").collect();
            let dep_name: &str = dep_path_split[dep_path_split.len() - 1];
            if dep_name != "node_modules"
                && dep_name != "scripts"
                && !dep_name.starts_with("node_modules")
            {
                dependencies.push(dep_name.to_string());
                println!("{} {}", "-".bright_yellow(), dep_name.bright_blue().bold());
                let dirs = WalkDir::new(format!("node_modules/{}/node_modules", dep_name))
                    .follow_links(true);
                let dependency_paths: Vec<_> = dirs
                    .into_iter()
                    .filter_map(Result::ok)
                    .filter(|entry| entry.file_type().is_dir() || entry.file_type().is_symlink())
                    .filter_map(|entry| Some(entry))
                    .collect();

                for dep in dependency_paths {
                    let dep_path = dep.path().to_str().unwrap();
                    // println!("path: {}", dep_path);
                    let dep_path_split: Vec<&str> = dep_path.split("\\").collect();
                    let dep_name: &str = dep_path_split[dep_path_split.len() - 1];
                    if dep_name != "node_modules"
                        && dep_name != "scripts"
                        && !dep_path.contains("lib")
                        && !dep_path.contains("src")
                        && !dep_path.contains("dist")
                        && !dep_path.contains("test")
                        && !dep_name.starts_with("node_modules")
                    {
                        dependencies.push(dep_name.to_string());
                        for _ in 0..dep_path_split.len() {
                            print!("  ");
                        }
                        println!("{} {}", "-".bright_purple(), dep_name);
                    }
                }
            }
        }

        Ok(())
    }
}