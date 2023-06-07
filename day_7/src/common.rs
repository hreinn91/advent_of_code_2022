use std::collections::{BTreeMap};
use std::fs::read_to_string;
use std::ops::{Add};

pub const FILESYSTEM_SIZE: i32 = 70000000;


pub struct Filesystem {
    pub files: BTreeMap<String, File>,
    pub current_directory_name: String,
}

impl Filesystem {
    pub fn new(current_dir: File) -> Filesystem {
        let mut files = BTreeMap::new();
        files.insert(current_dir.name.clone(), current_dir.clone());
        return Filesystem { current_directory_name: current_dir.name.clone(), files };
    }

    pub fn run_all_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            self.run_command(&command);
        }
    }

    pub fn run_command(&mut self, command: &Command) {
        if command.is_ls() {
            self.ls(command);
        }
        if command.is_cd() {
            self.cd(command);
        }
    }

    fn cd(&mut self, command: &Command) {
        let cd_name = command.get_cd_dir_name();
        self.current_directory_name = if cd_name.eq("..") {
            self.cd_out()
        } else {
            self.cd_in(&cd_name)
        }
    }

    fn cd_out(&mut self) -> String {
        return if self.current_directory_name.eq("/") {
            self.current_directory_name.clone()
        } else {
            let mut collected_string = String::new();
            let split: Vec<String> = self.current_directory_name.split("/").map(|s| s.to_string()).collect();
            split.iter().skip(1).take(split.len() - 3).for_each(|s| {
                collected_string.push_str("/");
                collected_string.push_str(s);
            });
            collected_string.push_str("/");
            collected_string
        };
    }

    fn cd_in(&mut self, cd_name: &String) -> String {
        let new_dir_name = self.current_directory_name.clone().add(&cd_name);
        self.add_if_missing_dir(&new_dir_name);
        new_dir_name
    }

    fn add_if_missing_dir(&mut self, new_dir_name: &String) {
        if !self.files.contains_key(new_dir_name) {
            self.files.insert(new_dir_name.clone(), File::new_directory(new_dir_name.clone()));
        }
    }

    fn add_if_missing_file(&mut self, new_file_name: &String, file_size: i32) {
        if !self.files.contains_key(new_file_name) {
            self.files.insert(new_file_name.clone(), File { name: new_file_name.clone(), size: file_size });
        }
    }

    fn ls(&mut self, command: &Command) {
        command.terminal_outputs.iter()
            .for_each(|s| {
                if s.starts_with("dir ") {
                    let dir_name = self.current_directory_name.clone()
                        .add(s.strip_prefix("dir ").unwrap())
                        .add("/");
                    self.add_if_missing_dir(&dir_name);
                } else {
                    let mut whitespace_split = s.split_whitespace();
                    let size = whitespace_split.next().unwrap().parse().unwrap();
                    let filename = self.current_directory_name.clone().add(whitespace_split.next().unwrap());
                    self.add_if_missing_file(&filename, size);
                }
            });
    }

    pub fn get_directory_size(&self, directory_name: &String) -> i32 {
        let mut sum = 0;
        for (filename, file) in &self.files {
            if filename.contains(directory_name) && directory_name.ne(filename) {
                if !file.is_directory() {
                    sum += file.size;
                }
            }
        }
        return sum;
    }

    pub fn get_directories_sum(&self, directory_max_size: i32) -> i32 {
        let mut sum = 0;
        for (_key, file) in &self.files {
            if file.is_directory() {
                let file_size = self.get_directory_size(&file.name);
                if file_size <= directory_max_size {
                    sum += file_size;
                }
            }
        }
        return sum;
    }

    pub fn get_smallest_bigger_than(&self, threshold_size: i32) -> i32 {
        let mut closest_above_threshold = FILESYSTEM_SIZE;
        for (filename, file) in &self.files {
            if file.is_directory() {
                let size = self.get_directory_size(filename);
                if size > threshold_size && size < closest_above_threshold {
                    closest_above_threshold = size;
                }
            }
        }
        closest_above_threshold
    }

    #[allow(dead_code)]
    pub fn print_state(&self) {
        for (key, value) in &self.files {
            println!("{}    {:?}", key, value);
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct File {
    pub name: String,
    pub size: i32,
}

impl File {
    pub fn is_directory(&self) -> bool {
        return self.name.ends_with("/");
    }

    pub fn new_directory(name: String) -> File {
        File { size: 0, name }
    }
}

pub struct Command {
    pub command: String,
    pub terminal_outputs: Vec<String>,
}

impl Command {
    pub fn build(raw: String) -> Command {
        let mut lines = raw.lines();
        let command = lines.next().unwrap().to_string();
        let terminal_outputs = lines.map(|s| s.to_string()).collect();
        return Command { command, terminal_outputs };
    }

    pub fn is_cd(&self) -> bool {
        return self.command.starts_with("cd ").to_owned();
    }

    pub fn is_ls(&self) -> bool {
        return self.command.starts_with("ls").to_owned();
    }

    pub fn get_cd_dir_name(&self) -> String {
        let cd_name = self.command.split("cd ")
            .skip(1)
            .next()
            .expect("No dir name found.")
            .to_string();
        return if cd_name.ne("/") && cd_name.ne("..") {
            cd_name.add("/")
        } else {
            cd_name
        };
    }

    #[allow(dead_code)]
    pub fn print_command(&self) {
        println!("Command: [{}]", self.command);
    }

    #[allow(dead_code)]
    pub fn print_output(&self) {
        println!("{:?}", self.terminal_outputs);
    }
}


pub fn read_input_to_filesystem(filename: &str) -> Filesystem {
    let mut commands = read_to_string(filename)
        .expect("Could not read file")
        .split("$ ")
        .skip(1)
        .map(|s| Command::build(s.to_string()))
        .collect::<Vec<Command>>();

    let first_command = commands.remove(0);
    let root_directory = File { name: first_command.get_cd_dir_name(), size: 0 };
    let mut filesystem = Filesystem::new(root_directory);
    filesystem.run_all_commands(commands);
    filesystem
}