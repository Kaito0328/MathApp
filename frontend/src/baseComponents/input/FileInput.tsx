import React from 'react'
import { CoreColorKey, SizeKey } from '../../design/tokens'
import { inputColorMap, inputSizeMap } from '../../design/maps/input'

export type FileInputProps = Omit<React.InputHTMLAttributes<HTMLInputElement>, 'type' | 'onChange' | 'size' | 'color'> & {
  accept?: string
  multiple?: boolean
  onFiles: (files: FileList) => void
  color?: CoreColorKey
  size?: SizeKey
  invalid?: boolean
}

/**
 * FileInput: 素の <input type="file"> を薄くラップした基礎コンポーネント。
 * - デザインは親側で制御（この層では hidden などもしない）
 * - FW 非依存
 */
export const FileInput = React.forwardRef<HTMLInputElement, FileInputProps>(
  ({ accept = '*', multiple, onFiles, color, size = SizeKey.MD, invalid, className, ...rest }, ref) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      if (e.target.files && e.target.files.length > 0) {
        onFiles(e.target.files)
        // 同じファイルを連続で選んだときにも change を発火させるためリセット
        e.currentTarget.value = ''
      }
    }
  const sz = size ?? SizeKey.MD
  const col = (color ?? CoreColorKey.Base) as CoreColorKey
    const sizeCls = inputSizeMap[sz]
    const colorCls = inputColorMap[col] ?? inputColorMap[CoreColorKey.Base]
    const invalidCls = invalid ? 'input-invalid' : ''
    const elCls = 'input-el'
    return (
      <input
        ref={ref}
        type="file"
        accept={accept}
        multiple={multiple}
        onChange={handleChange}
        className={[elCls, sizeCls, colorCls, invalidCls, className].filter(Boolean).join(' ')}
        {...rest}
      />
    )
  }
)

FileInput.displayName = 'FileInput'

export default FileInput
