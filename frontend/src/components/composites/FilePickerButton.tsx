"use client"
import React from 'react'
import { Button } from '../../baseComponents/controls/Button'
import FileInput from '../../baseComponents/input/FileInput'

export type FilePickerButtonProps = React.ComponentProps<typeof Button> & {
  accept?: string
  multiple?: boolean
  onFiles: (files: FileList) => void
  inputProps?: Omit<React.ComponentProps<typeof FileInput>, 'onFiles' | 'accept' | 'multiple'>
}

/**
 * FilePickerButton: ボタンと隠し FileInput の複合。
 * - 見た目は Button に準拠
 * - クリックでファイルダイアログを開く
 */
export const FilePickerButton: React.FC<FilePickerButtonProps> = ({
  accept = '*',
  multiple,
  onFiles,
  inputProps,
  children,
  ...buttonProps
}) => {
  const ref = React.useRef<HTMLInputElement>(null)
  return (
    <>
      <Button onClick={() => ref.current?.click()} {...buttonProps}>{children}</Button>
  <FileInput ref={ref} accept={accept} multiple={multiple} onFiles={onFiles} style={{ display: 'none' }} {...(inputProps||{})} />
    </>
  )
}

export default FilePickerButton
