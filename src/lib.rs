use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;

// Set the number of buckets in a hashmap to 16
const NBUCKETS: usize = 16;

// Buckets are sets of (key,value) pair tuples
type Bucket<K, V> = Vec<(K, V)>;

pub struct HashMap<K, V> {
	buckets: Vec<Bucket<K, V>>, // Buckets speed up get()'ing values
}

/* Constrain K's type to:
	- require implementation of Hash trait
	- be comparable with == via PartialEq trait
	- be copy-able via Copy trait (shared by V)
*/
impl<K: Hash + PartialEq + Copy, V: Copy> HashMap<K, V> {
	// Create and return a new hashmap
	pub fn mk() -> Self {
		// Affix the max size of the buckets vector
		let mut buckets = Vec::with_capacity(NBUCKETS);

		// Initialize the bucket vectors
		for _ in 0..NBUCKETS {
			buckets.push(Vec::new());
		}

		Self { buckets }
	}

	// Insert a new value into the hashmap - should never fail
	pub fn insert(&mut self, key: K, val: V) -> Option<K> {
		let bucket = self.key2bucket_mut(&key);

		match bucket.iter_mut().find_map(|pair| {
			if pair.0 == key {
				// Swap collisions with new value
				std::mem::swap(pair, &mut (key, val));

				return Some(key);
			} else {
				return None;
			}
		}) {
			// Insert value into target bucket
			None => bucket.push((key, val)),

			// Nop
			_ => (),
		}

		return Some(key);
	}

	// Retrieve an existing value from the hashmap
	pub fn get(&self, key: K) -> Option<&V> {
		return self.key2bucket(&key).iter().find_map(|pair| {
			if pair.0 == key {
				return Some(&pair.1);
			} else {
				return None;
			}
		});
	}

	// Remove an existing value from the hashmap
	pub fn delete(&mut self, key: K) -> Option<V> {
		let bucket = self.key2bucket_mut(&key);

		// Enumerate on the iterator lets us extract the index of the pair
		match bucket.iter_mut().enumerate().find_map(|pair| {
			if (*pair.1).0 == key {
				// If we find the matching key, return index to delete
				return Some(pair.0);
			} else {
				return None;
			}
		}) {
			// Delete by index
			Some(i) => Some(bucket.remove(i).1),

			None => None,
		}
	}

	/* Support methods */

	// Calculate bucket index for a given key
	fn key2index(&self, key: &K) -> u64 {
		// Required to call hasher.finish()
		use std::hash::Hasher;

		// Hash the key
		let mut hasher = DefaultHasher::new();

		key.hash(&mut hasher);
		let hash = hasher.finish();

		let index = hash % (NBUCKETS as u64);

		return index;
	}

	// Return target bucket for a given key - immutable
	fn key2bucket(&self, key: &K) -> &Vec<(K, V)> {
		let index = self.key2index(key);
		let bucket = &self.buckets[index as usize];

		return bucket;
	}

	// Acquire target bucket for a given key - mutable
	fn key2bucket_mut(&mut self, key: &K) -> &mut Vec<(K, V)> {
		let index = self.key2index(key);
		let bucket = &mut self.buckets[index as usize];

		return bucket;
	}
}

/* Library unit test(s) */

// CFG -- Compiler FlaG
// Only compiling the `test` module
// Will not be compiled at all outside of `test` target
#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn map_works() {
		let mut map = HashMap::mk();

		assert_eq!(map.insert("duck", "quack"), Some("duck"));
		map.insert("foo", "bar");
		map.insert("foo", "lol");

		assert_eq!(map.get(&"foo"), Some(&"lol"));
		assert_eq!(map.get(&"foo"), Some(&"lol"));
		assert_eq!(map.get(&"qux"), None);

		assert_eq!(map.delete(&"foo"), Some("lol"));
		assert_eq!(map.get(&"foo"), None);
	}
}
