use cucumber::{given, then, when, World as _};
use raytracer::tuples::{point, vector, Tuple};
use std::collections::HashMap;

#[derive(cucumber::World, Debug, Default)]
struct World {
    t: HashMap<String, Tuple>,
}

#[given(expr = "{word} ← tuple\\({float}, {float}, {float}, {float})")]
fn make_tuple(world: &mut World, n: String, x: f64, y: f64, z: f64, w: f64) {
    world.t.insert(
        n,
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        },
    );
}

#[given(expr = "{word} ← point\\({float}, {float}, {float})")]
fn make_point(world: &mut World, n: String, x: f64, y: f64, z: f64) {
    world.t.insert(n, point(x, y, z));
}

#[given(expr = "{word} ← vector\\({float}, {float}, {float})")]
fn make_vector(world: &mut World, n: String, x: f64, y: f64, z: f64) {
    world.t.insert(n, vector(x, y, z));
}

#[then(expr = "{word}.x = {float}")]
fn check_x(world: &mut World, n: String, x: f64) {
    assert!(world.t.get(&n).unwrap().x == x);
}

#[then(expr = "{word}.y = {float}")]
fn check_y(world: &mut World, n: String, y: f64) {
    assert!(world.t.get(&n).unwrap().y == y);
}

#[then(expr = "{word}.z = {float}")]
fn check_z(world: &mut World, n: String, z: f64) {
    assert!(world.t.get(&n).unwrap().z == z);
}

#[then(expr = "{word}.w = {float}")]
fn check_w(world: &mut World, n: String, w: f64) {
    assert!(world.t.get(&n).unwrap().w == w);
}

#[then(expr = "{word} is a point")]
fn check_is_point(world: &mut World, n: String) {
    assert!(world.t.get(&n).unwrap().w == 1.0);
}

#[then(expr = "{word} is not a point")]
fn check_is_not_point(world: &mut World, n: String) {
    assert!(world.t.get(&n).unwrap().w != 1.0);
}

#[then(expr = "{word} is a vector")]
fn check_is_vector(world: &mut World, n: String) {
    assert!(world.t.get(&n).unwrap().w == 0.0);
}

#[then(expr = "{word} = tuple\\({float}, {float}, {float}, {float})")]
fn compare_tuple(world: &mut World, n: String, x: f64, y: f64, z: f64, w: f64) {
    assert!(
        *world.t.get(&n).unwrap()
            == Tuple {
                x: x,
                y: y,
                z: z,
                w: w
            }
    );
}

#[then(expr = "{word} + {word} = tuple\\({float}, {float}, {float}, {float})")]
fn add_and_compare_tuple(
    world: &mut World,
    n1: String,
    n2: String,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let t1 = *world.t.get(&n1).unwrap();
    let t2 = *world.t.get(&n2).unwrap();

    assert!(
        t1 + t2
            == Tuple {
                x: x,
                y: y,
                z: z,
                w: w
            }
    );
}

#[then(expr = "{word} - {word} = vector\\({float}, {float}, {float})")]
fn subtract_and_compare_vector(world: &mut World, n1: String, n2: String, x: f64, y: f64, z: f64) {
    let t1 = *world.t.get(&n1).unwrap();
    let tw = *world.t.get(&n2).unwrap();

    assert!(t1 - tw == vector(x, y, z));
}

#[then(expr = "{word} - {word} = point\\({float}, {float}, {float})")]
fn subtract_and_compare_point(world: &mut World, n1: String, n2: String, x: f64, y: f64, z: f64) {
    let t1 = *world.t.get(&n1).unwrap();
    let t2 = *world.t.get(&n2).unwrap();

    assert!(t1 - t2 == point(x, y, z));
}

#[then(expr = "- {word} = tuple\\({float}, {float}, {float}, {float})")]
fn subtract_and_compare_tuple(world: &mut World, n: String, x: f64, y: f64, z: f64, w: f64) {
    let t = *world.t.get(&n).unwrap();

    assert!(
        -t == Tuple {
            x: x,
            y: y,
            z: z,
            w: w
        }
    );
}

#[then(expr = "{word} * {float} = tuple\\({float}, {float}, {float}, {float})")]
fn multiply_and_compare_tuple(
    world: &mut World,
    n: String,
    f: f64,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
) {
    let t = *world.t.get(&n).unwrap();

    assert!(
        t * f
            == Tuple {
                x: x,
                y: y,
                z: z,
                w: w
            }
    );
}

#[then(expr = "{word} \\/ {float} = tuple\\({float}, {float}, {float}, {float})")]
fn divide_and_compare_tuple(world: &mut World, n: String, f: f64, x: f64, y: f64, z: f64, w: f64) {
    let t = *world.t.get(&n).unwrap();

    assert!(
        t / f
            == Tuple {
                x: x,
                y: y,
                z: z,
                w: w
            }
    );
}

#[then(expr = "magnitude\\({word}) = {float}")]
fn check_magnitude(world: &mut World, n: String, m: f64) {
    let t = *world.t.get(&n).unwrap();

    assert!(t.magnitude() == m);
}

#[then(expr = "magnitude\\({word}) = √{float}")]
fn check_magnitude_sqrt(world: &mut World, n: String, m: f64) {
    let t = *world.t.get(&n).unwrap();

    assert!(t.magnitude() == m.sqrt());
}

#[then(expr = "normalize\\({word}) = vector\\({float}, {float}, {float})")]
fn check_normalize(world: &mut World, n: String, x: f64, y: f64, z: f64) {
    let t = *world.t.get(&n).unwrap();

    assert!(t.normalize() == vector(x, y, z));
}

#[then(
    expr = "normalize\\({word}) = vector\\({int}\\/√{float}, {int}\\/√{float}, {int}\\/√{float})"
)]
fn check_normalize_sqrt(
    world: &mut World,
    n: String,
    x: i32,
    x2: f64,
    y: i32,
    y2: f64,
    z: i32,
    z2: f64,
) {
    let t = *world.t.get(&n).unwrap();

    assert!(
        t.normalize()
            == vector(
                (x as f64) / x2.sqrt(),
                (y as f64) / y2.sqrt(),
                (z as f64) / z2.sqrt()
            )
    );
}

#[when(expr = "{word} ← normalize\\({word})")]
fn normalize(world: &mut World, n: String, m: String) {
    let t = *world.t.get(&m).unwrap();
    world.t.insert(n, t.normalize());
}

#[then(expr = "dot\\({word}, {word}) = {float}")]
fn check_dot(world: &mut World, n1: String, n2: String, d: f64) {
    let t1 = *world.t.get(&n1).unwrap();
    let t2 = *world.t.get(&n2).unwrap();

    assert!(t1.dot(t2) == d);
}

#[then(expr = "cross\\({word}, {word}) = vector\\({float}, {float}, {float})")]
fn check_cross(world: &mut World, n1: String, n2: String, x: f64, y: f64, z: f64) {
    let t1 = *world.t.get(&n1).unwrap();
    let t2 = *world.t.get(&n2).unwrap();

    assert!(t1.cross(t2) == vector(x, y, z));
}

pub async fn test() {
    World::run("tests/features/tuples.feature").await;
}
