import { Button } from "../../baseComponents/controls/Button"

export const CalculateButton: React.FC<{
    onCalc: () => void,
}> = ({
    onCalc,
}) => {
    return (
            <Button 
                onClick={onCalc}
            >
                計算
            </Button>
    )
}