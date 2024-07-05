use std::collections::VecDeque;
use std::num::NonZeroU32;
use std::rc::Rc;

use flo_canvas::{Color, GraphicsContext, GraphicsPrimitives};
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{ElementState, KeyEvent, MouseScrollDelta, TouchPhase},
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    event_loop::EventLoopWindowTarget,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

#[derive(Debug, Clone)]
pub enum MainMessage {
    Move(i32, i32),
    Resize((u32, u32)),
    ZoomIn,
    ZoomOut,
    AltKey(bool),
}

#[derive(Debug, Clone)]
pub enum Command {
    None,
    Resize(i32, i32),
}

fn create_window(ev: &EventLoopWindowTarget<()>) -> Window {
    let w = winit::window::WindowBuilder::new()
        .with_title("Zoomer")
        .with_active(true)
        .with_resizable(false)
        .with_decorations(false)
        .with_inner_size(PhysicalSize::new(300, 300))
        // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)))
        // .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
        .build(ev)
        .unwrap();
    // w.set_cursor_icon(winit::window::CursorIcon::Crosshair);
    w
}

fn main() -> Result<(), winit::error::EventLoopError> {
    let mut canvas = flo_render::initialize_offscreen_rendering().unwrap();

    let mut messages = VecDeque::new();

    let event_loop = EventLoop::new().unwrap();
    let mut close_requested = false;

    let mut window: Option<Rc<Window>> = None;
    let mut context = None;
    let mut surface = None;

    let mut frame = 0usize;
    let mut camera: (f32, f32, f32) = (0., 0., -1.);

    event_loop.run(move |event, event_loop| {
        match event {
            Event::Resumed => {
                window = Some(Rc::new(create_window(&event_loop)));
                ()
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => close_requested = true,
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::KeyA),
                            ..
                        },
                    ..
                } => camera.0 -= 1.,
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::KeyD),
                            ..
                        },
                    ..
                } => camera.0 += 1.,
                WindowEvent::CursorMoved {
                    position: PhysicalPosition { x, y },
                    ..
                } => {
                    messages.push_back(MainMessage::Move(x as i32, y as i32));
                }
                WindowEvent::Resized(PhysicalSize { width, height }) => {
                    if let Some(window) = window.clone() {
                        let context = context
                            .get_or_insert(softbuffer::Context::new(window.clone()).unwrap());
                        let surface = surface.get_or_insert(
                            softbuffer::Surface::new(&context, window.clone()).unwrap(),
                        );
                        surface
                            .resize(
                                NonZeroU32::new(width).unwrap(),
                                NonZeroU32::new(height).unwrap(),
                            )
                            .unwrap();
                        messages.push_back(MainMessage::Resize((width, height)));
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = window.clone() {
                        frame += 1;
                        // send messages
                        // while let Some(msg) = messages.pop_front().as_ref() {
                        //     process_cmd(&window, &app.update(msg));
                        // }
                        let context = context
                            .get_or_insert(softbuffer::Context::new(window.clone()).unwrap());
                        let surface = surface.get_or_insert(
                            softbuffer::Surface::new(&context, window.clone()).unwrap(),
                        );

                        let PhysicalSize { width, height } = window.inner_size();
                        surface
                            .resize(width.try_into().unwrap(), height.try_into().unwrap())
                            .unwrap();

                        let width = width / 1;
                        let height = height / 1;

                        // Render
                        window.pre_present_notify();
                        let mut buffer = surface.buffer_mut().unwrap();
                        // let mut dt = raqote::DrawTarget::from_backing(width as i32, height as i32, buffer.as_mut());
                        // dt.fill_rect(0.,0., width as f32, height as f32, &raqote::Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)), &DrawOptions::new());
                        let mut draw_buffer = vec![];
                        
                        draw_buffer.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));

                        // Set up the canvas
                        draw_buffer.canvas_height(height as f32);
                        draw_buffer.center_region(0.0, 0.0, width as f32, height as f32);


                        // for y in 0..height {
                        //     for x in 0..width {
                        //         let c = 10;
                        //
                        //         let (r, g, b, a) = (c, c, c, 255);
                        //         buffer[y as usize * width as usize + x as usize] = (a as u32) << 24
                        //             | (r as u32) << 16
                        //             | (g as u32) << 8
                        //             | b as u32;
                        //     }
                        // }
                        let frame_cycle = (frame % 360) as f32;

                        let min_x = -10.;
                        let max_x = 10.;
                        let min_y = -10.;
                        let max_y = 10.;
                        let min_z = -10. ; // 2. * (frame_cycle.to_radians().sin() + 1.);
                        let max_z = 10. ; // 3. * (frame_cycle.to_radians().sin() + 1.);

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
                        draw_buffer.fill_color(flo_canvas::Color::Rgba(0., 0., 0., 0.));
                        draw_buffer.rect(0., 0., width as f32, height as f32);
                        draw_buffer.fill();

                        draw_buffer.new_path();
                        for (idx, vert) in cube_vertices.iter().enumerate() {
                            let pos = (
                                vert.0 ,
                                vert.1 ,
                                vert.2,
                            );

                            let mag = (pos.0.powi(2) + pos.1.powi(2) + pos.2.powi(2)).sqrt();

                            let pos = [
                                vert.0/mag,
                                vert.1/mag,
                                vert.2/mag,
                            ];

                            let pos = quaternion::rotate_vector(cube_rotation_b, pos);
                            let pos = quaternion::rotate_vector(cube_rotation_a, pos);

                            // let pos = [
                            //     pos[0] * mag,
                            //     pos[1] * mag,
                            //     pos[2] * mag,
                            // ];

                            // println!("--- {vert:?}({cube_rotation:?}) -> {pos:?}");

                            let mut pos = (
                                pos[0] - camera.0,
                                pos[1] - camera.1,
                                pos[2] - camera.2,
                            );

                            // if pos.0 <= 1e-4 && pos.0 >= -1e-4 {
                            //     pos.0 = 0.;
                            // }
                            //
                            // if pos.1 <= 1e-4 && pos.1 >= -1e-4 {
                            //     pos.1 = 0.;
                            // }

                            // if pos.2 <= 1e-2 {
                            //     pos.2 = 0.01;
                            // }

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
                                40.
                            );

                            let b = (
                                (e.2 / d.2) * d.0 + e.0,
                                (e.2 / d.2) * d.1 + e.1
                            );

                            if idx == 0 {
                                draw_buffer.move_to(b.0, b.1);
                                // println!("----------{e:?}");
                            } else {
                                draw_buffer.line_to(b.0, b.1);
                            }
                            // println!("{pos:?} | {b:?} | {d:?}");
                        }
                        // pb.close();
                        // let path = pb.finish();

                        draw_buffer.stroke_color(flo_canvas::Color::Rgba(1., 1., 1., 1.));
                        draw_buffer.line_width(2.);
                        draw_buffer.stroke();

                        // dt.stroke(
                        //     &path,
                        //     &raqote::Source::Solid(raqote::SolidSource::from_unpremultiplied_argb(0xFF, 0xFF, 0xFF, 0xFF)),
                        //     &raqote::StrokeStyle {
                        //         width: 1.,
                        //         ..Default::default()
                        //     },
                        //     &raqote::DrawOptions::new(),
                        // );

                        let buffer_render = futures::executor::block_on(
                            flo_render_canvas::render_canvas_offscreen(
                                &mut canvas, width as usize, height as usize, 1., futures::stream::iter(draw_buffer)
                            )
                        );

                        for i in 0..width as usize * height as usize {
                            let ib = i * 4;
                            let a = buffer_render[ib];
                            let r = buffer_render[ib + 1];
                            let g = buffer_render[ib + 2];
                            let b = buffer_render[ib + 3];

                            // if !matches!((a, r, g, b), (_, 0, 0, 0)) {
                            //     // println!("{r}, {g}, {b}");
                            // }
                            // let i = i * 9;
                            let c = (a as u32) << 24
                                | (r as u32) << 16
                                | (g as u32) << 8
                                | b as u32;

                            buffer[i + 0] = c;
                            // buffer[i + 1] = c;
                            // buffer[i + 2] = c;
                            // buffer[i + 3] = c;
                            // buffer[i + 4] = c;
                            // buffer[i + 5] = c;
                            // buffer[i + 6] = c;
                            // buffer[i + 7] = c;
                            // buffer[i + 8] = c;
                        }

                        buffer.present().unwrap();
                    }
                }
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, y),
                    phase: TouchPhase::Moved,
                    ..
                } => {
                    if y < 0. {
                        messages.push_back(MainMessage::ZoomIn);
                    } else {
                        messages.push_back(MainMessage::ZoomOut);
                    }
                }
                WindowEvent::TouchpadMagnify {
                    delta,
                    phase: TouchPhase::Moved,
                    ..
                } => {
                    if delta < 0. {
                        messages.push_back(MainMessage::ZoomIn);
                    } else {
                        messages.push_back(MainMessage::ZoomOut);
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                if close_requested {
                    event_loop.exit();
                } else {
                    if let Some(window) = window.as_ref() {
                        window.request_redraw();
                    }
                }
            }
            _ => (),
        }
    })
}

fn process_cmd(w: &Window, cmd: &Command) {
    match cmd {
        Command::Resize(width, height) => {
            w.set_min_inner_size(Some(LogicalSize::new(*width, *height)))
        }
        _ => {}
    }
}
