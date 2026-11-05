pub struct MemoryPage {
    address: u64,
}

pub trait MemoryProvider {
    fn allocate_page() -> MemoryPage;
}
