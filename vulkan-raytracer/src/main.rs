use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, ImmutableBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBuffer},
    descriptor::{descriptor_set::PersistentDescriptorSet, PipelineLayoutAbstract},
    device::{Device, DeviceExtensions, Features},
    format::Format,
    image::{Dimensions, StorageImage},
    instance::{Instance, InstanceExtensions, PhysicalDevice},
    pipeline::ComputePipeline,
    sync::GpuFuture,
};

use std::sync::Arc;

mod camera;
mod vec3;

mod shader {
    vulkano_shaders::shader! {
    ty: "compute",
    path: "shaders/main.glsl",
    }
}

use camera::Camera;
use shader::ty::{Material, Plane, Sphere};
use vec3::Vec3;

fn main() {
    let total_clock = std::time::Instant::now();
    let width = 1920;
    let height = 1080;

    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("Failed to create vulkan instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("No devices available");

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Couldn't find a suitable queue familty");

    let mut ext = DeviceExtensions::none();
    ext.khr_storage_buffer_storage_class = true;

    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("Failed to create logical device")
    };

    let queue = queues.next().unwrap();

    let image = StorageImage::new(
        device.clone(),
        Dimensions::Dim2d { width, height },
        Format::R8G8B8A8Unorm,
        Some(queue.family()),
    )
    .unwrap();

    macro_rules! buffer {
	( $( $x:expr ),* ) => {
	    {
		let contents = [
		    $(
			$x,
		    )*
		];

		let (buffer, future) =
		    ImmutableBuffer::from_data(contents, BufferUsage::all(), queue.clone()).unwrap();

		future
		    .then_signal_fence_and_flush()
		    .unwrap()
		    .wait(None)
		    .unwrap();

		buffer
	    }
	};
    }

    let material_buffer = buffer![
        Material {
            reflection: [1.0, 1.0, 1.0],
            emission: [0.0, 0.0, 0.0],
            scattering: 1.0,
            _dummy0: [0; 4],
        },
        Material {
            reflection: [1.0, 1.0, 1.0],
            emission: [0.0, 0.0, 0.0],
            scattering: 0.0,
            _dummy0: [0; 4],
        },
        Material {
            reflection: [0.3, 0.3, 1.0],
            emission: [0.0, 0.0, 0.0],
            scattering: 1.0,
            _dummy0: [0; 4],
        },
        Material {
            reflection: [1.0, 0.3, 0.3],
            emission: [0.0, 0.0, 0.0],
            scattering: 1.0,
            _dummy0: [0; 4],
        },
        Material {
            reflection: [1.0, 1.0, 1.0],
            emission: [0.7, 0.7, 0.7],
            scattering: 1.0,
            _dummy0: [0; 4],
        }
    ];

    let sphere_buffer = buffer![
        Sphere {
            center: [0.25, -0.3, -0.15],
            radius: 0.2,
            mat_index: 0,
            _dummy0: [0; 12],
        },
        Sphere {
            center: [-0.25, -0.3, -0.25],
            radius: 0.2,
            mat_index: 1,
            _dummy0: [0; 12],
        },
        Sphere {
            center: [0.0, 0.6, -0.25],
            radius: 0.25,
            mat_index: 4,
            _dummy0: [0; 12],
        }
    ];

    let plane_buffer = buffer![
        Plane {
            normal: [0.0, 1.0, 0.0],
            dist: -0.5,
            mat_index: 0,
            _dummy0: [0; 12],
        },
        Plane {
            normal: [0.0, -1.0, 0.0],
            dist: -0.5,
            mat_index: 0,
            _dummy0: [0; 12],
        },
        Plane {
            normal: [0.0, 0.0, -1.0],
            dist: -0.5,
            mat_index: 0,
            _dummy0: [0; 12],
        },
        Plane {
            normal: [0.0, 0.0, 1.0],
            dist: -0.5,
            mat_index: 0,
            _dummy0: [0; 12],
        },
        Plane {
            normal: [-1.0, 0.0, 0.0],
            dist: -0.5,
            mat_index: 2,
            _dummy0: [0; 12],
        },
        Plane {
            normal: [1.0, 0.0, 0.0],
            dist: -0.5,
            mat_index: 3,
            _dummy0: [0; 12],
        }
    ];

    let random_buffer = {
        let mut seed: u32 = 3534535353;
        let iter = (0..(width * height)).map(|_| {
            seed ^= seed >> 17;
            seed ^= seed << 13;
            seed ^= seed >> 5;
            if seed == 0 {
                seed = 63634636;
                seed
            } else {
                seed
            }
        });

        let (buffer, future) =
            ImmutableBuffer::from_iter(iter, BufferUsage::all(), queue.clone()).unwrap();

        future
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        buffer
    };

    let shader = shader::Shader::load(device.clone()).expect("Failed to create shader");

    let pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
            .expect("Failed to create pipeline"),
    );

    let layout = pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_image(image.clone())
            .unwrap()
            .add_buffer(random_buffer.clone())
            .unwrap()
            .add_buffer(material_buffer.clone())
            .unwrap()
            .add_buffer(sphere_buffer.clone())
            .unwrap()
            .add_buffer(plane_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    let buffer = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::all(),
        false,
        (0..width * height * 4).map(|_| 0u8),
    )
    .expect("Failed to create buffer");

    // let camera = Camera::new(
    //     Vec3::new(0.0, 0.0, 0.0),
    //     1.0,
    //     std::f32::consts::PI / 2.0,
    //     16.0 / 9.0,
    // );

    let camera = Camera::look_at(
        Vec3::new(0.0, 0.0, 0.5),
        Vec3::new(0.0, -0.25, -1.0),
        std::f32::consts::FRAC_PI_2,
        width as f32 / height as f32,
    );

    let push_constants = shader::ty::Camera {
        origin: camera.origin.as_array(),
        horizontal: camera.horizontal.as_array(),
        vertical: camera.vertical.as_array(),
        lower_left: camera.lower_left.as_array(),
        bounces: 8,
        samples: 256,
        _dummy0: [0; 4],
        _dummy1: [0; 4],
        _dummy2: [0; 4],
    };

    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder
        .dispatch(
            [width / 8, height / 8, 1],
            pipeline.clone(),
            set.clone(),
            push_constants,
        )
        .unwrap()
        .copy_image_to_buffer(image.clone(), buffer.clone())
        .unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    let setup_time = total_clock.elapsed().as_millis();
    let gpu_clock = std::time::Instant::now();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let gpu_time = gpu_clock.elapsed().as_millis();

    let image_write_clock = std::time::Instant::now();
    let content = buffer.read().unwrap();
    let file = std::fs::File::create("image.png").expect("Failed to create file image.png");

    let ref mut w = std::io::BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&content[..]).unwrap();
    println!(
        "Total: {}ms, Setup: {}ms, GPU: {}ms, Image write: {}ms",
        total_clock.elapsed().as_millis(),
        setup_time,
        gpu_time,
        image_write_clock.elapsed().as_millis()
    );
}
