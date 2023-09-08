// main.rs

// Importing necessary dependencies
use warp::Filter;

// Define an async function to handle the "/hello" endpoint
async fn hello() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::html("Hello, World!"))
}

// Define an async function to handle the "/goodbye" endpoint
async fn goodbye() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::html("Goodbye, World!"))
}

#[tokio::main]
async fn main() {
    // Define routes for "/hello" and "/goodbye"
    let hello_route = warp::path!("hello" / ..).and_then(hello);
    let goodbye_route = warp::path!("goodbye" / ..).and_then(goodbye);

    // Combine the routes using the `or` combinator
    let routes = hello_route.or(goodbye_route);

    // Start the server on localhost:3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
