#![allow(clippy::unnecessary_wraps)]

use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main(
    "app",
    pre_build = pre_build,
    serve = serve,
    build_args = BuildCommand,
    serve_args = ServeCommand,
)]
#[derive(StructOpt, Debug)]
enum Cli {}

#[derive(StructOpt, Debug)]
pub struct BuildCommand {
    #[structopt(flatten)]
    pub base: DefaultBuildArgs,

    #[structopt(long)]
    pub features: Option<String>,
}

impl BuildArgs for BuildCommand {
    fn build_path(&self) -> &PathBuf {
        self.base.build_path()
    }

    fn profiling(&self) -> bool {
        self.base.profiling()
    }
}

#[derive(StructOpt, Debug)]
pub struct ServeCommand {
    #[structopt(long)]
    pub log: bool,

    #[structopt(long, short = "h", default_value = "127.0.0.1")]
    pub ip: String,

    #[structopt(long, short = "p", default_value = "3000")]
    pub port: u16,

    #[structopt(flatten)]
    pub build_args: BuildCommand,
}

impl ServeArgs for ServeCommand {
    fn log(&self) -> bool {
        self.log
    }

    fn ip(&self) -> &str {
        &self.ip
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn build_args(&self) -> &dyn BuildArgs {
        &self.build_args
    }
}

fn pre_build(args: &BuildCommand, profile: BuildProfile, command: &mut Command) -> Result<()> {
    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            command.args(&["--features", "wee_alloc"]);
        }
        BuildProfile::Dev => {
            command.args(&["--features", "console_error_panic_hook,mock"]);
        }
    }

    if let Some(features) = args.features.as_ref() {
        command.args(&["--features", features]);
    }

    Ok(())
}

fn serve(args: &ServeCommand, server: &mut Server<()>) -> Result<()> {
    use tide::{Body, Request, Response};

    let build_path = args.build_args().build_path().to_owned();
    let index_path = build_path.join("index.html");

    server.at("/").serve_dir(args.build_args().build_path())?;
    server.at("/").get(move |_| {
        let index_path = index_path.clone();
        async move { Ok(Response::from(Body::from_file(index_path).await?)) }
    });
    server.at("/*path").get(move |req: Request<()>| {
        let build_path = build_path.clone();
        async move {
            match Body::from_file(build_path.join(req.param("path").unwrap())).await {
                Ok(body) => Ok(Response::from(body)),
                Err(_) => Ok(Response::from(
                    Body::from_file(build_path.join("index.html")).await?,
                )),
            }
        }
    });
    Ok(())
}
