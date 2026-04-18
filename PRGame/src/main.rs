fn main() {
    let api = pr_api::PRApi::new("./libengine.so");

    let result = api.test();
    println!("{:?}", result);
}
