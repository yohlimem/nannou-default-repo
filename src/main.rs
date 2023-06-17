use nannou::prelude::*;
use rand::Rng;


const STEP:f32 = 0.000001;
const STEPS:u32 = 100;

struct Model {
    _window: window::Id,
    points: Vec<Vec2>,
    graph: Vec<Vec2>,
    residuals: Vec<f32>,
    error: f32,
    steps_gone: u32,
    m: f32,
    b: f32,
}

/*
    1. take the derivative of the loss function (the residual) for each parameter in it
    2. pick random values for the parameters
    3. plug in the parameter values into the derivatives <----------------------------------------------- 
    4. calculate the step size: stepSize = Slope * learning rate                                         |
    5. calculate the new parameters by taking they difference: new parameter = old parameter - step size | repeater until you reached a very small number or step limit
*/

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    let mut rng = rand::thread_rng();
    let points:Vec<Vec2> =  (0..20).map(|_v3| vec2(rng.clone().gen_range(-0.0..200.0), rng.gen_range(-10.0..200.0))).collect();
    // let points:Vec<Vec2> =  (-20..20).map(|v3| vec2((v3 * 10) as f32, 0.0)).collect();
    let graph:Vec<Vec2> = (-200..200).map(|p| vec2(p as f32, line_func(p as f32, 10.0, 2.0))).collect();

    let residuals:Vec<f32> = points.iter().map(|p| residual_line(*p, 0.0, 0.0)).collect();
    let error = error(&residuals);

    let mut average_pos:Vec2 = points.iter().sum();
    println!("{}", error);

    Model {
        _window,
        points,
        graph,
        residuals,
        error,
        steps_gone: 0,
        m: 0.0,
        b: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    // show the graph
    let graph:Vec<Vec2> = (-400..400).map(|p| vec2(p as f32,line_func(p as f32, model.m, model.b))).collect();
    if app.mouse.buttons.left().is_down() && !model.points.contains(&app.mouse.position()) {
        model.points.push(app.mouse.position());
        model.steps_gone = 0;
    }
    // the distance between every point and the line (the predicted value)
    model.residuals = model.points.iter().map(|p| residual_line(*p, model.m, model.b)).collect();

    // find how much the new intercept and slopes should "move"
    let new_line = slopes(&model.points, &model.residuals);
    // apply that only a number of times
    if model.steps_gone <= STEPS {
        model.m += new_line.0 * STEP;
        model.b += new_line.1 * STEP;
        model.steps_gone += 1;
        println!("slope:{}, intercept:{}, stepsGone:{}", new_line.0, new_line.1, model.steps_gone);
    }

    // println!("m: {}, b:{}, error:{}", model.m, model.b, error(&model.residuals));




    model.graph = graph;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();


    draw.background().color(WHITE);

    for p in 0..model.points.len() {
        draw.ellipse().color(RED).radius(10.0).xy(model.points[p]);
        draw.line().start(model.points[p]).end(vec2(model.points[p].x, residual_line(model.points[p], model.m, model.b) + model.points[p].y));
    }
    for p in 1..model.graph.len() - 1 {
        draw.line().start(model.graph[p - 1]).end(model.graph[p]);
    }

    draw.ellipse().radius(10.0).color(GREENYELLOW).x_y(model.m, model.b);
    draw.to_frame(app, &frame).unwrap();

}
// draw a function
fn f(x: f32) -> f32{
    x*x+x
}

fn line_func(x: f32, m: f32, b: f32) -> f32{
    m*x+b
}


// find the derivative of a square equation
fn square_derivative(point: Vec2, m: f32, b: f32, respect_to: bool) -> f32{
    if respect_to {
        -2.0 * (point.y - line_func(point.x, m, b))
    }
    else if !respect_to {
        -2.0 * (point.y - line_func(point.x, m, b)) * point.x
    }
    else{
        0.0
    }
}

// find the derivative using the residual
fn residual_derivative(point: &Vec2, residual: &f32, respect_to: bool) -> f32{
    // b
    if respect_to {
        -2.0 * residual
    }
    // m
    else if !respect_to {
        -2.0 * residual * point.x
    }
    else{
        0.0
    }
}


// distance of point from line
fn residual_line(point:Vec2, m:f32,b:f32) -> f32{
   -(point.y - line_func(point.x, m, b))
}

// fn residual(point:Vec2)/


// calculate the error of the line from the points
fn error(residuals: &Vec<f32>) -> f32{
    let mut num = 0.0;

    for i in residuals{
        num += i.pow(2);
    }

    num
}



// calculate the slope of the function of all of the derivatives of the residuals squared
fn slopes(points: &Vec<Vec2>, residuals: &Vec<f32>) -> (f32, f32){
    // b
    let mut intercept = 0.0;
    // m
    let mut slope = 1.0;
    for (point, residual) in points.iter().zip(residuals) {
        intercept += residual_derivative(point, residual, true);
        slope += residual_derivative(point, residual, false);
    }
    // m, b
    (slope, intercept)

}


