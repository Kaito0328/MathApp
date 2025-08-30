// Design tokens: keys and properties with strict enums

export enum CoreColorKey {
  Base = 'base',
  Primary = 'primary',
  Secondary = 'secondary',
  Danger = 'danger',
  Success = 'success',
}

export enum ColorViewProperty {
  Bg = 'bg',
  Border = 'border',
}

export enum ColorTextProperty {
  Text = 'text',
}

export enum ColorValueProperty {
  Placeholder = 'placeholder',
  Selection = 'selection',
  Icon = 'icon',
  Border = 'borderColorOnly',
}

export type ColorKey = CoreColorKey;

export enum SizeKey {
  SM = 'sm',
  MD = 'md',
  LG = 'lg',
  XL = 'xl',
}

export enum SizeViewProperty {
  Padding = 'padding',
  PaddingHorizontal = 'paddingX',
  PaddingVertical = 'paddingY',
  Gap = 'gap',
}

export enum SizeTextProperty {
  FontSize = 'fontSize',
}

export enum RoundKey {
  None = 'none',
  Sm = 'sm',
  Md = 'md',
  Lg = 'lg',
  Full = 'full',
}

export enum ShadowKey {
  None = 'none',
  Sm = 'sm',
  Md = 'md',
  Lg = 'lg',
}

export enum FontWeightKey {
  Light = 'light',
  Normal = 'normal',
  Medium = 'medium',
  Bold = 'bold',
}

export enum StyleState {
  Default = 'default',
  Hover = 'hover',
  Active = 'active', // pressed 相当
  Disabled = 'disabled',
  Focus = 'focus',
}

export type NonDefaultStates = Exclude<StyleState, StyleState.Default>;

export type ColorViewApply = Partial<Record<StyleState, ColorViewProperty[]>>;
export type ColorTextApply = Partial<Record<StyleState, ColorTextProperty[]>>;
export type ColorValueApply = Partial<Record<StyleState, ColorValueProperty[]>>;
export type SizeViewApply = Partial<Record<StyleState, SizeViewProperty[]>>;
export type SizeTextApply = Partial<Record<StyleState, SizeTextProperty[]>>;

export type ColorViewStyleKit = { colorKey: ColorKey; apply: ColorViewApply };
export type ColorTextStyleKit = { colorKey: ColorKey; apply: ColorTextApply };
export type ColorValueStyleKit = { colorKey: ColorKey; apply: ColorValueApply };
export type SizeViewKit = { sizeKey: SizeKey; apply: SizeViewApply; fullWidth?: boolean };
export type SizeTextKit = { sizeKey: SizeKey; apply: SizeTextApply };

export type ViewStyleKit = {
  color: ColorViewStyleKit;
  size?: SizeViewKit;
  roundKey?: RoundKey;
  shadowKey?: ShadowKey;
};

export type TextStyleKit = {
  color: ColorTextStyleKit;
  size: SizeTextKit;
  fontWeightKey?: FontWeightKey;
};
