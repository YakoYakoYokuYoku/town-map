extern crate num_complex as nc;
extern crate num_traits as nt;
extern crate sdl2;

use nc::Complex;
use nt::{
    cast::{FromPrimitive, ToPrimitive},
    Float, NumOps, One,
};
use sdl2::{
    event::Event,
    gfx::primitives::DrawRenderer,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};
use std::f64::consts;
mod plot2d;
pub use plot2d::{Circle, Coordinate, PlotMethod, Polar};

pub trait Plot<T>
where
    T: FromPrimitive + NumOps + One + Copy,
{
    fn plotmethod(
        &self,
        canvas: &mut Canvas<Window>,
        scale: T,
        squish: T,
        a: T,
        p: &mut Coordinate,
    );

    #[inline]
    fn plot(&self) -> Result<(), String> {
        let mut a = T::from_f64(0.5).unwrap();
        let (width, height) = (800, 600);
        let mut center = Coordinate { x: 300, y: 0 };

        let sdl_context = sdl2::init()?;
        let video_subsys = sdl_context.video()?;
        let window = video_subsys
            .window("Domain Coloring", width, height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        canvas.set_draw_color(Color::RGB(252, 239, 197));
        canvas.clear();
        canvas.present();

        let mut events = sdl_context.event_pump()?;

        let scaler = T::from_f64(10f64).unwrap();
        let mut scale = T::one();
        let mut squish = T::one();
        let light = T::from_f64(0.01).unwrap();

        Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);

        canvas.present();

        'main: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,

                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => match keycode {
                        Keycode::Escape => break 'main,
                        Keycode::KpPlus => {
                            scale = scale * scaler;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::KpMinus => {
                            scale = scale / scaler;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::L => {
                            a = a - light;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::D => {
                            a = a + light;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::KpMultiply => {
                            squish = squish * scale;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::KpDivide => {
                            squish = squish / scale;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::Up => {
                            center.y -= 10;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::Down => {
                            center.y += 10;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::Left => {
                            center.x -= 10;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::Right => {
                            center.x += 10;
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        Keycode::R => {
                            scale = T::one();
                            a = T::from_f64(0.5).unwrap();
                            squish = T::one();
                            center = Coordinate { x: 300, y: 300 };
                            Plot::plotmethod(self, &mut canvas, scale, squish, a, &mut center);
                            canvas.present();
                        }
                        _ => {}
                    },

                    Event::MouseButtonDown { x, y, .. } => {
                        println!("mouse btn down at ({},{})", x, y);
                    }

                    _ => {}
                }
            }
        }

        Ok(())
    }
}

impl<T, F> Plot<T> for F
where
    T: FromPrimitive + NumOps + One + Copy + Float,
    Complex<T>: PlotMethod<T>,
    F: Fn(Complex<T>) -> Complex<T>,
{
    fn plotmethod(
        &self,
        canvas: &mut Canvas<Window>,
        scale: T,
        squish: T,
        a: T,
        p: &mut Coordinate,
    ) {
        PlotMethod::plottype(self, canvas, scale, squish, a, p)
    }
}

/* impl<T, F> Plot<T> for F
where
    T: FromPrimitive + NumOps + One + Copy + Float + ToPrimitive,
    F: Fn(T, u8) -> Polar<T>,
{
    fn plotmethod(
        &self,
        canvas: &mut Canvas<Window>,
        scale: T,
        squish: T,
        a: T,
        p: &mut Coordinate
    ) {
        PlotMethod::plottype(self, canvas, scale, squish, a, p)
    }
} */
