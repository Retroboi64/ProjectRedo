use pr_api::PRApi;

fn main() {
    let api = PRApi::new("./libengine.so");

    let result = api.create_engine();
    println!("{:?}", result);
}
