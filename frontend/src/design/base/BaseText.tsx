import React from 'react';
import type { TextStyleKit } from '../tokens';
import { baseTextMaps } from '../maps/base';
import { useTextClasses, resolveClass, StateFlags } from '../core/resolvers';

export type BaseTextProps = React.HTMLAttributes<HTMLElement> & {
  as?: 'span' | 'p' | 'label' | 'strong' | 'em';
  styleKit?: Partial<TextStyleKit> & { color?: Partial<TextStyleKit['color']>; size?: Partial<TextStyleKit['size']> };
  disabled?: boolean;
};

const DEFAULT_TEXT_KIT: TextStyleKit = {
  color: { colorKey: 'base' as any, apply: { default: ['text'] as any, disabled: ['text'] as any } },
  size: { sizeKey: 'md' as any, apply: { default: ['fontSize'] as any } },
  fontWeightKey: 'normal' as any,
};

export const BaseText: React.FC<BaseTextProps> = ({ as = 'span', styleKit, disabled, className, ...props }) => {
  const finalKit: TextStyleKit = {
    ...DEFAULT_TEXT_KIT,
    ...styleKit,
    color: { ...DEFAULT_TEXT_KIT.color, ...(styleKit?.color || {}) },
    size: { ...DEFAULT_TEXT_KIT.size, ...(styleKit?.size || {}) },
  } as TextStyleKit;

  const classes = useTextClasses(finalKit, baseTextMaps);
  const flags: StateFlags = { Disabled: !!disabled } as any;
  const cls = resolveClass(classes, flags);

  const Comp: any = as;
  return <Comp className={[cls, className].filter(Boolean).join(' ')} {...props} />;
};
