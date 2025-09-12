import { NextResponse } from 'next/server'
import { promises as fs } from 'fs'
import path from 'path'

// Root we allow to read from
const BACKEND_ROOT = '/workspaces/backend'
const MAX_SIZE = 256 * 1024 // 256KB

function sanitize(relPath: string | null): string {
  if (!relPath) return ''
  // forbid absolute and traversal
  if (relPath.startsWith('/') || relPath.includes('..')) return ''
  return relPath
}

function detectLanguageByExt(p: string): string {
  const ext = path.extname(p).toLowerCase()
  if (ext === '.rs') return 'rust'
  if (ext === '.toml') return 'toml'
  if (ext === '.md' || ext === '.markdown') return 'markdown'
  if (ext === '.json') return 'json'
  if (ext === '.ts') return 'typescript'
  if (ext === '.tsx') return 'tsx'
  if (ext === '.js' || ext === '.mjs' || ext === '.cjs') return 'javascript'
  if (ext === '.txt') return 'text'
  return 'text'
}

function isTextByExt(p: string): boolean {
  const textExts = ['.rs', '.toml', '.md', '.markdown', '.json', '.ts', '.tsx', '.js', '.mjs', '.cjs', '.txt']
  return textExts.includes(path.extname(p).toLowerCase())
}

export async function GET(req: Request) {
  try {
    const url = new URL(req.url)
    const rel = sanitize(url.searchParams.get('path'))
    const abs = path.join(BACKEND_ROOT, rel)

    // Ensure within root
    const normalized = path.normalize(abs)
    if (!normalized.startsWith(BACKEND_ROOT)) {
      return NextResponse.json({ error: 'Forbidden path' }, { status: 400 })
    }

    let stat
    try {
      stat = await fs.stat(normalized)
    } catch {
      return NextResponse.json({ error: 'Not found' }, { status: 404 })
    }

    if (stat.isDirectory()) {
      const entries = await fs.readdir(normalized, { withFileTypes: true })
      const items = entries
        .filter((e) => !e.name.startsWith('.'))
        .map((e) => ({ name: e.name, kind: e.isDirectory() ? 'dir' : 'file' }))
        .sort((a, b) => (a.kind === b.kind ? a.name.localeCompare(b.name) : a.kind === 'dir' ? -1 : 1))
      return NextResponse.json({ type: 'dir', path: rel, items })
    }

    if (stat.isFile()) {
      if (stat.size > MAX_SIZE) {
        return NextResponse.json({ type: 'file', path: rel, tooLarge: true, size: stat.size }, { status: 200 })
      }
      if (!isTextByExt(abs)) {
        return NextResponse.json({ type: 'file', path: rel, binary: true, size: stat.size }, { status: 200 })
      }
      const buf = await fs.readFile(normalized)
      // naive UTF-8 decode; files in repo should be UTF-8
      const content = buf.toString('utf8')
      return NextResponse.json({ type: 'file', path: rel, language: detectLanguageByExt(abs), content, size: stat.size })
    }

    return NextResponse.json({ error: 'Unsupported node type' }, { status: 400 })
  } catch (e: any) {
    return NextResponse.json({ error: String(e?.message || e) }, { status: 500 })
  }
}
