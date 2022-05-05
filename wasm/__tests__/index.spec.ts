import assert from 'assert'
import { add } from '../pkg-node'

describe('test', () => {
  it('should work', () => {
    assert.equal(add(1, 2), 3)
  })
})
