use rand::{distributions::Uniform, random, Rng};
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
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        // image.set_pixel(self.0, y, color)
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

        loop {
            if current_x >= 0 && current_x < image.width 
                && current_y >= 0 && current_y < image.height 
            {
                image.set_pixel(current_x , current_y, self.color()).unwrap();
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
        // Color::blue()
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.set_pixel(self.0, self.1, self.color()).unwrap();
    }

    fn color(&self) -> Color {
        
        let mut rng = rand::thread_rng();
        let range = Uniform::from(0..255);

        Color::rgb(rng.sample(range), rng.sample(range), rng.sample(range))
    }
}
impl Drawable for Circle {
    fn color(&self) -> Color {
        // let mut rng = rand::thread_rng();
        // let range = Uniform::from(0..255);

        Color::()
    }

    fn draw(&self, image: &mut Image) {
        let Circle(Point(cx, cy), radius) = self;
        let mut x = *radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            // Vérifie si les pixels sont à l'intérieur de l'image avant de les dessiner
            if cx + x < image.width && cy + y < image.height {
                image.set_pixel(cx + x, cy + y, self.color()).unwrap();
            }
            if cx + y < image.width && cy + x < image.height {
                image.set_pixel(cx + y, cy + x, self.color()).unwrap();
            }
            if cx - y >= 0 && cy + x < image.height {
                image.set_pixel(cx - y, cy + x, self.color()).unwrap();
            }
            if cx - x >= 0 && cy + y < image.height {
                image.set_pixel(cx - x, cy + y, self.color()).unwrap();
            }
            if cx - x >= 0 && cy - y >= 0 {
                image.set_pixel(cx - x, cy - y, self.color()).unwrap();
            }
            if cx - y >= 0 && cy - x >= 0 {
                image.set_pixel(cx - y, cy - x, self.color()).unwrap();
            }
            if cx + y < image.width && cy - x >= 0 {
                image.set_pixel(cx + y, cy - x, self.color()).unwrap();
            }
            if cx + x < image.width && cy - y >= 0 {
                image.set_pixel(cx + x, cy - y, self.color()).unwrap();
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
        Circle(point, radius)
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let range_x = Uniform::from(0..width);
        let range_y = Uniform::from(0..height);

        // generate random value
       let center =  Point::new(rng.sample(range_x), rng.sample(range_y));
       let radius = rng.sample(Uniform::from(0..width.min(height)/2)); 
       Circle(center, radius)
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