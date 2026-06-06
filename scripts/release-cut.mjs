#!/usr/bin/env node

import { spawnSync } from 'node:child_process'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const ROOT = resolve(__dirname, '..')

const args = process.argv.slice(2)
const version = args.find((arg) => !arg.startsWith('--'))
const noPush = args.includes('--no-push')

if (!version) {
  console.error('Usage: node scripts/release-cut.mjs <version> [--no-push]')
  process.exit(1)
}

if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?$/.test(version)) {
  console.error(`Invalid version: ${version}`)
  console.error('Expected: X.Y.Z or X.Y.Z-suffix')
  process.exit(1)
}

function run(command, commandArgs, options = {}) {
  const result = spawnSync(command, commandArgs, {
    cwd: ROOT,
    stdio: options.capture ? ['ignore', 'pipe', 'pipe'] : 'inherit',
    encoding: 'utf8'
  })

  if (result.status !== 0) {
    if (options.capture) {
      if (result.stdout) process.stdout.write(result.stdout)
      if (result.stderr) process.stderr.write(result.stderr)
    }
    process.exit(result.status ?? 1)
  }

  return options.capture ? result.stdout.trim() : ''
}

function ensureCleanWorktree() {
  const status = run('git', ['status', '--porcelain'], { capture: true })
  if (status) {
    console.error('Working tree is not clean. Commit or stash changes before cutting a release.')
    process.exit(1)
  }
}

function ensureTagDoesNotExist(tag) {
  const existing = spawnSync('git', ['rev-parse', '--verify', '--quiet', tag], {
    cwd: ROOT,
    stdio: 'ignore'
  })
  if (existing.status === 0) {
    console.error(`Tag already exists: ${tag}`)
    process.exit(1)
  }
}

function currentBranch() {
  const branch = run('git', ['rev-parse', '--abbrev-ref', 'HEAD'], { capture: true })
  if (branch === 'HEAD') {
    console.error('Detached HEAD is not supported for release-cut.')
    process.exit(1)
  }
  return branch
}

ensureCleanWorktree()

const tag = `v${version}`
ensureTagDoesNotExist(tag)
const branch = currentBranch()

console.log(`Cutting release ${tag} on branch ${branch}...`)

run('node', ['scripts/bump-version.mjs', version])

const changedFiles = run('git', ['status', '--porcelain'], { capture: true })
if (!changedFiles) {
  console.error('No version changes detected after bump-version. Aborting release-cut.')
  process.exit(1)
}

run('git', ['add', 'package.json', 'src-tauri/tauri.conf.json', 'src-tauri/Cargo.toml', 'README.md'])
run('git', ['commit', '-m', `chore(release): cut ${tag}`])
run('git', ['tag', '-a', tag, '-m', tag])

if (noPush) {
  console.log(`Release commit and tag created locally:`)
  console.log(`- branch commit: chore(release): cut ${tag}`)
  console.log(`- tag: ${tag}`)
  console.log('Push manually when ready:')
  console.log(`  git push origin ${branch}`)
  console.log(`  git push origin ${tag}`)
  process.exit(0)
}

const upstream = spawnSync('git', ['rev-parse', '--abbrev-ref', '--symbolic-full-name', '@{u}'], {
  cwd: ROOT,
  stdio: 'ignore'
})

if (upstream.status === 0) {
  run('git', ['push'])
} else {
  run('git', ['push', '-u', 'origin', branch])
}

run('git', ['push', 'origin', tag])

console.log(`Release ${tag} pushed successfully.`)
console.log('Next: wait for the GitHub Actions release workflow to create/update the draft release.')
