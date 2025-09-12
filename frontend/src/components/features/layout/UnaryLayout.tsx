"use client"
import React from 'react'
import Stack from '../../../baseComponents/layout/Stack'
import OperationBlock from '../operations/OperationBlock'
import OperandBlock from '../operand/OperandBlock'
import ResultBlock from '../result/ResultBlock'
import VerficationBlock from '../verification/VerficationBlock'
import { OperationSettingProps } from '../operations/OperationSetting'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'

export interface UnaryLayoutProps {
  operation_left?: React.ReactNode
  operation_right?: React.ReactNode
  operand?: React.ReactNode
  operand_copyContent?: string
  operand_buildSavePayload: () => any
  operand_afterSave: (name: string) => void
  result?: React.ReactNode
  verification?: React.ReactNode
  document?: React.ReactNode
  documentTitle?: string
}

type Props = UnaryLayoutProps & OperationSettingProps

export const UnaryLayout: React.FC<Props> = ({
  operation_left,
  operation_right,
  operations,
  operation,
  accuracy,
  onOperationChange,
  onAccuracyChange,
  onCalc,
  label,
  accuracy_able,
  calc_button_able,
  operand,
  operand_copyContent,
  operand_buildSavePayload,
  operand_afterSave,
  result,
  verification,
  document,
  documentTitle,
}) => {
  return (
    <Stack gap={12}>
      <OperationBlock
        operations={operations}
        left={operation_left}
        right={operation_right}
        operation={operation}
        accuracy={accuracy}
        onOperationChange={onOperationChange}
        onAccuracyChange={onAccuracyChange}
        onCalc={onCalc}
        label={label}
        accuracy_able={accuracy_able}
        calc_button_able={calc_button_able}
      />

      <OperandBlock title="オペランド" copyContent={operand_copyContent} buildSavePayload={operand_buildSavePayload} onAfterSave={operand_afterSave}>
        {operand}
      </OperandBlock>

      <ResultBlock>{result}</ResultBlock>
      <VerficationBlock>{verification}</VerficationBlock>
      {document && (
        <SectionPanelWithTitle title={documentTitle ?? 'ドキュメント'}>
          {document}
        </SectionPanelWithTitle>
      )}
    </Stack>
  )
}

export default UnaryLayout
