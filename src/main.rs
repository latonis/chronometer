use slint::{SharedString, Timer, TimerMode};

slint::slint! {
    import {MainWindow} from "./assets/appwindow.slint";
}

/// Parses the arguments that may contain hours, minutes, and seconds for the timer to start
/// # Examples
/// 
/// ```
/// // this sets a timer for 15 minutes from the command line
/// cargo run -- -m 15
/// ```
fn parse_args(args: Vec<String>) -> Option<i32> {
    let mut seconds = 0;

    if args.len() > 1 {
        for (idx, elem) in args.iter().enumerate() {
            if idx < args.len() - 1 {
                match elem.as_str() {
                    "-h" => {
                        seconds += 3600 * args[idx + 1].parse::<i32>().unwrap();
                    }
                    "-m" => {
                        seconds += 60 * args[idx + 1].parse::<i32>().unwrap();
                    }
                    "-s" => {
                        seconds += args[idx + 1].parse::<i32>().unwrap();
                    }
                    _ => {}
                }
            }
        }
        return Some(seconds);
    }

    None
}

/// Validates that the input string provided by the user is in the format of `HH:MM:SS`
/// If it is, returns the values parsed as i32
fn validate_time(input: SharedString) -> Option<(i32, i32, i32)> {
    let elems: std::str::Split<'_, &str> = input.split(":");
    if elems.clone().count() == 3 as usize {
        let mut iterator = elems.map(|s| s.parse::<i32>().unwrap());
        let hours = iterator.next().unwrap();
        let minutes = iterator.next().unwrap();
        let seconds = iterator.next().unwrap();
        return Some((hours, seconds, minutes));
    }

    None
}

/// Main driver and control flow for the Slint application as well as the argument parsing
/// and validation
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let main_window: MainWindow = MainWindow::new().unwrap();
    let timer = Timer::default();
    let handle = main_window.as_weak();
    let handle_weak: MainWindow = handle.unwrap();

    if let Some(arg_seconds) = parse_args(args) {
        if arg_seconds > 0 {
            let main_window = handle.upgrade().unwrap();
            main_window.set_started(true);
            main_window.set_button_text("Timer started!".into());
            main_window.set_seconds(arg_seconds as i32);
            handle_weak.set_time(
                format!(
                    "{:02}:{:02}:{:02}",
                    arg_seconds / 3600,
                    arg_seconds / 60 % 60,
                    arg_seconds % 60
                )
                .into(),
            );
        }
    }

    main_window.on_button_pressed(move || {
        let main_window = handle.upgrade().unwrap();
        if let Some((hours, sec, minutes)) = validate_time(main_window.get_time()) {
            main_window.set_started(true);
            main_window.set_button_text("Timer started!".into());
            main_window.set_seconds((3600 * hours) + (60 * minutes) + sec);
        }
    });

    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_secs(1),
        move || {
            if handle_weak.get_started() {
                let mut seconds = handle_weak.get_seconds();
                if seconds == 0 {
                    handle_weak.set_button_text("Timer finished!".into());
                    handle_weak.set_started(false);
                } else {
                    seconds -= 1;
                }

                handle_weak.set_seconds(seconds);
                handle_weak.set_time(
                    format!(
                        "{:02}:{:02}:{:02}",
                        seconds / 3600,
                        seconds / 60 % 60,
                        seconds % 60
                    )
                    .into(),
                );
                handle_weak.window().request_redraw();
            }
        },
    );

    main_window.run().unwrap();
}
