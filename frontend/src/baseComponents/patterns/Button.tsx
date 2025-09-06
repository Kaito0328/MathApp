import React from 'react';
import { CoreColorKey, SizeKey, VariantKey, SizeViewProperty } from '../../design/tokens';
import { View } from '../foundation/View';
import { Text } from '../foundation/Text';

export type ButtonProps = Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, 'color'> & {
  color?: CoreColorKey;
  variant?: VariantKey;
  size?: SizeKey;
};

export const Button: React.FC<ButtonProps> = ({ color = CoreColorKey.Primary, variant = VariantKey.Solid, size = SizeKey.MD, className, children, disabled, ...rest }) => {
  // Map size to padding via styleKit to keep consistent with Foundation
  const sizeApply = { default: [SizeViewProperty.Padding] } as any;
  return (
    <View
      as="button"
      className={["btn", className].filter(Boolean).join(' ')}
      color={color}
      variant={variant}
      styleKit={{ size: { sizeKey: size, apply: sizeApply } as any }}
      disabled={disabled}
      {...rest}
    >
      <Text color={color as any} variant={variant} style={{ lineHeight: 1 }}>{children}</Text>
    </View>
  );
};
