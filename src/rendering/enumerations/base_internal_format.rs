// Incomplete. https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml 

#[allow(dead_code)]
#[repr(u32)]
pub enum BaseInternalFormat {
    DepthComponent = gl::DEPTH_COMPONENT,
    DepthStencil = gl::DEPTH_STENCIL,
    R = gl::RED,
    RG = gl::RG,
    RGB = gl::RGB,
    RGBA = gl::RGBA,
}