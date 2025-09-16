"use client"
import { useState } from 'react'
import { View } from '../baseComponents/foundation/View'
import { Text } from '../baseComponents/foundation/Text'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey } from '../design/tokens'
import Link from 'next/link'
import Image from 'next/image'
import { useTheme } from '../design/ThemeProvider'
import { usePathname } from 'next/navigation'
import { navSections } from './routes'
import { Button } from '../baseComponents/controls/Button'

export function AppHeader({ onMenu }: { onMenu: () => void }) {
  const { theme } = useTheme();
  return (
    <View color={CoreColorKey.Primary} size={SizeKey.MD} round={RoundKey.None} style={{ padding: 12 }}>
      <div style={{ display: 'grid', gridTemplateColumns: '1fr auto 1fr', alignItems: 'center', gap: 12 }}>
        <Button aria-label="menu" onClick={onMenu} style={{ justifySelf: 'start' }}>
          ☰
        </Button>
        <Link href="/" style={{ textDecoration: 'none', justifySelf: 'center' }}>
          <span style={{ display: 'inline-flex', alignItems: 'center', gap: 8, color: 'inherit' }}>
            <Image src={theme === 'dark' ? '/grath-logo-dark.svg' : '/grath-logo-light.svg'} alt="Grath" width={24} height={24} />
            <Text weight={FontWeightKey.Medium}>Grath</Text>
          </span>
        </Link>
        <div />
      </div>
    </View>
  )
}

export function AppDrawer({ open, onClose }: { open: boolean; onClose: () => void }) {
  const pathname = usePathname()
  const [openSections, setOpenSections] = useState<Record<string, boolean>>(() => Object.fromEntries(navSections.map(s => [s.title, !!s.defaultOpen])))
  const toggle = (title: string) => setOpenSections(prev => ({ ...prev, [title]: !prev[title] }))
  const isActive = (href: string) => pathname === href
  return (
    <div style={{ position: 'fixed', inset: 0, pointerEvents: open ? 'auto' : 'none', zIndex: 1000 }}>
      {/* backdrop */}
  <div onClick={onClose} style={{ position: 'absolute', inset: 0, background: 'rgba(0,0,0,0.35)', opacity: open ? 1 : 0, transition: 'opacity 150ms' }} />
      {/* drawer */}
      <View color={CoreColorKey.Base} size={SizeKey.MD} round={RoundKey.Md}
        style={{ position: 'absolute', top: 0, bottom: 0, left: 0, width: 300, borderRightWidth: 1, transform: `translateX(${open ? 0 : -320}px)`, transition: 'transform 220ms ease', boxShadow: '0 10px 30px rgba(0,0,0,0.25)', padding: 12, overflow: 'auto' }}>
        <div style={{ display: 'flex', alignItems: 'center' }}>
          <Text weight={FontWeightKey.Medium}>メニュー</Text>
          <div style={{ marginLeft: 'auto' }}>
            <Button aria-label="close" onClick={onClose}>✕</Button>
          </div>
        </div>
        <nav style={{ display: 'grid', gap: 10, marginTop: 12 }}>
          <Link href="/" onClick={onClose} className="nav-link" aria-current={pathname === '/' ? 'page' : undefined}>ホーム</Link>

          {navSections.map((section) => {
            const opened = openSections[section.title]
            return (
              <div key={section.title}>
                <Button
                  onClick={() => toggle(section.title)}
                  className="nav-toggle"
                  style={{ display:'flex', alignItems:'center', gap:8, width:'100%', justifyContent:'flex-start' }}
                >
                  <span aria-hidden>{opened ? '▾' : '▸'}</span>
                  <span>{section.title}</span>
                </Button>
                <div style={{ display: 'grid', gap: 6, marginTop: 6, maxHeight: opened ? 600 : 0, overflow: 'hidden', transition: 'max-height 200ms ease' }}>
                  {section.items.map(item => (
                    <Link key={item.href} href={item.href} onClick={onClose} className="nav-link" aria-current={isActive(item.href) ? 'page' : undefined} style={{ marginLeft: 12 }}>{item.title}</Link>
                  ))}
                </div>
              </div>
            )
          })}
        </nav>
    </View>
    </div>
  )
}
