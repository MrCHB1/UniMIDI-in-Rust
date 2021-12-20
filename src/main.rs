use midi_toolkit::{
    events::{Event, MIDIEvent},
    io::MIDIFile,
    pipe,
    sequence::{event::merge_events_array, to_vec, unwrap_items, event::cancel_tempo_events, TimeCaster, event::scale_event_time}
};

use std::env;
use std::{thread, time};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;
use rand::thread_rng;

use kdmapi::KDMAPI;

fn main() {
    let args: Vec<String> = env::args().collect();

    //println!("{:?}", kdmapi::is_kdmapi_available());

    println!("Loading MIDI...");

    let file = MIDIFile::open_in_ram(&args[1], None).unwrap();
    //let file2 = MIDIFile::open_in_ram(&args[1], None).unwrap();
    let ppq = file.ppq();
    //let ppq2 = file2.ppq();

    let mut transpose_value: i32 = 0;
    let mut playback_speed: f64 = 1.0;
    let mut randomize_colors = false;
    let mut allow_black_notes = true;

    let mut color_index = [0,1,2,3,4,5,6,7];
   
    if args.contains(&"-transpose".to_string()) {
        transpose_value = args[args.iter().position(|r| r == "-transpose").unwrap()+1].parse::<i32>().unwrap();
    }

    if args.contains(&"-playbackSpeed".to_string()) {
       playback_speed = args[args.iter().position(|r| r == "-playbackSpeed").unwrap()+1].parse::<f64>().unwrap();
    }

    if args.contains(&"-randomizeColors".to_string()) {
        randomize_colors = args[args.iter().position(|r| r == "-randomizeColors").unwrap()+1].parse::<bool>().unwrap();
    }

    if args.contains(&"-blackNotes".to_string()) {
        allow_black_notes = args[args.iter().position(|r| r == "-blackNotes").unwrap()+1].parse::<bool>().unwrap();
    }

    if randomize_colors {
        let mut rng = thread_rng();
        color_index.shuffle(&mut rng);
    }

    println!("Merging events...");

    let merged = pipe!(
        file.iter_all_tracks()
        |>to_vec()
        |>merge_events_array()
        |>TimeCaster::<f64>::cast_event_delta()
        |>cancel_tempo_events((250000.0 / playback_speed) as u32)
        |>scale_event_time(1.0 / ppq as f64)
        |>unwrap_items()
    );

    /*let amerged = pipe!(
        file2.iter_all_tracks()
        |>to_vec()
        |>merge_events_array()
        |>TimeCaster::<f64>::cast_event_delta()
        |>cancel_tempo_events(250000)
        |>scale_event_time(1.0 / ppq2 as f64)
        |>unwrap_items()
    );*/

    println!("Loading events to RAM...");

    let merged = to_vec(merged);
    //let amerged = to_vec(amerged);

    println!("Done!");

    let mut num_overlaps: [i32; 128] = [0; 128];

    let keyboard_string: Arc<Mutex<[&str]>> = Arc::new(Mutex::new([" "; 128]));

    let now = Instant::now();
    let mut time = 0.0;

    let midi_ended = Arc::new(Mutex::new(false));

    let keyboard_thread = Arc::clone(&keyboard_string);
    let midi_end = Arc::clone(&midi_ended);

    let kdmapi = KDMAPI.open_stream();

    /*let audio_thread = thread::spawn(move || {
        for e in amerged {
            if e.delta() != 0.0 {
                atime += e.delta();
                let diff = atime - now.elapsed().as_secs_f64();
                
                if diff > 0.0 {
                    spin_sleep::sleep(time::Duration::from_secs_f64(diff));
                }
            }
        }
    });*/

    //let crossterm = Crossterm::new();

    let note_shades_b: [&str; 8] = ["\x1b[38;2;255;0;0m#\x1b[0m",
                                    "\x1b[38;2;255;128;0m#\x1b[0m",
                                    "\x1b[38;2;255;255;0m#\x1b[0m",
                                    "\x1b[38;2;0;255;0m#\x1b[0m",
                                    "\x1b[38;2;0;255;255m#\x1b[0m",
                                    "\x1b[38;2;0;0;255m#\x1b[0m",
                                    "\x1b[38;2;128;0;255m#\x1b[0m",
                                    "\x1b[38;2;255;0;255m#\x1b[0m"];
    let note_shades_w: [&str; 8] = ["\x1b[48;2;255;0;0m\x1b[38;2;255;0;0m#\x1b[0m",
                                    "\x1b[48;2;255;128;0m\x1b[38;2;255;128;0m#\x1b[0m",
                                    "\x1b[48;2;255;255;0m\x1b[38;2;255;255;0m#\x1b[0m",
                                    "\x1b[48;2;0;255;0m\x1b[38;2;0;255;0m#\x1b[0m",
                                    "\x1b[48;2;0;255;255m\x1b[38;2;0;255;255m#\x1b[0m",
                                    "\x1b[48;2;0;0;255m\x1b[38;2;0;0;255m#\x1b[0m",
                                    "\x1b[48;2;128;0;255m\x1b[38;2;128;0;255m#\x1b[0m",
                                    "\x1b[48;2;255;0;255m\x1b[38;2;255;0;255m#\x1b[0m"];

    let thread_1 = thread::spawn(move || {
        let mut keyboard_string = [" "; 128];
        for e in merged {
            if e.delta() != 0.0 {
                {
                    let mut ks = keyboard_thread.lock().unwrap();
                    for i in 0..ks.len() {
                        ks[(i+transpose_value as usize)%128] = keyboard_string[(i+transpose_value as usize)%128];
                    }
                }
                time += e.delta();
                let diff = time - now.elapsed().as_secs_f64();
                
                if diff > 0.0 {
                    spin_sleep::sleep(time::Duration::from_secs_f64(diff));
                }
            }

            if let Some(mut serialized) = e.as_u32() {
                if (transpose_value > 0 || transpose_value < 0) && (serialized & 0xf0 == 0x80 || serialized & 0xf0 == 0x90) {
                    serialized = (serialized & 0xff00ff) + ((serialized & 0x00ff00) + ((transpose_value as u32) << 8)) as u32;
                }
                kdmapi.send_direct_data(serialized);
            }

            match e {
                Event::NoteOn(e) => {
                    let n = (e.key as i32 + (transpose_value as i32)) as u8 % 12;
                    let black_note = n == 1 || n == 3 || n == 6 || n == 8 || n == 10;
                    
                    keyboard_string[((e.key+transpose_value as u8)%128) as usize] = note_shades_w[(color_index[(e.channel % 8) as usize]) as usize];
                    if black_note && allow_black_notes {
                        keyboard_string[((e.key+transpose_value as u8)%128) as usize] = note_shades_b[(color_index[(e.channel % 8) as usize]) as usize];
                    }
                    
                    num_overlaps[((e.key+transpose_value as u8)%128) as usize] += 1;
                }
                Event::NoteOff(e) => {
                    num_overlaps[((e.key+transpose_value as u8)%128) as usize] -= 1;
                    if num_overlaps[((e.key+transpose_value as u8)%128) as usize] == 0 {
                        keyboard_string[((e.key+transpose_value as u8)%128) as usize] = &" ";
                    }
                },
                _ => {}
            }
        }

        let mut mid_end = midi_end.lock().unwrap();
        *mid_end = true;
    });

    let keyboard_thread = Arc::clone(&keyboard_string);
    let midi_end = Arc::clone(&midi_ended);

    let thread_2 = thread::spawn(move || {
        while !(*midi_end.lock().unwrap()) {
            println!("{}", keyboard_thread.lock().unwrap().join(""));
            thread::sleep(time::Duration::from_millis((5.0/playback_speed) as u64));
        }
    });

    //audio_thread.join().unwrap();
    thread_1.join().unwrap();
    thread_2.join().unwrap();
}