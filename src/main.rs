use std::fs;
use std::io::{self, Write};

fn list_files_recursively(
    dir: &std::path::Path,
    output: &mut dyn Write,
    depth: usize,
    is_last: bool,
) -> io::Result<()> {
    let prefix = if depth == 0 {
        String::new()
    } else {
        let mut prefix = String::from("│   ").repeat(depth - 1);
        prefix += if is_last { "└── " } else { "├── " };
        prefix
    };

    for (index, entry) in fs::read_dir(dir)?.enumerate() {
        let entry = entry?;
        let path = entry.path();

        if let Some(file_name) = path.file_name() {
            if let Some(name) = file_name.to_str() {
                if !name.starts_with('.') {
                    writeln!(
                        output,
                        "{}{}",
                        prefix,
                        name
                    )?;

                    let is_last_entry = index == fs::read_dir(dir)?.count() - 1;
                    if path.is_dir() {
                        list_files_recursively(&path, output, depth + 1, is_last_entry)?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let root_path = std::path::Path::new("/home/westermann");
    let output_file = std::fs::File::create("out.txt")?;
    let mut output_writer = std::io::BufWriter::new(output_file);

    list_files_recursively(&root_path, &mut output_writer, 0, true)?;

    Ok(())
}

