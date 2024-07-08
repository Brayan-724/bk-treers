extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let mut frame = 0usize;
    let mut jumping = false;
    let mut falling = false;
    let mut camera: (f32, f32, f32) = (0., 0., -1.);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    repeat: false,
                    ..
                } => camera.0 -= 0.5,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    repeat: false,
                    ..
                } => camera.0 += 0.5,

                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    repeat: false,
                    ..
                } => camera.2 -= 0.5,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    repeat: false,
                    ..
                } => camera.2 += 0.5,

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    repeat: false,
                    ..
                } => jumping = true,

                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        frame += 1;

        if jumping {
            camera.1 -= 0.1;

            if camera.1 <= -3. {
                camera.1 = -3.;
                jumping = false;
                falling = true;
            }
        }

        if falling {
            camera.1 += 0.1;

            if camera.1 >= 0. {
                camera.1 = 0.;
                falling = false;
            }
        }

        let (width, height) = canvas.window().size();

        let width = width / 1;
        let height = height / 1;

        let frame_cycle = (frame % 360) as f32;

        let min_x = -10.;
        let max_x = 10.;
        let min_y = -10.;
        let max_y = 10.;
        let min_z = -10.; // 2. * (frame_cycle.to_radians().sin() + 1.);
        let max_z = 10.; // 3. * (frame_cycle.to_radians().sin() + 1.);

        let cube_position = (0f32, 0f32, (max_z + min_z) / 2.);

        #[rustfmt::skip]
        let cube_vertices: &[(f32, f32, f32)] = &[



            (min_x, max_y, max_z),
            (min_x, min_y, max_z),
            (max_x, min_y, max_z),
            (max_x, max_y, max_z),

            (min_x, max_y, max_z),

            (min_x, max_y, max_z),
            (min_x, max_y, min_z),
            (min_x, min_y, min_z),
            (min_x, min_y, max_z),

            (min_x, min_y, min_z),

            (min_x, max_y, min_z),
            (min_x, min_y, min_z),
            (max_x, min_y, min_z),
            (max_x, max_y, min_z),

            (max_x, max_y, min_z),
            (max_x, max_y, max_z),
            (max_x, min_y, max_z),
            (max_x, min_y, min_z),

            (max_x, min_y, max_z),
            (max_x, max_y, max_z),
            (max_x, max_y, min_z),
            (min_x, max_y, min_z),
        ];

        // let cube_rotation = quaternion::euler_angles(0., 0., 0.);
        let cube_rotation_a = quaternion::euler_angles(0., 0., frame_cycle.to_radians());
        let cube_rotation_b = quaternion::euler_angles(0., frame_cycle.to_radians(), 0.);

        // let rotation: (f32, f32, f32) = (5f32.to_radians(), 0., 15f32.to_radians());
        // let rotation: (f32, f32, f32) = (0., 25f32.to_radians(), 0.);
        let rotation: (f32, f32, f32) = (0., 0., 0.);

        // let mut pb = raqote::PathBuilder::new();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let mut last_point = None;
        for vert in cube_vertices {
            let pos = (vert.0, vert.1, vert.2);

            let mag = (pos.0.powi(2) + pos.1.powi(2) + pos.2.powi(2)).sqrt();

            let pos = [vert.0 / mag, vert.1 / mag, vert.2 / mag];

            let pos = quaternion::rotate_vector(cube_rotation_b, pos);
            let pos = quaternion::rotate_vector(cube_rotation_a, pos);

            // let pos = [
            //     pos[0] * mag,
            //     pos[1] * mag,
            //     pos[2] * mag,
            // ];

            // println!("--- {vert:?}({cube_rotation:?}) -> {pos:?}");

            let mut pos = (pos[0] - camera.0, pos[1] - camera.1, pos[2] - camera.2);

            // if pos.0 <= 1e-4 && pos.0 >= -1e-4 {
            //     pos.0 = 0.;
            // }
            //
            // if pos.1 <= 1e-4 && pos.1 >= -1e-4 {
            //     pos.1 = 0.;
            // }

            if pos.2 <= 0. {
                continue;
            }

            let (sx, cx) = rotation.0.sin_cos();
            let (sy, cy) = rotation.1.sin_cos();
            let (sz, cz) = rotation.2.sin_cos();

            #[rustfmt::skip]
            let d = (
                cy * (sz * pos.1 + cz * pos.0) - sy * pos.2,
                sx * (cy * pos.2 + sy * (sz * pos.1 + cz * pos.0)) + cx * (cz * pos.1 - sz * pos.0),
                cx * (cy * pos.2 + sy * (sz * pos.1 + cz * pos.0)) - sx * (cz * pos.1 - sz * pos.0),
            );

            #[rustfmt::skip]
            let e = (
                width as f32 / 2.,
                height as f32 / 2.,
                110.
            );

            let b = ((e.2 / d.2) * d.0 + e.0, (e.2 / d.2) * d.1 + e.1);

            if let Some(last_point_) = last_point {
                canvas.draw_fline(last_point_, b).unwrap();
                last_point = Some(b);
            } else {
                last_point = Some(b);
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
