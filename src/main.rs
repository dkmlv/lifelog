use cursive::align::HAlign;
use cursive::views::{Dialog, DialogFocus};

fn main() {
    let mut siv = cursive::default();

    let mut dialog = Dialog::text("welcome to lifelog, a log of your uneventful life.")
        .button("entries", |_s| {})
        .button("new entry", |_s| {})
        .button("about", |_s| {})
        .button("quit", |s| s.quit())
        .h_align(HAlign::Center);

    // focus on 'new entry' button
    dialog.set_focus(DialogFocus::Button(1));

    siv.add_layer(dialog.title("lifelog."));

    siv.run();
}
