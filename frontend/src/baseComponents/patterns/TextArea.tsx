import React from 'react'
import { CoreColorKey, SizeKey } from '../../design/tokens'

export type TextAreaProps = React.TextareaHTMLAttributes<HTMLTextAreaElement> & {
  variant?: CoreColorKey
  size?: SizeKey
  invalid?: boolean
}

export const TextArea: React.FC<TextAreaProps> = ({ variant = CoreColorKey.Base, size = SizeKey.MD, invalid, className, style, ...rest }) => {
  const sizeCls = size === SizeKey.SM ? 'py-sm px-sm' : size === SizeKey.LG ? 'py-lg px-lg' : size === SizeKey.XL ? 'py-xl px-xl' : 'py-md px-md'
  const variantCls = variant === CoreColorKey.Primary ? 'border-primary' : variant === CoreColorKey.Secondary ? 'border-secondary' : variant === CoreColorKey.Danger ? 'border-danger' : variant === CoreColorKey.Success ? 'border-success' : 'border-base'
  const invalidCls = invalid ? 'input-invalid' : ''
  return (
    <textarea
      className={["input-el", sizeCls, variantCls, invalidCls, className].filter(Boolean).join(' ')}
      style={{ color: 'inherit', background: 'transparent', ...(style as any) }}
      {...rest}
    />
  )
}

export default TextArea
