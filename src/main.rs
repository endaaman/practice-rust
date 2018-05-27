use std::process::Command;
// use std::os::unix::process::CommandExt;

fn main() {
    // let result = Command::new("dwajogjoawid")
    //     .current_dir("/")
    //     .exec();
    let args = [
      "echo hogehoge | open_jtalk ",
      "-m ./voices/mei_sad.htsvoice",
      "-x naist-jdic",
      "-ow #{wavFilename} && ",
      "play -q #{wavFilename}",
    ];

    let output = Command::new("sh")
            // .arg("-c")
            // .arg("echo hello")
            .args(&args)
            .output()
            .expect("failed to execute process");
    let result = output.stdout;
    println!("{}", std::str::from_utf8(&result).unwrap());

}
