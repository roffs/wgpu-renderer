mod camera_controller;
use cgmath::{perspective, Angle, Deg, Matrix, Matrix4, Point3, Rad, Vector3};

pub use camera_controller::CameraController;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferDescriptor,
    BufferUsages, Device, Queue,
};

pub struct Camera<'a> {
    queue: &'a Queue,

    pub(self) position: Point3<f32>,
    pub(self) yaw: Rad<f32>,
    pub(self) pitch: Rad<f32>,

    pub(self) look_dir: Vector3<f32>,
    pub(self) right: Vector3<f32>,
    pub(self) up: Vector3<f32>,
    pub(self) forward: Vector3<f32>,

    fovy: f32,
    aspect: f32,
    near: f32,
    far: f32,

    view_projection_buffer: Buffer,
    pub view_projection_bind_group: BindGroup,
}

impl<'a> Camera<'a> {
    pub fn new<T: Into<cgmath::Rad<f32>>>(
        device: &Device,
        queue: &'a Queue,
        camera_bind_group_layout: &BindGroupLayout,
        position: (f32, f32, f32),

        yaw: T,
        pitch: T,

        fovy: f32,
        aspect: f32,
        near: f32,
        far: f32,
    ) -> Camera<'a> {
        let yaw: Rad<f32> = yaw.into();
        let pitch: Rad<f32> = pitch.into();

        let (look_dir, up, right, forward) = calculate_local_directions(yaw, pitch);

        let view_projection_size = std::mem::size_of::<cgmath::Matrix4<f32>>();
        let view_projection_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("View buffer"),
            size: view_projection_size as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let projection_matrix = perspective(Deg(fovy), aspect, near, far);
        let view_matrix = Matrix4::look_to_rh(position.into(), look_dir, up);
        let view_proj_matrix = projection_matrix * view_matrix;

        let view_proj_matrix = unsafe {
            std::slice::from_raw_parts(view_proj_matrix.as_ptr() as *const u8, view_projection_size)
        };

        queue.write_buffer(&view_projection_buffer, 0, view_proj_matrix);

        let view_projection_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Camera bind group"),
            layout: camera_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: view_projection_buffer.as_entire_binding(),
            }],
        });

        Camera {
            queue,
            position: Point3::from(position),
            yaw,
            pitch,
            look_dir,
            right,
            up,
            forward,
            fovy,
            aspect,
            near,
            far,
            view_projection_buffer,
            view_projection_bind_group,
        }
    }

    pub fn get_position(&self) -> cgmath::Point3<f32> {
        self.position
    }

    pub fn get_rotation(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_to_rh((0.0, 0.0, 0.0).into(), self.look_dir, self.up)
    }
    pub fn get_view(&self) -> cgmath::Matrix4<f32> {
        Matrix4::look_to_rh(self.position, self.look_dir, self.up)
    }

    pub fn get_projection(&self) -> cgmath::Matrix4<f32> {
        perspective(Deg(self.fovy), self.aspect, self.near, self.far)
    }

    pub fn update_aspect(&mut self, aspect: f32) {
        self.aspect = aspect;
        self.update_buffer();
    }

    pub(self) fn update_directions(&mut self) {
        let (look_dir, up, right, forward) = calculate_local_directions(self.yaw, self.pitch);

        self.look_dir = look_dir;
        self.up = up;
        self.right = right;
        self.forward = forward;
    }

    fn update_buffer(&self) {
        let projection_matrix = perspective(Deg(self.fovy), self.aspect, self.near, self.far);
        let view_matrix = Matrix4::look_to_rh(self.position, self.look_dir, self.up);
        let view_proj_matrix = projection_matrix * view_matrix;

        let view_proj_matrix = unsafe {
            std::slice::from_raw_parts(
                view_proj_matrix.as_ptr() as *const u8,
                std::mem::size_of::<cgmath::Matrix4<f32>>(),
            )
        };

        self.queue
            .write_buffer(&self.view_projection_buffer, 0, view_proj_matrix);
    }
}

fn calculate_local_directions(
    yaw: Rad<f32>,
    pitch: Rad<f32>,
) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let x = yaw.cos() * pitch.cos();
    let y = pitch.sin();
    let z = yaw.sin() * pitch.cos();

    let look_dir = Vector3::new(x, y, z);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let right = look_dir.cross(up);
    let forward = up.cross(right);

    (look_dir, up, right, forward)
}
