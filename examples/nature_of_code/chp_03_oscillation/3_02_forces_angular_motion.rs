// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Exercise 3-02: Forces Angular Motion
extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model, event, view).run();
}

// A type for a draggable attractive body in our world
struct Attractor {
    mass: f32,             // Mass, tied to size
    location: Point2,      // Location
    g: f32,                // Gravity
}

impl Attractor {
    fn new(rect: Rect) -> Self {
        let location = rect.xy();
        let mass = 20.0;
        let g = 0.4;
        Attractor { location, mass, g }
    }

    fn attract(&self, m: &Mover) -> Vector2<f32> {
        let mut force = self.location - m.location; // Calculate direction of force
        let mut distance = force.magnitude(); // Distance between objects
        distance = distance.max(5.0).min(25.0); // Limiting the distance to eliminate "extreme" results for very cose or very far object
        force = force.normalize(); // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let strength = (self.g * self.mass * m.mass) / (distance * distance); // Calculate gravitational force magnitude
        force * strength // Get force vector --> magnitude * direction
    }

    // Method to display
    fn display(&self, draw: &app::Draw) {
        draw.ellipse()
            .x_y(self.location.x, self.location.y)
            .w_h(48.0, 48.0)
            .rgb(0.5, 0.5, 0.5);
    }
}

struct Mover {
    location: Point2,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
    mass: f32,
    angle: f32,
    a_velocity: f32,
    a_acceleration: f32,
}

impl Mover {
    fn new(m: f32, x: f32, y: f32) -> Self {
        let mass = m;
        let location = pt2(x, y);
        let velocity = vec2(random_f32() * 2.0 - 1.0, random_f32() * 2.0 - 1.0);
        let acceleration = vec2(0.0, 0.0);
        let angle = 0.0;
        let a_velocity = 0.0;
        let a_acceleration = 0.0;
        Mover {
            mass,
            location,
            velocity,
            acceleration,
            angle,
            a_velocity,
            a_acceleration,
        }
    }

    fn apply_force(&mut self, force: Vector2<f32>) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.location += self.velocity;

        self.a_acceleration = self.acceleration.x / 10.0;
        self.a_velocity += self.a_acceleration;
        self.a_velocity = clamp(self.a_velocity, -0.1, 0.1);
        self.angle += self.a_velocity;

        self.acceleration *= 0.0;
    }

    fn display(&self, draw: &app::Draw) {
        draw.rect()
            .x_y(self.location.x, self.location.y)
            .w_h(self.mass * 16.0, self.mass * 16.0)
            .rgba(0.6, 0.6, 0.6, 0.78)
            .rotate(self.angle);
    }
}

struct Model {
    movers: Vec<Mover>,
    attractor: Attractor,
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(800.0, 200.0);
    let _window = app
        .new_window()
        .with_dimensions(rect.w() as u32, rect.h() as u32)
        .build()
        .unwrap();

    let movers = (0..20)
        .map(|_| {
            Mover::new(
                random_range(0.1f32, 2.0),
                random_range(rect.left(), rect.right()),
                random_range(rect.top(), rect.bottom()),
            )
        })
        .collect();

    let attractor = Attractor::new(rect);

    Model { movers, attractor }
}

fn event(_app: &App, mut m: Model, event: Event) -> Model {
    // update gets called just before view every frame
    if let Event::Update(_update) = event {
        for i in 0..m.movers.len() {
            let force = m.attractor.attract(&m.movers[i]);
            m.movers[i].apply_force(force);
            m.movers[i].update();
        }
    }
    m
}

fn view(app: &App, m: &Model, frame: Frame) -> Frame {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    m.attractor.display(&draw);

    // Draw movers
    for mover in &m.movers {
        mover.display(&draw);
    }

    // Write the result of our drawing to the window's OpenGL frame.
    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}