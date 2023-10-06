fn main() {
 let path = std::env::args().nth(1).unwrap();
 let text = std::fs::read_to_string(&path).unwrap();
 let mut output = String::new();
 output.push_str("solid tmp;\n");
 for line in text.lines().into_iter() {
  let vs = line.split(" ").into_iter().map(|v| v.parse::<f32>().unwrap()).collect::<Vec<_>>();
  let x0 = *vs.get(0).unwrap();
  let y0 = *vs.get(1).unwrap();
  let z0 = *vs.get(2).unwrap();
  let x1 = *vs.get(3).unwrap();
  let y1 = *vs.get(4).unwrap();
  let z1 = *vs.get(5).unwrap();
  let x2 = *vs.get(6).unwrap();
  let y2 = *vs.get(7).unwrap();
  let z2 = *vs.get(8).unwrap();
  // normal vector
  let nx = (y1 - y0) * (z2 - z0) - (z1 - z0) * (y2 - y0);
  let ny = (z1 - z0) * (x2 - x0) - (x1 - x0) * (z2 - z0);
  let nz = (x1 - x0) * (y2 - y0) - (y1 - y0) * (x2 - x0);
  // normalize normal vector
  let n = (nx * nx + ny * ny + nz * nz).sqrt();
  let nx = nx / n;
  let ny = ny / n;
  let nz = nz / n;
  // to STL facet section
  output.push_str(&format!("facet normal {} {} {}\n", nx, ny, nz));
  output.push_str("outer loop\n");
  output.push_str(&format!("vertex {} {} {}\n", x0, y0, z0));
  output.push_str(&format!("vertex {} {} {}\n", x1, y1, z1));
  output.push_str(&format!("vertex {} {} {}\n", x2, y2, z2));
  output.push_str("endloop\n");
  output.push_str("endfacet\n");
 }
 let output_filename = format!("{}.stl", path);
 std::fs::write(output_filename, output).unwrap();
}
