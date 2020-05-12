let cluster = require('./cacheless_clustering');
let jimp = require('jimp');

/**
 * Returns the hamming distance (number of different bits) between two binary hashes.
 * 
 * @param {string} hash1 - A string that reperesents the binary format of a perceptual hash.
 * @param {string} hash2 - Another string that reperesents the binary format of a perceptual hash.
 */
let hammingDistance = (hash1, hash2) => {
    return Math.round(jimp.compareHashes(hash1, hash2) * 64);
}

/**
 * Returns the hamming distance between the hash properties of two objects.
 * @param {Object} image1 - An object, contents don't matter as long as it has a hash property.
 * @param {string} image1.hash - A string that reperesents the binary format of a perceptual hash.
 * @param {Object} image2 - An object, contents don't matter as long as it has a hash property.
 * @param {string} image2.hash - A string that reperesents the binary format of a perceptual hash.
 */
let singleHashDistance = (image1, image2) => hammingDistance(image1.hash, image2.hash);

/**
 * Returns the smallest hamming distance between two sets of hashes via each object's hashes property.
 * @param {Object} image1 - An object, contents don't matter as long as it has a hashes property.
 * @param {Array<string>} image1.hashes - An array of strings that reperesent the binary format of a perceptual hash.
 * @param {Object} image2 - An object, contents don't matter as long as it has a hashes property.
 * @param {Array<string>} image2.hashes - An array of strings that reperesent the binary format of a perceptual hash.
 */
let multiHashDistance = (image1, image2) => {
    let distances = [];
    image1.hashes.forEach(hash1 => {
        image2.hashes.forEach(hash2 => {
            const currentDifference = hammingDistance(hash1, hash2);
            distances.push(currentDifference);
        })
    })
    return singleLink(distances);
}

/**
 * Returns the minimum distance in a distances array.
 * @param {Array<number>} distances - An array of integers that represent multiple hamming distances
 */
let singleLink = (distances) => Math.min.apply(null, distances);

/**
 * Returns the maximum distance in a distances array.
 * @param {Array<number>} distances - An array of integers that represent multiple hamming distances
 */
let completeLink = (distances) => Math.max.apply(null, distances);

/**
 * Returns the average distance of a distances array.
 * @param {Array<number>} distances - An array of integers that represent multiple hamming distances
 */
let averageLink = (distances) => {
    let sum = 0;
    distances.forEach(distance => {
        sum = sum + distance;
    });
    return sum / distances.length;
}

/**
 * Creates a set of clusters 
 * 
 * @param {Array<>} images - Array of objects that contain a hash (or hashes) property
 * @param {number} maxLinkage - Cut-off linkage value at which two images or clusters will not link
 * @param {function} distance - Function that computes the distance between two images
 * @param {function} linkage - Function that describes the linkage between two clusters
 */
let makeClusters = (images, maxLinkage = 5, distance = singleHashDistance, linkage = completeLink) => {
    return cluster({
        input: images,
        distance,
        linkage,
        maxLinkage
    });
}

module.exports = {
    makeClusters,
    multiHashDistance,
    singleLink,
    completeLink,
    averageLink
}