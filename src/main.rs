use cursive::align::HAlign;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, DialogFocus, HideableView, LinearLayout, RadioGroup, TextArea};
use cursive::{Cursive, XY};

mod month_log;
use month_log::MonthLog;

fn main() {
    let mut siv = cursive::default();

    let mut dialog = Dialog::text("welcome to lifelog, a log of your uneventful life.")
        .title("lifelog")
        .button("entries", |_s| {})
        .button("new entry", new_entry)
        .button("about", show_about)
        .button("quit", Cursive::quit)
        .h_align(HAlign::Center);

    // focus on 'new entry' button
    dialog.set_focus(DialogFocus::Button(1));

    siv.add_layer(HideableView::new(dialog).with_name("main"));

    siv.run();
}

// ----------------------------------------------------------------------------
fn hide_main_menu(s: &mut Cursive) {
    s.call_on_name("main", |view: &mut HideableView<Dialog>| {
        view.hide();
    });
}

fn unhide_main_menu(s: &mut Cursive) {
    s.call_on_name("main", |view: &mut HideableView<Dialog>| {
        view.unhide();
    });
}
// ----------------------------------------------------------------------------

// ----------------------------- New Entry Button -----------------------------
fn new_entry(s: &mut Cursive) {
    let month_log = MonthLog::current_month_log();
    hide_main_menu(s);

    if month_log.get_todays_entry().is_default() {
        s.add_layer(
            Dialog::new()
                .title("how was your day?")
                .content(TextArea::new().with_name("diary_entry"))
                .button("Ok", ask_rating)
                .fixed_size(XY { x: 64, y: 20 }),
        );
    } else {
        s.add_layer(
            Dialog::text("you already have an entry for today.").button("Ok", |s| {
                s.pop_layer();
                unhide_main_menu(s);
            }),
        );
    }
}

fn ask_rating(s: &mut Cursive) {
    let text = s
        .call_on_name("diary_entry", |view: &mut TextArea| {
            view.get_content().to_string()
        })
        .unwrap();

    let mut options = RadioGroup::new();
    let mut linear_layout = LinearLayout::vertical();

    let ratings = [
        (2, "+2 (awesome)"),
        (1, "+1"),
        (0, " 0 (okay)"),
        (-1, "-1"),
        (-2, "-2 (horrible)"),
    ];

    for (value, label) in ratings {
        linear_layout.add_child(options.button(value, label));
    }

    s.pop_layer();
    s.add_layer(
        Dialog::new()
            .title("how was your day?")
            .content(linear_layout)
            .button("Save", move |s| {
                let rating = *options.selection();
                save_entry(s, rating, &text);
            }),
    )
}

fn save_entry(s: &mut Cursive, rating: i8, text: &str) {
    let mut month_log = MonthLog::current_month_log();
    month_log.update_todays_entry(rating, text.to_string());
    month_log.save_to_disk();

    s.pop_layer();
    s.add_layer(Dialog::text("entry saved!").button("Ok", |s| {
        s.pop_layer();
        unhide_main_menu(s);
    }));
}
// ----------------------------------------------------------------------------

// ------------------------------- About Button -------------------------------
fn show_about(s: &mut Cursive) {
    s.add_layer(Dialog::info(
        "a simple diary that you can use from your terminal.",
    ));
}
// ----------------------------------------------------------------------------
