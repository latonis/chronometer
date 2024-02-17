use slint::{Timer, TimerMode};

slint::slint!{
    import {MainWindow} from "./assets/appwindow.slint";
}

fn main() {
    let mut seconds = 1;
    let mut minutes = 0;
    let mut hours = 0;
    let main_window: MainWindow = MainWindow::new().unwrap();
    let timer = Timer::default();
    let handle = main_window.as_weak();
    let handle_weak: MainWindow = handle.unwrap();

    timer.start(TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
        if seconds % 60 == 0 && seconds != 0 {
            minutes += 1;
            seconds = 0;
            if minutes % 60 == 0  {
                hours += 1;
            }
        }
 
        handle_weak.set_time(format!("{:02}:{:02}:{:02}", hours, minutes, seconds).into());
        handle_weak.window().request_redraw();
        seconds += 1;
    });

    main_window.run().unwrap();
}
