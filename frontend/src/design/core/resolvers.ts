import { cx } from './classgen';
import {
  ColorTextProperty,
  ColorViewProperty,
  SizeTextProperty,
  SizeViewProperty,
  StyleState,
  ViewStyleKit,
  TextStyleKit,
} from '../tokens';
import { TextStyleMaps, ViewStyleMaps } from './types';

export type StateFlags = Partial<Record<
  StyleState.Active | StyleState.Disabled | StyleState.Focus | StyleState.Hover,
  boolean
>>;

export const useViewClasses = (kit: ViewStyleKit, maps: ViewStyleMaps) => {
  const { color, size, roundKey, shadowKey } = kit;
  const classesByState: Partial<Record<StyleState, string>> = {};
  const colorMap = maps.color[color.colorKey] || {};
  const sizeMap = size ? maps.size[size.sizeKey] || {} : {};
  const roundCls = roundKey && maps.round ? maps.round[roundKey] : '';
  const shadowCls = shadowKey && maps.shadow ? maps.shadow[shadowKey] : '';

  const states = new Set<StyleState>([...Object.keys(colorMap), ...Object.keys(sizeMap)] as StyleState[]);
  states.add(StyleState.Default);

  for (const state of states) {
    const colorProps = (color.apply?.[state] || []) as ColorViewProperty[];
    const sizeProps = (size?.apply?.[state] || []) as SizeViewProperty[];
    const colorCls = colorProps.map((p: ColorViewProperty) => (colorMap[state]?.[p] || '')).join(' ');
    const sizeCls = sizeProps.map((p: SizeViewProperty) => (sizeMap[state]?.[p] || '')).join(' ');
    classesByState[state] = cx(colorCls, sizeCls, roundCls, shadowCls);
  }
  return classesByState;
};

export const useTextClasses = (kit: TextStyleKit, maps: TextStyleMaps) => {
  const { color, size, fontWeightKey } = kit;
  const classesByState: Partial<Record<StyleState, string>> = {};
  const colorMap = maps.color[color.colorKey] || {};
  const sizeMap = maps.size[size.sizeKey] || {};
  const weightCls = fontWeightKey && maps.fontWeight ? maps.fontWeight[fontWeightKey] : '';

  const states = new Set<StyleState>([...Object.keys(colorMap), ...Object.keys(sizeMap)] as StyleState[]);
  states.add(StyleState.Default);

  for (const state of states) {
    const colorProps = (color.apply?.[state] || []) as ColorTextProperty[];
    const sizeProps = (size.apply?.[state] || []) as SizeTextProperty[];
    const colorCls = colorProps.map((p: ColorTextProperty) => (colorMap[state]?.[p] || '')).join(' ');
    const sizeCls = sizeProps.map((p: SizeTextProperty) => (sizeMap[state]?.[p] || '')).join(' ');
    classesByState[state] = cx(colorCls, sizeCls, weightCls);
  }
  return classesByState;
};

export const resolveClass = (byState: Partial<Record<StyleState, string>>, flags: StateFlags) => {
  if (flags?.[StyleState.Disabled] && byState[StyleState.Disabled]) return byState[StyleState.Disabled]!;
  if (flags?.[StyleState.Active] && byState[StyleState.Active]) return byState[StyleState.Active]!;
  if (flags?.[StyleState.Hover] && byState[StyleState.Hover]) return byState[StyleState.Hover]!;
  if (flags?.[StyleState.Focus] && byState[StyleState.Focus]) return byState[StyleState.Focus]!;
  return byState[StyleState.Default] || '';
};
