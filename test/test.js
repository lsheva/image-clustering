//@ts-check

const fs = require('fs');
const assert = require('assert')
const { makeClusters } = require('../javascript')
const addon = require('../native');

const hashArr = JSON.parse(fs.readFileSync('./test/hashes.json', "utf-8")).slice(0, 500);
const input = hashArr.map(hash => ({ hash }))

const compareHashes = (s1, s2) => {
  let counter = 0;

  for (let k = 0; k < s1.length; k++) {
    if (s1[k] !== s2[k]) {
      counter++;
    }
  }

  return counter / s1.length;
}
let singleHashDistance1 = (image1, image2) => {
  const res1 = Math.round(compareHashes(image1.hash, image2.hash) * 64);
  return res1;
}

let singleHashDistance2 = (image1, image2) => {
  const res2 = addon.compareHashes(image1.hash, image2.hash)
  return res2;
}

console.log(`JS duplicate detection of ${input.length} images started`)

let hrstart = process.hrtime();
const jsClusters = makeClusters(
  input,
  5
)
console.log(`It took ${hrToMs(process.hrtime(hrstart))} ms`);

hrstart = process.hrtime();
const jsClusters2 = makeClusters(
  input,
  5,
  singleHashDistance2
)
console.log(`It took ${hrToMs(process.hrtime(hrstart))} ms`);


store("./test/jsClusters.json", jsClusters)
store("./test/jsClusters2.json", jsClusters2)
assert.deepEqual(jsClusters, jsClusters2)

function hrToMs(hr) {
  return hr[0] * 1000 + hr[1] / 1000000
}

function store(path, data) {
  const str = JSON.stringify(data, null, 2);
  fs.writeFileSync(path, str);
}