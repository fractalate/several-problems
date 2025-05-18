use std::io;
use std::io::BufRead;

const NO_FILE: i64 = -1;

fn allocate_files(file_layout_descriptor: String) -> Vec<i64> {
  let mut files: Vec<i64> = Vec::new();
  let mut reading_state = 0;
  let mut next_file_no = 0;

  for c in file_layout_descriptor.bytes() {
    if c >= b'0' && c <= b'9' {
      let run_length = (c - b'0') as i64;

      let file_no = if reading_state == 0 {
        let file_no = next_file_no;
        next_file_no += 1;
        file_no
      } else {
        NO_FILE
      };

      for _i in 0..run_length {
        files.push(file_no);
      }

      reading_state = !reading_state;
    }
  }

  return files;
}

fn defragment_files(files: &mut Vec<i64>) {
  let mut i = 0;
  let mut j = files.len() - 1;

  while i < j {
    if files[i] != NO_FILE {
      i += 1;
    } else if files[j] == NO_FILE {
      j -= 1;
    } else {
      files.swap(i, j);
      i += 1;
      j -= 1;
    }
  }
}

fn calculate_checksum(files: &Vec<i64>) -> i64 {
  let mut checksum = 0;

  for i in 0..files.len() as i64 {
    if files[i as usize] == NO_FILE {
      break;
    }

    checksum += files[i as usize] * i;
  }

  return checksum;
}

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();
  while let Some(Ok(line)) = lines.next() {
    let mut files = allocate_files(line);
    defragment_files(&mut files);
    let checksum = calculate_checksum(&files);
    println!("{checksum}");
  }
}
