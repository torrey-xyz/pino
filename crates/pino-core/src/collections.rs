//! Stack-allocated collections for zero-heap-allocation patterns.
//!
//! This module provides efficient, bounded collections that use stack memory
//! instead of heap allocation, perfect for Solana programs where CU efficiency
//! is critical.

use core::{
    mem::{MaybeUninit, size_of},
    slice, ptr, fmt,
};
use crate::error::PinoError;

/// A stack-allocated vector with a fixed maximum capacity.
///
/// This provides Vec-like functionality without heap allocation.
pub struct StackVec<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> StackVec<T, N> {
    /// Creates a new empty StackVec.
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the number of elements in the vector.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the maximum capacity of the vector.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns true if the vector is at capacity.
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Pushes an element to the end of the vector.
    pub fn push(&mut self, value: T) -> Result<(), PinoError> {
        if self.len >= N {
            return Err(PinoError::Custom(0x1001)); // StackVec capacity exceeded
        }

        self.data[self.len] = MaybeUninit::new(value);
        self.len += 1;
        Ok(())
    }

    /// Removes and returns the last element, or None if empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        Some(unsafe { self.data[self.len].assume_init_read() })
    }

    /// Returns a slice of the elements.
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data.as_ptr() as *const T, self.len) }
    }

    /// Returns a mutable slice of the elements.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data.as_mut_ptr() as *mut T, self.len) }
    }

    /// Clears the vector, removing all values.
    pub fn clear(&mut self) {
        for i in 0..self.len {
            unsafe {
                self.data[i].assume_init_drop();
            }
        }
        self.len = 0;
    }

    /// Returns an iterator over the elements.
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.as_slice().iter()
    }

    /// Returns a mutable iterator over the elements.
    pub fn iter_mut(&mut self) -> core::slice::IterMut<T> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T, const N: usize> Drop for StackVec<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<T, const N: usize> Default for StackVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// A stack-allocated map with a fixed maximum capacity.
///
/// This provides HashMap-like functionality without heap allocation.
/// Uses a simple linear search for small maps (ideal for N < 32).
pub struct StackMap<K, V, const N: usize> {
    data: [MaybeUninit<(K, V)>; N],
    len: usize,
}

impl<K, V, const N: usize> StackMap<K, V, N> {
    /// Creates a new empty StackMap.
    pub const fn new() -> Self {
        Self {
            data: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the number of key-value pairs in the map.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the map is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the maximum capacity of the map.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns true if the map is at capacity.
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Clears the map, removing all key-value pairs.
    pub fn clear(&mut self) {
        for i in 0..self.len {
            unsafe { self.data[i].assume_init_drop() };
        }
        self.len = 0;
    }
}

impl<K: PartialEq, V, const N: usize> StackMap<K, V, N> {
    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: K, value: V) -> Result<Option<V>, PinoError> {
        // First check if key already exists
        for i in 0..self.len {
            let (existing_key, _) = unsafe { self.data[i].assume_init_ref() };
            if existing_key == &key {
                let (_, old_value) = unsafe { self.data[i].assume_init_read() };
                self.data[i] = MaybeUninit::new((key, value));
                return Ok(Some(old_value));
            }
        }

        // Key doesn't exist, add new entry
        if self.len >= N {
            return Err(PinoError::Custom(0x1002)); // StackMap is full
        }

        self.data[self.len] = MaybeUninit::new((key, value));
        self.len += 1;
        Ok(None)
    }

    /// Gets a reference to the value associated with the key.
    pub fn get(&self, key: &K) -> Option<&V> {
        for i in 0..self.len {
            let (existing_key, value) = unsafe { self.data[i].assume_init_ref() };
            if existing_key == key {
                return Some(value);
            }
        }
        None
    }

    /// Gets a mutable reference to the value associated with the key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for i in 0..self.len {
            // Check the key first without borrowing the value mutably
            let key_matches = {
                let (existing_key, _) = unsafe { self.data[i].assume_init_ref() };
                existing_key == key
            };
            
            if key_matches {
                let (_, value) = unsafe { self.data[i].assume_init_mut() };
                return Some(value);
            }
        }
        None
    }

    /// Removes a key-value pair from the map.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        for i in 0..self.len {
            let (existing_key, _) = unsafe { self.data[i].assume_init_ref() };
            if existing_key == key {
                let (_, value) = unsafe { self.data[i].assume_init_read() };
                
                // Shift remaining elements down
                for j in i..self.len - 1 {
                    let moved = unsafe { self.data[j + 1].assume_init_read() };
                    self.data[j] = MaybeUninit::new(moved);
                }
                
                self.len -= 1;
                return Some(value);
            }
        }
        None
    }

    /// Returns true if the map contains the key.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }
}

impl<K, V, const N: usize> Drop for StackMap<K, V, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

impl<K, V, const N: usize> Default for StackMap<K, V, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// A stack-allocated set with a fixed maximum capacity.
pub type StackSet<T, const N: usize> = StackMap<T, (), N>;

/// A stack-allocated string with a fixed maximum capacity.
///
/// This provides String-like functionality without heap allocation.
pub struct StackString<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> StackString<N> {
    /// Creates a new empty StackString.
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    /// Returns the length in bytes.
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the string is empty.
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the maximum capacity in bytes.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns true if the string is at capacity.
    pub const fn is_full(&self) -> bool {
        self.len == N
    }

    /// Appends a string slice to this string.
    pub fn push_str(&mut self, s: &str) -> Result<(), PinoError> {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N {
            return Err(PinoError::Custom(0x1003)); // StackString capacity exceeded
        }

        self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    /// Appends a character to this string.
    pub fn push(&mut self, ch: char) -> Result<(), PinoError> {
        let mut buffer = [0; 4];
        let s = ch.encode_utf8(&mut buffer);
        self.push_str(s)
    }

    /// Appends a formatted public key to this string.
    pub fn push_pubkey(&mut self, pubkey: &[u8; 32]) -> Result<(), PinoError> {
        // Simple hex encoding for pubkey
        for byte in pubkey {
            if self.len + 2 > N {
                return Err(PinoError::Custom(0x1003));
            }
            let hex_chars = [
                b"0123456789abcdef"[(*byte >> 4) as usize],
                b"0123456789abcdef"[(*byte & 0xf) as usize],
            ];
            self.data[self.len] = hex_chars[0];
            self.data[self.len + 1] = hex_chars[1];
            self.len += 2;
        }
        Ok(())
    }

    /// Returns the string as a &str.
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.data[..self.len]) }
    }

    /// Clears the string.
    pub fn clear(&mut self) {
        self.len = 0;
    }
} 