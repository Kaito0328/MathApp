export type NavItem = { title: string; href: string }
export type NavSection = { title: string; items: NavItem[]; defaultOpen?: boolean }

export const navSections: NavSection[] = [
  {
    title: '線形代数',
    defaultOpen: true,
    items: [
      { title: 'ホーム', href: '/linalg' },
  { title: '二項演算 (+, −, ×)', href: '/linalg/binary' },
  { title: '行列の単項/分解', href: '/linalg/matrix' },
  { title: '連立方程式 Ax=b', href: '/linalg/solve' },
      // { title: '分解 (LU/QR/SVD)', href: '/linalg/decomp' }, // 将来
    ],
  },
  {
    title: '多項式 / 有理関数',
    defaultOpen: true,
    items: [
      { title: 'ホーム', href: '/poly' },
      { title: '多項式の二項演算', href: '/poly/polynomial/binary' },
      { title: '多項式の単項演算', href: '/poly/polynomial/unary' },
      { title: '多項式の生成', href: '/poly/polynomial/generate' },
      { title: '有理関数の二項演算', href: '/poly/rational/binary' },
      { title: '有理関数の単項演算', href: '/poly/rational/unary' },
    ],
  },
  // 将来: FFT, 信号処理, 統計 など
]
