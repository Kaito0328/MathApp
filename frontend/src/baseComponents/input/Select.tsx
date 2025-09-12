import React from 'react';
import { CoreColorKey, SizeKey } from '../../design/tokens';
import { inputColorMap, inputSizeMap } from '../../design/maps/input';

export type SelectProps = Omit<React.SelectHTMLAttributes<HTMLSelectElement>, 'size' | 'color'> & {
  color?: CoreColorKey;
  size?: SizeKey;
  invalid?: boolean;
};

export const Select: React.FC<SelectProps> = ({
  color,
  size = SizeKey.MD,
  invalid,
  className,
  children,
  ...rest
}) => {
  const sz = size ?? SizeKey.MD;
  const col = (color ?? CoreColorKey.Base) as CoreColorKey;
  const sizeCls = inputSizeMap[sz];
  const colorCls = inputColorMap[col] ?? inputColorMap[CoreColorKey.Base];
  const invalidCls = invalid ? 'input-invalid' : '';
  const elCls = 'input-el';
  return (
    <select className={[elCls, sizeCls, colorCls, invalidCls, className].filter(Boolean).join(' ')} {...rest}>
      {children}
    </select>
  );
};

export default Select;
