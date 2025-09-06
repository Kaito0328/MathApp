import React from 'react';
import { CoreColorKey, SizeKey } from '../../design/tokens';

export type TextInputProps = React.InputHTMLAttributes<HTMLInputElement> & {
  variant?: CoreColorKey;
  size?: SizeKey;
  invalid?: boolean;
};

export const TextInput: React.FC<TextInputProps> = ({ variant = CoreColorKey.Base, size = SizeKey.MD, invalid, className, ...rest }) => {
  const sizeCls = size === SizeKey.SM ? 'py-sm px-sm' : size === SizeKey.LG ? 'py-lg px-lg' : size === SizeKey.XL ? 'py-xl px-xl' : 'py-md px-md';
  const variantCls = variant === CoreColorKey.Primary ? 'border-primary' : variant === CoreColorKey.Secondary ? 'border-secondary' : variant === CoreColorKey.Danger ? 'border-danger' : variant === CoreColorKey.Success ? 'border-success' : 'border-base';
  const invalidCls = invalid ? 'input-invalid' : '';
  return <input className={["input-el", sizeCls, variantCls, invalidCls, className].filter(Boolean).join(' ')} {...rest} />;
};
