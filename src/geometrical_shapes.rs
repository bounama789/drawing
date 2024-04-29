use rand::{distributions::Uniform, Rng};
use raster::{Color, Image};

pub struct Point(i32, i32, Option<Color>);
pub struct Triangle(Point, Point, Point, Option<Color>);
pub struct Line(Point, Point, Option<Color>);
pub struct Rectangle(Point, Point, Option<Color>);
pub struct Circle(Point, i32, Option<Color>);

pub struct Pentagon {
    pub side_length: f64,
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&mut self, color: Option<Color>);
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let mut rect = Rectangle(Point(p1.0, p1.1, None), Point(p2.0, p2.1, None), None);
        rect.color(None);
        rect
    }

    pub fn random(width: i32, height: i32) -> Self {
        Rectangle::new(&Point::random(width, height), &Point::random(width, height))
    }

    fn other_point(&self, top_left: &Point, bottom_right: &Point) -> (Point, Point) {
        let top_right = Point(bottom_right.0, top_left.1, None);
        let bottom_left = Point(top_left.0, bottom_right.1, None);
        (top_right, bottom_left)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let color = self.2.as_ref().unwrap();
        let (top_right, bottom_left) = self.other_point(&self.0, &self.1);
        let mut line = Line::new(&self.0, &top_right);
        line.color(Some(color.clone()));
        line.draw(image);

        let mut line = Line::new(&self.0, &bottom_left);
        line.color(Some(color.clone()));
        line.draw(image);

        let mut line = Line::new(&top_right, &self.1);
        line.color(Some(color.clone()));
        line.draw(image);

        let mut line = Line::new(&bottom_left, &self.1);
        line.color(Some(color.clone()));
        line.draw(image);
    }

    fn color(&mut self, color: Option<Color>) {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..255);

        let random_color = Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range));
        self.2 = color.or(Some(random_color));
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        let mut point = Point(x, y, None);
        point.color(None);
        point
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let range_x = Uniform::from(0..width);
        let range_y = Uniform::from(0..height);

        // generate random value
        Point::new(rng.sample(range_x), rng.sample(range_y))
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let color = self.2.as_ref().unwrap();
        image.set_pixel(self.0, self.1, color.clone()).unwrap();
    }

    fn color(&mut self, color: Option<Color>) {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..255);

        let random_color = Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range));
        self.2 = color.or(Some(random_color));
        //todo implement the function
    }
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let mut line = Line(Point(p1.0, p1.1, None), Point(p2.0, p2.1, None), None);
        line.color(None);
        line
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let Point(x1, y1, _) = self.0;
        let Point(x2, y2, _) = self.1;

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut current_x = x1;
        let mut current_y = y1;

        let color = self.2.as_ref().unwrap();

        loop {
            if current_x >= 0
                && current_x < image.width
                && current_y >= 0
                && current_y < image.height
            {
                image
                    .set_pixel(current_x, current_y, color.clone())
                    .unwrap();
            }

            if current_x == x2 && current_y == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                current_x += sx;
            }
            if e2 <= dx {
                err += dx;
                current_y += sy;
            }
        }
    }

    fn color(&mut self, color: Option<Color>) {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..255);

        // let mut line = self.clone();

        let random_color = Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range));
        self.2 = color.or(Some(random_color));
    }
}

impl Triangle {
    // Constructor method to create a new triangle
    pub fn new(p1: &Point, p2: &Point, p3: &Point,  color: Option<Color>) -> Self {
        Triangle(Point(p1.0, p1.1, None), Point(p2.0, p2.1, None), Point(p3.0, p3.1, None), color)
    }

    // Method to generate a random triangle within the given width and height
    pub fn random(width: i32, height: i32, color: Option<Color>) -> Self {
        Triangle::new(&Point::random(width, height), &Point::random(width, height), &Point::random(width, height), color)
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = match &self.3 {
        Some(c) => c.clone(),
        None => {
            let mut rng = rand::thread_rng();
            let range = Uniform::from(0..255);
            Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range))
            }
        };


        let p1 = &self.0;
        let p2 = &self.1;
        let p3 = &self.2;

        let mut line = Line::new(&p1, &p2);
        line.color(Some(color.clone()));
        line.draw(image);

        let mut line = Line::new(&p2, &p3);
        line.color(Some(color.clone()));
        line.draw(image);

        let mut line = Line::new(&p3, &p1);
        line.color(Some(color.clone()));
        line.draw(image);
    }

     fn color(&mut self, color: Option<Color>) {
        self.3 = color;
    }
}

// impl Pentagon {
//     // Constructor method to create a new pentagon
//     pub fn new(side_length: f64) -> Self {
//         Pentagon { side_length }
//     }

//     // Method to draw the pentagon on the given image
//     pub fn draw_pentagon(&self, img: &mut Image, x: i32, y: i32) {
//         // Define the coordinates for the vertices of the pentagon
//         let vertices = [
//             (x as f64, y as f64),
//             ((x + self.side_length as i32) as f64, y as f64),
//             ((x + (self.side_length as f64 * 0.5) as i32) as f64, (y - (self.side_length as f64 * 0.5 * (5.0_f64.sqrt() - 1.0)).round() as i32) as f64),
//             ((x - (self.side_length as f64 * 0.5) as i32) as f64, (y - (self.side_length as f64 * 0.5 * (5.0_f64.sqrt() - 1.0)).round() as i32) as f64),
//             ((x - (self.side_length as f64) as i32) as f64, y as f64),
//         ];

//         // Draw lines between the vertices to form the pentagon
//         for i in 0..vertices.len() {
//             let next_index = (i + 1) % vertices.len();
//             let start = vertices[i];
//             let end = vertices[next_index];
//             img.line(start.0 as i32, start.1 as i32, end.0 as i32, end.1 as i32, Color::black());
//         }
//     }
// }

