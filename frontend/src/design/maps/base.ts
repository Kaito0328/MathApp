import {
  CoreColorKey,
  ColorViewProperty,
  ColorTextProperty,
  SizeKey,
  SizeViewProperty,
  SizeTextProperty,
  RoundKey,
  ShadowKey,
  FontWeightKey,
  StyleState,
} from '../tokens';
import { TextStyleMaps, ViewStyleMaps } from '../core/types';

// BaseBox (div 相当)
export const baseBoxMaps: ViewStyleMaps = {
  color: {
    [CoreColorKey.Base]: {
      [StyleState.Default]: { [ColorViewProperty.Bg]: 'bg-base', [ColorViewProperty.Border]: 'border-base' },
      [StyleState.Hover]: { [ColorViewProperty.Bg]: 'bg-base' },
    },
    [CoreColorKey.Primary]: {
      [StyleState.Default]: { [ColorViewProperty.Bg]: 'bg-primary', [ColorViewProperty.Border]: 'border-primary' },
      [StyleState.Hover]: { [ColorViewProperty.Bg]: 'bg-primary' },
      [StyleState.Active]: { [ColorViewProperty.Bg]: 'bg-primary' },
    },
    [CoreColorKey.Secondary]: {
      [StyleState.Default]: { [ColorViewProperty.Bg]: 'bg-secondary', [ColorViewProperty.Border]: 'border-secondary' },
    },
    [CoreColorKey.Danger]: {
      [StyleState.Default]: { [ColorViewProperty.Bg]: 'bg-danger', [ColorViewProperty.Border]: 'border-danger' },
    },
    [CoreColorKey.Success]: {
      [StyleState.Default]: { [ColorViewProperty.Bg]: 'bg-success', [ColorViewProperty.Border]: 'border-success' },
    },
  },
  size: {
    [SizeKey.SM]: { [StyleState.Default]: { [SizeViewProperty.Padding]: 'pad-sm', [SizeViewProperty.Gap]: 'gap-sm' } },
    [SizeKey.MD]: { [StyleState.Default]: { [SizeViewProperty.Padding]: 'pad-md', [SizeViewProperty.Gap]: 'gap-md' } },
    [SizeKey.LG]: { [StyleState.Default]: { [SizeViewProperty.Padding]: 'pad-lg', [SizeViewProperty.Gap]: 'gap-lg' } },
    [SizeKey.XL]: { [StyleState.Default]: { [SizeViewProperty.Padding]: 'pad-xl', [SizeViewProperty.Gap]: 'gap-xl' } },
  },
  round: {
    [RoundKey.None]: 'round-none',
    [RoundKey.Sm]: 'round-sm',
    [RoundKey.Md]: 'round-md',
    [RoundKey.Lg]: 'round-lg',
    [RoundKey.Full]: 'round-full',
  },
  shadow: {
    [ShadowKey.None]: 'shadow-none',
    [ShadowKey.Sm]: 'shadow-sm',
    [ShadowKey.Md]: 'shadow-md',
    [ShadowKey.Lg]: 'shadow-lg',
  },
};

// BaseText (span/p 相当)
export const baseTextMaps: TextStyleMaps = {
  color: {
    [CoreColorKey.Base]: { [StyleState.Default]: { [ColorTextProperty.Text]: 'text-base' }, [StyleState.Disabled]: { [ColorTextProperty.Text]: 'text-secondary' } },
    [CoreColorKey.Primary]: { [StyleState.Default]: { [ColorTextProperty.Text]: 'text-primary' } },
    [CoreColorKey.Secondary]: { [StyleState.Default]: { [ColorTextProperty.Text]: 'text-secondary' } },
    [CoreColorKey.Danger]: { [StyleState.Default]: { [ColorTextProperty.Text]: 'text-danger' } },
    [CoreColorKey.Success]: { [StyleState.Default]: { [ColorTextProperty.Text]: 'text-success' } },
  },
  size: {
    [SizeKey.SM]: { [StyleState.Default]: { [SizeTextProperty.FontSize]: 'fs-sm' } },
    [SizeKey.MD]: { [StyleState.Default]: { [SizeTextProperty.FontSize]: 'fs-md' } },
    [SizeKey.LG]: { [StyleState.Default]: { [SizeTextProperty.FontSize]: 'fs-lg' } },
    [SizeKey.XL]: { [StyleState.Default]: { [SizeTextProperty.FontSize]: 'fs-xl' } },
  },
  fontWeight: {
    [FontWeightKey.Light]: 'fw-light',
    [FontWeightKey.Normal]: 'fw-normal',
    [FontWeightKey.Medium]: 'fw-medium',
    [FontWeightKey.Bold]: 'fw-bold',
  },
};
