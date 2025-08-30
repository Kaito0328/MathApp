import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  reactStrictMode: true,
  // wasm-pack のESMラッパーをクライアントで動的importする想定なので追加設定は最小限
}

export default nextConfig
