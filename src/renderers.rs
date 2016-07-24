use std::collections::HashMap;

use protocol::messages;
use protobuf::{RepeatedField};
use types::{Id, Transform};
use client::{Client};

pub use protocol::messages::CreateRendererRequest_RendererType as RendererType;

pub struct CreateRendererOperation<'a> {
    client: &'a mut Client,
    req: messages::CreateRendererRequest
}

impl<'a> CreateRendererOperation<'a> {
    pub fn new(
        client: &'a mut Client,
        renderer_options: &RendererOptions
    ) -> CreateRendererOperation<'a> {
        let mut req = messages::CreateRendererRequest::new();
        req.set_renderer_type(renderer_options.renderer_type());
        req.set_viewer_transform(vec![
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0
        ]);

        CreateRendererOperation{
            client: client,
            req: req
        }
    }

    pub fn with_viewer_transform(
        &mut self, transform: Transform
    ) -> &mut CreateRendererOperation<'a> {
        self.req.set_viewer_transform(transform.to_vec());
        self
    }

    pub fn with_camera(
        &mut self, camera_options: CameraOptions
    ) -> &mut CreateRendererOperation<'a> {
        let mut cameras = self.req.get_cameras().to_vec();
        cameras.push(camera_options.msg);
        self.req.set_cameras(RepeatedField::from_vec(cameras));
        self
    }

    pub fn send(&mut self) -> Renderer {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::CREATE_RENDERER);
        req.set_create_renderer_request(self.req.clone());
        let res = self.client.send(req);
        let renderer_id = res.get_item_created_response().get_item_id();
        print!("created renderer with id: {}\n", renderer_id);
        Renderer{
            client: self.client,
            id: renderer_id
        }
    }
}

pub struct UpdateCameraTransformOperation<'a> {
    client: &'a mut Client,
    renderer_id: Id,
    camera_name: &'a str,
    transform: Transform
}

impl<'a> UpdateCameraTransformOperation<'a> {
    pub fn send(&'a mut self) {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::UPDATE_CAMERA_TRANSFORM);
        let mut data = messages::UpdateCameraTransformRequest::new();
        data.set_renderer_id(self.renderer_id);
        data.set_camera_name(self.camera_name.to_string());
        data.set_transform(self.transform.to_vec());
        req.set_update_camera_transform_request(data);
        let _ = self.client.send(req);
    }
}

pub struct UpdateCameraProjectionOperation<'a> {
    client: &'a mut Client,
    renderer_id: Id,
    camera_name: &'a str,
    projection: Transform
}

impl<'a> UpdateCameraProjectionOperation<'a> {
    pub fn send(&'a mut self) {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::UDPATE_CAMERA_PROJECTION);
        let mut data = messages::UpdateCameraProjectionRequest::new();
        data.set_renderer_id(self.renderer_id);
        data.set_camera_name(self.camera_name.to_string());
        data.set_transform(self.projection.to_vec());
        req.set_update_camera_projection_request(data);
        let _ = self.client.send(req);
    }
}

pub struct CreateCameraOperation<'a> {
    client: &'a mut Client,
    renderer_id: Id,
    camera_options: CameraOptions<'a>
}

impl<'a> CreateCameraOperation<'a> {
    pub fn send(&'a mut self) {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::CREATE_CAMERA);
        let mut data = messages::CreateCameraRequest::new();
        data.set_renderer_id(self.renderer_id);
        data.set_camera(self.camera_options.msg.clone());
        req.set_create_camera_request(data);
        let _ = self.client.send(req);
    }
}

pub struct DeleteCameraOperation<'a> {
    client: &'a mut Client,
    renderer_id: Id,
    name: &'a str
}

impl<'a> DeleteCameraOperation<'a> {
    pub fn send(&'a mut self) {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::DELETE_CAMERA);
        let mut data = messages::DeleteCameraRequest::new();
        data.set_renderer_id(self.renderer_id);
        data.set_camera_name(self.name.to_string());
        req.set_delete_camera_request(data);

        let _ = self.client.send(req);
    }
}

pub struct DeleteRendererOperation<'a> {
    client: &'a mut Client,
    id: Id
}

