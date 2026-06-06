#!/usr/bin/env node

import { readFileSync, writeFileSync } from 'node:fs'
import { resolve, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const ROOT = resolve(__dirname, '..')

const nextVersion = process.argv[2]

if (!nextVersion) {
  console.error('Usage: node scripts/bump-version.mjs <version>')
  process.exit(1)
}

if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(nextVersion)) {
  console.error(`Invalid version: ${nextVersion}`)
  console.error('Expected: X.Y.Z or X.Y.Z-suffix')
  process.exit(1)
}

function updateJsonFile(path, updater) {
  const content = readFileSync(path, 'utf8')
  const data = JSON.parse(content)
  updater(data)
  writeFileSync(path, `${JSON.stringify(data, null, 2)}\n`)
}

function updateTextFile(path, transform) {
  const before = readFileSync(path, 'utf8')
  const after = transform(before)
  if (after === before) {
    console.warn(`No changes made in ${path}`)
  }
  writeFileSync(path, after)
}

const packageJsonPath = resolve(ROOT, 'package.json')
const tauriConfPath = resolve(ROOT, 'src-tauri', 'tauri.conf.json')
const cargoTomlPath = resolve(ROOT, 'src-tauri', 'Cargo.toml')
const readmePath = resolve(ROOT, 'README.md')

updateJsonFile(packageJsonPath, (data) => {
  data.version = nextVersion
})

updateJsonFile(tauriConfPath, (data) => {
  data.version = nextVersion
})

updateTextFile(cargoTomlPath, (content) => {
  let replaced = false
  const lines = content.split('\n')
  let inPackage = false

  const updated = lines.map((line) => {
    if (line.trim() === '[package]') {
      inPackage = true
      return line
    }
    if (inPackage && /^\[.+\]$/.test(line.trim())) {
      inPackage = false
      return line
    }
    if (inPackage && /^version\s*=\s*".*"\s*$/.test(line)) {
      replaced = true
      return `version = "${nextVersion}"`
    }
    return line
  })

  if (!replaced) {
    throw new Error('Could not find package.version in src-tauri/Cargo.toml')
  }

  return `${updated.join('\n').replace(/\n?$/, '\n')}`
})

updateTextFile(readmePath, (content) => {
  return content.replace(
    /https:\/\/img\.shields\.io\/badge\/version-[^-"\s]+-blue/g,
    `https://img.shields.io/badge/version-${nextVersion}-blue`
  )
})

console.log(`Updated Texere version to ${nextVersion} in:`)
console.log('- package.json')
console.log('- src-tauri/tauri.conf.json')
console.log('- src-tauri/Cargo.toml')
console.log('- README.md version badge')
