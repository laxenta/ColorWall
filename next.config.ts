import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  output: 'export',
  images: {
    unoptimized: true,
  },
  // CRITICAL LINES:
  assetPrefix: './',
  trailingSlash: true,
};

export default nextConfig;