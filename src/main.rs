use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .size(1280, 720)
        .update(update)
        .run();
}

struct Model {
    _window: window::Id,
    radius: f32,
    ball_amount: u16,
    col: bool,
}

fn model(app: &App) -> Model {
    let _window = app
                            .new_window()
                            .title("Waves")
                            .view(view)
                            .build()
                            .unwrap();

    //Custom Code
    let radius: f32 = 10.0;
    let ball_amount: u16 = 75;
    let col = false;
    Model { _window, radius, ball_amount, col }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //Custom Code
    if _app.keys.down.len() != 0 {
        if _app.keys.down.contains(&Key::Up) {
            _model.ball_amount += 1;
        }
        if _app.keys.down.contains(&Key::Down) && _model.ball_amount >= 2 {
            _model.ball_amount -= 1;
        }
        if _app.keys.down.contains(&Key::Right) {
            _model.radius += 0.01;
        }
        if _app.keys.down.contains(&Key::Left) {
            _model.radius -= 0.01;
        }
        if _app.keys.down.contains(&Key::Space) {
            _model.col = !_model.col;
        }
    }
    _app.main_window().set_title(&format!("Waves: Points {} : Pad {}", _model.ball_amount, _model.radius));
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(if _model.col { BLACK } else { WHITE });

    //Custom Code
    let win = app.window_rect().pad(_model.radius);
    let dist = win.w() / _model.ball_amount as f32;
    for i in 0.._model.ball_amount {
        draw.line()
            .stroke_weight(_model.radius)
            //So if anyone is reading this, I am sorry... There is basically no method to this madness
            //If you are trying to understand this, simply put it takes the x position as input and makes a sin wave of every point
            //hint the name i gave the variable, ball_amount. However to make it animated I assume the time since launch variable
            //I added is phase shifting the entire function. However this is a visual project and I really dont want to 
            //go and figure out the exact maths
            .start(pt2(i as f32 * dist - (win.w() / 2.0), (i as f32 * dist - (win.w() / 2.0) + app.time).cos() * win.h() / 2.0))
            .end(pt2((i as f32 + 1.0) * dist - (win.w() / 2.0), ((i as f32 + 1.0) * dist - (win.w() / 2.0) + app.time).cos() * win.h() / 2.0))
            .color( if _model.col { hsl(i as f32 * dist - (win.w() / 2.0), 1.0, 0.5) } else { hsl(0.0, 0.0, 0.0) });
        }


    draw.to_frame(app, &frame).unwrap();
}