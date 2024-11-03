use Durhack24::TreeAge;

use crate::SharedTree;

pub async fn get_big_tree(
    tree: SharedTree,
    age: TreeAge,
) -> Result<impl warp::Reply, warp::Rejection> {
    let trees = tree.read().unwrap();

    let top_post = trees.get_max(age);

    Ok(warp::reply::json(&top_post))
}

pub async fn get_small_tree(
    posts: SharedTree,
    age: TreeAge,
) -> Result<impl warp::Reply, warp::Rejection> {
    let posts = posts.read().unwrap();

    let top_post = posts.get_min(age);

    Ok(warp::reply::json(&top_post))
}
