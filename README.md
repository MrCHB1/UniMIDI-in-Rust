# UniMIDI-in-Rust
A faster version of the OG UniMIDI, which was written in C#.

### Usage

If you have Cargo and Cargo Nightly installed, then you can head to this directory and type in this command in your console:
```cargo +nightly run <path/to/midi_file.mid> [-transpose N (default is 0, has to be greater than or equal to 0)] [-playbackSpeed N (default is 1.0)] [-randomizeColors <true/false>] [-blackNotes <true/false>]
Example:
cargo +nightly run "/Black MIDIs/tau2.5.9.mid" -transpose 10 -randomizeColors true -blackNotes false
```
