// Copyright 2024-Current HyperPM & Contributors
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use clap::{Parser, Subcommand};
use colored::*;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use futures::future::join_all;
use std::process::Command;
use tokio::task;

/// The command-line interface for HyperPM, a fast, Rust-based npm package manager.
#[derive(Parser)]
#[clap(name = "hyperpm", about = "A fast, Rust-based npm package manager")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

/// Available commands for the CLI.
#[derive(Subcommand)]
enum Commands {
    New { project_name: String },      // Creates a new project with the specified name.
    Install { package_names: Vec<String> }, // Installs the specified npm packages.
}

/// The entry point for the application, handles CLI input and invokes relevant commands.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse the command line arguments into the Cli struct.
    let cli = Cli::parse();

    // Match the user command and call the corresponding function.
    match cli.command {
        Commands::New { project_name } => create_new_project(&project_name), // Create a new project
        Commands::Install { package_names } => install_packages(&package_names).await, // Install specified packages
    }
}

/// Creates a new project directory and initializes it using npm.
fn create_new_project(project_name: &str) -> anyhow::Result<()> {
    println!("{}", "Creating new project...".green().bold());
    
    // Create a directory for the new project.
    std::fs::create_dir(project_name)?;
    
    // Initialize a new npm project in the newly created directory.
    Command::new("npm")
        .args(&["init", "-y"])
        .current_dir(project_name)
        .output()?;
    
    println!("{} {} {}", "Project".green().bold(), project_name.green().bold(), "created successfully!".green().bold());
    Ok(())
}

/// Asynchronously installs a list of npm packages and shows progress using a spinner.
async fn install_packages(package_names: &[String]) -> anyhow::Result<()> {
    println!("{}", "Installing packages...".green().bold());

    let m = MultiProgress::new(); // Initializes a multi-progress manager for multiple spinners.
    let mut handles = vec![]; // Vector to store handle of each spawn task.

    for package_name in package_names {
        let pb = m.add(ProgressBar::new_spinner()); // Create a new spinner for the package installation.
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ") // Custom spinner characters
                .template("{spinner:.green} {msg}")
                .expect("Failed to set progress bar style") // Handle error in setting spinner style
        );

        let package_name = package_name.clone(); // Clone the package name for the async closure.
        
        // Spawn an asynchronous task to install the package.
        let handle = task::spawn(async move {
            install_package(&package_name, pb).await // Call the install_package function
        });
        handles.push(handle); // Store the task handle.
    }

    // Wait for all tasks to complete before proceeding.
    join_all(handles).await;
    
    Ok(())
}

/// Installs a specific npm package and updates the associated progress bar.
async fn install_package(package_name: &str, pb: ProgressBar) -> anyhow::Result<()> {
    pb.set_message(format!("Fetching package info for {}", package_name)); // Update spinner message

    let client = reqwest::Client::new();
    // Send a GET request to the npm registry for package info.
    let resp = client
        .get(&format!("https://registry.npmjs.org/{}", package_name))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Extract the latest version from the response, defaulting to "latest" if not found.
    let version = resp["dist-tags"]["latest"].as_str().unwrap_or("latest");
    pb.set_message(format!("Installing {} (v{})", package_name, version)); // Update spinner message for installation

    // Execute npm command to install the specified package version.
    let output = Command::new("npm")
        .args(&["install", &format!("{}@{}", package_name, version)])
        .output()?;

    if output.status.success() {
        // If installation succeeded, update the spinner with a success message.
        pb.finish_with_message(format!("{} {} {}", package_name.cyan(), "installed successfully!".green().bold(), format!("(v{})", version).dimmed()));
    } else {
        // If installation failed, update the spinner with an error message and print the stderr.
        pb.finish_with_message(format!("{} {}", "Failed to install".red().bold(), package_name));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr)); // Print out error details
    }

    Ok(())
}