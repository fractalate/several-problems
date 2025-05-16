use std::io;
use std::io::BufRead;

enum SpaceEntry {
  File(usize, i64),
  Free(usize),
}

fn allocate_space(file_layout_descriptor: String) -> Vec<SpaceEntry> {
  let mut space: Vec<SpaceEntry> = Vec::new();

  let mut reading_state = 0;
  let mut next_file_no = 0;

  for c in file_layout_descriptor.bytes() {
    if c >= b'0' && c <= b'9' {
      let run_length = (c - b'0') as usize;

      let entry = if reading_state == 0 {
        let file_no = next_file_no;
        next_file_no += 1;
        SpaceEntry::File(run_length, file_no)
      } else {
        SpaceEntry::Free(run_length)
      };

      space.push(entry);

      reading_state = !reading_state;
    }
  }

  return space;
}

fn defragment_space(space: &mut Vec<SpaceEntry>) {
  let mut i = 0;
  let mut j = space.len() - 1;

  while i < j {
    match space[j] {
      SpaceEntry::Free(_) => {
        j -= 1;
      },
      SpaceEntry::File(run_length_j, _) => {
        let mut k = i;
        while k < j {
          match space[k] {
            SpaceEntry::Free(run_length_k) => {
              if run_length_k == 0 {
                if k == i {
                  i += 1;
                }
                k += 1;
              } else if run_length_k < run_length_j {
                k += 1;
              } else if run_length_k == run_length_j {
                space.swap(k, j);
                break;
              } else {
                space.swap(k, j);
                // This insert could be more efficient if I used a linked list.
                space.insert(k+1, SpaceEntry::Free(run_length_k - run_length_j));
                j += 1;
                space[j] = SpaceEntry::Free(run_length_j);
                break;
              }
            },
            SpaceEntry::File(_, _) => {
              if k == i {
                i += 1;
              }
              k += 1;
            },
          }
        }

        j -= 1;
      },
    }
  }
}

fn calculate_checksum(space: &Vec<SpaceEntry>) -> i64 {
  let mut checksum = 0;
  let mut index = 0;

  for i in 0..space.len() {
    match space[i] {
      SpaceEntry::Free(run_length) => {
        index += run_length as i64;
      },
      SpaceEntry::File(run_length, file_no) => {
        for _ in 0..run_length {
          checksum += file_no * index;
          index += 1;
        }
      }
    }
  }

  return checksum;
}

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let mut space = allocate_space(line);
    defragment_space(&mut space);
    let checksum = calculate_checksum(&space);
    println!("{checksum}");
  }
}
