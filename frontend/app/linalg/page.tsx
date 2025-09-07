"use client"
import AppLink from '../../src/baseComponents/patterns/AppLink'
import { View } from '../../src/baseComponents/foundation/View'
import { Text } from '../../src/baseComponents/foundation/Text'
import { CoreColorKey, SizeKey, FontWeightKey } from '../../src/design/tokens'
import PageContainer from '../../src/baseComponents/patterns/PageContainer'
import VariableSection from '../../src/components/variables/VariableSection'

export default function LinalgHome() {
  return (
    <PageContainer title="線形代数" stickyHeader maxWidth={1080}>
      <div style={{ display: 'grid', gap: 12 }}>
        <View color={CoreColorKey.Base} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12 }}>
          <Text weight={FontWeightKey.Medium}>線形代数ツール</Text>
          <div style={{ display: 'grid', gap: 8, marginTop: 8 }}>
            <AppLink href="/linalg/binary">行列・ベクトルの二項演算</AppLink>
            <AppLink href="/linalg/matrix">行列の単項演算・分解</AppLink>
            <AppLink href="/linalg/solve">連立一次方程式 Ax = b の解法</AppLink>
          </div>
        </View>
        <VariableSection kind="matrix" />
        <VariableSection kind="vector" />
      </div>
    </PageContainer>
  )
}
