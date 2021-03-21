use rustracer::Tuple;

#[derive(Debug)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

#[derive(Debug)]
struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    Projectile{ position, velocity }
}

fn main() {
    let mut p = Projectile {
        position: Tuple::from_point(0.0, 1.0, 0.0),
        velocity: Tuple::from_vector(1.0, 1.0, 0.0).normalize(),
    };

    let e = Environment {
        gravity: Tuple::from_point(0.0, -0.1, 0.0),
        wind: Tuple::from_vector(-0.01, 0.0, 0.0).normalize(),
    };

    let mut count = 0;
    while p.position.y >= 0.0 {
        p = tick(&e, &p);
        count += 1;
        println!("tick {}: {:?}", count, p);
    }
}
