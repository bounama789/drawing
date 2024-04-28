use rand::{distributions::Uniform, Rng};
use raster::{Color, Image};

pub struct Point(i32, i32);
pub struct Triangle(Point, Point, Point);
pub struct Line(Point, Point);
pub struct Rectangle(Point, Point);
pub struct Circle(Point, i32);

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Rectangle(Point(p1.0, p1.1), Point(p2.0, p2.1))
    }

    pub fn random(width: i32, height: i32) -> Self {
        Rectangle::new(&Point::random(width, height), &Point::random(width, height))
    }

    fn other_point(&self, top_left: &Point, bottom_right: &Point) -> (Point, Point){
        let top_right = Point(bottom_right.0, top_left.1);
        let bottom_left = Point(top_left.0, bottom_right.1);
        (top_right, bottom_left)
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let (top_right, bottom_left) = self.other_point(&self.0, &self.1);
        // let color = self.color();
        let mut line = Line::new(&self.0, &top_right);
        line.draw(image);
        line = Line::new(&self.0, &bottom_left);
        line.draw(image);
        line = Line::new(&top_right, &self.1);
        line.draw(image);
        line = Line::new(&bottom_left, &self.1);
        line.draw(image);
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..255);

        Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range))

        //todo implement the function
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point(x, y)
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
        image.set_pixel(self.0, self.1, self.color()).unwrap();
    }

    fn color(&self) -> Color {
        //todo implement the function
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..255);

        Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range))
    }
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Line(Point(p1.0, p1.1), Point(p2.0, p2.1))
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let Point(x1, y1) = self.0;
        let Point(x2, y2) = self.1;

        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut current_x = x1;
        let mut current_y = y1;

        let color = &self.color();

        loop {
            if current_x >= 0 && current_x < image.width 
                && current_y >= 0 && current_y < image.height 
            {
                image.set_pixel(current_x , current_y, color.clone()).unwrap();
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

    fn color(&self) -> Color {
        //todo implement the function
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..=255);

        Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range))
    }
}

