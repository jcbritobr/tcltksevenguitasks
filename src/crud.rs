use fltk::{
    app::{channel, App, Scheme},
    browser::HoldBrowser,
    button::Button,
    enums::CallbackTrigger,
    input::Input,
    prelude::{BrowserExt, GroupExt, InputExt, WidgetExt},
    window::Window,
};

const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
}

fn main() {
    let app = App::default().with_scheme(Scheme::Gtk);
    let mut win = Window::default().with_label("CRUD");

    let (sender, receiver) = channel::<Message>();

    let mut filter_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
        .with_label("Filter prefix:");
    filter_input.set_trigger(CallbackTrigger::Changed);
    filter_input.emit(sender, Message::Filter);

    let mut list_browser = HoldBrowser::default()
        .with_pos(
            WIDGET_PADDING,
            filter_input.y() + filter_input.height() + WIDGET_PADDING,
        )
        .with_size(WIDGET_WIDTH * 3, WIDGET_HEIGHT * 4);
    list_browser.emit(sender, Message::Select);

    let name_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH,
            list_browser.y(),
        )
        .with_label("Name:");

    let surname_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&name_input, WIDGET_PADDING)
        .with_label("Surname:");

    let mut create_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            WIDGET_PADDING,
            list_browser.y() + list_browser.height() + WIDGET_PADDING,
        )
        .with_label("Create");
    create_button.emit(sender, Message::Create);

    let mut update_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&create_button, WIDGET_PADDING)
        .with_label("Update");
    update_button.emit(sender, Message::Update);
    update_button.deactivate();

    let mut delete_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&update_button, WIDGET_PADDING)
        .with_label("Delete");
    delete_button.emit(sender, Message::Delete);
    delete_button.deactivate();

    let mut model = vec![
        "Babbage, Charles".to_string(),
        "Lovelace, Ada".to_string(),
        "Turing, Alan".to_string(),
    ];
    
    filter(&filter_input, &mut &mut list_browser, &model);

    let formatted_name = || format!("{}, {}", surname_input.value(), name_input.value());
    win.set_size(
        name_input.x() + name_input.width() + WIDGET_PADDING,
        create_button.y() + create_button.height() + WIDGET_PADDING,
    );

    win.end();
    win.show();

    while app.wait() {
        match receiver.recv() {
            Some(Message::Create) => {
                model.push(formatted_name());
                sender.send(Message::Filter);
            }
            Some(Message::Delete) => {
                let selected_name = list_browser.text(list_browser.value()).unwrap();
                let index = model.iter().position(|s| s == &selected_name).unwrap();
                model.remove(index);
                sender.send(Message::Filter);
            }
            Some(Message::Filter) => {
                filter(&filter_input, &mut list_browser, &model);

                sender.send(Message::Select)
            }
            Some(Message::Select) => {
                if list_browser.value() == 0 {
                    update_button.deactivate();
                    delete_button.deactivate();
                } else {
                    update_button.activate();
                    delete_button.activate();
                }
            }
            Some(Message::Update) => {
                let selected_name = list_browser.text(list_browser.value()).unwrap();
                let index = model.iter().position(|s| s == &selected_name).unwrap();
                model[index] = formatted_name();
                sender.send(Message::Filter);
            }
            None => {}
        }
    }
}

fn filter(filter_input: &Input, list_browser: &mut HoldBrowser, model: &Vec<String>) {
    let prefix = filter_input.value().to_lowercase();
    list_browser.clear();
    for item in model {
        if item.to_lowercase().starts_with(&prefix) {
            list_browser.add(item);
        }
    }
}
