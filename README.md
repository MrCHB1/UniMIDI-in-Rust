# UniMIDI-in-Rust
A faster version of the OG UniMIDI, which was written in C#.

### Usage

If you have Cargo and Cargo Nightly installed, then you can head to this directory and type in this command in your console:
```
cargo +nightly run <path/to/midi_file.mid>

##### Available Arguments
-transpose N (must be greater than 0)
-playbackSpeed N (can have decimals, must be greater than 0)
-randomizeColors
-noteSpeed N (can have decimals, must be greater than 0)
-blackNotes <true/false>
-experimentalOverlaps (Warning: This will greatly reduce the performance of UniMIDI)

-barfMode (Added just for fun)
```
Example:
cargo +nightly run "/Black MIDIs/tau2.5.9.mid" -transpose 10 -randomizeColors true -blackNotes false
```
### Preview
![preview1](/preview_1.jpg)

![preview2](/preview_2.jpg)
