import {
  ColorTextStyleKit,
  ColorValueStyleKit,
  ColorViewStyleKit,
  FontWeightKey,
  RoundKey,
  ShadowKey,
  SizeTextKit,
  SizeViewKit,
  StyleState,
} from '../tokens';

export type ViewStyleMaps = {
  color: Record<string, Partial<Record<StyleState, Partial<Record<string, string>>>>>,
  size: Record<string, Partial<Record<StyleState, Partial<Record<string, string>>>>>,
  round?: Record<RoundKey, string>;
  shadow?: Record<ShadowKey, string>;
};

export type TextStyleMaps = {
  color: Record<string, Partial<Record<StyleState, Partial<Record<string, string>>>>>,
  size: Record<string, Partial<Record<StyleState, Partial<Record<string, string>>>>>,
  fontWeight?: Record<FontWeightKey, string>;
};

export type PartialColorViewKit = Partial<Omit<ColorViewStyleKit, 'apply'>> & { apply?: Partial<ColorViewStyleKit['apply']> };
export type PartialSizeViewKit = Partial<Omit<SizeViewKit, 'apply'>> & { apply?: Partial<SizeViewKit['apply']> };
export type PartialColorTextKit = Partial<Omit<ColorTextStyleKit, 'apply'>> & { apply?: Partial<ColorTextStyleKit['apply']> };
export type PartialSizeTextKit = Partial<Omit<SizeTextKit, 'apply'>> & { apply?: Partial<SizeTextKit['apply']> };

export type PartialViewStyleKit = Partial<ViewStyleKit> & {
  color?: PartialColorViewKit;
  size?: PartialSizeViewKit;
};

export type PartialTextStyleKit = Partial<Pick<import('../tokens').TextStyleKit, 'fontWeightKey'>> & {
  color?: PartialColorTextKit;
  size?: PartialSizeTextKit;
};

export type ViewStyleKit = import('../tokens').ViewStyleKit;
export type TextStyleKit = import('../tokens').TextStyleKit;
