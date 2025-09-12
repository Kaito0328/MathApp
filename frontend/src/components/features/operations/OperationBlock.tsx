"use client"
import React from 'react'
import OperationSetting, { OperationSettingProps } from './OperationSetting'
import OperationBaseBlock from './OperationBaseBlock'

export interface OperationBlockProps {
  left?: React.ReactNode
  right?: React.ReactNode
}

export type Props = OperationBlockProps & OperationSettingProps

export const OperationBlock: React.FC<Props> = ({left, right, operations, operation, accuracy, onOperationChange, onAccuracyChange, onCalc, label, accuracy_able, calc_button_able}) => {
  return (
    <OperationBaseBlock
        left = {left}
        center = {<OperationSetting
            operations={operations}
            operation={operation}
            accuracy={accuracy}
            onOperationChange={onOperationChange}
            onAccuracyChange={onAccuracyChange}
            onCalc={onCalc}
            label={label}
            accuracy_able={accuracy_able}
            calc_button_able={calc_button_able}
        />}
        right = {right}
    />
  )
}

export default OperationBlock
