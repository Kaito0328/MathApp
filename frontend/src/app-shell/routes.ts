export type NavItem = { title: string; href: string }
export type NavSection = { title: string; items: NavItem[]; defaultOpen?: boolean }

export const navSections: NavSection[] = [
  {
    title: '線形代数',
    defaultOpen: true,
    items: [
      { title: 'ホーム', href: '/linalg' },
      { title: '加算 A + B', href: '/linalg/add' },
      { title: '乗算 A × B', href: '/linalg/mul' },
      // { title: '分解 (LU/QR/SVD)', href: '/linalg/decomp' }, // 将来
    ],
  },
  // 将来: FFT, 信号処理, 統計 など
]
