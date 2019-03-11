extern crate num_complex as nc;
extern crate num_traits as nt;
extern crate sdl2;

use nc::Complex;
use nt::{
    cast::{FromPrimitive, ToPrimitive},
    Float,
};
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use std::f64::consts;

#[allow(unused_assignments)]
fn hsl_to_rgb(hue: f64, sat: f64, lgh: f64) -> (u8, u8, u8) {
    let (r, g, b) = if sat == 0.0 {
        (lgh, lgh, lgh)
    } else {
        fn hue2rgb(p: f64, q: f64, mut t: f64) -> f64 {
            if t < 0.0 {
                t += 1.0
            };
            if t > 1.0 {
                t -= 1.0
            };
            if t < 1.0 / 6.0 {
                return p + (q - p) * 6.0 * t;
            };
            if t < 1.0 / 2.0 {
                return q;
            };
            if t < 2.0 / 3.0 {
                return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
            };

            p
        }

        let q = if lgh < 0.5 {
            lgh * (1.0 + sat)
        } else {
            lgh + sat - lgh * sat
        };

        let pal = 2.0 * lgh - q;

        (
            hue2rgb(pal, q, hue + 1.0 / 3.0),
            hue2rgb(pal, q, hue),
            hue2rgb(pal, q, hue - 1.0 / 3.0),
        )
    };

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct Polar<T> {
    pub r: T,
    pub theta: T,
}

pub struct Circle {
    pub r: i32,
}

#[inline]
fn drawcircle<T: ToPrimitive>(canvas: &mut Canvas<Window>, p: &mut Coordinate, r: T, color: Color) {
    let r = r.to_i16().unwrap();
    let mut x = r - 1;
    let mut y = 0;
    let px = p.x as i16;
    let py = p.y as i16;
    let mut dx = 1;
    let mut dy = 1;
    let mut err = dx - (r << 1);

    while x >= y {
        let _ = canvas.pixel(px + x, py + y, color);
        let _ = canvas.pixel(px + y, py + x, color);
        let _ = canvas.pixel(px - y, py + x, color);
        let _ = canvas.pixel(px - x, py + y, color);
        let _ = canvas.pixel(px - x, py - y, color);
        let _ = canvas.pixel(px - y, py - x, color);
        let _ = canvas.pixel(px + y, py - x, color);
        let _ = canvas.pixel(px + x, py - y, color);

        if err <= 0 {
            y += 1;
            err += dy;
            dy += 2;
        }

        if err > 0 {
            x -= 1;
            dx += 2;
            err += dx - (r << 1);
        }
    }
}

pub trait PlotMethod<T> {
    fn plottype(
        function: &Fn(Self) -> Self,
        canvas: &mut Canvas<Window>,
        scale: T,
        squish: T,
        a: T,
        center: &mut Coordinate,
    );
}

/* impl<T> PlotMethod<T> for T
where
    T: FromPrimitive + Float,
{
    #[inline]
    fn plottype(
        function: &Fn(T) -> T,
        canvas: &mut Canvas<Window>,
        scale: T,
        squish: T,
        _a: T,
        center: &mut Coordinate
    ) {
        canvas.set_draw_color(Color::RGB(252, 239, 197));
        canvas.clear();
        canvas.present();

        for point in (10..600).step_by(10) {
            let start = Point::new(295, point);
            let end = Point::new(305, point);
            if point % 100 == 0 {
                let start = Point::new(290, point);
                let end = Point::new(310, point);
                canvas.set_draw_color(Color::RGB(61, 53, 50));
                let _ = canvas.draw_line(start, end);
            } else {
                canvas.set_draw_color(Color::RGB(145, 136, 133));
                let _ = canvas.draw_line(start, end);
            }

        }

        for point in (10..600).step_by(10) {
            let start = Point::new(point, 295);
            let end = Point::new(point, 305);
            if point % 100 == 0 {
                let start = Point::new(point, 290);
                let end = Point::new(point, 310);
                canvas.set_draw_color(Color::RGB(61, 53, 50));
                let _ = canvas.draw_line(start, end);
            } else {
                canvas.set_draw_color(Color::RGB(145, 136, 133));
                let _ = canvas.draw_line(start, end);
            }
        }

        let hundred = T::from_i32(100).unwrap();

        let fp = function(T::from_i32(-390).unwrap());
        let mut p = if fp.is_nan() || fp.is_infinite() {
            Point::new(10, 1000)
        } else {
            Point::new(10, (fp * hundred).to_i32().unwrap() + center.y)
        };

        canvas.set_draw_color(Color::RGB(61, 53, 50));

        for point in 11..=590 {
            let variable = T::from_i32(point - center.x).unwrap();
            let image = move || {
                let domain = function(variable * squish / hundred) * scale;
                if domain.is_nan() || domain.is_infinite() {
                    1000
                } else {
                    (domain * hundred).to_i32().unwrap()
                }
            };
            let np = Point::new(point, 300 - image() + center.y);

            if np.y() >= 10 && np.y() <= 590 && p.y() >= 10 && p.y() <= 590 {
                let _ = canvas.draw_line(p, np);
            }

            p = np;
        }

        let p1 = Point::new(10, 300);
        let p2 = Point::new(590, 300);
        let p3 = Point::new(300, 10);
        let p4 = Point::new(300, 590);
        let _ = canvas.draw_line(p1, p2);
        let _ = canvas.draw_line(p3, p4);
        let _ = canvas.draw_rect(Rect::new(10, 10, 581, 581));

        canvas.present();
    }
}
 */
/* impl<T> PlotMethod<T> for Polar<T>
where
    T: Float + FromPrimitive,
{
    fn plottype(
        function: &Fn(T, u8) -> Self,
        canvas: &mut Canvas<Window>,
        _scale: T,
        _squish: T,
        _b: T,
        _center: &mut Coordinate
    ) {
        canvas.set_draw_color(Color::RGB(252, 239, 197));
        canvas.clear();
        canvas.present();

        canvas.set_draw_color(Color::RGB(145, 136, 133));

        let a = 10;
        let sp = T::from_f64(consts::FRAC_PI_6).unwrap();
        let ta = T::from_f64(consts::PI * 2.0).unwrap();
        let ei = T::from_i32(80).unwrap();
        let th = T::from_i32(290).unwrap();
        let ot = T::from_i32(1000).unwrap();

        for i in 0..6 {
            let j = T::from_usize(i).unwrap();
            let te = sp * j;
            let x1 = (te.cos() * th).to_i32().unwrap();
            let y1 = (te.sin() * th).to_i32().unwrap();
            let start = Point::new(x1 + 300, y1 + 300);
            let end = Point::new(-x1 + 300, -y1 + 300);
            let _ = canvas.draw_line(start, end);
        }

        drawcircle(canvas, &mut Coordinate { x: 300, y: 300 }, 80, Color::RGB(145, 136, 133));
        drawcircle(canvas, &mut Coordinate { x: 300, y: 300 }, 160, Color::RGB(145, 136, 133));
        drawcircle(canvas, &mut Coordinate { x: 300, y: 300 }, 240, Color::RGB(145, 136, 133));

        canvas.set_draw_color(Color::RGB(61, 53, 50));
        drawcircle(canvas, &mut Coordinate { x: 300, y: 300 }, 290, Color::RGB(61, 53, 50));
        let _ = canvas.draw_line(Point::new(300, 10), Point::new(300, 590));
        let _ = canvas.draw_line(Point::new(10, 300), Point::new(590, 300));

        let fp = function(T::zero(), 0);
        let p = if fp.r.is_nan() || fp.r.is_infinite() {
            fp
        } else {
            Polar { r: ot, theta: T::zero() }
        };
        let mut rp = p.r * ei;
        let mut p1 = Point::new(
            (rp * p.theta.cos()).to_i32().unwrap() + 300,
            (rp * p.theta.sin()).to_i32().unwrap() + 300
        );

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for turn in 0..(a + 1) {
            for point in 1..1001 {
                let variable = T::from_i32(point).unwrap();
                let image = move || {
                    let domain = function(variable / ot * ta, turn);
                    if domain.r.is_finite() {
                        domain
                    } else {
                        Polar { r: ot, theta: domain.theta }
                    }
                };
                let np = image();

                let rn = np.r * ei;
                let p2 = Point::new(
                    (rn * np.theta.cos()).to_i32().unwrap() + 300,
                    (-rn * np.theta.sin()).to_i32().unwrap() + 300
                );

                if rn.to_i32().unwrap() <= 290 && rp.to_i32().unwrap() <= 290 {
                    let _ = canvas.draw_line(p1, p2);
                }

                rp = rn;
                p1 = p2;
            }
        }
    }
} */

impl<T> PlotMethod<T> for Complex<T>
where
    T: FromPrimitive + Float,
{
    #[inline]
    fn plottype(
        function: &Fn(Complex<T>) -> Complex<T>,
        canvas: &mut Canvas<Window>,
        scale: T,
        squish: T,
        a: T,
        p: &mut Coordinate,
    ) {
        for x in (10..590).step_by(4) {
            for y in (10..590).step_by(4) {
                let mut render = |u: i32, v: i32| {
                    let z = function(Complex::new(
                        T::from_i32(u - p.x).unwrap() / scale,
                        T::from_i32(p.y - v + 300).unwrap() / scale,
                    )) / squish;
                    let lgh = 1.0 - (a.powf(z.norm())).to_f64().unwrap();
                    let mut hue = z.arg().to_f64().unwrap() / consts::PI;

                    if hue >= 0.0 {
                        hue /= 2.0;
                    } else {
                        hue = 1.0 - hue.abs() / 2.0
                    }

                    let (red, green, blue) = hsl_to_rgb(hue, 1.0, lgh);

                    let rect = Rect::new(x as i32, y as i32, 4, 4);
                    canvas.set_draw_color(Color::RGB(red, green, blue));
                    let _ = canvas.draw_rect(rect);
                };

                render(x, y);
            }
        }
    }
}
