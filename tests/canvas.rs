use cucumber::{gherkin::Step, given, then, when, World as _};
use raytracer::canvas::Canvas;
use raytracer::tuples::{color, Tuple};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(cucumber::World, Debug, Default)]
struct World {
    t: HashMap<String, Tuple>,
    c: HashMap<String, Canvas>,
    p: HashMap<String, String>,
}

#[given(expr = "{word} ← canvas\\({int}, {int})")]
async fn make_canvas(world: &mut World, n: String, w: usize, h: usize) {
    let c = Canvas::new(w, h);
    world.c.insert(n, c);
}

#[given(expr = "{word} ← color\\({float}, {float}, {float})")]
async fn make_color(world: &mut World, n: String, r: f64, g: f64, b: f64) {
    world.t.insert(n, color(r, g, b));
}

#[when(expr = "write_pixel\\({word}, {int}, {int}, {word})")]
async fn write_pixel(world: &mut World, n: String, x: usize, y: usize, color_name: String) {
    let c = world.c.get_mut(&n).unwrap();
    let color = world.t.get(&color_name).unwrap();
    c.write_pixel(x, y, *color);
}

#[then(expr = "c.width = {int}")]
async fn canvas_width(world: &mut World, w: usize) {
    let c = world.c.get("c").unwrap();
    assert_eq!(c.width, w);
}

#[then(expr = "c.height = {int}")]
async fn canvas_height(world: &mut World, h: usize) {
    let c = world.c.get("c").unwrap();
    assert_eq!(c.height, h);
}

#[then(expr = "every pixel of {word} is color\\({float}, {float}, {float})")]
async fn check_every_pixel(world: &mut World, n: String, r: f64, g: f64, b: f64) {
    let c = world.c.get(&n).unwrap();
    for p in &c.p {
        assert_eq!(p.x, r);
        assert_eq!(p.y, g);
        assert_eq!(p.z, b);
    }
}

#[then(expr = "pixel_at\\({word}, {int}, {int}) = {word}")]
async fn check_pixel_at(world: &mut World, n: String, x: usize, y: usize, cn: String) {
    let c = world.c.get(&n).unwrap();
    let color = c.pixel_at(x, y);
    let expected_color = world.t.get(&cn).unwrap();
    assert_eq!(color, expected_color);
}

#[when(expr = "{word} ← canvas_to_ppm\\({word})")]
async fn canvas_to_ppm(world: &mut World, n: String, cn: String) {
    let c = world.c.get(&cn).unwrap();
    world.p.insert(n, c.to_ppm());
}

#[then(expr = "lines {int}-{int} of {word} are")]
async fn check_lines(world: &mut World, start: usize, end: usize, n: String, step: &Step) {
    let ppm = world.p.get(&n).unwrap();
    let lines: Vec<&str> = ppm.lines().collect();
    let expected_lines: Vec<&str> = step.docstring().unwrap().lines().skip(1).collect();
    for (i, line) in (start - 1..end).zip(expected_lines) {
        assert_eq!(lines[i], line);
    }
}

#[when(expr = "every pixel of {word} is set to color\\({float}, {float}, {float})")]
async fn set_every_pixel(world: &mut World, n: String, r: f64, g: f64, b: f64) {
    let c = world.c.get_mut(&n).unwrap();
    let color = color(r, g, b);
    for y in 0..c.height {
        for x in 0..c.width {
            c.write_pixel(x, y, color);
        }
    }
}

pub async fn test() {
    World::run("tests/features/canvas.feature").await;
}
