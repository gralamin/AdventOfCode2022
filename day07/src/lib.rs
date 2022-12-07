extern crate filelib;

pub use filelib::load_no_blanks;
use std::cell::RefCell;
use std::rc::Rc;

// Function for use with doctests.
pub fn get_puzzle_sample() -> Vec<String> {
    return vec![
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

// Representive of the file, has the label and size, we only actually use the size.
// By using lifetimes here, we can use &str, instead of strings that we need to clone.
#[derive(Debug)]
struct PuzzleFile<'a> {
    label: &'a str,
    size: usize,
}

impl<'a> PuzzleFile<'a> {
    fn get_label(&self) -> &'a str {
        return self.label;
    }
}

// We need to make a graph, which requires the Rc RefCell trick.
// Two ways to think of this
// 1: Just a bunch of syntax sugar, as long as you use it consistently,
// and use Clone() where required, and .borrow() or .borrow_mut() whenever
// you touch the internals.
// 2: Rc is a pointer with shared ownership
// RefCell provides interior mutability.
// Essentially this is a shared box that the owner can edit the contents of.
// You need the RefCell, because without it, rust gets mad at PuzzleDir in
// PuzzleDir. The RefCell provides a "box" to hide that.
// You then need Rc, which acts as a reference counter, to share it between
// subdirs.
#[derive(Debug)]
struct PuzzleDir<'a> {
    files: Vec<Rc<RefCell<PuzzleFile<'a>>>>,
    subdirs: Vec<Rc<RefCell<PuzzleDir<'a>>>>,
    label: &'a str,
}

impl<'a> PuzzleDir<'a> {
    pub fn new(label: &'a str) -> PuzzleDir<'a> {
        return PuzzleDir {
            files: vec![],
            subdirs: vec![],
            label: label,
        };
    }

    pub fn get_dir_size(&self) -> usize {
        let file_sizes: usize = self.files.iter().map(|f| f.borrow().size).sum();
        let subdir_sizes: usize = self.subdirs.iter().map(|d| d.borrow().get_dir_size()).sum();

        // This line is just here to make some "dead code" warnings go away, so I keep debug info in.
        let _ = self
            .files
            .iter()
            .map(|f| f.borrow().get_label())
            .collect::<Vec<&str>>();

        return file_sizes + subdir_sizes;
    }

    fn add_file(&mut self, f: PuzzleFile<'a>) {
        self.files.push(Rc::new(RefCell::new(f)));
    }

    fn add_directory(&mut self, d: Rc<RefCell<PuzzleDir<'a>>>) {
        self.subdirs.push(d);
    }

    fn find_subdir_by_label(&self, l: &str) -> Rc<RefCell<PuzzleDir<'a>>> {
        let matching: Rc<RefCell<PuzzleDir>> = self
            .subdirs
            .iter()
            .filter(|&d| d.borrow().label == l)
            .nth(0)
            .unwrap()
            .clone();
        return matching;
    }
}

// A consquence of this is its best to always pass around Rc RefCells, you can do it without, but this is
// honestly more intuitive.
fn parse_terminal_output(termtext: &Vec<String>) -> Rc<RefCell<PuzzleDir>> {
    let mut directory_stack: Vec<Rc<RefCell<PuzzleDir>>> = vec![];

    // cd / is first
    let top_dir = Rc::new(RefCell::new(PuzzleDir::new("/")));
    let mut cur_dir = top_dir.clone();

    let mut is_ls = false;
    for line in termtext.into_iter().skip(1) {
        if is_ls && line.starts_with("$") {
            // end if is_ls
            is_ls = false;
        }
        if is_ls {
            if line.starts_with("dir") {
                let (_, label) = line.split_once("dir ").unwrap();
                let new_dir = Rc::new(RefCell::new(PuzzleDir::new(label)));
                cur_dir.borrow_mut().add_directory(new_dir);
            } else {
                let (size, label) = line.split_once(" ").unwrap();
                let s = size.parse::<usize>().unwrap();
                let f = PuzzleFile {
                    label: label,
                    size: s,
                };
                cur_dir.borrow_mut().add_file(f);
            }
            continue;
        }
        if line.starts_with("$ ls") {
            is_ls = true;
            continue;
        }
        if line.starts_with("$ cd ") {
            let (_, cmd_dir) = line.split_once("$ cd ").unwrap();
            if cmd_dir == ".." {
                cur_dir = directory_stack.pop().unwrap().clone();
            } else {
                let found_dir = cur_dir.borrow().find_subdir_by_label(cmd_dir);
                directory_stack.push(cur_dir.clone());
                cur_dir = found_dir.clone();
            }
        }
    }

    return top_dir;
}

fn sum_matching_folders(root: Rc<RefCell<PuzzleDir>>, max_size: usize) -> usize {
    let mut sum = 0;

    let this_size = root.borrow().get_dir_size();
    if this_size <= max_size {
        sum += this_size;
    }
    for child in &root.borrow().subdirs {
        sum += sum_matching_folders(child.clone(), max_size);
    }
    return sum;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = day07::get_puzzle_sample();
/// assert_eq!(day07::puzzle_a(&vec1), 95437);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let max_size = 100000;
    let root = parse_terminal_output(input);
    return sum_matching_folders(root, max_size);
}

fn find_smallest_folder_of_min_size(root: Rc<RefCell<PuzzleDir>>, min_size: usize) -> usize {
    let mut cur_smallest = usize::MAX;

    let this_size = root.borrow().get_dir_size();
    if this_size >= min_size && this_size < cur_smallest {
        cur_smallest = this_size;
    }
    for child in &root.borrow().subdirs {
        let child_size = find_smallest_folder_of_min_size(child.clone(), min_size);
        if child_size < cur_smallest {
            cur_smallest = child_size;
        }
    }
    return cur_smallest;
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = day07::get_puzzle_sample();
/// assert_eq!(day07::puzzle_b(&vec1), 24933642);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let total_size = 70000000;
    let required_size = 30000000;
    let root = parse_terminal_output(input);
    let needed_space = total_size - root.borrow().get_dir_size();
    let min_size = required_size - needed_space;
    return find_smallest_folder_of_min_size(root, min_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Remove dead code warning on label by just calling it
    #[test]
    fn test_get_label() {
        let f = PuzzleFile {
            label: "foo",
            size: 5,
        };
        assert_eq!(f.get_label(), "foo");
    }
}
