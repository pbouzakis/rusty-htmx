use std::process::Command;

fn main() {
    println!("Building client scripts...");

    Command::new("sh")
        .args(["-c", "cd client && npm run build"])
        .output()
        .expect("failed to execute process");
}
