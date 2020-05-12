console.log(process.cwd())
const { cluster_from_js_array } = require("../native/index.node");
const { makeClusters } = require('../javascript')
const fs = require('fs');
const Assert = require('assert')

const hashArr = JSON.parse(fs.readFileSync('./test/hashes.json', "utf-8")).slice(0, 1000);
const input = hashArr.map(hash => ({ hash }))


let hrstart

// JS CLUSTERS

console.log(`JS duplicate detection of ${input.length} images started`)
hrstart = process.hrtime();
const jsClusters = makeClusters(input, 5);
console.log(`It took ${hrToMs(process.hrtime(hrstart))} ms`);

// NATIVE CLUSTERS

console.log(`Native duplicate detection of ${input.length} images started`)
hrstart = process.hrtime();
const nativeClusters = cluster_from_js_array(hashArr, 5);
console.log(`It took ${hrToMs(process.hrtime(hrstart))} ms`);

// COMPARISON

Assert.deepEqual(jsClusters[0].clusters, nativeClusters)

function hrToMs(hr) {
  return hr[0] * 1000 + hr[1] / 1000000
}