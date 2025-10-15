use hickory_resolver::Resolver;
use hickory_resolver::config::*;
use hickory_resolver::name_server::TokioConnectionProvider;
use std::net::*;
use tokio::runtime::Runtime;

fn main() {
    let hostname = get_hostname();
    println!("Resolving {hostname}");

    // We need a Tokio Runtime to run the resolver
    //  this is responsible for running all Future tasks and registering interest in IO channels
    let io_loop = Runtime::new().unwrap();

    // Construct a new Resolver with default configuration options
    let resolver = Resolver::builder_with_config(
        ResolverConfig::default(),
        TokioConnectionProvider::default(),
    )
    .build();

    // Lookup the IP addresses associated with a name.
    // This returns a future that will lookup the IP addresses, it must be run in the Core to
    //  to get the actual result.
    let lookup_future = resolver.lookup_ip(&hostname);

    // Run the lookup until it resolves or errors
    let response = io_loop.block_on(lookup_future).unwrap();

    // There can be many addresses associated with the name,
    //  this can return IPv4 and/or IPv6 addresses
    let addresses = response.iter().collect::<Vec<IpAddr>>();
    println!("Resolved {hostname} to addresses: {addresses:?}");
    //let address = response.iter().next().expect("no addresses returned!");
    //println!("Resolved {hostname} to {address}");
}

fn get_hostname() -> String {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: {} <hostname>", args[0]);
        std::process::exit(1);
    }

    args[1].clone()
}
