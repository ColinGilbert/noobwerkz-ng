use crate::index_types::*;
use crate::server_scene3d::ServerScene3D;

pub struct ServerUserContext {
    pub scenes: ServerScenes3D<ServerScene3D>,
    //pub active_scene: ServerScene3DIndex
}