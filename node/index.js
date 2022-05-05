const init = () => Promise.resolve()
module.exports = init

const { add } = require('./binding')

// you can modify binding here
module.exports.add = add
