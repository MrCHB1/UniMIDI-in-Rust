use midi_toolkit::{
    events::{Event, MIDIEvent},
    io::MIDIFile,
    pipe,
    sequence::{event::merge_events_array, to_vec, unwrap_items, event::cancel_tempo_events, TimeCaster, event::scale_event_time}
};

use std::env;
use std::{thread, time};
use std::io::{Write,stdout};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;
use rand::thread_rng;

use crossterm::terminal;
use crossterm::terminal::*;
use crossterm::{QueueableCommand, cursor};
use crossterm::event;
use crossterm::event::{KeyEvent, KeyCode, read};

use kdmapi::KDMAPI;

use wfd;

#[cfg(windows)]
pub fn enable_virtual_terminal_processing() {
    use winapi_util::console::Console;

    if let Ok(mut term) = Console::stdout() {
        let _ = term.set_virtual_terminal_processing(true);
    }

    if let Ok(mut term) = Console::stderr() {
        let _ = term.set_virtual_terminal_processing(true);
    }
}

pub fn write_text(stdout: &mut std::io::Stdout, x: u16, y: u16, text: &str) {
    stdout.queue(cursor::SavePosition).ok();
    stdout.queue(cursor::MoveTo(x,y)).ok();
    stdout.write(text.as_bytes()).ok();
    stdout.queue(cursor::RestorePosition).ok();
    stdout.flush().ok();
}

