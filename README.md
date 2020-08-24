# hm

Toy hashmap library in rust. 

Key collisions are handled by swapping in the newer value for the key. 

## Dependencies

Nope. 

## Build

	cargo build

## Test

	cargo test

## Documentation

	cargo doc --open

## Examples

Generic usage:

	fn some_func() {
		let mut map = HashMap::mk();
		map.insert("foo", "bar");
		map.insert("foo", "lol");

		let x = map.get(&"foo");

		let v = map.delete(&"foo"); 

		...
	}
