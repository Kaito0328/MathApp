import { Text } from "../../baseComponents/foundation/Text"
import NumberCellInput from "../../baseComponents/input/NumberCellInput"

export const AccuracyInput: React.FC<{
    value: number,
    onChange: (value: number) => void,
}> = ({
    value,
    onChange,
}) => {
    return (
        <>
            <Text>精度:</Text>
            <NumberCellInput
                value={value}
                onChange={onChange}
            />
        </>
    )
}