#![allow(clippy::unnecessary_wraps)]

use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main(
    pre_build = pre_build,
    serve = serve,
    build_args = BuildCommand,
)]
#[derive(StructOpt, Debug)]
enum Cli {}

#[derive(StructOpt, Debug)]
struct BuildCommand {
    #[structopt(flatten)]
    base: DefaultBuildArgs,

    #[structopt(long)]
    features: Option<String>,
}

impl BuildArgs for BuildCommand {
    fn build_path(&self) -> &PathBuf {
        self.base.build_path()
    }

    fn profiling(&self) -> bool {
        self.base.profiling()
    }
}

fn pre_build(args: &BuildCommand, profile: BuildProfile, command: &mut Command) -> Result<()> {
    let features = args
        .features
        .as_ref()
        .map(|x| format!(",{}", x))
        .unwrap_or_default();

    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            command.arg("--features");
            command.arg(format!("wee_alloc{}", features));
        }
        BuildProfile::Dev => {
            command.arg("--features");
            command.arg(format!("console_error_panic_hook,mock{}", features));
        }
    }

    Ok(())
}

fn serve(args: &DefaultServeArgs, server: &mut Server<()>) -> Result<()> {
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
