import { NextResponse } from 'next/server'
import path from 'node:path'
import { promises as fs } from 'node:fs'

// Simple file server for markdown docs under /workspaces/docs
export async function GET(req: Request) {
  try {
    const { searchParams } = new URL(req.url)
    const rel = searchParams.get('path') || ''
    // Basic sanitization: forbid traversal and absolute paths
    if (!rel || rel.includes('..') || rel.startsWith('/') || rel.startsWith('\\')) {
      return NextResponse.json({ error: 'invalid path' }, { status: 400 })
    }
    const docsRoot = '/workspaces/docs'
    const abs = path.resolve(docsRoot, rel)
    if (!abs.startsWith(docsRoot)) {
      return NextResponse.json({ error: 'out of scope' }, { status: 400 })
    }
    const content = await fs.readFile(abs, 'utf-8')
    return NextResponse.json({ content })
  } catch (err: any) {
    return NextResponse.json({ error: String(err?.message || err) }, { status: 404 })
  }
}
