use anyhow::Result;
use clap::Parser;
use rust_embed::{Embed, RustEmbed};
use std::path::PathBuf;
use warp::{Filter, http::Response, hyper::Body};

mod optimize;

#[derive(Parser, Debug)]
#[command(name="witness-studio-app", version, about="All-in-one optimizer + viz")]
struct Opt {
    /// Input .wl (s-expression of the IR)
    input: PathBuf,
    /// Max rewrite iterations
    #[arg(long, default_value_t = 8)]
    iters: usize,
    /// Ache weight (size - λ*ache)
    #[arg(long, default_value_t = 0.6)]
    lambda: f64,
    /// Number of candidate solutions (approx)
    #[arg(long, default_value_t = 3)]
    nbest: usize,
    /// HTTP port
    #[arg(long, default_value_t = 8080)]
    port: u16,
}

#[derive(RustEmbed)]
#[folder = "static/"]
struct Assets;

fn asset(path: &str) -> Option<(Vec<u8>, &'static str)> {
    let p = if path.is_empty() || path == "/" { "index.html" } else { path.trim_start_matches('/') };
    Assets::get(p).map(|f| (f.data.into_owned(), mime_for(p)))
}
fn mime_for(p: &str) -> &'static str {
    if p.ends_with(".js") { "application/javascript" }
    else if p.ends_with(".css") { "text/css" }
    else if p.ends_with(".json") { "application/json" }
    else { "text/html" }
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::parse();

    // Run optimizer once at startup
    let (graph, trace, nbest) =
        optimize::run_optimization(&opt.input, opt.iters, opt.lambda, opt.nbest)?;

    let graph_json = warp::path("graph.json").map({
        let graph = graph.clone();
        move || Response::builder().header("content-type", "application/json").body(Body::from(graph.clone()))
    });
    let trace_json = warp::path("trace.json").map({
        let trace = trace.clone();
        move || Response::builder().header("content-type", "application/json").body(Body::from(trace.clone()))
    });
    let nbest_json = warp::path("nbest.json").map({
        let nbest = nbest.clone();
        move || Response::builder().header("content-type", "application/json").body(Body::from(nbest.clone()))
    });

    let static_route = warp::path::tail().and(warp::get()).map(|tail: warp::filters::path::Tail| {
        let path = tail.as_str();
        match asset(path) {
            Some((bytes, mime)) => Response::builder().header("content-type", mime).body(Body::from(bytes)),
            None => {
                let (bytes, mime) = asset("index.html").unwrap();
                Response::builder().header("content-type", mime).body(Body::from(bytes))
            }
        }
    });

    let routes = graph_json.or(trace_json).or(nbest_json).or(static_route).with(warp::cors().allow_any_origin());
    println!("Witness Studio → http://localhost:{}/", opt.port);
    warp::serve(routes).run(([0,0,0,0], opt.port)).await;
    Ok(())
}
