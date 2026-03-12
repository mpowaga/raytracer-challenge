use crate::tuples::Tuple;

#[derive(Debug, Default)]
pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pub p: Vec<Tuple>,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    Self {
      width,
      height,
      p: vec![Tuple::default(); width * height],
    }
  }

  pub fn pixel_at(&self, x: usize, y: usize) -> &Tuple {
    &self.p[y * self.width + x]
  }

  pub fn write_pixel(&mut self, x: usize, y: usize, c: Tuple) {
    self.p[y * self.width + x] = c;
  }

  pub fn to_ppm(&self) -> String {
    let mut ppm = String::new();
    let max_w = self.width.min(5);

    ppm.push_str("P3\n");
    ppm.push_str(format!("{} {}\n", self.width, self.height).as_str());
    ppm.push_str("255\n");

    for (i, p) in self.p.iter().enumerate() {
      if i > 0 && i % max_w == 0 {
        ppm.push_str("\n");
      }

      ppm.push_str(&to_color_str(p));
      if (i + 1) % max_w != 0 {
        ppm.push_str(" ");
      }
    }

    ppm
  }
}

fn as_byte(x: f64) -> f64 {
  return (x.clamp(0.0, 1.0) * 255.0).round();
}

fn as_byte_tuple(t: &Tuple) -> Tuple {
  return Tuple {
    x: as_byte(t.x),
    y: as_byte(t.y),
    z: as_byte(t.z),
    w: as_byte(t.w),
  }
}

fn to_color_str(t: &Tuple) -> String {
  let b = as_byte_tuple(t);
  return format!("{} {} {}", b.x, b.y, b.z);
}