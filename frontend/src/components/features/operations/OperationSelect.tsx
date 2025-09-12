"use client"
import React from 'react'
import LabeledSelect, { SelectorOption } from '../../composites/forms/LabeledSelect'

export const OperationSelect: React.FC<{
    operations: { label: string; value: string }[],
    value: string,
    onChange: (value: string) => void,
    label?: string
}> = ({
    operations,
    value,
    onChange,
    label = '演算'
}) => {
    const options: SelectorOption[] = operations.map(op => ({ label: op.label, value: op.value, disabled: false }))
    options.unshift({ label: '選択', value: 'noop', disabled: true })
    return (
        <LabeledSelect
            label={label}
            value={value}
            onChange={onChange}
            options={options}
            inline={true}
        />
    )
}

export default OperationSelect