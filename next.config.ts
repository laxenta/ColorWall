import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  output: 'export',
  images: {
    unoptimized: true,
  },
  // 🔥 ADD THESE TWO CRITICAL LINES:
  assetPrefix: './',
  trailingSlash: true,
};

export default nextConfig;