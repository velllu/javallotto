use class_loader::ClassFile;

mod class_loader;
mod constants;
mod utilities;

fn main() {
    let contents = include_bytes!("../tests/Main.class");
    let class_file = ClassFile::from_bytes(contents);

    println!("{:?}", class_file);
}
