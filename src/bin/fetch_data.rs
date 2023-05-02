fn main()
{
    if std::env::var("auth_token").is_ok() {
        println!("juicy data");
    } else {
        println!("no auth token provided");
    }
}
