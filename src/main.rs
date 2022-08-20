use std::cell::RefCell;
use std::rc::Rc;

use chrono::{Date, Datelike, Local};
use cursive::align::HAlign;
use cursive::event::Key;
use cursive::view::{Nameable, Resizable};
use cursive::views::{
    Dialog, DialogFocus, HideableView, LinearLayout, OnEventView, RadioGroup, ScrollView, TextArea,
    TextView,
};
use cursive::{Cursive, XY};
use cursive_calendar_view::{CalendarView, EnglishLocale};

mod calendar;
mod month_log;

use month_log::MonthLog;

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', Cursive::quit);

    let mut dialog = Dialog::text("welcome to lifelog, a log of your uneventful life.")
        .title("lifelog")
        .button("entries", entries)
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
    s.pop_layer();
    s.call_on_name("main", |view: &mut HideableView<Dialog>| {
        view.unhide();
    });
}
// ----------------------------------------------------------------------------

// ------------------------------ Entries Button ------------------------------
fn entries(s: &mut Cursive) {
    hide_main_menu(s);

    let month_log = Rc::new(RefCell::new(MonthLog::current_month_log()));
    let month_log_clone = Rc::clone(&month_log);

    let mut calendar = CalendarView::<Local, EnglishLocale>::new(Local::today());

    let (earliest_date, latest_date) = calendar::earliest_latest();
    calendar.set_earliest_date(Some(earliest_date));
    calendar.set_latest_date(Some(latest_date));
    calendar.set_on_select(move |siv: &mut Cursive, date: &Date<Local>| {
        siv.call_on_name("preview", |view: &mut TextView| {
            let mut log = month_log_clone.borrow_mut();

            let month_year = date.format("%B/%Y").to_string();
            if log.month_year() != month_year {
                *log = MonthLog::get_month_log(&month_year)
            }

            let selected_entry = log.get_entry(date.day() as usize);
            view.set_content(selected_entry.to_string());
        });
        siv.call_on_name("statistics", |view: &mut TextView| {
            view.set_content(month_log_clone.borrow().get_statistics())
        });
    });

    let log = month_log.borrow();
    let today_entry = log.get_todays_entry();
    let preview = Dialog::around(ScrollView::new(
        TextView::new(today_entry.to_string()).with_name("preview"),
    ))
    .title("preview")
    .fixed_size(XY { x: 64, y: 20 });

    let calendar = Dialog::around(calendar).title("select date");
    let statistics = TextView::new(log.get_statistics()).with_name("statistics");
    let column = LinearLayout::vertical()
        .child(OnEventView::new(calendar).on_event(Key::Esc, unhide_main_menu))
        .child(TextView::new(" press <ESC> to go back."))
        .child(Dialog::around(statistics));

    let layout = LinearLayout::horizontal().child(column).child(preview);
    s.add_layer(layout);
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
                .button("Cancel", unhide_main_menu)
                .fixed_size(XY { x: 64, y: 20 }),
        );
    } else {
        s.add_layer(
            Dialog::text("you already have an entry for today.").button("Ok", unhide_main_menu),
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
    s.add_layer(Dialog::text("entry saved!").button("Ok", unhide_main_menu));
}
// ----------------------------------------------------------------------------

// ------------------------------- About Button -------------------------------
fn show_about(s: &mut Cursive) {
    s.add_layer(Dialog::info(
        "a simple diary that you can use from your terminal.",
    ));
}
// ----------------------------------------------------------------------------
