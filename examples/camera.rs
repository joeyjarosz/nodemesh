extern crate nalgebra as na;
extern crate libnm;

use na::{PerspectiveMatrix3, Identity, Vector4};
use libnm::{Client, Transform};
use libnm::renderers::{WebGLRendererOptions, CameraOptions};

fn main() {
    let mut client = Client::new("tcp://localhost:5555");
    let projection = *PerspectiveMatrix3::new(
        2.0,   // aspect
        40.0,  // fov
        0.0,   // znear
        1000.0 // zfar
    ).as_matrix();

    client
        .create_renderer(&WebGLRendererOptions::new("tcp://0.0.0.0:5556"))
        // Simple offset origin position.
        // .with_viewer_transform(Matrix4::new())
        // .with_camera(
        //     CameraOptions::new("left_eye")
        //         .with_projection(Transform(projection))
        //         // .with_transform(Identity * Vector4::new(0, -10, 0, 0))
        //         .finalize()
        // )
        // .with_camera(
        //     CameraOptions::new("right_eye")
        //         .with_projection(Transform(projection))
        //         // .with_transform(Identity * Vector4::new(0, 10, 0, 0))
        //         .finalize()
        // )
        .send();

    // // deleteRenderer
    // renderer.delete().send();
    // let renderer = client.renderer_with_id("foo"); // Result

    // // updateViewerTransform
    // renderer.update_viewer_transform(Matrix4);

    // // createCamera
    // let camera = renderer.create_camera(camera_builder);

    // // deleteCamera
    // camera.delete();
    // let camera = renderer.camera_with_name(""); // Result<Camera>

    // // updateCameraTransform
    // camera.update_transform();
    // camera.update_projection();

    // let mut camera = renderer.camera_with_name("left").unwrap();

    // let results = client.send_batch(vec![
    //     renderer.set_viewer_position(matrix),
    //     camera.set_position(matrix),
    //     camera.set_projection(matrix)
    // ]);
}
