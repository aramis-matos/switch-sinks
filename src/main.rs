use std::env;
use std::iter::zip;
use std::process::Command;

fn get_items(item: &str) -> Vec<String> {
    let cmd = format!("pactl list sinks | grep -E '{item}: .*' | sed 's/{item}: //'");
    let items = Command::new("bash")
        .args(&["-c", &cmd])
        .output()
        .expect("Failed to get sinks");

    let items = String::from_utf8(items.stdout)
        .expect("Could not get stdout from item")
        .split("\n")
        .map(|x| {
            let x = x.trim();
            x.to_owned()
        })
        .filter(|x| x != "")
        .collect::<Vec<_>>();

    items
}

fn get_current_sink() -> String {
    let cmd = format!("pactl get-default-sink | xargs echo -n");
    let current_sink = Command::new("bash")
        .args(&["-c", &cmd])
        .output()
        .expect("Failed to get current sink");
    String::from_utf8(current_sink.stdout).expect("Count not get stdout from ")
}

fn set_default_sink(sink: &str) {
    let cmd = format!("pactl set-default-sink {sink}");
    Command::new("bash")
        .args(&["-c", &cmd])
        .output()
        .expect("Failed set default sink");
}

fn main() {
    let sinks: Vec<_> = zip(get_items("Description"), get_items("Name")).collect();
    let filtered_sinks: Vec<_> = env::args().collect();
    let current_sink = get_current_sink();

    let sinks: Vec<_> = sinks
        .iter()
        .filter(|(x, _)| !filtered_sinks.iter().any(|y| x == y))
        .collect();

    let pos = sinks
        .iter()
        .position(|(_, x)| (*x) == current_sink)
        .unwrap_or_default();

    let next_sink = &sinks[(pos + 1) % sinks.len()].1;

    set_default_sink(next_sink);
}
