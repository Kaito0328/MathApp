import React from 'react'
import { View } from '../baseComponents/foundation/View'
import { Text } from '../baseComponents/foundation/Text'
import { CoreColorKey, SizeKey, VariantKey, RoundKey } from '../design/tokens'

export type PageContainerProps = {
  title?: React.ReactNode
  actions?: React.ReactNode
  children?: React.ReactNode
}

export const PageContainer: React.FC<PageContainerProps> = ({ title, actions, children }) => {
  return (
    <div>
      {(title || actions) && (
        <View color={CoreColorKey.Base} variant={VariantKey.Soft} size={SizeKey.MD} round={RoundKey.None} style={{ display: 'grid', gridTemplateColumns: '1fr auto 1fr', alignItems: 'center', gap: 8, borderBottomWidth: 1 }}>
          <div />
          {title && <Text style={{ fontWeight: 700 }}>{title}</Text>}
          <div style={{ justifySelf: 'end' }}>{actions}</div>
        </View>
      )}
      <div style={{ padding: 12 }}>{children}</div>
    </div>
  )
}

export default PageContainer
