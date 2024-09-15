use std::mem;

fn distance2(ax: i32, ay: i32, bx: i32, by: i32) -> i32 {
  let dy = ax - bx;
  let dz = ay - by;
  
  dy*dy + dz*dz
}

fn find_minimum_walk(mut l:i32, mut w: i32, mut h: i32, mut x1: i32, mut y1: i32, mut z1: i32, mut x2: i32, mut y2: i32, mut z2: i32) -> i32 {
  // Reorient the object until P1 is on the x=0 or x=l plane.
  if x1 != 0 && x1 != l {
    // It must be on the y=0, y=w, z=0, or z=h planes.
    if y1 == 0 || y1 == w {
      mem::swap(&mut l, &mut w);
      mem::swap(&mut x1, &mut y1);
      mem::swap(&mut x2, &mut y2);
    } else if z1 == 0 || z1 == h {
      mem::swap(&mut l, &mut h);
      mem::swap(&mut x1, &mut z1);
      mem::swap(&mut x2, &mut z2);
    } else {
      panic!();
    }
  }

  // Reorient the object until P1 is on the x=0 plane.
  if x1 != 0 {
    x1 = 0;
    x2 = l - x2;
  }

  // If P2 is on the x=0 plane, then the distance between the points is the minimum.
  if x2 == 0 {
    return distance2(y1, z1, y2, z2);
  }

  if y2 == w {
    y1 = w - y1;
    y2 = 0;
  }

  if y2 != 0 {
    mem::swap(&mut h, &mut w);
    mem::swap(&mut z1, &mut y1);
    mem::swap(&mut z2, &mut y2);
  }

  if y2 == w {
    y1 = w - y1;
    y2 = 0;
  }

  // Imagine a box unfolded. 1 is the top, 6 is the bottom.
  //
  // 2
  // 1 3 6
  // 4
  //
  // P1 is on face 1, P2 is on faces 3 (and maybe on 2 or 4 or 6) or just 6.

  let p2on2 = z2 == h;
  let p2on3 = y2 == 0;
  let p2on4 = z2 == 0;
  let p2on6 = x2 == l;

  let mut best_d = 0x7FFFFFFF;

  if p2on3 {
    // 1 3
    best_d = std::cmp::min(
      best_d, 
      distance2(0, 0, y1 + x2, z2 - z1),
    );
    // 1 2 3
    best_d = std::cmp::min(
      best_d, 
      distance2(0, 0, y1 + x2, z2 - z1),
    );
    // 1 4 3
  }
  // 1 2
  // 1 4
  // 1 3 6
  // 1 2 6
  // 1 4 6
  // 1 2 3 6
  // 1 4 3 6
  // 1 3 2 6
  // 1 3 4 6

  println!("{} {} {}  {} {} {}  {} {} {}", l, w, h, x1, y1, z1, x2, y2, z2);

  best_d
}

fn main() {
  println!("{}", find_minimum_walk(5, 5, 2, 3, 1, 2, 3, 5, 0));
  println!("{}", find_minimum_walk(300, 600, 900, 300, 550, 0, 0, 550, 900));
  println!("{}", find_minimum_walk(5, 5, 2, 3, 1, 2, 2, 4, 0));
}
