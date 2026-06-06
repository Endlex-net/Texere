#!/usr/bin/env node

/**
 * Prepare stable macOS release artifacts for Homebrew cask consumption.
 *
 * Usage: node scripts/prepare-macos-release.mjs <version>
 *
 * Finds DMG files under src-tauri/target/**\/bundle/dmg/ and copies them
 * into release-artifacts/ with stable, predictable names:
 *   Texere-<version>-<arch>.dmg
 *
 * Also generates:
 *   - Texere-<version>-checksums.txt  (SHA256)
 *   - Texere-<version>-brew-meta.txt   (cask maintenance info)
 */

import { readdirSync, readFileSync, copyFileSync, mkdirSync, existsSync, writeFileSync } from 'node:fs'
import { createHash } from 'node:crypto'
import { join, basename, resolve, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const ROOT = resolve(__dirname, '..')
const ARTIFACTS_DIR = join(ROOT, 'release-artifacts')
const TARGET_DIR = join(ROOT, 'src-tauri', 'target')

// --- helpers ---

function findDmgs(dir) {
  const results = []
  const stack = [dir]
  while (stack.length > 0) {
    const current = stack.pop()
    let entries
    try {
      entries = readdirSync(current, { withFileTypes: true })
    } catch {
      continue
    }
    for (const e of entries) {
      const full = join(current, e.name)
      if (e.isDirectory() && e.name === 'dmg') {
        // Direct dmg bundle directory from tauri
        try {
          const dmgs = readdirSync(full).filter(f => f.endsWith('.dmg'))
          for (const d of dmgs) {
            results.push(join(full, d))
          }
        } catch { /* skip */ }
      } else if (e.isDirectory()) {
        stack.push(full)
      }
    }
  }
  return results
}

/**
 * Map a source DMG filename to a stable arch label.
 * tauri bundler produces names like: Texere_0.1.0_aarch64.dmg
 */
function detectArch(filename) {
  const lower = filename.toLowerCase()
  if (lower.includes('universal')) return 'universal'
  if (lower.includes('aarch64') || lower.includes('arm64')) return 'aarch64'
  if (lower.includes('x86_64') || lower.includes('x64') || lower.includes('amd64')) return 'x64'
  return 'unknown'
}

function sha256File(filePath) {
  const hash = createHash('sha256')
  hash.update(readFileSync(filePath))
  return hash.digest('hex')
}

// --- main ---

const version = process.argv[2]
if (!version) {
  console.error('Usage: node scripts/prepare-macos-release.mjs <version>')
  process.exit(1)
}

console.log(`Preparing macOS release artifacts for version ${version}...`)

// Find existing DMGs
const dmgFiles = findDmgs(TARGET_DIR)

if (dmgFiles.length === 0) {
  console.error('No .dmg files found under src-tauri/target/. Did you build first?')
  process.exit(1)
}

console.log(`Found ${dmgFiles.length} DMG file(s):`)
dmgFiles.forEach(f => console.log(`  ${f}`))

// Create artifacts directory
if (!existsSync(ARTIFACTS_DIR)) {
  mkdirSync(ARTIFACTS_DIR, { recursive: true })
}

const seen = new Map() // arch -> { name, path, arch }

for (const src of dmgFiles) {
  const arch = detectArch(basename(src))
  if (arch === 'unknown') {
    console.warn(`  Skipping ${basename(src)} — could not detect architecture`)
    continue
  }
  const stableName = `Texere-${version}-${arch}.dmg`
  const dest = join(ARTIFACTS_DIR, stableName)

  // Prefer release builds over debug when both exist
  const isRelease = src.includes('/release/')
  const existing = seen.get(arch)
  if (existing && !isRelease) {
    console.warn(`  Skipping ${basename(src)} — preferring existing from release build`)
    continue
  }
  if (existing && isRelease) {
    console.warn(`  Replacing ${basename(existing.src)} with release build`)
  }
  seen.set(arch, { name: stableName, path: dest, arch, src })

  copyFileSync(src, dest)
  console.log(`  Copied: ${basename(src)} → ${stableName}`)
}

const stableFiles = [...seen.values()]

if (stableFiles.length === 0) {
  console.error('No stable artifacts produced.')
  process.exit(1)
}

// Generate SHA256 checksums
const checksumLines = []
for (const f of stableFiles) {
  const hash = sha256File(f.path)
  checksumLines.push(`${hash}  ${f.name}`)
}
const checksumFile = join(ARTIFACTS_DIR, `Texere-${version}-checksums.txt`)
writeFileSync(checksumFile, checksumLines.join('\n') + '\n')
console.log(`Generated: Texere-${version}-checksums.txt`)

// Generate Homebrew cask maintenance info
const metaLines = [
  `# Texere ${version} — Homebrew Cask Maintenance Info`,
  `# Generated: ${new Date().toISOString()}`,
  '',
  `version "${version}"`,
  '',
]
for (const f of stableFiles) {
  const hash = sha256File(f.path)
  const downloadUrl = `https://github.com/Endlex-net/Texere/releases/download/v${version}/${f.name}`
  metaLines.push(`# ${f.arch}`)
  metaLines.push(`#   url "${downloadUrl}"`)
  metaLines.push(`#   sha256 "${hash}"`)
  metaLines.push('')
}

const metaFile = join(ARTIFACTS_DIR, `Texere-${version}-brew-meta.txt`)
writeFileSync(metaFile, metaLines.join('\n'))
console.log(`Generated: Texere-${version}-brew-meta.txt`)

console.log(`\nDone. Artifacts in ${ARTIFACTS_DIR}/`)
