pub struct Buffer {
    pub id: u32,
    target: u32
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}

impl Buffer {
    pub unsafe fn new(target: gl::types::GLenum) -> Result<Self, std::io::Error> {
        let mut id = 0;
        gl::GenBuffers(1, &mut id);

        Ok(Self{ id, target })
    }

    pub unsafe fn bind<T>(&self, data: &[T], usage: gl::types::GLenum) {
        gl::BindBuffer(self.target, self.id);
        gl::BufferData(self.target, std::mem::size_of_val(data) as isize, data.as_ptr() as *const _, usage);
    }

    pub unsafe fn unbind(&self) {
        gl::BindBuffer(self.target, 0);
    }
}