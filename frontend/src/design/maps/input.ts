import { CoreColorKey, SizeKey } from '../tokens'

// Input size -> padding classes
export const inputSizeMap: Record<SizeKey, string> = {
  [SizeKey.SM]: 'py-sm px-sm',
  [SizeKey.MD]: 'py-md px-md',
  [SizeKey.LG]: 'py-lg px-lg',
  [SizeKey.XL]: 'py-xl px-xl',
}

// Input color -> border classes
export const inputColorMap: Record<CoreColorKey, string> = {
  [CoreColorKey.Base]: 'border-base',
  [CoreColorKey.Primary]: 'border-primary',
  [CoreColorKey.Secondary]: 'border-secondary',
  [CoreColorKey.Danger]: 'border-danger',
  [CoreColorKey.Success]: 'border-success',
}
