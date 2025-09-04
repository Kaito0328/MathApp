import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  reactStrictMode: true,
  // WebAssembly を webpack で解決するための実験フラグを有効化
  webpack: (config) => {
    config.experiments = {
      ...(config.experiments || {}),
      asyncWebAssembly: true,
    }
    return config
  },
}

export default nextConfig
