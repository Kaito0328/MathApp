import React from 'react';
import type { ViewStyleKit } from '../tokens';
import {
  CoreColorKey,
  StyleState,
  ColorViewProperty,
  SizeKey,
  SizeViewProperty,
  RoundKey,
  ShadowKey,
} from '../tokens';
import { baseBoxMaps } from '../maps/base';
import { useViewClasses, resolveClass, StateFlags } from '../core/resolvers';

export type BaseBoxProps = React.HTMLAttributes<HTMLElement> & {
  styleKit?: Partial<ViewStyleKit> & { color?: Partial<ViewStyleKit['color']>; size?: Partial<ViewStyleKit['size']> };
  as?: 'div' | 'section' | 'article' | 'header' | 'footer' | 'main' | 'nav';
  disabled?: boolean;
};

const DEFAULT_KIT: ViewStyleKit = {
  color: { colorKey: CoreColorKey.Base, apply: { [StyleState.Default]: [ColorViewProperty.Bg] } },
  size: { sizeKey: SizeKey.MD, apply: { [StyleState.Default]: [SizeViewProperty.Padding] } },
  roundKey: RoundKey.Md,
  shadowKey: ShadowKey.None,
};

export const BaseBox: React.FC<BaseBoxProps> = ({ styleKit, as = 'div', disabled, className, ...props }) => {
  const finalKit: ViewStyleKit = {
    ...DEFAULT_KIT,
    ...styleKit,
    color: { ...DEFAULT_KIT.color, ...(styleKit?.color || {}) },
    size: { ...DEFAULT_KIT.size, ...(styleKit?.size || {}) },
  } as ViewStyleKit;

  const classes = useViewClasses(finalKit, baseBoxMaps);
  const flags: StateFlags = { [StyleState.Disabled]: !!disabled };
  const cls = resolveClass(classes, flags);

  const Comp = as as keyof HTMLElementTagNameMap;
  return <Comp className={[cls, className].filter(Boolean).join(' ')} {...(props as any)} />;
};
