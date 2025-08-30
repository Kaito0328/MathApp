import React from 'react';
import type { ViewStyleKit } from '../tokens';
import { baseBoxMaps } from '../maps/base';
import { useViewClasses, resolveClass, StateFlags } from '../core/resolvers';

export type BaseBoxProps = React.HTMLAttributes<HTMLElement> & {
  styleKit?: Partial<ViewStyleKit> & { color?: Partial<ViewStyleKit['color']>; size?: Partial<ViewStyleKit['size']> };
  as?: 'div' | 'section' | 'article' | 'header' | 'footer' | 'main' | 'nav';
  disabled?: boolean;
};

const DEFAULT_KIT: ViewStyleKit = {
  color: { colorKey: 'base' as any, apply: { default: ['bg'] as any } },
  size: { sizeKey: 'md' as any, apply: { default: ['padding'] as any } },
  roundKey: 'md' as any,
  shadowKey: 'none' as any,
};

export const BaseBox: React.FC<BaseBoxProps> = ({ styleKit, as = 'div', disabled, className, ...props }) => {
  const finalKit: ViewStyleKit = {
    ...DEFAULT_KIT,
    ...styleKit,
    color: { ...DEFAULT_KIT.color, ...(styleKit?.color || {}) },
    size: { ...DEFAULT_KIT.size, ...(styleKit?.size || {}) },
  } as ViewStyleKit;

  const classes = useViewClasses(finalKit, baseBoxMaps);
  const flags: StateFlags = { Disabled: !!disabled } as any;
  const cls = resolveClass(classes, flags);

  const Comp: any = as;
  return <Comp className={[cls, className].filter(Boolean).join(' ')} {...props} />;
};
