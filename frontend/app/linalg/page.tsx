"use client"
import Link from 'next/link'
import { View } from '../../src/baseComponents/foundation/View'
import { Text } from '../../src/baseComponents/foundation/Text'
import { CoreColorKey, SizeKey, FontWeightKey } from '../../src/design/tokens'
import { VariableManager } from '../../src/components/variables/VariableManager'

export default function LinalgHome() {
  return (
    <div style={{ display: 'grid', gap: 12 }}>
      <Text style={{ fontWeight: 600 }}>変数</Text>
      <VariableManager />

      <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12 }}>
        <Text weight={FontWeightKey.Medium}>各種演算</Text>
        <div style={{ display: 'grid', gap: 8, marginTop: 8 }}>
          <Link href="/linalg/add">A + B（加算）</Link>
          <Link href="/linalg/mul">A × B（乗算）</Link>
          <Link href="/linalg/decomp">分解（LU/QR/SVD）</Link>
        </div>
      </View>
    </div>
  )
}
