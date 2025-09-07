"use client"
import React from 'react'
import { View } from '../baseComponents/foundation/View'
import { Text } from '../baseComponents/foundation/Text'
import { CoreColorKey, SizeKey, RoundKey, FontWeightKey } from '../design/tokens'

export const Footer: React.FC = () => {
  return (
    <View color={CoreColorKey.Base} size={SizeKey.MD} round={RoundKey.None} style={{ borderTopWidth: 1, marginTop: 24 }}>
      <div style={{ maxWidth: 1080, margin: '0 auto', padding: 12, display: 'flex', alignItems: 'center' }}>
  <Text weight={FontWeightKey.Normal} style={{ opacity: 0.8 }}>© {new Date().getFullYear()} Grath</Text>
        <div style={{ marginLeft: 'auto', display: 'flex', gap: 12 }}>
          <a href="/about" style={{ color: 'inherit', textDecoration: 'none' }}>About</a>
          <a href="/privacy" style={{ color: 'inherit', textDecoration: 'none' }}>Privacy</a>
        </div>
      </div>
    </View>
  )
}

export default Footer
