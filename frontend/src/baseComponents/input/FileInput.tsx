import React from 'react'

export type FileInputProps = Omit<React.InputHTMLAttributes<HTMLInputElement>, 'type' | 'onChange'> & {
  accept?: string
  multiple?: boolean
  onFiles: (files: FileList) => void
}

/**
 * FileInput: 素の <input type="file"> を薄くラップした基礎コンポーネント。
 * - デザインは親側で制御（この層では hidden などもしない）
 * - FW 非依存
 */
export const FileInput = React.forwardRef<HTMLInputElement, FileInputProps>(
  ({ accept = '*', multiple, onFiles, className, ...rest }, ref) => {
    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
      if (e.target.files && e.target.files.length > 0) {
        onFiles(e.target.files)
        // 同じファイルを連続で選んだときにも change を発火させるためリセット
        e.currentTarget.value = ''
      }
    }
    return (
      <input ref={ref} type="file" accept={accept} multiple={multiple} onChange={handleChange} className={className} {...rest} />
    )
  }
)

FileInput.displayName = 'FileInput'

export default FileInput
