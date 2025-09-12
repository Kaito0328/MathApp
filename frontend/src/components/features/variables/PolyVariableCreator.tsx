"use client"
import React from 'react'
import { Text } from '../../../baseComponents/foundation/Text'
import { View } from '../../../baseComponents/foundation/View'
import { Button } from '../../../baseComponents/controls/Button'
import { TextInput } from '../../../baseComponents/input/TextInput'
import PolynomialInput from '../../../widgets/input/PolynomialInput'
import RationalFunctionInput from '../../../widgets/input/RationalFunctionInput'
import { useVariableStore } from '../../../state/VariableStore'
import { CoreColorKey, SizeKey, VariantKey } from '../../../design/tokens'

type Kind = 'polynomial' | 'rational'

export const PolyVariableCreator: React.FC<{ kind: Kind; initialName?: string }> = ({ kind, initialName }) => {
  const { upsert, names, get } = useVariableStore()
  const [name, setName] = React.useState(initialName ?? '')
  const [poly, setPoly] = React.useState<{ coeffs: number[] }>({ coeffs: [0, 0, 0] })
  const [rf, setRf] = React.useState<{ numerator: { coeffs: number[] }; denominator: { coeffs: number[] } }>({ numerator: { coeffs: [1] }, denominator: { coeffs: [1] } })

  React.useEffect(() => {
    if (!initialName) return
    const v: any = get(initialName)
    if (!v) return
    if (v.kind === 'polynomial') setPoly({ coeffs: v.coeffs.slice() })
    if (v.kind === 'rational') setRf({ numerator: { coeffs: v.numerator.slice() }, denominator: { coeffs: v.denominator.slice() } })
  }, [initialName, get])

  const exists = (n: string) => names.includes(n)
  const canSave = name.trim().length > 0 && !exists(name)

  const save = () => {
    if (!canSave) return
    if (kind === 'polynomial') upsert(name, { kind: 'polynomial', coeffs: poly.coeffs })
    else upsert(name, { kind: 'rational', numerator: rf.numerator.coeffs, denominator: rf.denominator.coeffs })
    window.dispatchEvent(new CustomEvent('poly-variable-creator:close'))
  }

  const panelVariant = VariantKey.Solid

  return (
    <View color={CoreColorKey.Secondary} variant={panelVariant} size={SizeKey.MD} style={{ borderWidth: 1, padding: 12, display: 'grid', gap: 8 }}>
      <div style={{ display: 'grid', gap: 8 }}>
        <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
          <Text>名前</Text>
          <TextInput value={name} onChange={(e) => setName(e.target.value)} placeholder={'名前を入力してください...'} style={{ background: 'transparent' }} />
          {exists(name) && <Text style={{ color: 'crimson' }}>同名の変数が存在します</Text>}
        </div>
        {kind === 'polynomial' ? (
          <PolynomialInput value={poly} onChange={setPoly} />
        ) : (
          <RationalFunctionInput value={rf} onChange={setRf} />
        )}
        <div style={{ display: 'flex', gap: 8 }}>
          <Button onClick={save} disabled={!canSave}>保存</Button>
          <Button onClick={() => window.dispatchEvent(new CustomEvent('poly-variable-creator:close'))} color={CoreColorKey.Base}>閉じる</Button>
        </div>
      </div>
    </View>
  )
}

export default PolyVariableCreator