pub fn set_palette(color_type: i32, note_shades_b: &mut Vec<&str>, note_shades_w: &mut Vec<&str>) {
    match color_type%3 {
        0 => {
            // Default rainbow
            let nb: Vec<&str> = vec!["\x1b[38;2;255;0;0m#\x1b[0m",
                                 "\x1b[38;2;255;128;0m#\x1b[0m",
                                 "\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[38;2;0;255;0m#\x1b[0m",
                                 "\x1b[38;2;0;255;255m#\x1b[0m",
                                 "\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[38;2;128;0;255m#\x1b[0m",
                                 "\x1b[38;2;255;0;255m#\x1b[0m",
                                 "\x1b[38;2;255;0;0m#\x1b[0m",
                                 "\x1b[38;2;255;128;0m#\x1b[0m",
                                 "\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[38;2;0;255;0m#\x1b[0m",
                                 "\x1b[38;2;0;255;255m#\x1b[0m",
                                 "\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[38;2;128;0;255m#\x1b[0m",
                                 "\x1b[38;2;255;0;255m#\x1b[0m"];
            *note_shades_b = nb;

            let nw: Vec<&str> = vec!["\x1b[48;2;255;0;0m\x1b[38;2;255;0;0m#\x1b[0m",
                                 "\x1b[48;2;255;128;0m\x1b[38;2;255;128;0m#\x1b[0m",
                                 "\x1b[48;2;255;255;0m\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[48;2;0;255;0m\x1b[38;2;0;255;0m#\x1b[0m",
                                 "\x1b[48;2;0;255;255m\x1b[38;2;0;255;255m#\x1b[0m",
                                 "\x1b[48;2;0;0;255m\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[48;2;128;0;255m\x1b[38;2;128;0;255m#\x1b[0m",
                                 "\x1b[48;2;255;0;255m\x1b[38;2;255;0;255m#\x1b[0m",
                                 "\x1b[48;2;255;0;0m\x1b[38;2;255;0;0m#\x1b[0m",
                                 "\x1b[48;2;255;128;0m\x1b[38;2;255;128;0m#\x1b[0m",
                                 "\x1b[48;2;255;255;0m\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[48;2;0;255;0m\x1b[38;2;0;255;0m#\x1b[0m",
                                 "\x1b[48;2;0;255;255m\x1b[38;2;0;255;255m#\x1b[0m",
                                 "\x1b[48;2;0;0;255m\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[48;2;128;0;255m\x1b[38;2;128;0;255m#\x1b[0m",
                                 "\x1b[48;2;255;0;255m\x1b[38;2;255;0;255m#\x1b[0m"];

            *note_shades_w = nw;
        },
        1 => {
            // Extended rainbow
            let nb: Vec<&str> = vec!["\x1b[38;2;255;0;0m#\x1b[0m",
                                 "\x1b[38;2;255;64;0m#\x1b[0m",
                                 "\x1b[38;2;255;128;0m#\x1b[0m",
                                 "\x1b[38;2;255;192;0m#\x1b[0m",
                                 "\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[38;2;128;255;0m#\x1b[0m",
                                 "\x1b[38;2;0;255;0m#\x1b[0m",
                                 "\x1b[38;2;0;255;128m#\x1b[0m",
                                 "\x1b[38;2;0;255;255m#\x1b[0m",
                                 "\x1b[38;2;0;128;255m#\x1b[0m",
                                 "\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[38;2;64;0;255m#\x1b[0m",
                                 "\x1b[38;2;128;0;255m#\x1b[0m",
                                 "\x1b[38;2;192;0;255m#\x1b[0m",
                                 "\x1b[38;2;255;0;255m#\x1b[0m",
                                 "\x1b[38;2;255;0;128m#\x1b[0m"];

            *note_shades_b = nb;

            let nw: Vec<&str> = vec!["\x1b[48;2;255;0;0m\x1b[38;2;255;0;0m#\x1b[0m",
                                 "\x1b[48;2;255;64;0m\x1b[38;2;255;64;0m#\x1b[0m",
                                 "\x1b[48;2;255;128;0m\x1b[38;2;255;128;0m#\x1b[0m",
                                 "\x1b[48;2;255;192;0m\x1b[38;2;255;192;0m#\x1b[0m",
                                 "\x1b[48;2;255;255;0m\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[48;2;128;255;0m\x1b[38;2;128;255;0m#\x1b[0m",
                                 "\x1b[48;2;0;255;0m\x1b[38;2;0;255;0m#\x1b[0m",
                                 "\x1b[48;2;0;255;128m\x1b[38;2;0;255;128m#\x1b[0m",
                                 "\x1b[48;2;0;255;255m\x1b[38;2;0;255;255m#\x1b[0m",
                                 "\x1b[48;2;0;128;255m\x1b[38;2;0;128;255m#\x1b[0m",
                                 "\x1b[48;2;0;0;255m\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[48;2;64;0;255m\x1b[38;2;64;0;255m#\x1b[0m",
                                 "\x1b[48;2;128;0;255m\x1b[38;2;128;0;255m#\x1b[0m",
                                 "\x1b[48;2;192;0;255m\x1b[38;2;192;0;255m#\x1b[0m",
                                 "\x1b[48;2;255;0;255m\x1b[38;2;255;0;255m#\x1b[0m",
                                 "\x1b[48;2;255;0;128m\x1b[38;2;255;0;128m#\x1b[0m"];

            *note_shades_w = nw;
        },
        2 => {
            // Rainbow variant 2
            let nb: Vec<&str> = vec!["\x1b[38;2;228;14;22m#\x1b[0m",
                                 "\x1b[38;2;255;28;33m#\x1b[0m",
                                 "\x1b[38;2;255;107;33m#\x1b[0m",
                                 "\x1b[38;2;255;116;0m#\x1b[0m",
                                 "\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[38;2;254;245;80m#\x1b[0m",
                                 "\x1b[38;2;142;251;2m#\x1b[0m",
                                 "\x1b[38;2;38;224;0m#\x1b[0m",
                                 "\x1b[38;2;34;190;2m#\x1b[0m",
                                 "\x1b[38;2;0;205;245m#\x1b[0m",
                                 "\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[38;2;0;53;239m#\x1b[0m",
                                 "\x1b[38;2;34;0;167m#\x1b[0m",
                                 "\x1b[38;2;117;0;159m#\x1b[0m",
                                 "\x1b[38;2;209;3;130m#\x1b[0m",
                                 "\x1b[38;2;255;0;255m#\x1b[0m"];

            *note_shades_b = nb;

            let nw: Vec<&str> =  vec!["\x1b[48;2;228;14;22m\x1b[38;2;228;14;22m#\x1b[0m",
                                 "\x1b[48;2;255;28;33m\x1b[38;2;255;28;33m#\x1b[0m",
                                 "\x1b[48;2;255;107;33m\x1b[38;2;255;107;33m#\x1b[0m",
                                 "\x1b[48;2;255;116;0m\x1b[38;2;255;116;0m#\x1b[0m",
                                 "\x1b[48;2;255;255;0m\x1b[38;2;255;255;0m#\x1b[0m",
                                 "\x1b[48;2;254;245;80m\x1b[38;2;254;245;80m#\x1b[0m",
                                 "\x1b[48;2;142;251;2m\x1b[38;2;142;251;2m#\x1b[0m",
                                 "\x1b[48;2;38;224;0m\x1b[38;2;38;224;0m#\x1b[0m",
                                 "\x1b[48;2;34;190;2m\x1b[38;2;34;190;2m#\x1b[0m",
                                 "\x1b[48;2;0;205;245m\x1b[38;2;0;205;245m#\x1b[0m",
                                 "\x1b[48;2;0;0;255m\x1b[38;2;0;0;255m#\x1b[0m",
                                 "\x1b[48;2;0;53;239m\x1b[38;2;0;53;239m#\x1b[0m",
                                 "\x1b[48;2;34;0;167m\x1b[38;2;34;0;167m#\x1b[0m",
                                 "\x1b[48;2;117;0;159m\x1b[38;2;117;0;159m#\x1b[0m",
                                 "\x1b[48;2;209;3;130m\x1b[38;2;209;3;130m#\x1b[0m",
                                 "\x1b[48;2;255;0;255m\x1b[38;2;255;0;255m#\x1b[0m"];

            *note_shades_w = nw;
        },
        _ => {}
    }
}

