"use client"
import React from 'react'
import { Button, ButtonProps } from './Button'

export type FilePickerButtonProps = Omit<ButtonProps, 'onClick'> & {
  accept?: string
  multiple?: boolean
  onFiles: (files: FileList) => void
}

export const FilePickerButton: React.FC<FilePickerButtonProps> = ({ accept = '*', multiple, onFiles, children, ...rest }) => {
  const ref = React.useRef<HTMLInputElement>(null)
  return (
    <>
      <Button onClick={() => ref.current?.click()} {...rest}>{children}</Button>
      <input
        ref={ref}
        type="file"
        accept={accept}
        multiple={multiple}
        style={{ display: 'none' }}
        onChange={(e) => { if (e.target.files && e.target.files.length > 0) onFiles(e.target.files); (e.currentTarget as HTMLInputElement).value = '' }}
      />
    </>
  )
}

export default FilePickerButton
