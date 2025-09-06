import React from 'react';
import type { ViewStyleKit } from '../../design/tokens';
import { CoreColorKey, SizeKey, RoundKey, ShadowKey, ColorViewProperty, SizeViewProperty, VariantKey, SurfaceKey } from '../../design/tokens';
import { baseBoxMaps } from '../../design/maps/base';
import { useViewClasses, resolveClass, StateFlags } from '../../design/core/resolvers';

export type ViewProps = React.HTMLAttributes<HTMLElement> & {
  color?: CoreColorKey;
  size?: SizeKey;
  round?: RoundKey;
  shadow?: ShadowKey;
  variant?: VariantKey;
  styleKit?: Partial<ViewStyleKit> & { color?: Partial<ViewStyleKit['color']>; size?: Partial<ViewStyleKit['size']> };
  as?: 'div' | 'section' | 'article' | 'header' | 'footer' | 'main' | 'nav' | 'button';
  disabled?: boolean;
};

const DEFAULT_KIT: ViewStyleKit = {
  color: { colorKey: SurfaceKey.Surface as any, apply: { default: [ColorViewProperty.Bg] } },
  size: { sizeKey: SizeKey.MD, apply: { default: [SizeViewProperty.Padding] } },
  roundKey: RoundKey.Md,
  shadowKey: ShadowKey.None,
};

export const View: React.FC<ViewProps> = ({ color, size, round, shadow, variant, styleKit, as = 'div', disabled, className, ...props }) => {
  // Compute color apply rules based on variant
  const styleKitColorKey = styleKit?.color?.colorKey as any;
  const baseColorKey = (color ?? styleKitColorKey ?? SurfaceKey.Surface) as any;
  const variantApply = (() => {
    switch (variant) {
      case VariantKey.Outline:
        return { default: [ColorViewProperty.Border] } as any;
      case VariantKey.Soft:
        // Keep bg as surface, add border of color via maps by setting colorKey to the core color but applying only Border
        return { default: [ColorViewProperty.Border] } as any;
      case VariantKey.Ghost:
        // No bg/border; rely on text only. We'll set no color apply to effectively be transparent here.
        return { default: [] } as any;
      case VariantKey.Solid:
      default:
        return { default: [ColorViewProperty.Bg, ColorViewProperty.Border] } as any;
    }
  })();

  const finalKit: ViewStyleKit = {
    ...DEFAULT_KIT,
    ...styleKit,
    color: { ...DEFAULT_KIT.color, ...(styleKit?.color || {}), colorKey: baseColorKey, apply: variantApply },
    size: { ...DEFAULT_KIT.size, ...(styleKit?.size || {}), ...(size ? { sizeKey: size } : {}) },
    roundKey: round ?? styleKit?.roundKey ?? DEFAULT_KIT.roundKey,
    shadowKey: shadow ?? styleKit?.shadowKey ?? DEFAULT_KIT.shadowKey,
  variant: variant ?? styleKit?.variant,
  } as ViewStyleKit;

  const classes = useViewClasses(finalKit, baseBoxMaps);
  const flags: StateFlags = { Disabled: !!disabled } as any;
  const cls = resolveClass(classes, flags);

  const Tag: any = as;
  return <Tag className={[cls, className].filter(Boolean).join(' ')} {...props} />;
};
