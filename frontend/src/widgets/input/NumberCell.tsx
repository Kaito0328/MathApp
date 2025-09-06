"use client"
import React from 'react'
import BaseNumberCell, { NumberCellProps as BaseNumberCellProps } from '../../baseComponents/inputs/NumberCell'

export type NumberCellProps = BaseNumberCellProps

export const NumberCell: React.FC<NumberCellProps> = (props) => {
  return <BaseNumberCell {...props} />
}

export default NumberCell