fn main() {
    #[cfg(windows)]
    enable_virtual_terminal_processing();

    let mut args: Vec<String> = env::args().collect();

    let is_main_menu = Arc::new(Mutex::new(true));

    let mm = Arc::clone(&is_main_menu);

    let mut color_type = 0; // 0 is the default (rainbow)
    if args.contains(&"-colorPalette".to_string()) {
        color_type = args[args.iter().position(|r| r == "-colorPalette").unwrap()+1].parse::<i32>().unwrap();
    }

    let colors: Vec<&str> = vec!["\x1b[38;2;255;0;0m",
                                "\x1b[38;2;255;128;0m",
                                "\x1b[38;2;255;255;0m",
                                "\x1b[38;2;0;255;0m",
                                "\x1b[38;2;0;255;255m",
                                "\x1b[38;2;0;0;255m",
                                "\x1b[38;2;128;0;255m",
                                "\x1b[38;2;255;0;255m",
                                "\x1b[38;2;255;0;0m",
                                "\x1b[38;2;255;128;0m",
                                "\x1b[38;2;255;255;0m",
                                "\x1b[38;2;0;255;0m",
                                "\x1b[38;2;0;255;255m",
                                "\x1b[38;2;0;0;255m",
                                "\x1b[38;2;128;0;255m",
                                "\x1b[38;2;255;0;255m"];
    
    let mut note_shades_b: Vec<&str> = Vec::new();
    let mut note_shades_w: Vec<&str> = Vec::new();

    set_palette(color_type, &mut note_shades_b, &mut note_shades_w);

    let mut s = stdout();
    s.queue(terminal::Clear(ClearType::All)).ok();
    s.queue(cursor::SavePosition).ok();
    s.queue(cursor::MoveTo(38,0)).ok();
    s.write("\x1b[38;2;0;255;255mv0.3b\x1b[0m".as_bytes()).ok();
    s.queue(cursor::RestorePosition).ok();
    s.queue(cursor::MoveTo(0,0)).ok();
    s.write("Welcome to \x1b[38;2;0;255;0mUniMIDI\x1b[0m.".as_bytes()).ok();
    s.queue(cursor::MoveTo(0,2)).ok();
    s.write(format!("\x1b[38;2;0;255;0mCurrent MIDI path\x1b[0m: {}",&args[1]).as_bytes()).ok();
    s.queue(cursor::MoveTo(15,4)).ok();
    s.write(format!("Channel colors:\n0: {} 1: {} 2:  {}  3: {}  4: {}  5: {}  6: {}  7: {}\n8: {} 9: {} 10: {} 11: {} 12: {} 13: {} 14: {} 15: {}",
note_shades_w[0],
note_shades_w[1],
note_shades_w[2],
note_shades_w[3],
note_shades_w[4],
note_shades_w[5],
note_shades_w[6],
note_shades_w[7],
note_shades_w[8],
note_shades_w[9],
note_shades_w[10],
note_shades_w[11],
note_shades_w[12],
note_shades_w[13],
note_shades_w[14],
note_shades_w[15]
).as_bytes()).ok();
    s.queue(cursor::MoveTo(0,8)).ok();
    s.write("\x1b[1mContinue ..... [\x1b[38;2;0;255;0mAny key\x1b[0m]\n\x1b[1mChange MIDI .. [\x1b[38;2;0;255;0mM\x1b[0m]\n\x1b[1mChange palette [\x1b[38;2;0;255;0m←/→\x1b[0m]\n\x1b[1mHelp ......... [\x1b[38;2;0;255;0mH\x1b[0m]\n\x1b[1mQuit ......... [\x1b[38;2;0;255;0mEsc\x1b[0m]".as_bytes()).ok();
    s.flush().ok();

    enable_raw_mode().unwrap();

    let mm_thread = thread::spawn(move || {
        let mut stdout = stdout();
        let mut col_idx = 0;
        while *mm.lock().unwrap() {
            stdout.queue(cursor::SavePosition).ok();
            stdout.queue(cursor::MoveTo(0,0)).ok();
            stdout.write(format!("Welcome to {}U{}n{}i{}M{}I{}D{}I\x1b[0m.",colors[col_idx%16],colors[(col_idx+1)%16],colors[(col_idx+2)%16],colors[(col_idx+3)%16],colors[(col_idx+4)%16],colors[(col_idx+5)%16],colors[(col_idx+6)%16]).as_bytes()).ok();
            stdout.queue(cursor::RestorePosition).ok();
            stdout.flush().ok();
            thread::sleep(time::Duration::from_millis(100));
            col_idx += 1;
        }
    });

    let mm = Arc::clone(&is_main_menu);
    let mut is_help = false;

    while *mm.lock().unwrap() {
        match read().unwrap() {
            event::Event::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: _no_modifiers,
            }) => {
                s.queue(terminal::Clear(ClearType::All)).ok();
                s.queue(cursor::MoveTo(0,0)).ok();
                std::process::exit(0);
            },
            event::Event::Key(KeyEvent {
                code: KeyCode::Char('m'),
                modifiers: _no_modifiers,
            }) => {
                let params = wfd::DialogParams {
                    title: "Choose a MIDI file.",
                    file_types: vec![("MIDI Files","*.mid")],
                    ..Default::default()
                };
                let result = wfd::open_dialog(params).unwrap();
                let path = result.selected_file_path.to_str();
                match path {
                    Some(p) => {
                        args[1] = p.to_string();
                    },
                    None => {
                        println!("\x1b[38;2;255;255;0mNo path specified, using intialized path.\x1b[0m");
                    }
                }
                s.queue(cursor::SavePosition).ok();
                s.queue(cursor::MoveTo(0,2)).ok();
                s.queue(terminal::Clear(ClearType::CurrentLine)).ok();
                s.write(format!("\x1b[38;2;0;255;0mCurrent MIDI path\x1b[0m: {}",&args[1]).as_bytes()).ok();
                s.queue(cursor::RestorePosition).ok();
                s.flush().ok();
            },
            event::Event::Key(KeyEvent{
                code: KeyCode::Right,
                modifiers: _no_modifiers,
            }) => {
                color_type = (color_type+1)%3;
                set_palette(color_type, &mut note_shades_b, &mut note_shades_w);
                s.queue(cursor::SavePosition).ok();
                s.queue(cursor::MoveTo(15,4)).ok();
                s.write(format!("Channel colors:\n0: {} 1: {} 2:  {}  3: {}  4: {}  5: {}  6: {}  7: {}\n8: {} 9: {} 10: {} 11: {} 12: {} 13: {} 14: {} 15: {}",
                    note_shades_w[0],
                    note_shades_w[1],
                    note_shades_w[2],
                    note_shades_w[3],
                    note_shades_w[4],
                    note_shades_w[5],
                    note_shades_w[6],
                    note_shades_w[7],
                    note_shades_w[8],
                    note_shades_w[9],
                    note_shades_w[10],
                    note_shades_w[11],
                    note_shades_w[12],
                    note_shades_w[13],
                    note_shades_w[14],
                    note_shades_w[15]
                ).as_bytes()).ok();
                s.queue(cursor::RestorePosition).ok();
              
                s.flush().ok();
            },
            event::Event::Key(KeyEvent{
                code: KeyCode::Left,
                modifiers: _no_modifiers,
            }) => {
                color_type = (color_type+2)%3;
                set_palette(color_type, &mut note_shades_b, &mut note_shades_w);
                s.queue(cursor::SavePosition).ok();
                s.queue(cursor::MoveTo(15,4)).ok();
                s.write(format!("Channel colors:\n0: {} 1: {} 2:  {}  3: {}  4: {}  5: {}  6: {}  7: {}\n8: {} 9: {} 10: {} 11: {} 12: {} 13: {} 14: {} 15: {}",
                    note_shades_w[0],
                    note_shades_w[1],
                    note_shades_w[2],
                    note_shades_w[3],
                    note_shades_w[4],
                    note_shades_w[5],
                    note_shades_w[6],
                    note_shades_w[7],
                    note_shades_w[8],
                    note_shades_w[9],
                    note_shades_w[10],
                    note_shades_w[11],
                    note_shades_w[12],
                    note_shades_w[13],
                    note_shades_w[14],
                    note_shades_w[15]
                ).as_bytes()).ok();
                s.queue(cursor::RestorePosition).ok();
                s.flush().ok();
            },
            event::Event::Key(KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: _no_modifiers,
            }) => {
                is_help = !is_help;
                if is_help {
                    write_text(&mut s, 0, 14, "\x1b[4m\x1b[38;2;0;255;0mHelp:\x1b[0m \n→ - Skip ahead by 3 seconds\n p - Pause");
                } else {
                    s.queue(cursor::SavePosition).ok();
                    s.queue(terminal::Clear(ClearType::FromCursorDown)).ok();
                    s.queue(cursor::RestorePosition).ok();
                    s.flush().ok();
                }

            },
            _ => {
                {
                    let mut is_mm = mm.lock().unwrap();
                    *is_mm = false;
                }
            }
        }
    }

    mm_thread.join().unwrap();

    s.queue(terminal::Clear(ClearType::All)).ok();
    s.queue(cursor::MoveTo(0,0)).ok();
    s.flush().ok();
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

    let mut use_colors = true;

    let mut color_index = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
   
    if args.contains(&"-transpose".to_string()) {
        transpose_value = args[args.iter().position(|r| r == "-transpose").unwrap()+1].parse::<i32>().unwrap();
        if transpose_value < 0 {
            println!("\x1b[38;2;255;255;0mTranspose value is below 0, defaulting to no transpose...\x1b[0m");
            transpose_value = 0;
        }
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

    if args.contains(&"-noColors".to_string()) {
        use_colors = false;
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

    println!("Initializing visualizer...");

    let mut num_overlaps: [i32; 128] = [0; 128];

    let keyboard_string: Arc<Mutex<[&str]>> = Arc::new(Mutex::new([" "; 128]));

    let kdmapi = KDMAPI.open_stream();

    //let crossterm = Crossterm::new();

    let mut overlap_colors: Vec<Vec<i32>> = vec![Vec::new(); 128];
    let mut overlap_index: Vec<Vec<i32>> = vec![Vec::new(); 128];

    let now = Instant::now();

    let mut time = 0.0;
    let mut atime = 0.0;

    let midi_ended = Arc::new(Mutex::new(false));
    let skip_length = Arc::new(Mutex::new(0.0));
    let playback_offset = Arc::new(Mutex::new(0.0));
    let paused = Arc::new(Mutex::new(false));

    let keyboard_thread = Arc::clone(&keyboard_string);
    let midi_end = Arc::clone(&midi_ended);
    let skip_len = Arc::clone(&skip_length);
    let play_offset = Arc::clone(&playback_offset);
    let paused_midi = Arc::clone(&paused);

    println!("Done!");

    let audio_thread = thread::spawn(move || {
        for e in amerged {
            if e.delta() != 0.0 {
                {
                    while *paused_midi.lock().unwrap() {
                        thread::sleep(time::Duration::from_secs_f64(0.1));
                    }
                }

                atime += e.delta();
                let diff = atime - ((now.elapsed().as_secs_f64()+(*play_offset.lock().unwrap()))-(*skip_len.lock().unwrap()));
                
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

    let paused_midi = Arc::clone(&paused);
    let play_offset = Arc::clone(&playback_offset);
    let skip_len = Arc::clone(&skip_length);

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

                {
                    while *paused_midi.lock().unwrap() {
                        thread::sleep(time::Duration::from_secs_f64(0.1));
                        let mut skip_l = skip_len.lock().unwrap();
                        *skip_l += 0.1;
                    }
                }

                time += e.delta();
                let diff = time - ((now.elapsed().as_secs_f64()+(*play_offset.lock().unwrap()))-(*skip_len.lock().unwrap()));
                
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
                    let n_idx = (e.channel) as usize;

                    if use_colors {
                        keyboard_string[kb_idx] = note_shades_w[n_idx];
                        if black_note && allow_black_notes {
                            keyboard_string[kb_idx] = note_shades_b[n_idx];
                        }
                    } else {
                        keyboard_string[kb_idx] = ["`",".",",","!","#","&","$","@","`",".",",","!","#","&","$","@"][n_idx];
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
                        let tmp_pos = overlap_colors[kb_idx].iter().position(|&r| r == (e.channel % 16) as i32).unwrap();
                        overlap_colors[kb_idx].remove(tmp_pos);

                        let mut overlap_colors_len = 0;
                        if overlap_colors[kb_idx].len() > 0 {
                            overlap_colors_len = overlap_colors[kb_idx].len() - 1;
                        }

                        if overlap_colors[kb_idx].len() > 0 {
                            let new_n_idx = overlap_colors[kb_idx][(overlap_colors_len) as usize] as usize;
                            if use_colors {
                                keyboard_string[kb_idx] = note_shades_w[new_n_idx];
                                if black_note && allow_black_notes {
                                    keyboard_string[kb_idx] = note_shades_b[new_n_idx];
                                }
                            } else {
                                keyboard_string[kb_idx] = ["`",".",",","!","#","&","$","@","`",".",",","!","#","&","$","@"][new_n_idx];
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
        std::process::exit(0);
    });

    let keyboard_thread = Arc::clone(&keyboard_string);
    let midi_end = Arc::clone(&midi_ended);
    let paused_midi = Arc::clone(&paused);

    let thread_2 = thread::spawn(move || {
        while !(*midi_end.lock().unwrap()) {
            println!("{}", keyboard_thread.lock().unwrap().join(""));
            thread::sleep(time::Duration::from_millis(((note_size as f64)/playback_speed) as u64));
            {
                while *paused_midi.lock().unwrap() {
                    thread::sleep(time::Duration::from_secs_f64(0.1));
                }
            }
        }
    });

    let midi_end = Arc::clone(&midi_ended);
    let paused_midi = Arc::clone(&paused);
    let play_offset = Arc::clone(&playback_offset);

    let keyboard_inputs = thread::spawn(move || {
        while !(*midi_end.lock().unwrap()) {
            match read().unwrap() {
                event::Event::Key(KeyEvent {
                    code: KeyCode::Char('p'),
                    modifiers: _no_modifiers,
                }) => {
                    {
                        let mut psd = paused_midi.lock().unwrap();
                        *psd = !*psd;
                    }
                },
                event::Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    modifiers: _no_modifiers,
                }) => {
                    {
                        let mut play_offs = play_offset.lock().unwrap();
                        *play_offs = *play_offs + 3.0;
                    }
                },
                _ => (),
            }
        }
    });

    audio_thread.join().unwrap();
    thread_1.join().unwrap();
    thread_2.join().unwrap();
    keyboard_inputs.join().unwrap();

    disable_raw_mode().unwrap();
}
