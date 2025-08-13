pub struct ArrayProcessor<'a> {
    pub data: &'a [i32],
}
impl<'a> ArrayProcessor<'a> {
    pub fn update_data<'b>(&'b mut self, new_data: &'a [i32]) -> &'b [i32] {
        let prev_data = self.data;
        self.data = new_data;
        prev_data
    }
}
