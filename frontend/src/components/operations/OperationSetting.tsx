import { AccuracyInput } from "./AccuracyInput";
import { CalculateButton } from "./CalculateButton";
import OperationSelect from "./OperationSelect";

export const OperationSetting: React.FC<{
    operations: { label: string; value: string }[],
    operation: string,
    accuracy?: number,
    onOperationChange: (value: string) => void,
    onAccuracyChange: (value: number) => void,
    onCalc?: () => void,
    label?: string,
    accuracy_able?: boolean,
    calc_button_able?: boolean,
}> = ({
    operations,
    operation,
    accuracy,
    onOperationChange,
    onAccuracyChange,
    onCalc,
    label,
    accuracy_able = false,
    calc_button_able = false,
}) => {
    return (
        <div style={{ display: 'inline-flex', alignItems: 'center', gap: 8, flexWrap: 'wrap' }}>
            {accuracy_able && onAccuracyChange && accuracy !== undefined && (
                <AccuracyInput
                    value={accuracy}
                    onChange={onAccuracyChange}
                />
            )}

            <OperationSelect
                value={operation}
                onChange={onOperationChange}
                operations={operations}
                label={label}
            />

            {calc_button_able && onCalc && (
                <CalculateButton
                    onCalc={onCalc}
                />
            )}
        </div>

    )
}

export default OperationSetting