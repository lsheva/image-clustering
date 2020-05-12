# Duplicate Detection

This library is intended to aid in the detection of duplicate images, including those that have some changes between them but are overall the same (like sepia filters, watermarks, have been resized).

This is achieved by using [perceptual hashing](https://en.wikipedia.org/wiki/Perceptual_hashing) and [hierarchical clustering](https://en.wikipedia.org/wiki/Hierarchical_clustering).

It's likely that the two main functions provided by this library will be used by different subsystems.


## Installation

`npm install git+ssh//git@github.com:wrethink/duplicate-detection.git`

SSH must be configured correctly on the machine for this to work. See the [official github documentation](https://help.github.com/en/articles/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent#adding-your-ssh-key-to-the-ssh-agent).

After the initial version, git tagging will be used to keep track of versions, they can be appended in `package.json` to use that specific release only.

## Creating perceptual hash

`hashImage(image)`

Where `image` is a `Jimp` instance that contains a buffer with the image's data. Be sure to take a look at the `Basic usage` and `Custom Constructor` section of the official [Jimp repo](https://github.com/oliver-moran/jimp/tree/master/packages/jimp) to see the different options for creating one.

Output will be a 64 length `string` binary representation like this:
`1000001000111000001001011000010000111000110001110011100010100001`


## Creating a cluster from hashed images

`makeClusters(images)`

Where `images` is an Array of objects that contain a `hash` property.
Also has a group of optional paramaters that can be used at a later date to quickly change the behavior. Defaults should provide a good starting point.

The most relevant clusters are located on the last level of the output object
```javascript
const levels = makeClusters(images);
let importantClusters = levels[levels.length - 1].clusters;
```
But feel free to explore the entire output to understand its behavior.