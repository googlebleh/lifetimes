fn without_token()
{
    let cmd_output = std::process::Command::new("./target/debug/fetch_data")
        .args([
              "--server", "NA",
              "key_to_fetch",
        ])
        .output()
        .unwrap();
    let cmd_output_str = String::from_utf8(cmd_output.stdout)
        .unwrap();

    // use output later
    println!("command output: {}", cmd_output_str);
}

#[cfg(use_token)]
fn with_token()
{
    let cmd = std::process::Command::new("./target/debug/fetch_data")
        .args([
              "--server", "NA",
              "key_to_fetch",
        ]);
    if std::env::var("auth_token").is_err() {
        cmd = cmd.env("auth_token", "secret");
    }
    let cmd_output = cmd.output()
        .unwrap();
    let cmd_output_str = String::from_utf8(cmd_output.stdout)
        .unwrap();

    // use output later
    println!("command output: {}", cmd_output_str);
}

fn main()
{
    without_token();
}
