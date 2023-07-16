use clap::{Arg, App};

fn main() {
    let matches = App::new("greeting")
        .arg(
            Arg::with_name("frist_player").
            index(1).
            required(false)
        ).arg(
            Arg::with_name("second_player")
            .index(2)
            .required(false)
        )
        .setting(clap::AppSettings::AllowExternalSubcommands)
        .get_matches();

    let first_player = matches.value_of("frist_player").unwrap_or("N/A");
    let second_player = matches.value_of("second_player").unwrap_or("N/A");

    println!("Player 1: {}\nPlayer 2: {}", first_player, second_player);
}