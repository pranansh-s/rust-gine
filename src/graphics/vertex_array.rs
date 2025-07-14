pub struct VertexArray {
    pub id: u32,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl VertexArray {
    pub unsafe fn new() -> Result<Self, std::io::Error> {
        let mut id = 0;
        gl::GenVertexArrays(1, &mut id);

        Ok(Self{ id })
    }

    pub unsafe fn set_attrb<V: Sized>(&self, attrb_pos: u32, data: i32, vertex_size: i32, offset: usize) {
        gl::VertexAttribPointer(attrb_pos, data, gl::FLOAT, gl::FALSE, vertex_size * std::mem::size_of::<V>() as i32, ((offset * std::mem::size_of::<f32>()) as i32) as *const _);
        gl::EnableVertexAttribArray(attrb_pos);
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id)
    }

    pub unsafe fn unbind(&self) {
        gl::BindVertexArray(0);
    }
}