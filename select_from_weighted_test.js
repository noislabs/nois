// Build lib: wasm-pack build --target nodejs -- --features js
// Run tests: node pick_test.js

const { select_from_weighted } = require('./pkg/nois');

let count = 0;

function test(f) {
  count += 1;
  try {
    const res = f();
    console.log(`Test ${count}:`, res);
  } catch (e) {
    console.error(`Test ${count}:`, e);
  }
}

const randomness = "2267ba7356c01a58e405d4194a31bddc3fd3eb1f0a86758f7a609ba8a47420ba"

test(() => select_from_weighted(randomness, [["red", 15]]));
test(() => select_from_weighted(randomness, [["red", 15], ["blue", 15]]));
test(() => select_from_weighted(randomness, [["left", 99], ["right", 1]]));
test(() => select_from_weighted(randomness, [["left", 1], ["right", 99]]));

// Cannot pick from empty
test(() => select_from_weighted(randomness, []));

// Cannot weight is 0
test(() => select_from_weighted(randomness, [["red", 0]]));

// Wrong weight type
test(() => select_from_weighted(randomness, [["red", null]]));
test(() => select_from_weighted(randomness, [["red", "42"]]));
test(() => select_from_weighted(randomness, [["red", Number.NaN]]));
test(() => select_from_weighted(randomness, [["red", Number.MAX_SAFE_INTEGER]])); // Exceeds u32

// Sum of weights exceeds uint32
test(() => select_from_weighted(randomness, [["red", 4294967295], ["blue", 1]]));

// Element with 3 components
test(() => select_from_weighted(randomness, [["left", 1], ["right", 99, "troll"]]));
