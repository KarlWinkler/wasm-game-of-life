// impl Grid {
//   pub fn new(width: u32, height: u32) -> Grid {

//     Grid {
//       width: width,
//       height: height,

//       vertices: Vec::<f32>::new(),
//       idxs: Vec::<u16>::new()
//     }
//   }

//   fn create_vertex_array(&mut self) {

//     for each cell 
//       add_square(&mut vertices, 0.8, 0.8, (1/width) 0.05, (1/height) 0.05);
//   }

//   fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
//     context.clear_color(0.0, 0.0, 0.0, 1.0);
//     context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

//     context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);
// }

//   fn _get_bit_by_index(&self, idx: u32) -> u8 {
//     // divide by 8 to find byte
//     let byte = self.cells[(idx/8) as usize];

//     // modulo 8 to find index
//     let bit = idx % 8;
//     let mask = 1 << bit;

//     // return
//     (byte & mask) >> bit
//   }
// }