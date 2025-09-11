import React from 'react';
import { CoreColorKey, SizeKey } from '../../design/tokens';
import { inputColorMap, inputSizeMap } from '../../design/maps/input';

export type TextInputProps = Omit<React.InputHTMLAttributes<HTMLInputElement>, 'size' | 'color'> & {
  /** 色キー（variant は後方互換のため残す） */
  color?: CoreColorKey;
  variant?: CoreColorKey;
  /** サイズキー */
  size?: SizeKey;
  /** 入力が不正な時の見た目切り替え */
  invalid?: boolean;
};

export const TextInput: React.FC<TextInputProps> = ({
  color,
  variant = CoreColorKey.Base,
  size = SizeKey.MD,
  invalid,
  className,
  style,
  ...rest
}) => {
  const sz = size ?? SizeKey.MD;
  const col = (color ?? variant) as CoreColorKey;
  const sizeCls = inputSizeMap[sz];
  const colorCls = inputColorMap[col] ?? inputColorMap[CoreColorKey.Base];
  const invalidCls = invalid ? 'input-invalid' : '';
  const elCls = 'input-el';
  return (
    <input
      className={[elCls, sizeCls, colorCls, invalidCls, className].filter(Boolean).join(' ')}
      style={{ color: 'inherit', background: 'transparent', ...(style as any) }}
      {...rest}
    />
  );
};
