import React from 'react'
import { CoreColorKey, SizeKey } from '../../design/tokens'
import { Text } from '../../baseComponents/foundation/Text'
import Select from '../../baseComponents/input/Select'

export type SelectorOption = {
  value: string
  label: React.ReactNode
  disabled?: boolean
}

export type SelectorProps = Omit<React.SelectHTMLAttributes<HTMLSelectElement>, 'onChange' | 'value' | 'size'> & {
  label?: React.ReactNode
  options: SelectorOption[]
  value?: string
  onChange?: (value: string, e: React.ChangeEvent<HTMLSelectElement>) => void
  controlSize?: SizeKey
  color?: CoreColorKey
  invalid?: boolean
  inline?: boolean
}

/**
 * Selector
 * A labeled select control with consistent spacing and accessible labeling.
 */
export const Selector: React.FC<SelectorProps> = ({
  label,
  options,
  value,
  onChange,
  controlSize = SizeKey.MD,
  color = CoreColorKey.Base,
  invalid,
  inline = true,
  id,
  className,
  style,
  ...rest
}) => {
  const uid = React.useId()
  const selectId = id || `selector-${uid}`
  const WrapperTag: any = inline ? 'label' : 'div'
  return (
    <WrapperTag className={[inline ? 'selector-inline' : 'selector-block', className].filter(Boolean).join(' ')} style={{ display: inline ? 'inline-flex' : 'flex', alignItems: 'center', gap: 6, ...style }}>
      {label && (
        <Text as="span" style={{ whiteSpace: 'nowrap' }}>{label}</Text>
      )}
      <Select
        id={selectId}
        value={value}
        onChange={(e) => onChange?.(e.target.value, e)}
        size={controlSize as any}
        color={color}
        invalid={invalid}
        {...rest}
      >
        {options.map((opt, idx) => (
          <option key={idx} value={opt.value} disabled={opt.disabled}>{opt.label}</option>
        ))}
      </Select>
    </WrapperTag>
  )
}

export default Selector
