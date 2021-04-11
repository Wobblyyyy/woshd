# woshd / whdOS
Rust-based OS.

## Planned Features and Order
- Global descriptor table
- Usable VGA-based terminal (0xb8000)
- Interoperable command-based system processes
- Permission system
- Paging (memory allocation)
- Dynamic filesystem (move out of RAM)
- Parallel and asynchronous task scheduling
- Multithreading 
- Global HAL

## Running
Build the project:

`> cargo build`

Create a boot image:

`> cargo bootimage`

Run the binary (I use QEMU)

`> qemu-system-x86_64 -drive format=raw,file=target/x86_64-v7em/debug/bootimage-woshd.bin`

If you get an issue with locating BIOS, add:

`> -L <path to qemu executables>`