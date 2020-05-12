let jimp = require('jimp');

/**
 * Computes and returns a 64 bit perceptual hash (in string format) for a given image.
 * 
 * @param {Jimp} image  - A Jimp instance that contains a buffer with the image's data.
 */
let hashImage = async (image) => {
    const BINARY = 2
    const copy = image.clone();
    await copy.resize(500, 500);
    copy.greyscale();
    return copy.hash(BINARY);
}

module.exports = {
    hashImage: hashImage
}