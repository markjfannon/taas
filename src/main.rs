use std::sync::{Arc, RwLock};

use handlers::{get_big_tree, get_small_tree};
use warp::Filter;
use Durhack24::{deserialise_trees, prepare_tree, TreeAge, TreeTree};
mod handlers;

type SharedTree = Arc<RwLock<TreeTree>>;

#[tokio::main]
async fn main() {
    let trees = deserialise_trees(45709);
    let treetree = prepare_tree(trees);

    let shared_treetree = Arc::new(RwLock::new(treetree));

    let tree_filter = warp::any().map(move || shared_treetree.clone());

    let get_big_tree_route = warp::path!("largest_tree" / String)
        .and(tree_filter.clone())
        .and_then(|age: String, trees: SharedTree| async move {
            let age_enum = match age.as_str() {
                "young" => TreeAge::Young,
                "mature" => TreeAge::Mature,
                "earlymature" => TreeAge::EarlyMature,
                "semimature" => TreeAge::SemiMature,
                _ => return Err(warp::reject::not_found()),
            };
            get_big_tree(trees, age_enum).await
        });

    let get_small_tree_route = warp::path!("smallest_tree" / String)
        .and(tree_filter.clone())
        .and_then(|age: String, trees: SharedTree| async move {
            let age_enum = match age.as_str() {
                "young" => TreeAge::Young,
                "mature" => TreeAge::Mature,
                "earlymature" => TreeAge::EarlyMature,
                "semimature" => TreeAge::SemiMature,
                _ => return Err(warp::reject::not_found()),
            };
            get_small_tree(trees, age_enum).await
        });

    let routes = get_big_tree_route.or(get_small_tree_route);

    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
