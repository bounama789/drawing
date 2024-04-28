use std::ops::Div;

use rand::{distributions::Uniform, Rng};
use raster::{Color, Image};

pub struct Point(i32, i32, Option<Color>);
pub struct Triangle(Point, Point, Point, Option<Color>);
pub struct Line(Point, Point, Option<Color>);
pub struct Rectangle(Point, Point, Option<Color>);
pub struct Circle(Point, i32, Option<Color>);

pub struct Cube(Point,Point,Option<Color>);

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

    pub fn distance(&self, point: &Point) -> f64 {
        let (dx, dy) = ((point.0 - self.0)as f64, (point.1 - self.1)as f64);
        (dx * dx + dy * dy).sqrt()
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
impl Drawable for Circle {
    fn color(&mut self, color: Option<Color>)  {
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..255);

        let random_color = Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range));
        self.2 = color.or(Some(random_color))
    }

    fn draw(&self, image: &mut Image) {
        let Circle(Point(cx, cy,_), radius, _) = self;
        let mut x = *radius;
        let mut y = 0;
        let mut err = 0;
        let color = self.2.as_ref().unwrap();

        while x >= y {
            // Vérifie si les pixels sont à l'intérieur de l'image avant de les dessiner
            if cx + x < image.width && cy + y < image.height {
                image.set_pixel(cx + x, cy + y, color.clone()).unwrap();
            }
            if cx + y < image.width && cy + x < image.height {
                image.set_pixel(cx + y, cy + x, color.clone()).unwrap();
            }
            if cx - y >= 0 && cy + x < image.height {
                image.set_pixel(cx - y, cy + x, color.clone()).unwrap();
            }
            if cx - x >= 0 && cy + y < image.height {
                image.set_pixel(cx - x, cy + y, color.clone()).unwrap();
            }
            if cx - x >= 0 && cy - y >= 0 {
                image.set_pixel(cx - x, cy - y, color.clone()).unwrap();
            }
            if cx - y >= 0 && cy - x >= 0 {
                image.set_pixel(cx - y, cy - x, color.clone()).unwrap();
            }
            if cx + y < image.width && cy - x >= 0 {
                image.set_pixel(cx + y, cy - x, color.clone()).unwrap();
            }
            if cx + x < image.width && cy - y >= 0 {
                image.set_pixel(cx + x, cy - y, color.clone()).unwrap();
            }

            y += 1;
            err += 1 + 2*y;
            if 2*(err - x) + 1 > 0 {
                x -= 1;
                err += 1 - 2*x;
            }
        }
    }
}
impl Circle {
    pub fn new(point : Point, radius: i32) -> Self {
        let mut  c = Circle(point, radius, None);
        c.color(None);
        c
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let range_x = Uniform::from(0..width);
        let range_y = Uniform::from(0..height);

        // generate random value
       let center =  Point::new(rng.sample(range_x), rng.sample(range_y));
       let radius = rng.sample(Uniform::from(0..width.min(height)/2)); 
       Circle::new(center, radius)}

    }
impl Cube {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let mut cube = Cube(Point(p1.0, p1.1, None), Point(p2.0, p2.1, None), None);
        cube.color(None);
        cube
    }

    pub fn random(width: i32, height: i32) -> Self {
        Cube::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        let color = self.2.as_ref().unwrap();
        let mut first_rect = Rectangle::new(&self.0, &self.1);
        first_rect.color(Some(color.clone()));
        first_rect.draw(image);
        let top_left = &first_rect.0;
        let bottom_right = &first_rect.1;
        let (top_right, bottom_left) = first_rect.other_point(&top_left, &bottom_right);
        let m =  top_left.distance(&top_right).div(2.0) as i32;

        let x = top_left.0 + m;
        let y = top_left.1 + m;

        let top_left1 = Point(x,y,None);
        
        let x1 = top_right.0+ m;
        let y1 = bottom_right.1 +m;

        let bottom_right1 = Point(x1,y1,None);
        let mut second_rect = Rectangle::new(&top_left1, &bottom_right1);

        second_rect.color(Some(color.clone()));
        second_rect.draw(image);

        let (top_right1, bottom_left1) = second_rect.other_point(&top_left1, &bottom_right1);


       let mut line = Line::new(&top_left1, &top_left);
       line.color(Some(color.clone()));
       line.draw(image);

       let mut line = Line::new(&bottom_left, &bottom_left1);
       line.color(Some(color.clone()));
       line.draw(image);

       let mut line = Line::new(&top_right, &top_right1);
       line.color(Some(color.clone()));
       line.draw(image);

       let mut line = Line::new(&bottom_right, &bottom_right1);
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
