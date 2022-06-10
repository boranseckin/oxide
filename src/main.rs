use std::fs;
use std::path::Path;
use std::env;

fn main() {
  let mut args = env::args().skip(1);

  let path = &args.next().expect("No path");
  println!("{}", path);

  let path = Path::new(path);
  let depth: usize = args.next().expect("No depth").parse().expect("Depth is not a number");

  tree(path, depth, 0);
}

fn tree(path: &Path, depth: usize, i: usize) {
  if i > depth { return };

  match fs::read_dir(path) {
    Err(e) => println!("{}", e.kind()),
    Ok(entries) => {
      let mut vec = Vec::new();

      for entry in entries {
        if let Ok(entry) = entry {
          vec.push(entry);
        }
      }
      vec.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

      let length = vec.len();

      for (num, entry) in vec.iter_mut().enumerate() {
        if let Ok(metadata) = entry.metadata() {
          let filename = entry.file_name();
          let filename = filename.to_str().expect("Cannot parse filename");

          print!("{}", "│   ".repeat(i));

          if num == length - 1 { print!("└── "); }
          else { print!("├── "); }

          println!("{}", filename);

          if metadata.is_dir() {
            tree(&entry.path(), depth, i + 1);
          }
        }
      }
    },
  }
}
