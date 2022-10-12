//! A thin wrapper around Daniel Thompson's [pynus](https://github.com/daniel-thompson/pynus/tree/5572b01b26e04c7f8c79e8407f4d202e7258bf6b).
//! This currently straights up calls wasptool, which is a python wrapper around pynus.
//!
//! It would be great to reimplement both pynus and wasptool in rust in the future.

use std::process::Command;

// NOTE: Careful with this, it executes an arbritrary user command in python
pub(crate) fn eval(command: &str) {

    let output = Command::new("bin/wasptool")
        .arg("--eval")
        .arg(format!("from gadgetbridge import GB; GB({})", command))
        .output()
        .unwrap();

    println!("{:?}", output);

    println!("{}", output.status);

}

#[cfg(test)]
mod tests {
    use super::eval;

    #[test]
    fn test_eval() {
        eval("{'t': 'vibrate', 'n': true}");
    }
}
