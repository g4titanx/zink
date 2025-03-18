//! Double key mapping

use crate::{
    ffi,
    storage::{StorageValue, TransientStorageValue},
    Asm,
};

/// Storage mapping interface
pub trait DoubleKeyMapping {
    const STORAGE_SLOT: i32;

    type Key1: Asm;
    type Key2: Asm;
    type Value: StorageValue;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key1: Self::Key1, key2: Self::Key2) -> [u8; 32];

    /// Get value from storage key.
    #[inline(always)]
    fn get(key1: Self::Key1, key2: Self::Key2) -> Self::Value {
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        Self::Value::sload()
    }

    /// Set key and value
    #[inline(always)]
    fn set(key1: Self::Key1, key2: Self::Key2, value: Self::Value) {
        value.push();
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        unsafe {
            ffi::evm::sstore();
        }
    }
}

/// Transient storage mapping interface
pub trait DoubleKeyTransientMapping {
    const STORAGE_SLOT: i32;

    type Key1: Asm;
    type Key2: Asm;
    type Value: TransientStorageValue;

    #[cfg(not(target_family = "wasm"))]
    fn storage_key(key1: Self::Key1, key2: Self::Key2) -> [u8; 32];

    /// Get value from transient storage key.
    #[inline(always)]
    fn get(key1: Self::Key1, key2: Self::Key2) -> Self::Value {
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        Self::Value::tload()
    }

    /// Set key and value in transient storage
    #[inline(always)]
    fn set(key1: Self::Key1, key2: Self::Key2, value: Self::Value) {
        value.push();
        load_double_key(key1, key2, Self::STORAGE_SLOT);
        unsafe {
            ffi::evm::tstore();
        }
    }
}

/// Load storage key to stack
#[inline(always)]
pub fn load_double_key(key1: impl Asm, key2: impl Asm, index: i32) {
    unsafe {
        ffi::label_reserve_mem_64();

        // write key1 to memory
        key1.push();
        ffi::evm::push0();
        ffi::evm::mstore();

        // write index to memory
        index.push();
        ffi::asm::push_u8(0x20);
        ffi::evm::mstore();

        // hash key
        ffi::asm::push_u8(0x40);
        ffi::evm::push0();
        ffi::evm::keccak256();

        // stores the hash
        ffi::evm::push0();
        ffi::evm::mstore();

        // write index to memory
        key2.push();
        ffi::asm::push_u8(0x20);
        ffi::evm::mstore();

        // hash key
        ffi::asm::push_u8(0x40);
        ffi::evm::push0();
        ffi::evm::keccak256();
    }
}
