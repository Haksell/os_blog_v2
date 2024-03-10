# KFS

### todo

-   [ ] chapter08
-   [ ] chapter09
-   [ ] chapter10
-   [ ] chapter11
-   [ ] chapter12
-   [ ] diff with https://github.com/phil-opp/blog_os/tree/post-12
-   [ ] colorful tests (with color module based on `colored`)
-   [ ] read https://os.phil-opp.com/disable-simd/
-   [ ] read https://os.phil-opp.com/red-zone/
-   [ ] fix warnings
-   [ ] 64-bit -> 32-bit (https://wiki.osdev.org/Bare_Bones)
-   [ ] keep 64-bit as turbo-bonus?
-   [ ] rust 2018 → 2021
-   [ ] write own bootloader for multiboot/grub (https://os.phil-opp.com/edition-1/)
-   [ ] stop using x86_64 module? (https://os.phil-opp.com/edition-1/extra/naked-exceptions/)
-   [ ] write own `pic8259` or `apic` module?
-   [ ] write own `pc-keyboard` module?
-   [ ] `qemu` in terminal like lsimanic
-   [ ] finish kfs1
-   [ ] clean
-   [ ] find project name and rebrand
-   [ ] push kfs1
-   [ ] nice help menu with `Code page 437` border characters
-   [ ] VGA history and scroll
-   [ ] use KVM on top of QEMU?

### subject

-   [ ] A kernel you can boot via GRUB
-   [ ] An ASM bootable base
-   [ ] A basic kernel library, with basic functions and types
-   [x] Some basic code to print some stuff on the screen
-   [ ] A basic "Hello world" kernel
-   [ ] You cannot use an existing linker in order to link your kernel. As written above, your kernel will not boot. So, you must create a linker for your kernel. Be careful, you CAN use the ’ld’ binary available on your host, but you CANNOT use the .ld file of your host.
-   [ ] The i386 (x86) architecture is mandatory (you can thank me later).
-   [ ] Install GRUB on a virtual image (???)
-   [ ] Write an ASM boot code that handles multiboot header, and use GRUB to init and call main function of the kernel itself.
-   [ ] Write basic kernel code of the choosen language.
-   [ ] Compile it with correct flags, and link it to make it bootable.
-   [ ] Once all of those steps above are done, you can write some helpers like kernel types or basic functions (strlen, strcmp, ...)
-   [ ] Your work must not exceed 10 MB.
-   [ ] Code the interface between your kernel and the screen.
-   [x] Display "42" on the screen.
-   [ ] For the link part, you must create a linker file with the GNU linker (ld).
-   [ ] Your Makefile must compile all your source files with the right flags and the right compiler.
-   [ ] After compilation, all the objects must be linked together in order to create the final Kernel binary.
-   [ ] Add scroll and cursor support to your I/O interface.
-   [ ] Add colors support to your I/O interface.
-   [x] Add helpers like printf / printk in order to print information / debug easily.
-   [ ] Handle keyboard entries and print them.
-   [ ] Handle different screens, and keyboard shortcuts to switch easily between then.

### resources

-   https://os.phil-opp.com/
-   https://os.phil-opp.com/edition-1/
-   https://osdev.org/Main_Page
-   http://www.brokenthorn.com/Resources/OSDevIndex.html
-   https://github.com/rust-osdev
-   https://singlelogin.re/book/25182527/e03396/modern-operating-systems.html
-   https://www.gnu.org/software/grub/manual/multiboot2/multiboot.pdf
-   `#os-dev`: https://discord.com/channels/273534239310479360/375706574133526529
-   improvement ideas: https://chat.openai.com/share/8aff468f-4ab9-4f35-80ca-a0425d1e8d83

### garbage

```rust
use blog_os::print;
let mut c: u8 = 32;
loop {
    print!("{}", c as char);
    c = if c == 126 { 32 } else { c + 1 };
    for _ in 0..10000 {
        volatile::Volatile::new(0).read();
    }
}
```

![artistic direction](https://upload.wikimedia.org/wikipedia/commons/a/a0/VirtualBox_TempleOS_x64_27_02_2021_20_43_48.png)
