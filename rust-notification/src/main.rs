use std::{
    env,
    process::{Command, Stdio},
};

fn join(args: &Vec<String>) -> String {
    let result = args
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    result
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let res = notify();
    //let formatted_args = join(&args);
    /*    let mut cmd = std::process::Command::new("sh");
      let mut child = cmd
          .args(["-c", &formatted_args])
          .spawn()
          .expect("failed to execute process");
      let ecode = child.wait().expect("failed to wait on child");
      assert!(ecode.success());
      println!("command: {}", &args[0]);
      println!("args: {:?}", &args[1..]);
      let output = Command::new(&args[0])
          .args(&args[1..])
          .stdout(Stdio::inherit())
          .stderr(Stdio::inherit())
          .output()
          .expect("Failed to execute command");

      if output.status.success() {
          println!("Command executed");
      } else {
          println!("Command failed: {}", output.status);
    }*/
}

async fn notify() -> Result<String, Box<dyn std::error::Error>> {
    let webhook_url = "https://discord.com/api/webhooks/1140775363010641983/G-Uvp8omqfRVq2BS5AnaZOHoSnDc3tx_bPHLBhyzBBWIPdav-tK4rDgTL7aO-U8rjDV2";

    let client = reqwest::Client::new();
    let body = client
        .post(webhook_url)
        .form(&[("content", "hello")])
        .send()
        .await?
        .text()
        .await?;
    Ok(body)
}