impl<'a> DeleteRendererOperation<'a> {
    pub fn send(&'a mut self) {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::DELETE_RENDERER);
        let mut data = messages::DeleteRendererRequest::new();
        data.set_renderer_id(self.id);
        req.set_delete_renderer_request(data);

        let _ = self.client.send(req);
    }
}

pub struct UpdateViewerTransformOperation<'a> {
    client: &'a mut Client,
    id: Id,
    transform: Transform
}

impl<'a> UpdateViewerTransformOperation<'a> {
    pub fn send(&self) {
        let mut req = messages::Request::new();
        req.set_request_type(messages::Request_RequestType::UPDATE_VIEWER_TRANSFORM);
        let mut data = messages::UpdateViewerTransformRequest::new();
        data.set_renderer_id(self.id);
        data.set_transform(self.transform.to_vec());
        req.set_update_viewer_transform_request(data);
    }
}

// Structs

pub struct Camera<'a> {
    renderer_id: Id,
    client: &'a mut Client,
    name: &'a str
}

impl<'a> Camera<'a> {
    pub fn delete(&'a mut self) -> DeleteCameraOperation<'a> {
        DeleteCameraOperation{
            renderer_id: self.renderer_id,
            client: self.client,
            name: self.name
        }
    }

    pub fn update_transform(
        &'a mut self,
        transform: Transform
    ) -> UpdateCameraTransformOperation<'a> {
        UpdateCameraTransformOperation{
            client: self.client,
            renderer_id: self.renderer_id,
            camera_name: self.name,
            transform: transform
        }
    }

    pub fn update_projection(
        &'a mut self,
        projection: Transform
    ) -> UpdateCameraProjectionOperation<'a> {
        UpdateCameraProjectionOperation{
            client: self.client,
            renderer_id: self.renderer_id,
            camera_name: self.name,
            projection: projection
        }
    }
}

pub struct CameraOptions<'a> {
    name: &'a str,
    msg: messages::Camera
}

impl<'a> CameraOptions<'a> {
    pub fn new(camera_name: &str) -> CameraOptions {
        let mut msg = messages::Camera::new();
        // TODO: set default transforms

        CameraOptions{
            name: camera_name,
            msg: msg
        }
    }

    pub fn with_transform(&'a mut self, transform: Transform) -> &mut CameraOptions {
        self.msg.set_transform(transform.to_vec());
        self
    }

    pub fn with_projection(&'a mut self, projection: Transform) -> &mut CameraOptions {
        self.msg.set_projection(projection.to_vec());
        self
    }

    // pub fn finalize(&'a self) -> CameraOptions<'a> {
    //     self
    // }
}

pub trait RendererOptions {
    fn options(&self) -> HashMap<&str, String>;
    fn renderer_type(&self) -> RendererType;
}

pub struct WebGLRendererOptions<'a> {
    addr: &'a str
}

impl<'a> WebGLRendererOptions<'a> {
    pub fn new(addr: &str) -> WebGLRendererOptions {
        WebGLRendererOptions{
            addr: addr
        }
    }
}

impl<'a> RendererOptions for WebGLRendererOptions<'a> {

    fn options(&self) -> HashMap<&str, String> {
        let mut options = HashMap::new();
        options.insert("addr", self.addr.to_string());
        options
    }

    fn renderer_type(&self) -> RendererType {
        RendererType::WEBGL
    }
}

pub struct Renderer<'a> {
    pub client: &'a mut Client,
    pub id: Id
}

impl<'a> Renderer<'a> {
    pub fn delete(&mut self) -> DeleteRendererOperation {
        DeleteRendererOperation{
            client: self.client,
            id: self.id
        }
    }

    pub fn update_viewer_transform(
        &mut self, transform: Transform
    ) -> UpdateViewerTransformOperation {
        UpdateViewerTransformOperation{
            client: self.client,
            id: self.id,
            transform: transform
        }
    }

    pub fn create_camera(
        &'a mut self, camera_options: CameraOptions<'a>
    ) -> CreateCameraOperation<'a> {
        CreateCameraOperation{
            renderer_id: self.id,
            client: self.client,
            camera_options: camera_options
        }
    }

    pub fn get_camera_with_name(
        &'a mut self, name: &'a str
    ) -> Camera<'a> {
        Camera{
            renderer_id: self.id,
            client: self.client,
            name: name
        }
    }
}
