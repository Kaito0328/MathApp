import React from 'react';
import { View as FoundationView } from '../../baseComponents/foundation/View';
import { CoreColorKey, SizeKey, RoundKey, ShadowKey } from '../tokens';

// Constrained BaseBox: only styling + semantic tag
export type BaseBoxProps = {
  color?: CoreColorKey;
  size?: SizeKey;
  round?: RoundKey;
  shadow?: ShadowKey;
  as?: 'div' | 'section' | 'article' | 'header' | 'footer' | 'main' | 'nav';
  disabled?: boolean;
  className?: string;
  children?: React.ReactNode;
};

export const BaseBox: React.FC<BaseBoxProps> = (props) => {
  return <FoundationView {...props} />;
};
