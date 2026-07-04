use std::process::Command;

use datalink::{DataExt, data::format::DEBUG, prelude::*};
use serde_json::from_reader;

struct Inxi;

impl Data for Inxi {
    fn query(&self, mut request: impl Request) {
        // inxi -exxxv 8 --output json --output-file print
        let mut cmd = Command::new("inxi");
        cmd.args(["-exxxv", "8", "--output", "json", "--output-file", "print"]);
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::null());
        // cmd.stdin(std::process::Stdio::null()); // Bug in inxi, thinks it should print help but can't because it thinks it's in an IRC

        let Ok(mut child) = cmd.spawn() else {
            return;
        };

        let Some(stdout) = child.stdout.take() else {
            return;
        };

        let reader = std::io::BufReader::new(stdout);

        let res = from_reader::<_, InxiOutput>(reader);

        let output = match res {
            Ok(out) => out,
            Err(e) => {
                eprintln!("{e}");
                return;
            }
        };

        dbg!(&output);

        for item in output {
            request.try_provide_link_with(|| item.into_iter().next());
        }
    }
}

type InxiOutput = Vec<std::collections::BTreeMap<String, serde_json::Value>>;

fn main() {
    let inxi = Inxi;

    dbg!(inxi.format::<DEBUG>());
}
