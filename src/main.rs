extern crate iron;
#[macro_use(router)]
extern crate router;
extern crate staticfile;

mod actions;

use iron::prelude::*;
use iron::status;

fn response_printer(_req: &mut Request, res: Response) -> IronResult<Response> {
    println!("Response produced: {}", res);
    Ok(res)
}

fn shutdown_handler(_: &mut Request) -> IronResult<Response> {
    match actions::system::shutdown_system() {
        Ok(_) => Ok(Response::with((status::Ok))),
        Err(message) => Ok(Response::with((status::InternalServerError, message)))
    }
}

fn reboot_handler(_: &mut Request) -> IronResult<Response> {
    match actions::system::reboot_system() {
        Ok(_) => Ok(Response::with((status::Ok))),
        Err(message) => Ok(Response::with((status::InternalServerError, message)))
    }
}

fn restart_plex(_: &mut Request) -> IronResult<Response> {
    match actions::plex::restart() {
        Ok(_) => Ok(Response::with((status::Ok))),
        Err(message) => Ok(Response::with((status::InternalServerError, message)))
    }
}

fn ping(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok)))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let default_port = String::from("3000");
    let port = if args.len() > 1 { &args[1] } else { &default_port };

    let staticfiles = staticfile::Static::new("public/");
    let router = router!(
        shutdown_os_action: get "/actions/os/shutdown" => shutdown_handler,
        reboot_os_action: get "/actions/os/reboot" => reboot_handler,
        reboot_plex_action: get "/actions/plex/reboot" => restart_plex,
        ping: get "actions/ping" => ping,
        index: get "/" => staticfiles);

    let mut chain = Chain::new(router);
    chain.link_after(response_printer);

    let url = format!("localhost:{}", port);
    println!("{}", url);
    let _server = Iron::new(chain).http(url).unwrap();
    println!("Running on {}", port);
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
