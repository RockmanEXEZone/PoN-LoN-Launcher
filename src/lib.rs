use std::env;
use std::ffi::OsStr;
use std::process::{Command, ExitCode};

pub fn run(game: &str) -> ExitCode {
    let base_path = env::current_exe()
        .map(|mut x| {
            x.pop();
            x
        })
        .unwrap_or(env::current_dir().unwrap_or_default());

    let mut doja_path = base_path.clone();
    doja_path.push("bin");
    doja_path.push("doja.exe");

    let mut game_path = base_path.clone();
    game_path.push(game);

    let mut jar_path = game_path.clone();
    jar_path.push("bin");
    jar_path.push(game);

    let mut jam_path = jar_path.clone();
    jar_path.set_extension("jar");
    jam_path.set_extension("jam");

    let mut sp_path = game_path;
    sp_path.push("sp");
    sp_path.push(game);
    sp_path.set_extension("sp");

    let mut error = false;
    for path in [&doja_path, &jar_path, &jam_path, &sp_path] {
        if !path.exists() {
            let path = path.strip_prefix(&base_path).unwrap_or(&path);
            println!("Not found: {}", path.display());
            error = true;
        }
    }

    if !error {
        println!("Starting the game...");
        if Command::new(doja_path)
            .args([OsStr::new("-i"), &jam_path.into_os_string()])
            .spawn()
            .is_err()
        {
            error = true
        }
    }

    if error {
        println!("The game cannot be started. Check the Readme for setup instructions.");
        println!();
        let _ = Command::new("cmd.exe").arg("/c").arg("pause").status();
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
