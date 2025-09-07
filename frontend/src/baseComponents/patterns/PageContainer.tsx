import React from 'react'
import { View } from '../foundation/View'
import { Text } from '../foundation/Text'
import { CoreColorKey, SizeKey, VariantKey, RoundKey, FontWeightKey } from '../../design/tokens'
import Link from 'next/link'

export type PageContainerProps = {
  title?: React.ReactNode
  actions?: React.ReactNode
  children?: React.ReactNode
  breadcrumbs?: Array<{ label: React.ReactNode; href?: string; onClick?: () => void }>
  tabs?: Array<{ key: string; label: React.ReactNode; href?: string; active?: boolean; onClick?: () => void }>
  stickyHeader?: boolean
  maxWidth?: number | string
  gutters?: number
}

export const PageContainer: React.FC<PageContainerProps> = ({
  title,
  actions,
  children,
  breadcrumbs,
  tabs,
  stickyHeader,
  maxWidth = 1080,
  gutters = 12,
}) => {
  return (
    <div>
      {(title || actions || (breadcrumbs && breadcrumbs.length) || (tabs && tabs.length)) && (
    <div style={{ position: stickyHeader ? 'sticky' as const : undefined, top: 0, zIndex: 5 }}>
          {(title || actions || (breadcrumbs && breadcrumbs.length)) && (
            <View
              color={CoreColorKey.Base}
              variant={VariantKey.Soft}
              size={SizeKey.MD}
              round={RoundKey.None}
      style={{ borderBottomWidth: 1, boxShadow: stickyHeader ? '0 4px 10px rgba(0,0,0,0.06)' : undefined }}
            >
              <div style={{ maxWidth, margin: '0 auto', paddingInline: gutters }}>
                <div
                  style={{
                    display: 'grid',
                    gridTemplateColumns: '1fr auto 1fr',
                    alignItems: 'center',
                    gap: 8,
                  }}
                >
                  {/* left: breadcrumbs */}
                  <div style={{ justifySelf: 'start', minHeight: 24, display: 'flex', alignItems: 'center', gap: 6 }}>
                    {breadcrumbs && breadcrumbs.length > 0 && (
                      <nav aria-label="breadcrumbs" style={{ display: 'flex', alignItems: 'center', gap: 6, fontSize: 12, opacity: 0.9 }}>
                        {breadcrumbs.map((bc, i) => (
                          <span key={i} style={{ display: 'inline-flex', alignItems: 'center', gap: 6 }}>
                            {bc.href ? (
                              <Link href={bc.href} onClick={bc.onClick} style={{ color: 'inherit', textDecoration: 'none' }}>
                                {bc.label}
                              </Link>
                            ) : (
                              <span>{bc.label}</span>
                            )}
                            {i < breadcrumbs.length - 1 && <span style={{ opacity: 0.6 }}>/</span>}
                          </span>
                        ))}
                      </nav>
                    )}
                  </div>
                  {/* center: title */}
                  {title ? (
                    <Text weight={FontWeightKey.Medium} style={{ justifySelf: 'center' }}>
                      {title}
                    </Text>
                  ) : (
                    <div />
                  )}
                  {/* right: actions */}
                  <div style={{ justifySelf: 'end' }}>{actions}</div>
                </div>
              </div>
            </View>
          )}
          {tabs && tabs.length > 0 && (
            <View color={CoreColorKey.Base} size={SizeKey.SM} round={RoundKey.None} style={{ borderBottomWidth: 1 }}>
              <div style={{ maxWidth, margin: '0 auto', paddingInline: gutters }}>
                <nav aria-label="tabs" style={{ display: 'flex', gap: 12, overflowX: 'auto' }}>
                  {tabs.map((t) => (
                    <Link
                      key={t.key}
                      href={t.href ?? '#'}
                      onClick={t.onClick}
                      aria-current={t.active ? 'page' : undefined}
                      style={{
                        padding: '8px 10px',
                        borderBottom: t.active ? '2px solid currentColor' : '2px solid transparent',
                        opacity: t.active ? 1 : 0.8,
                        textDecoration: 'none',
                        color: 'inherit',
                        whiteSpace: 'nowrap',
                      }}
                    >
                      {t.label}
                    </Link>
                  ))}
                </nav>
              </div>
            </View>
          )}
        </div>
      )}

      <div style={{ maxWidth, margin: '0 auto', padding: gutters }}>{children}</div>
    </div>
  )
}

export default PageContainer
