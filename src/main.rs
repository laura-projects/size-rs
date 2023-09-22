use std::mem::size_of;
use libc::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
}

fn print_size(struct_type: &str, size: usize) {
    println!("\t{struct_type}: {size} bytes - {} bits", size*8)
}


fn print_integer_sizes() {
    println!("Integer sizes:");

    print_size("u8", size_of::<u8>());
    print_size("u16", size_of::<u16>());
    print_size("u32", size_of::<u32>());
    print_size("u64", size_of::<u64>());
    print_size("u128", size_of::<u128>());
}

fn print_ids_types() {
    println!("Ids types:");

    print_size("pid_t", size_of::<pid_t>());
    print_size("uid_t", size_of::<uid_t>());
    print_size("gid_t", size_of::<gid_t>());
    print_size("id_t", size_of::<id_t>());
}



fn print_files_related_sizes() {
    println!("Files related sizes:");

    print_size("ino_t", size_of::<ino_t>());
    print_size("off_t", size_of::<off_t>());
    print_size("loff_t", size_of::<loff_t>());
    print_size("dev_t", size_of::<dev_t>());
    print_size("dirent", size_of::<dirent>());

    print_size("ino64_t", size_of::<ino64_t>());
    print_size("off64_t", size_of::<off64_t>());
    print_size("dirent64", size_of::<dirent64>());
}

fn print_lengths() {

    println!("Length sizes:");

    print_size("size_t", size_of::<size_t>());
    print_size("ssize_t", size_of::<ssize_t>());

}


fn main() {
    let _args = Args::parse();
    print_integer_sizes();
    print_ids_types();
    print_files_related_sizes();
    print_lengths();
}
