use cursive::align::HAlign;
use cursive::views::{Dialog, DialogFocus};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    let mut dialog = Dialog::text("welcome to lifelog, a log of your uneventful life.")
        .button("entries", |_s| {})
        .button("new entry", |_s| {})
        .button("about", show_about)
        .button("quit", |s| s.quit())
        .h_align(HAlign::Center);

    // focus on 'new entry' button
    dialog.set_focus(DialogFocus::Button(1));

    siv.add_layer(dialog.title("lifelog."));

    siv.run();
}

fn show_about(s: &mut Cursive) {
    s.add_layer(Dialog::info(
        "a simple diary that you can use from your terminal.",
    ));
}
