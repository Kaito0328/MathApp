import React from 'react';
import type { TextStyleKit } from '../../design/tokens';
import { CoreColorKey, SizeKey, FontWeightKey, ColorTextProperty, SizeTextProperty, VariantKey, SurfaceKey, OnColorKey } from '../../design/tokens';
import { baseTextMaps } from '../../design/maps/base';
import { useTextClasses, resolveClass, StateFlags } from '../../design/core/resolvers';

export type TextProps = React.HTMLAttributes<HTMLElement> & {
  color?: CoreColorKey;
  size?: SizeKey;
  weight?: FontWeightKey;
  as?: 'span' | 'p' | 'label' | 'strong' | 'em';
  variant?: VariantKey;
  styleKit?: Partial<TextStyleKit> & { color?: Partial<TextStyleKit['color']>; size?: Partial<TextStyleKit['size']> };
  disabled?: boolean;
};

const DEFAULT_TEXT_KIT: TextStyleKit = {
  color: { colorKey: SurfaceKey.OnSurface as any, apply: { default: [ColorTextProperty.Text], disabled: [ColorTextProperty.Text] } },
  size: { sizeKey: SizeKey.MD, apply: { default: [SizeTextProperty.FontSize] } },
  fontWeightKey: FontWeightKey.Normal,
};

const toOnColor = (core: CoreColorKey): OnColorKey => {
  switch (core) {
    case CoreColorKey.Primary: return OnColorKey.OnPrimary;
    case CoreColorKey.Secondary: return OnColorKey.OnSecondary;
    case CoreColorKey.Danger: return OnColorKey.OnDanger;
    case CoreColorKey.Success: return OnColorKey.OnSuccess;
    case CoreColorKey.Base:
    default:
      return OnColorKey.OnBase;
  }
};

export const Text: React.FC<TextProps> = ({ as = 'span', color, size, weight, variant, styleKit, disabled, className, ...props }) => {
  // Decide effective color key considering variant: solid -> use OnColor of the core color; otherwise pass through
  const styleKitColorKey = styleKit?.color?.colorKey as any;
  const baseColorKey = (color ?? styleKitColorKey ?? SurfaceKey.OnSurface) as any;
  const effectiveColorKey = (variant === VariantKey.Solid && Object.values(CoreColorKey).includes(baseColorKey))
    ? toOnColor(baseColorKey as CoreColorKey)
    : baseColorKey;

  const finalKit: TextStyleKit = {
    ...DEFAULT_TEXT_KIT,
    ...styleKit,
    color: { ...DEFAULT_TEXT_KIT.color, ...(styleKit?.color || {}), colorKey: effectiveColorKey },
    size: { ...DEFAULT_TEXT_KIT.size, ...(styleKit?.size || {}), ...(size ? { sizeKey: size } : {}) },
    fontWeightKey: weight ?? styleKit?.fontWeightKey ?? DEFAULT_TEXT_KIT.fontWeightKey,
  variant: variant ?? styleKit?.variant,
  } as TextStyleKit;

  const classes = useTextClasses(finalKit, baseTextMaps);
  const flags: StateFlags = { Disabled: !!disabled } as any;
  const cls = resolveClass(classes, flags);

  const Tag: any = as;
  return <Tag className={[cls, className].filter(Boolean).join(' ')} {...props} />;
};
