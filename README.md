# UniMIDI-in-Rust
A faster version of the OG UniMIDI, which was written in C#.

### Usage

If you have Cargo and Cargo Nightly installed, then you can head to the same directory as the .exe file and type in this command in your console:
```
UniMIDI.exe <path/to/midi_file.mid>
```
#### Available Arguments
```
[Visuals]
-randomizeColors
-noteSpeed N (can have decimals, must be greater than 0)
-blackNotes <true/false>
-experimentalOverlaps (Warning: This will greatly reduce the performance of UniMIDI)

[Audio]
-playbackSpeed N (can have decimals, must be greater than 0)
-transpose N (must be greater than 0)

[Extra]
-barfMode (Added just for fun)
```
#### Example:
```
UniMIDI.exe "/Black MIDIs/tau2.5.9.mid" -transpose 10 -randomizeColors true -blackNotes false
```
### Preview
![preview1](/preview_1.jpg)

![preview2](/preview_2.jpg)
