import {expect, test} from 'vitest'
import * as fs from 'fs';

const version = require("../index").version
const document = require("../index").document
test('Version', () => {
    expect(version()).toBe("0.2.0")
})
test('Document', async () => {
    const jsonFile = fs.readFileSync("./benchmark/docs/complex.pdf")
    const content = document(jsonFile)
    expect(content).not.toBeNull()

    console.log(content)

})