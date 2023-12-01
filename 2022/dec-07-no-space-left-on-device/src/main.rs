use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

const SPACE_AVAIL: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;

#[derive(Default)]
struct SizeObj {
    real_size: usize,
    size_for_problem: usize,
}

#[derive(Default)]
struct ElfFile {
    pub name: String,
    pub size: usize,
}

impl ElfFile {
    pub fn from_str_size(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: size,
        }
    }
}

#[derive(Default, Eq, Ord, PartialEq, PartialOrd)]
struct FileSizeObj {
    pub size: usize,
    pub name: String,
}

impl FileSizeObj {
    pub fn from_name_size(name: &str, size: usize) -> Self {
        Self {
            name: name.to_string(),
            size: size,
        }
    }
}

#[derive(Default)]
struct ElfDir {
    pub name: String,
    files: Vec<ElfFile>,
    dirs: Vec<ElfDir>,
}

impl ElfDir {
    pub fn from_str(dir_name: &str) -> Self {
        Self {
            name: dir_name.to_string(),
            files: Vec::<ElfFile>::default(),
            dirs: Vec::<ElfDir>::default(),
        }
    }

    pub fn get_dir_size_objs(&self) -> Vec<FileSizeObj> {
        let mut file_sizes = Vec::<FileSizeObj>::new();
        for dir in self.dirs.iter() {
            file_sizes.extend(dir.get_dir_size_objs());
        }
        file_sizes.push(FileSizeObj::from_name_size(&self.name, self.get_dir_size()));
        file_sizes
    }

    pub fn get_dir_size(&self) -> usize {
        let mut total = 0;
        for file in self.files.iter() {
            total += file.size;
        }
        for dir in self.dirs.iter() {
            total += dir.get_dir_size();
        }
        total
    }

    pub fn sum_sizes_if_big_enough_double_count(&self) -> SizeObj {
        let mut total = SizeObj::default();
        let mut size_of_files = 0;
        for file in self.files.iter() {
            size_of_files += file.size;
        }
        let mut size_of_dirs = 0;
        for dir in self.dirs.iter() {
            let total_inside = dir.sum_sizes_if_big_enough_double_count();
            size_of_dirs += total_inside.real_size;
            total.size_for_problem += total_inside.size_for_problem;
        }
        total.real_size = size_of_dirs + size_of_files;
        println!("Size of {} is {}", self.name, total.real_size);
        if total.real_size < 100000 {
            println!("Total size is small enough");
            total.size_for_problem += total.real_size;
        }
        total
    }

    pub fn add_dir(&mut self, path: &[String], dir_name: &str) {
        if path.len() == 0 {
            self.dirs.push(ElfDir::from_str(dir_name));
            return;
        }
        println!("Looking for {}", &path[0]);
        let next_dir = self.get_dir(&path[0]);
        println!("Entering {}", path[0]);
        next_dir.add_dir(&path[1..], dir_name);
    }

    pub fn add_file(&mut self, path: &[String], file_name: &str, file_size: usize) {
        if path.len() == 0 {
            self.files
                .push(ElfFile::from_str_size(file_name, file_size));
            println!("Added file to {}", self.name);
            return;
        }
        let next_dir = self.get_dir(&path[0]);
        next_dir.add_file(&path[1..], file_name, file_size);
    }

    fn get_dir(&mut self, name: &str) -> &mut ElfDir {
        for dir in self.dirs.iter_mut() {
            if dir.name == name {
                return dir;
            }
        }
        panic!("Directory {} not found in {}", name, self.name);
    }
}

#[derive(Default)]
struct DirBuilder {
    pwd: Vec<String>,
    last_cmd: String,
    fs_root: ElfDir,
}

impl DirBuilder {
    pub fn add_line(&mut self, line: &str) {
        if line.chars().nth(0).unwrap() == '$' {
            self.execute_command(line);
        } else if true {
            self.add_fs_item(line);
        }
    }

    fn add_fs_item(&mut self, line: &str) {
        println!("Found file item: {}", line);
        let line_iter: Vec<&str> = line.split(' ').collect();
        let left = line_iter[0].to_string();
        let right = line_iter[1].to_string();
        if left == "dir" {
            self.add_dir(&right);
        } else {
            let size = left.parse::<usize>().unwrap();
            self.add_file(&right, size);
        }
    }

    fn execute_command(&mut self, line: &str) {
        println!("Executing command: {}", line);
        let line_iter: Vec<&str> = line.split(' ').collect();
        self.last_cmd = line_iter[1].to_string();
        if line_iter[1] == "cd" {
            if line_iter[2] == ".." {
                self.pwd.pop();
            } else {
                self.change_dir(line_iter[2]);
            }
        } else if line_iter[1] == "ls" {
            println!("TODO start ls context");
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        println!("Adding file {}", name);
        self.fs_root.add_file(&self.pwd, name, size);
    }

    fn add_dir(&mut self, name: &str) {
        println!("Adding dir {}", name);
        self.fs_root.add_dir(&self.pwd, name);
    }

    fn change_dir(&mut self, dir_name: &str) {
        println!("Changing dir to {}", dir_name);
        if self.fs_root.dirs.len() == 0 {
            self.fs_root = ElfDir::from_str(dir_name);
            return;
        }
        self.pwd.push(dir_name.to_string());
    }

    pub fn sum(&self) -> SizeObj {
        self.fs_root.sum_sizes_if_big_enough_double_count()
    }

    pub fn get_dir_sizes(&self) -> Vec<FileSizeObj> {
        self.fs_root.get_dir_size_objs()
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let fname = &args[0];
    println!("Filename: {}", fname);
    let file = File::open(fname)?;
    let reader = BufReader::new(file);
    let mut cmd_builder = DirBuilder::default();
    for read_line in reader.lines() {
        let line = read_line?;
        cmd_builder.add_line(&line);
    }
    let total_space_obj = cmd_builder.sum();
    let total_size_of_smalls = total_space_obj.size_for_problem;
    println!("Total sizes of big dirs: {}", total_size_of_smalls);

    let total_size = total_space_obj.real_size;
    let mut file_size_objs = cmd_builder.get_dir_sizes();
    file_size_objs.sort();
    let empty_space = SPACE_AVAIL - total_size;
    for dir in file_size_objs {
        let freed_space = empty_space + dir.size;
        println!(
            "Total size for delete {}: {} will give {}",
            dir.name, dir.size, freed_space
        );
        if freed_space > REQUIRED_SPACE {
            break;
        }
    }

    Ok(())
}
