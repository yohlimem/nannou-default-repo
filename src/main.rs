use nannou::prelude::*;

struct Model {
    _window: window::Id,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();

    Model {
        _window,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    
    draw.to_frame(app, &frame).unwrap();
}
