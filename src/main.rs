use std::process::Command;

use clap::{App, Arg};
use colored::Colorize;

use exit::ExitOnError;
use model::{PaperAllVersionBuilds, PaperAllVersions, PaperVersionBuild};

mod exit;
mod model;

fn main() {
    let arguments = App::new("PaperBuilder")
            .version("0.1.0")
            .author("Alexander <protonull@protonmail.com>")
            .arg(Arg::with_name("version")
                    .long("rev")
                    .takes_value(true)
                    .help("The minecraft version, e.g: 1.16.5"))
            .get_matches();

    let version: String;
    let version_is_user_defined: bool;

    match arguments.value_of("version") {
        Some(found) => {
            version_is_user_defined = true;
            version = String::from(found);
        }
        None => {
            version_is_user_defined = false;
            version = reqwest::blocking::get("https://papermc.io/api/v2/projects/paper/")
                    .and_then(|response| response.json::<PaperAllVersions>())
                    .unwrap_or_exit("Could not get a response from PaperMC")
                    .versions.last()
                    .unwrap_or_exit("Could not find any versions")
                    .clone();
        }
    }
    println!("Version: {} ({})", version,
             (if version_is_user_defined { "arguments" } else { "default" }));

    let build: u32 = reqwest::blocking::get(
        format!("https://papermc.io/api/v2/projects/paper/versions/{}", version))
            .and_then(|response| response.json::<PaperAllVersionBuilds>())
            .unwrap_or_exit("Could not get a response from PaperMC, are you sure that version exists?")
            .builds.last()
            .unwrap_or_exit("Could not find any builds for that version")
            .clone();
    println!("Build: {}", build);

    let commit: String = reqwest::blocking::get(
        format!("https://papermc.io/api/v2/projects/paper/versions/{}/builds/{}", version, build))
            .and_then(|response| response.json::<PaperVersionBuild>())
            .unwrap_or_exit("Could not get a response from PaperMC")
            .changes.first() // For some reason newest is first here
            .unwrap_or_exit("Could not find any builds for that version")
            .commit.clone();
    println!("Commit: {}", commit);

    println!("{}", "Setting up repo".blue());
    let paper_dir = String::from("paper");
    let paper_git = String::from("https://github.com/PaperMC/Paper.git");

    let current_working_dir = std::env::current_dir().expect("");
    let paper_git_dir = current_working_dir.join(&paper_dir);

    if paper_git_dir.is_dir() {
        std::fs::remove_dir_all(&paper_git_dir)
                .unwrap_or_exit("Could not clean up Paper");
    }

    Command::new("git")
            .args(&["init", paper_dir.as_str()])
            .current_dir(&current_working_dir)
            .spawn()
            .and_then(|mut child| child.wait())
            .unwrap_or_exit("Could not set up git repository");

    Command::new("git")
            .args(&["config", "core.autocrlf", "false"])
            .current_dir(&paper_git_dir)
            .spawn()
            .and_then(|mut child| child.wait())
            .unwrap_or_exit("Could not set autocrlf to false");

    Command::new("git")
            .args(&["remote", "add", "origin", paper_git.as_str()])
            .current_dir(&paper_git_dir)
            .spawn()
            .and_then(|mut child| child.wait())
            .unwrap_or_exit("Could not add remote origin");

    Command::new("git")
            .args(&["fetch", "origin", "master", "--depth=1"])
            .current_dir(&paper_git_dir)
            .spawn()
            .and_then(|mut child| child.wait())
            .unwrap_or_exit("Could not fetch from remote origin");

    Command::new("git")
            .args(&["reset", "--hard", commit.as_str()])
            .current_dir(&paper_git_dir)
            .spawn()
            .and_then(|mut child| child.wait())
            .unwrap_or_exit("Could not reset to build commit");

    println!("{}", "Building repo".blue());

    Command::new("./paper")
            .args(&["jar"])
            .current_dir(&paper_git_dir)
            .spawn()
            .unwrap_or_exit("Could not build Paper");

    println!("{}", "Done :)".blue());
}
