import React from 'react';
import { CoreColorKey, SizeKey, VariantKey } from '../../design/tokens';
import { View } from '../foundation/View';

type BaseDivProps = Omit<React.HTMLAttributes<HTMLDivElement>, 'title'>;

export type CardProps = BaseDivProps & {
  title?: React.ReactNode;
  color?: CoreColorKey;
  variant?: VariantKey | CoreColorKey;
  size?: SizeKey;
};

export const Card: React.FC<CardProps> = ({
  color = CoreColorKey.Base,
  variant = VariantKey.Soft,
  size = SizeKey.MD,
  className,
  title,
  children,
  ...rest
}) => {
  const isCoreColor = Object.values(CoreColorKey).includes(variant as CoreColorKey);
  const finalColor = isCoreColor ? (variant as CoreColorKey) : color;
  const finalVariant = isCoreColor ? VariantKey.Soft : (variant as VariantKey);
  return (
    <View
      className={["card", className].filter(Boolean).join(' ')}
      color={finalColor}
      variant={finalVariant}
      styleKit={{ size: { sizeKey: size, apply: { default: ['padding', 'gap'] } as any } as any }}
      {...rest}
    >
      {title ? <div style={{ marginBottom: 8 }}>{title}</div> : null}
      {children}
    </View>
  );
};
