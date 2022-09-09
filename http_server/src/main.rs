fn main() {
    let post = Method::POST;
    let get = Method::GET;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr:String) -> Server {
        Server { addr }
    }

    fn run(self){
        println!("Server is listening on {}",self.addr);
    }
}

struct Request{
    method: Method,
    query_string: Option<String>,
    path: String
}

enum Method {
    GET,
    PUT,
    DELETE,
    POST
}
