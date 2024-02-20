use slint::{SharedString, Timer, TimerMode};

slint::slint! {
    import {MainWindow} from "./assets/appwindow.slint";
}

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

fn main() {
    let main_window: MainWindow = MainWindow::new().unwrap();
    let timer = Timer::default();
    let handle = main_window.as_weak();
    let handle_weak: MainWindow = handle.unwrap();

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
                handle_weak.set_time(format!("{:02}:{:02}:{:02}", seconds / 3600, seconds / 60, seconds % 60).into());
                handle_weak.window().request_redraw();
            }
        },
    );

    main_window.run().unwrap();
}
