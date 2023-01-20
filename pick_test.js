// Build lib: wasm-pack build --target nodejs -- --features js
// Run tests: node pick_test.js

const { pick_one_from_weighted_list } = require('./pkg/nois');

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

test(() => pick_one_from_weighted_list(randomness, [["red", 15]]));
test(() => pick_one_from_weighted_list(randomness, [["red", 15], ["blue", 15]]));
test(() => pick_one_from_weighted_list(randomness, [["left", 99], ["right", 1]]));
test(() => pick_one_from_weighted_list(randomness, [["left", 1], ["right", 99]]));

// Cannot pick from empty
test(() => pick_one_from_weighted_list(randomness, []));

// Cannot weight is 0
test(() => pick_one_from_weighted_list(randomness, [["red", 0]]));

// Wrong weight type
test(() => pick_one_from_weighted_list(randomness, [["red", null]]));
test(() => pick_one_from_weighted_list(randomness, [["red", "42"]]));
test(() => pick_one_from_weighted_list(randomness, [["red", Number.NaN]]));
test(() => pick_one_from_weighted_list(randomness, [["red", Number.MAX_SAFE_INTEGER]])); // Exceeds u32

// Sum of weights exceeds uint32
test(() => pick_one_from_weighted_list(randomness, [["red", 4294967295], ["blue", 1]]));

// Element with 3 components
test(() => pick_one_from_weighted_list(randomness, [["left", 1], ["right", 99, "troll"]]));
