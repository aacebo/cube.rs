use std::io;

use cube::http::server::Server;

#[tokio::main]
async fn main() -> io::Result<()> {
    return Server::run("0.0.0.0:3000");
}
