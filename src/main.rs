#![allow(clippy::unnecessary_wraps)]

use anyhow::Result;
use std::process::Command;
use structopt::StructOpt;
use wasm_run::prelude::*;

#[wasm_run::main(
    pre_build = pre_build,
    serve = serve,
)]
#[derive(StructOpt, Debug)]
enum Cli {}

fn pre_build(_args: &DefaultBuildArgs, profile: BuildProfile, command: &mut Command) -> Result<()> {
    match profile {
        BuildProfile::Profiling | BuildProfile::Release => {
            command.args(&["--features", "wee_alloc"]);
        }
        BuildProfile::Dev => {
            command.args(&["--features", "console_error_panic_hook"]);
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
