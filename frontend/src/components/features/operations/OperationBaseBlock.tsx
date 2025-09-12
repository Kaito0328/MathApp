"use client"
import React from 'react'
import SectionPanel from '../../composites/panels/SectionPanel'
import Row from '../../../baseComponents/layout/Row'

export interface OperationBaseBlockProps {
  left?: React.ReactNode
  right?: React.ReactNode
  center?: React.ReactNode
}

export const OperationBaseBlock: React.FC<OperationBaseBlockProps> = ({left, right, center}) => {
  return (
    <SectionPanel>
        {<Row
            left = {left}
            center = {center}
            right = {right}
        />}
    </SectionPanel>
  )
}

export default OperationBaseBlock
