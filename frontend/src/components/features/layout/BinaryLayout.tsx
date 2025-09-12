"use client"
import React from 'react'
import Stack from '../../../baseComponents/layout/Stack'
import OperationBlock from '../operations/OperationBlock'
import Grid from '../../../baseComponents/layout/Grid'
import OperandBlock from '../operand/OperandBlock'
import ResultBlock from '../result/ResultBlock'
import VerficationBlock from '../verification/VerficationBlock'
import { OperationSettingProps } from '../operations/OperationSetting'
import SectionPanelWithTitle from '../../composites/panels/SectionPanelWithTitle'

export interface BinaryLayoutProps {
    operation_left?: React.ReactNode
    operation_right?: React.ReactNode
    operand_left?: React.ReactNode
    operand_left_buildSavePayload: () => any
    operand_left_afterSave: (name: string) => void
    operand_left_copyContent?: string
    operand_right?: React.ReactNode
    operand_right_buildSavePayload: () => any
    operand_right_afterSave: (name: string) => void
    operand_right_copyContent?: string
    /**
     * 2ブロック間のギャップ。例: 24 or "24px"
     */
    operandGap?: number | string
    /**
     * CSS ベースの固定幅レンジ（推奨）。clamp(min, (100%-gap)/2, max) でページ幅に収まる等幅を作る。
     * 数値や CSS 長さが使えます（例: 320, "320px"／560, "560px"）。
     */
    operandWidthMin?: number | string
    operandWidthMax?: number | string
    /**
     * 旧来の固定幅（px or CSS 長さ）。指定した場合はこの幅を優先（columns=2 で固定）
     */
    operandWidth?: number | string
    result?: React.ReactNode
    verification?: React.ReactNode
    document?: React.ReactNode
    documentTitle?: string
}

type Props = BinaryLayoutProps & OperationSettingProps

export const BinaryLayout: React.FC<Props> = ({
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
    operand_left,
    operand_left_buildSavePayload,
    operand_left_afterSave,
    operand_left_copyContent,
    operand_right,
    operand_right_buildSavePayload,
    operand_right_afterSave,
            operand_right_copyContent,
            operandGap = 12,
            operandWidthMin = 320,
            operandWidthMax = 560,
            operandWidth,
    result,
    verification,
    document,
    documentTitle
}) => {
        const toCss = (v: number | string | undefined): string | undefined => {
            if (v === undefined) return undefined
            return typeof v === 'number' ? `${v}px` : v
        }
        const gapCss = toCss(operandGap) ?? '12px'
        // CSSベースの固定幅：clamp(min, (100% - gap)/2, max)
        const columnWidth = operandWidth
            ? toCss(operandWidth)
            : `clamp(${toCss(operandWidthMin) ?? '320px'}, calc((100% - ${gapCss}) / 2), ${toCss(operandWidthMax) ?? '560px'})`
  return (
    <Stack
        gap={12}
    >
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
        
                        <div>
                            <Grid columns={2} columnWidth={columnWidth} gap={operandGap}>
                        <OperandBlock title="左オペランド" buildSavePayload={operand_left_buildSavePayload} onAfterSave={operand_left_afterSave} copyContent={operand_left_copyContent} >{operand_left}</OperandBlock>
                        <OperandBlock title="右オペランド" buildSavePayload={operand_right_buildSavePayload} onAfterSave={operand_right_afterSave} copyContent={operand_right_copyContent} >{operand_right}</OperandBlock>
                    </Grid>
                </div>

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

export default BinaryLayout
