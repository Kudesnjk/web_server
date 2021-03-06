use thread_pool::ThreadPool;
mod thread_pool;

use http_handler::HTTPHandler;
mod http_handler;

fn main() {
    let tp = ThreadPool::new();
    let hh = HTTPHandler::new();
}
