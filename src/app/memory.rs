use ic_stable_structures::memory_manager::{MemoryId, MemoryManager as IcMemoryManager};
use ic_stable_structures::DefaultMemoryImpl;

pub const TOKENS_MEMORY_ID: MemoryId = MemoryId::new(10);
pub const TRANSACTIONS_MEMORY_ID: MemoryId = MemoryId::new(11);

pub const LOGO_MEMORY_ID: MemoryId = MemoryId::new(20);
pub const NAME_MEMORY_ID: MemoryId = MemoryId::new(21);
pub const SYMBOL_MEMORY_ID: MemoryId = MemoryId::new(22);
pub const CREATED_AT_MEMORY_ID: MemoryId = MemoryId::new(23);
pub const UPGRADED_AT_MEMORY_ID: MemoryId = MemoryId::new(24);
pub const CUSTODIANS_MEMORY_ID: MemoryId = MemoryId::new(25);
pub const SUPPORTED_INTERFACES_MEMORY_ID: MemoryId = MemoryId::new(26);

thread_local! {
    /// Memory manager
    pub static MEMORY_MANAGER: IcMemoryManager<DefaultMemoryImpl> = IcMemoryManager::init(DefaultMemoryImpl::default());
}
