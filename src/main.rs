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
    let file2 = MIDIFile::open_in_ram(&args[1], None).unwrap();
    let ppq = file.ppq();
    let ppq2 = file2.ppq();

    let mut transpose_value: i32 = 0;
    let mut playback_speed: f64 = 1.0;
    let mut randomize_colors = false;
    let mut barf_mode = false;
    let mut allow_black_notes = true;
    let mut note_size = 5;
    let mut experimental_overlaps = false;

    let mut color_index = [0,1,2,3,4,5,6,7];
   
    if args.contains(&"-transpose".to_string()) {
        transpose_value = args[args.iter().position(|r| r == "-transpose").unwrap()+1].parse::<i32>().unwrap();
    }

    if args.contains(&"-playbackSpeed".to_string()) {
        playback_speed = args[args.iter().position(|r| r == "-playbackSpeed").unwrap()+1].parse::<f64>().unwrap();
    }

    if args.contains(&"-randomizeColors".to_string()) {
        randomize_colors = true;
    }

    if args.contains(&"-barfMode".to_string()) {
        barf_mode = true;
    }

    if args.contains(&"-flatColors".to_string()) {
        allow_black_notes = false;
    }

    if args.contains(&"-noteSize".to_string()) || args.contains(&"-noteSpeed".to_string()) {
        note_size = args[args.iter().position(|r| r == "-noteSize" || r == "-noteSpeed").unwrap()+1].parse::<i32>().unwrap();
    }

    if args.contains(&"-experimentalOverlaps".to_string()) {
        experimental_overlaps = true;
        println!("\x1b[38;2;255;32;32mWARNING: UniMIDI will run slower with 'experimentalOverlaps'.\x1b[0m");
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

    println!("Preparing audio...");

    let amerged = pipe!(
        file2.iter_all_tracks()
        |>to_vec()
        |>merge_events_array()
        |>TimeCaster::<f64>::cast_event_delta()
        |>cancel_tempo_events((250000.0 / playback_speed) as u32)
        |>scale_event_time(1.0 / ppq2 as f64)
        |>unwrap_items()
    );


    println!("Loading events to RAM...");

    let merged = to_vec(merged);

    println!("Finalizing audio...");
    let amerged = to_vec(amerged);

    println!("Done!");

    let mut num_overlaps: [i32; 128] = [0; 128];

    let keyboard_string: Arc<Mutex<[&str]>> = Arc::new(Mutex::new([" "; 128]));

    let now = Instant::now();
    let mut time = 0.0;
    let mut atime = 0.0;

    let midi_ended = Arc::new(Mutex::new(false));

    let keyboard_thread = Arc::clone(&keyboard_string);
    let midi_end = Arc::clone(&midi_ended);

    let kdmapi = KDMAPI.open_stream();

    //let crossterm = Crossterm::new();

    let note_shades_b: Vec<&str> = vec!["\x1b[38;2;255;0;0m#\x1b[0m",
                                    "\x1b[38;2;255;128;0m#\x1b[0m",
                                    "\x1b[38;2;255;255;0m#\x1b[0m",
                                    "\x1b[38;2;0;255;0m#\x1b[0m",
                                    "\x1b[38;2;0;255;255m#\x1b[0m",
                                    "\x1b[38;2;0;0;255m#\x1b[0m",
                                    "\x1b[38;2;128;0;255m#\x1b[0m",
                                    "\x1b[38;2;255;0;255m#\x1b[0m"];

    let note_shades_w: Vec<&str> = vec!["\x1b[48;2;255;0;0m\x1b[38;2;255;0;0m#\x1b[0m",
                                    "\x1b[48;2;255;128;0m\x1b[38;2;255;128;0m#\x1b[0m",
                                    "\x1b[48;2;255;255;0m\x1b[38;2;255;255;0m#\x1b[0m",
                                    "\x1b[48;2;0;255;0m\x1b[38;2;0;255;0m#\x1b[0m",
                                    "\x1b[48;2;0;255;255m\x1b[38;2;0;255;255m#\x1b[0m",
                                    "\x1b[48;2;0;0;255m\x1b[38;2;0;0;255m#\x1b[0m",
                                    "\x1b[48;2;128;0;255m\x1b[38;2;128;0;255m#\x1b[0m",
                                    "\x1b[48;2;255;0;255m\x1b[38;2;255;0;255m#\x1b[0m"];

    let mut overlap_colors: Vec<Vec<i32>> = vec![Vec::new(); 128];
    let mut overlap_index: Vec<Vec<i32>> = vec![Vec::new(); 128];

    let audio_thread = thread::spawn(move || {
        for e in amerged {
            if e.delta() != 0.0 {
                atime += e.delta();
                let diff = atime - now.elapsed().as_secs_f64();
                
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
        }
    });

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

                if diff > 0.01 && barf_mode {
                    let mut rng = thread_rng();
                    color_index.shuffle(&mut rng);
                }
            }

            match e {
                Event::NoteOn(e) => {
                    let n = (e.key as i32 + (transpose_value as i32)) as u8 % 12;
                    let black_note = n == 1 || n == 3 || n == 6 || n == 8 || n == 10;
                    let kb_idx = ((e.key+transpose_value as u8)%128) as usize;
                    let n_idx = (e.channel % 8) as usize;

                    keyboard_string[kb_idx] = note_shades_w[n_idx];
                    if black_note && allow_black_notes {
                        keyboard_string[kb_idx] = note_shades_b[n_idx];
                    }

                    if experimental_overlaps {
                        overlap_index[kb_idx].push(num_overlaps[kb_idx]);
                        overlap_colors[kb_idx].push(n_idx as i32);
                    }
                    num_overlaps[kb_idx] += 1;
                }
                Event::NoteOff(e) => {
                    let kb_idx = ((e.key+transpose_value as u8)%128) as usize;
                    let n = (e.key as i32 + (transpose_value as i32)) as u8 % 12;
                    let black_note = n == 1 || n == 3 || n == 6 || n == 8 || n == 10;
                    
                    if experimental_overlaps {
                        let tmp_pos = overlap_colors[kb_idx].iter().position(|&r| r == (e.channel % 8) as i32).unwrap();
                        overlap_colors[kb_idx].remove(tmp_pos);

                        let mut overlap_colors_len = 0;
                        if overlap_colors[kb_idx].len() > 0 {
                            overlap_colors_len = overlap_colors[kb_idx].len() - 1;
                        }

                        if overlap_colors[kb_idx].len() > 0 {
                            let new_n_idx = overlap_colors[kb_idx][(overlap_colors_len) as usize] as usize;
                            keyboard_string[kb_idx] = note_shades_w[new_n_idx];
                            if black_note && allow_black_notes {
                                keyboard_string[kb_idx] = note_shades_b[new_n_idx];
                            }
                        }
                    }

                    num_overlaps[kb_idx] -= 1;
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
            thread::sleep(time::Duration::from_millis(((note_size as f64)/playback_speed) as u64));
        }
    });

    audio_thread.join().unwrap();
    thread_1.join().unwrap();
    thread_2.join().unwrap();
}
