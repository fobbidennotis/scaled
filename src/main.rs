use clap::{Arg, Command};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static MODES_INTERVAL_SEQUENCES: Lazy<HashMap<&'static str, [u8; 7]>> = Lazy::new(|| {
    HashMap::from([
        ("major", [2, 2, 1, 2, 2, 2, 1]),
        ("dorian", [2, 1, 2, 2, 2, 1, 2]),
        ("phrygian", [1, 2, 2, 2, 1, 2, 2]),
        ("lydian", [2, 2, 2, 1, 2, 2, 1]),
        ("mixolydian", [2, 2, 1, 2, 2, 1, 2]),
        ("minor", [2, 1, 2, 2, 1, 2, 2]),
        ("locrian", [1, 2, 2, 1, 2, 2, 2]),
    ])
});

const NOTES_SEQUENCE: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

const STANDARD_TUNING_INTERVALS: [i32; 6] = [0, 5, 10, 15, 19, 24];

fn get_scale_notes(root: &str, mode: &str) -> Result<Vec<&'static str>, String> {
    let mut scale: Vec<&str> = Vec::new();

    let normalized_root = normalize_note(root)?;

    let root_note_index = NOTES_SEQUENCE
        .iter()
        .position(|note| *note == normalized_root)
        .ok_or_else(|| format!("Invalid root note: {}", root))?;

    let mut current_note_index = root_note_index;

    let intervals = MODES_INTERVAL_SEQUENCES
        .get(mode.to_lowercase().as_str())
        .ok_or_else(|| format!("Invalid mode: {}. Available modes: major, dorian, phrygian, lydian, mixolydian, minor, locrian", mode))?;

    for interval in intervals {
        scale.push(NOTES_SEQUENCE[current_note_index]);
        current_note_index = (current_note_index + *interval as usize) % 12;
    }

    Ok(scale)
}

fn normalize_note(note: &str) -> Result<&str, String> {
    let upper_note = note.to_uppercase();
    NOTES_SEQUENCE
        .iter()
        .find(|&n| *n == upper_note)
        .copied()
        .ok_or_else(|| format!("Invalid note: {}", note))
}

fn get_tuning(tuning_root: &str, drop: bool) -> Result<Vec<&'static str>, String> {
    let normalized_root = normalize_note(tuning_root)?;

    let root_index = NOTES_SEQUENCE
        .iter()
        .position(|note| *note == normalized_root)
        .unwrap();

    let mut guitar_tuning = Vec::new();

    if drop {
        let drop_intervals = [0, 7, 12, 17, 21, 26];

        for interval in drop_intervals.iter() {
            let string_index = (root_index as i32 + interval).rem_euclid(12) as usize;
            guitar_tuning.push(NOTES_SEQUENCE[string_index]);
        }
    } else {
        for interval in STANDARD_TUNING_INTERVALS.iter() {
            let string_index = (root_index as i32 + interval).rem_euclid(12) as usize;
            guitar_tuning.push(NOTES_SEQUENCE[string_index]);
        }
    }

    guitar_tuning.reverse();

    Ok(guitar_tuning)
}

fn display_notes_guitar(scale: Vec<&str>, tuning: Vec<&str>) {
    println!("┌────────────────────────────────────────────────────────────────────────────────┐");
    println!("│  0  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  |  10  |  11  |  12  │");
    println!("│────────────────────────────────────────────────────────────────────────────────│");

    for string_root in tuning {
        let root_index = NOTES_SEQUENCE
            .iter()
            .position(|note| *note == string_root)
            .unwrap();

        let mut string_notes = Vec::with_capacity(13);
        for i in 0..=12 {
            string_notes.push(NOTES_SEQUENCE[(root_index + i) % 12]);
        }

        let mut string_display = String::from("│");
        for (i, note) in string_notes.iter().enumerate() {
            string_display += "  ";
            if scale.contains(note) || i == 0 {
                string_display += note;
                string_display += " ";
                if note.len() == 1 {
                    string_display += " ";
                }
            } else {
                string_display += "   ";
            }
            if i > 9 {
                string_display += " ";
            }
            if i == 12 {
                string_display += "│";
            } else {
                string_display += "|";
            }
        }
        println!("{}", string_display);
    }
    println!("└────────────────────────────────────────────────────────────────────────────────┘");
}

fn main() {
    let matches = Command::new("scaled")
        .version("1.0.0")
        .author("NotixV")
        .about("Display musical scales on guitar fretboard")
        .help_template("{name} {version}\n{author-with-newline}{about-with-newline}\n{usage-heading} {usage}\n\n{all-args}{after-help}")
        .arg(
            Arg::new("root")
                .short('r')
                .long("root")
                .value_name("NOTE")
                .help("Root note of the scale (e.g., C, C#, D, Eb, F#)")
                .required(true)
        )
        .arg(
            Arg::new("mode")
                .short('m')
                .long("mode")
                .value_name("MODE")
                .help("Musical mode: major, minor, dorian, phrygian, lydian, mixolydian, locrian")
                .required(true)
        )
        .arg(
            Arg::new("tuning")
                .short('t')
                .long("tuning")
                .value_name("NOTE")
                .help("Lowest string tuning note (e.g., E, D, C#, F#) - calculates full tuning from this")
                .default_value("e")
        )
        .arg(
            Arg::new("drop")
                .long("drop")
                .help("Drop the lowest string by a whole step (e.g., Drop D, Drop C)")
                .action(clap::ArgAction::SetTrue)
        )
        .after_help("EXAMPLES:\n    scaled --root C --mode major\n    scaled -r F# -m dorian -t d\n    scaled --root C --mode major --tuning F# --drop\n    scaled --root A --mode minor --tuning Bb")
        .get_matches();

    let root = matches.get_one::<String>("root").unwrap();
    let mode = matches.get_one::<String>("mode").unwrap();
    let tuning = matches.get_one::<String>("tuning").unwrap();
    let drop = matches.get_flag("drop");

    match get_scale_notes(root, mode) {
        Ok(scale) => match get_tuning(tuning, drop) {
            Ok(guitar_tuning) => {
                println!("Scale: {} {}", root.to_uppercase(), mode);
                println!(
                    "Tuning: {} {}",
                    tuning.to_uppercase(),
                    if drop { "Drop" } else { "Standard" }
                );
                println!("Strings: {}", guitar_tuning.join("-"));
                println!();
                display_notes_guitar(scale, guitar_tuning);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
