import {expect, test} from 'vitest'
import * as fs from 'fs';

const version = require("../index").version
const document = require("../index").document
test('Version', () => {
    expect(version()).toBe("0.0.1")
})
test('Document', async () => {
    const jsonFile = fs.readFileSync("Dewmal Resume.pdf")
    console.log(jsonFile)
    expect(document(jsonFile)).not.toBeNull()
})