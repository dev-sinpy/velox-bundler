extern crate ar;
extern crate cab;
extern crate chrono;
extern crate dirs;
#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate icns;
extern crate image;
extern crate libflate;
extern crate md5;
extern crate msi;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate strsim;
extern crate tar;
extern crate target_build_utils;
extern crate term;
extern crate toml;
extern crate uuid;
extern crate walkdir;

mod bundle;

use bundle::{bundle_project, BuildArtifact, PackageType, Settings};
// use clap::{App, AppSettings, Arg, SubCommand};
use std::env;
use std::process;

error_chain! {
    foreign_links {
        Glob(::glob::GlobError);
        GlobPattern(::glob::PatternError);
        Io(::std::io::Error);
        Image(::image::ImageError);
        Target(::target_build_utils::Error);
        Term(::term::Error);
        Toml(::toml::de::Error);
        Walkdir(::walkdir::Error);
    }
    errors { }
}

/// Runs `cargo build` to make sure the binary file is up-to-date.
fn build_project_if_unbuilt(settings: &Settings) -> crate::Result<()> {
    let mut args = vec!["build".to_string()];
    if let Some(triple) = settings.target_triple() {
        args.push(format!("--target={}", triple));
    }
    if let Some(features) = settings.features() {
        args.push(format!("--features={}", features));
    }
    match *settings.build_artifact() {
        BuildArtifact::Main => {}
        BuildArtifact::Bin(ref name) => {
            args.push(format!("--bin={}", name));
        }
        BuildArtifact::Example(ref name) => {
            args.push(format!("--example={}", name));
        }
    }
    if settings.is_release_build() {
        args.push("--release".to_string());
    }
    if settings.all_features() {
        args.push("--all-features".to_string());
    }
    if settings.no_default_features() {
        args.push("--no-default-features".to_string());
    }
    let status = process::Command::new("cargo").args(args).status()?;
    if !status.success() {
        bail!(
            "Result of `cargo build` operation was unsuccessful: {}",
            status
        );
    }
    Ok(())
}

pub fn bundle_binary() -> crate::Result<()> {
    let output_paths = env::current_dir()
        .map_err(From::from)
        .and_then(|d| Settings::new(d))
        .and_then(|s| {
            build_project_if_unbuilt(&s)?;
            Ok(s)
        })
        .and_then(bundle_project)?;
    bundle::print_finished(&output_paths)?;
    Ok(())
}

// fn bundle_binary() {
//     if let Err(error) = run() {
//         bundle::print_error(&error).unwrap();
//         std::process::exit(1);
//     }
// }
