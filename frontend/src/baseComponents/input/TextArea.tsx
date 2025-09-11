import React from 'react'
import { CoreColorKey, SizeKey } from '../../design/tokens'
import { inputColorMap, inputSizeMap } from '../../design/maps/input'

export type TextAreaProps = React.TextareaHTMLAttributes<HTMLTextAreaElement> & {
  color?: CoreColorKey
  variant?: CoreColorKey
  size?: SizeKey
  invalid?: boolean
}

export const TextArea: React.FC<TextAreaProps> = ({
  color,
  variant = CoreColorKey.Base,
  size = SizeKey.MD,
  invalid,
  className,
  style,
  ...rest
}) => {
  const sz = size ?? SizeKey.MD
  const col = (color ?? variant) as CoreColorKey
  const sizeCls = inputSizeMap[sz]
  const colorCls = inputColorMap[col] ?? inputColorMap[CoreColorKey.Base]
  const invalidCls = invalid ? 'input-invalid' : ''
  const elCls = 'input-el'
  return (
    <textarea
      className={[elCls, sizeCls, colorCls, invalidCls, className].filter(Boolean).join(' ')}
      style={{ color: 'inherit', background: 'transparent', ...(style as any) }}
      {...rest}
    />
  )
}

export default TextArea
