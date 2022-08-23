use std::cell::{Ref, RefCell, RefMut};
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

    let theme_file = month_log::data_dir()
        .parent()
        .unwrap()
        .join("theme")
        .join("theme.toml");
    if theme_file.exists() {
        siv.load_theme_file(theme_file)
            .expect("invalid theme.toml file");
    }

    siv.add_global_callback('q', Cursive::quit);

    let mut dialog = Dialog::text("welcome to lifelog, a log of your uneventful life.")
        .title("lifelog")
        .button("entries", show_entries)
        .button("new entry", new_entry)
        .button("about", show_about)
        .button("quit", Cursive::quit)
        .h_align(HAlign::Center);

    // focus on 'new entry' button
    dialog.set_focus(DialogFocus::Button(1));

    siv.add_layer(HideableView::new(dialog).with_name("main"));

    siv.run();
}

// ============================================================================
// ------------------------------ Entries Button ------------------------------
// ============================================================================
fn show_entries(s: &mut Cursive) {
    hide_main_menu(s);

    let month_log = Rc::new(RefCell::new(MonthLog::current_month_log()));

    let mut calendar = CalendarView::<Local, EnglishLocale>::new(Local::today());

    let (earliest_date, latest_date) = calendar::earliest_latest();
    calendar.set_earliest_date(Some(earliest_date));
    calendar.set_latest_date(Some(latest_date));

    let month_log_clone = Rc::clone(&month_log);
    calendar.set_on_select(move |siv: &mut Cursive, date: &Date<Local>| {
        update_preview(siv, date, month_log_clone.borrow_mut());
        update_statistics(siv, month_log_clone.borrow());
    });

    let another_log_clone = Rc::clone(&month_log);
    calendar.set_on_submit(move |siv: &mut Cursive, date: &Date<Local>| {
        let log = another_log_clone.borrow_mut();
        edit_entry(siv, date, log);
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

fn update_preview(s: &mut Cursive, date: &Date<Local>, mut log: RefMut<MonthLog>) {
    s.call_on_name("preview", |view: &mut TextView| {
        let month_year = date.format("%B/%Y").to_string();
        if log.month_year() != month_year {
            *log = MonthLog::get_month_log(&month_year)
        }

        let selected_entry = log.get_entry(date.day());
        view.set_content(selected_entry.to_string());
    });
}

fn update_statistics(s: &mut Cursive, log: Ref<MonthLog>) {
    s.call_on_name("statistics", |view: &mut TextView| {
        view.set_content(log.get_statistics())
    });
}

fn edit_entry(s: &mut Cursive, date: &Date<Local>, log: RefMut<MonthLog>) {
    let day = date.day();
    let day_clone = day;
    let month_year = date.format("%B/%Y").to_string();
    let month_year_clone = month_year.clone();
    let selected_entry = log.get_entry(day);

    let content = if selected_entry.is_default() {
        ""
    } else {
        selected_entry.get_text()
    };

    let mut dialog = Dialog::new()
        .title(date.format("%d %B, %Y").to_string())
        .content(TextArea::new().content(content).with_name("diary_entry"));

    dialog.add_button("Update", move |siv| {
        ask_rating(siv, &month_year, day, "entries".to_string())
    });
    dialog.add_button("Delete", move |siv| {
        let mut log = MonthLog::get_month_log(&month_year_clone);
        log.delete_entry(day_clone);
        log.save_to_disk();
        back_to_entries(siv);
    });
    dialog.add_button("Back", back_to_entries);

    s.pop_layer();
    s.add_layer(dialog.fixed_size(XY { x: 64, y: 20 }))
}

// ============================================================================
// ----------------------------- New Entry Button -----------------------------
// ============================================================================
fn new_entry(s: &mut Cursive) {
    let month_log = MonthLog::current_month_log();
    hide_main_menu(s);

    if month_log.get_todays_entry().is_default() {
        s.add_layer(
            Dialog::new()
                .title("how was your day?")
                .content(TextArea::new().with_name("diary_entry"))
                .button("Ok", |siv| {
                    let today = Local::today();
                    let (month_year, day) = (today.format("%B/%Y").to_string(), today.day());
                    ask_rating(siv, &month_year, day, "main".to_string());
                })
                .button("Cancel", unhide_main_menu)
                .fixed_size(XY { x: 64, y: 20 }),
        );
    } else {
        s.add_layer(
            Dialog::text("you already have an entry for today.").button("Ok", unhide_main_menu),
        );
    }
}

// ============================================================================
// ------------------------------- About Button -------------------------------
// ============================================================================
fn show_about(s: &mut Cursive) {
    s.add_layer(
        Dialog::info(format!(
            "a simple diary that you can use from your terminal.\n\n\
        - this is designed for you to only have 1 entry per day, in which \
        you rate how your day went.\n\
        - you can press <q> anytime to quit the program.\n\
        - all the diary entries are saved in: '{}'\n\
        - you can customize the program by creating your own theme file at: '{}'\n\
        (for more info on customization, check 'https://tinyurl.com/fpc2yau2')",
            month_log::data_dir().to_str().unwrap(),
            month_log::data_dir()
                .parent()
                .unwrap()
                .join("theme")
                .join("theme.toml")
                .to_str()
                .unwrap()
        ))
        .max_width(80),
    );
}

// ============================================================================
// ---------------------------------- Common ----------------------------------
// ============================================================================
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

fn back_to_entries(s: &mut Cursive) {
    s.pop_layer();
    show_entries(s);
}

fn ask_rating(s: &mut Cursive, month_year: &str, day: u32, exit_to: String) {
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
    let month_year = month_year.to_string();

    s.pop_layer();
    s.add_layer(
        Dialog::new()
            .title("how was your day?")
            .content(linear_layout)
            .button("Save", move |siv| {
                let rating = *options.selection();
                save_entry(siv, &month_year, day, rating, &text, &exit_to);
            }),
    )
}

fn save_entry(s: &mut Cursive, month_year: &str, day: u32, rating: i8, text: &str, exit_to: &str) {
    let mut month_log = MonthLog::get_month_log(month_year);
    month_log.update_entry(day, rating, text.to_string());
    month_log.save_to_disk();

    let dialog = match exit_to {
        "main" => Dialog::text("entry saved!").button("Ok", unhide_main_menu),
        "entries" => Dialog::text("entry updated!").button("Ok", back_to_entries),
        _ => panic!("exit_to was not equal to either 'main' or 'entries'."),
    };

    s.pop_layer();
    s.add_layer(dialog);
}
