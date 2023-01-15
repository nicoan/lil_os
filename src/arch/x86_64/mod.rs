pub mod address;
pub mod gdt;
pub mod idt;
pub mod privilege;
pub mod registers;

/// Loads the x86 system tables
fn load_tables() {
    gdt::load_gdt();
    idt::load_idt();
}

/// Initializes the x86_64 arch
pub fn initialize_x86_64_arch() {
    load_tables();
}
