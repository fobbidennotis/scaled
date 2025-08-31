# Scaled 

A command-line tool for visualizing musical scales on guitar fretboards with dynamic tuning support.

## Features

- **Any root note**: Support for all chromatic notes (C, C#, D, Eb, F#, etc.)
- **All modes**: Major, Minor, Dorian, Phrygian, Lydian, Mixolydian, Locrian
- **Dynamic tunings**: Any note can be used as the lowest string for the tuning
- **Drop tunings**: Automatic drop tuning calculation for any root note
- **Visual fretboard**: Clear ASCII representation showing scale notes

## Installation

### From Release (Recommended)
Download the latest binary from the [releases page](https://github.com/fobbidennotis/scaled/releases).

### From Source
```bash
git clone https://github.com/fobbidennotis/scaled.git
cd scaled
cargo build --release
```

The binary will be available at `target/release/scaled`.

## Usage

```bash
scaled --root <NOTE> --mode <MODE> [--tuning <NOTE>] [--drop]
```

### Required Arguments
- `--root, -r`: Root note of the scale (C, C#, D, Eb, F, F#, G, G#, A, Bb, B)
- `--mode, -m`: Musical mode (major, minor, dorian, phrygian, lydian, mixolydian, locrian)

### Optional Arguments  
- `--tuning, -t`: Lowest string note for tuning calculation (default: E)
- `--drop`: Use drop tuning variant
- `--help, -h`: Show help information

## Examples

### Basic Usage
```bash
# C Major in standard tuning (E-A-D-G-B-E)
scaled --root C --mode major

# F# Dorian in standard tuning
scaled -r F# -m dorian
```

### Custom Tunings
```bash
# A Minor in D standard tuning (D-G-C-F-A-D)
scaled --root A --mode minor --tuning D

# C# Phrygian in B standard tuning (B-E-G#-C#-F#-B)
scaled -r C# -m phrygian -t B
```

### Drop Tunings
```bash
# C Major in Drop D (D-A-D-G-B-E)
scaled --root C --mode major --tuning D --drop

# G Minor in Drop B (B-F#-B-E-G#-C#)
scaled --root G --mode minor --tuning B --drop

# E Minor in Drop C (C-G-C-F-A-D)
scaled -r E -m minor -t C --drop
```

## How Tunings Work

**Standard Tunings**: Any note can be specified as the lowest string, and the tool calculates the remaining strings using standard tuning intervals (perfect 4ths with a major 3rd between G-B strings).

**Drop Tunings**: The tool calculates a drop tuning variant where the lowest string and second string create power chord intervals.

Examples:
- E Standard: E-A-D-G-B-E
- D Standard: D-G-C-F-A-D  
- Drop D: D-A-D-G-B-E
- Drop C: C-G-C-F-A-D

## Output

The tool displays:
1. Scale and tuning information
2. String tuning (low to high)
3. ASCII fretboard showing:
   - Fret numbers (0-12)
   - Open string notes
   - Scale notes displayed on each string

Example output:
```
Scale: C# PHRYGIAN
Tuning: B Drop
Strings: B-F#-B-E-G#-C#

┌────────────────────────────────────────────────────────────────────────────────┐
│  0  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  |  10  |  11  |  12  │
│────────────────────────────────────────────────────────────────────────────────│
│  B  |  C  |  C# |     |  D# |  E  |  F  |  F# |  G  |  G# |      |      |  B   │
│ F#  |  G  |  G# |     |     |  B  |  C  |  C# |     |  D# |  E   |  F   |  F#  │
│  B  |  C  |  C# |     |  D# |  E  |  F  |  F# |  G  |  G# |      |      |  B   │
│  E  |  F  |  F# |  G  |  G# |     |     |  B  |  C  |  C# |      |  D#  |  E   │
│ G#  |     |     |  B  |  C  |  C# |     |  D# |  E  |  F  |  F#  |  G   |  G#  │
│ C#  |     |  D# |  E  |  F  |  F# |  G  |  G# |     |     |  B   |  C   |  C#  │
└────────────────────────────────────────────────────────────────────────────────┘
```

## Development

### Prerequisites
- Rust 1.70+ 
- Cargo

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Running Locally
```bash
cargo run -- --root C --mode major
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see LICENSE file for details.
