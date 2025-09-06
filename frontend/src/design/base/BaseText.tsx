import React from 'react';
import { Text as FoundationText } from '../../baseComponents/foundation/Text';
import { CoreColorKey, SizeKey, FontWeightKey } from '../tokens';

// Constrained BaseText: accept text directly and a unified set of style props only
export type BaseTextProps = {
  text: string;
  color?: CoreColorKey;
  size?: SizeKey;
  weight?: FontWeightKey;
  as?: 'span' | 'p' | 'label' | 'strong' | 'em';
  disabled?: boolean;
};

export const BaseText: React.FC<BaseTextProps> = ({ text, ...style }) => {
  return <FoundationText {...style}>{text}</FoundationText>;
};
