use nalgebra::{Rotation3, Translation2, Unit};
use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, KeyboardInput, MouseButton, MouseScrollDelta,
        VirtualKeyCode,
    },
};

use crate::graphics::Transform;

pub struct Handler {
    cursor: Option<PhysicalPosition<f64>>,
    rotating: bool,
    moving: bool,
    zoom_speed: f32,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            cursor: None,
            rotating: false,
            moving: false,
            zoom_speed: 0.0,
        }
    }

    pub fn handle_keyboard_input(
        &mut self,
        input: KeyboardInput,
        actions: &mut Actions,
    ) {
        if let KeyboardInput {
            state: ElementState::Pressed,
            virtual_keycode: Some(virtual_key_code),
            ..
        } = input
        {
            match virtual_key_code {
                VirtualKeyCode::Escape => actions.exit = true,
                VirtualKeyCode::Key1 => actions.toggle_model = true,
                VirtualKeyCode::Key2 => actions.toggle_mesh = true,
                _ => (),
            }
        }
    }

    pub fn handle_cursor_moved(
        &mut self,
        position: PhysicalPosition<f64>,
        transform: &mut Transform,
    ) {
        if let Some(previous) = self.cursor {
            let diff_x = position.x - previous.x;
            let diff_y = position.y - previous.y;

            if self.rotating {
                // TASK: Rotate the model around the point on the surface that
                //       the cursor is currently pointing at.

                let f = 0.005;

                let x_angle = diff_y as f32 * f;
                let y_angle = diff_x as f32 * f;

                let x_rot = Rotation3::from_axis_angle(
                    &Unit::new_unchecked([1.0, 0.0, 0.0].into()),
                    x_angle,
                );
                let y_rot = Rotation3::from_axis_angle(
                    &Unit::new_unchecked([0.0, 1.0, 0.0].into()),
                    y_angle,
                );

                transform.rotation = y_rot * x_rot * transform.rotation;
            }
            if self.moving {
                // TASK: Moving feels good, if you're dragging the model exactly
                //       where your mouse goes. It feels weird, if the mouse
                //       cursor moves faster or slower than the model you're
                //       moving.
                //
                //       The following factor achieves this good-feeling move
                //       for relatively small models at the default distance
                //       between camera and model origin. It breaks down when
                //       moving the camera closer or away from the model, which
                //       is the far more common case.
                //
                //       It would be nicer to have a zoom factor that depends on
                //       the distance between camera and model origin, or even
                //       the distance between the camera and the part of the
                //       model the mouse is currently pointing at (or more
                //       precisely, the distance between the camera and a plane
                //       that touches the surface of the model where the mouse
                //       is pointing, and whose normal is parallel to the
                //       camera's viewing direction).
                let f = 0.2;

                let x_trans = diff_x as f32 * f;
                let y_trans = -diff_y as f32 * f;

                let translation = Translation2::new(x_trans, y_trans);

                transform.translation = translation * transform.translation;
            }
        }

        self.cursor = Some(position);
    }

    pub fn handle_mouse_input(
        &mut self,
        button: MouseButton,
        state: ElementState,
    ) {
        match (button, state) {
            (MouseButton::Left, ElementState::Pressed) => {
                self.rotating = true;
            }
            (MouseButton::Left, ElementState::Released) => {
                self.rotating = false;
            }
            (MouseButton::Right, ElementState::Pressed) => {
                self.moving = true;
            }
            (MouseButton::Right, ElementState::Released) => {
                self.moving = false;
            }
            _ => {}
        }
    }

    pub fn handle_mouse_wheel(&mut self, delta: MouseScrollDelta) {
        let delta = match delta {
            MouseScrollDelta::LineDelta(_, y) => y * 10.0,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                y as f32
            }
        };

        // TASK: Zooming doesn't work well, and the zoom speed idea might have
        //       been misguided for this application.
        //
        //       I'm not sure what the right solution is, but it needs to have
        //       the following properties:
        //       - While zoomed in closely, camera movement needs to be very
        //         precise.
        //       - But it should still be possible to zoom out quickly, if
        //         desired.
        //       - While less close, camera movement should be quicker and less
        //         precise.
        if delta > 0.0 && self.zoom_speed < 0.0
            || delta < 0.0 && self.zoom_speed > 0.0
        {
            self.zoom_speed = 0.0;
        } else {
            self.zoom_speed += delta * 0.02;
        }
    }

    pub fn update(&mut self, delta_t: f32, transform: &mut Transform) {
        transform.distance += self.zoom_speed;

        // Reduce zoom speed such, that it is `zoom_speed * f` after one
        // second.
        let f: f32 = 0.2;
        let n = 1.0 / delta_t;
        self.zoom_speed *= f.powf(1.0 / n);
    }
}

pub struct Actions {
    pub exit: bool,

    pub toggle_model: bool,
    pub toggle_mesh: bool,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            exit: false,

            toggle_model: false,
            toggle_mesh: false,
        }
    }
}