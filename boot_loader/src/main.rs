#![no_std]
#![no_main]

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) {
    println!("Hello, world!");
}
