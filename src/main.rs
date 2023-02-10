use inquire::Select;
use inquire::Text;
use std::process::Command;
use std::path::Path;

fn main() {
    let options = vec!["Encrypt", "Decrypt"];

    let ans = Select::new("What?", options)
        .prompt()
        .expect("No option selected.");
    let filename = Text::new("What filename?").prompt().expect("No filename.");

    if !Path::new(&filename).exists() {
        println!("No file.");
        return;
    }

    let passphrase = Text::new("Passphrase?").prompt().expect("No passphrase.");
    let passphrase2 = Text::new("Passphrase again?")
        .prompt()
        .expect("No passphrase");

    if passphrase != passphrase2 {
        println!("Passphrases wrong buddy");
        return;
    }

    if ans == "Encrypt" {
        // Run the following command: gpg --batch --passphrase "<passphrase>" -c <filename>
        let mut child = Command::new("gpg")
            .arg("--batch")
            .arg("--passphrase")
            .arg(passphrase)
            .arg("-c")
            .arg(filename)
            .spawn()
            .expect("Encrypting the file...");
        let exit_status = child.wait().expect("Encrypting the file...");
        if !exit_status.success() {
            println!("Something went wrong");
            return;
        }
    } else {
        // Run the following command: gpg --passphrase "<passphrase>" <filename>
        let mut child = Command::new("gpg")
            .arg("--batch")
            .arg("--passphrase")
            .arg(passphrase)
            .arg(filename)
            .spawn()
            .expect("Decrypting the file...");
        let exit_status = child.wait().expect("Decrypting the file...");
        if !exit_status.success() {
            println!("Wrong passphrase");
            return;
        }
    }

    // Save to file

    // Done

    println!("You're welcome");
}
