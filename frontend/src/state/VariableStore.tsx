"use client"
import React from 'react'

export type MatrixVar = { kind: 'matrix'; rows: number; cols: number; data: number[] }
export type VectorVar = { kind: 'vector'; length: number; data: number[] }
export type PolynomialVar = { kind: 'polynomial'; coeffs: number[] }
export type RationalVar = { kind: 'rational'; numerator: number[]; denominator: number[] }
export type AnyVar = MatrixVar | VectorVar | PolynomialVar | RationalVar

type VarsMap = Record<string, AnyVar>

type Ctx = {
  vars: VarsMap
  names: string[]
  get: (name: string) => AnyVar | undefined
  upsert: (name: string, v: AnyVar) => void
  remove: (name: string) => void
  clear: () => void
}

const VarContext = React.createContext<Ctx | undefined>(undefined)
const STORAGE_KEY = 'mathapp:variables:v1'

export function VariableStoreProvider({ children }: { children: React.ReactNode }) {
  const [vars, setVars] = React.useState<VarsMap>({})

  // load
  React.useEffect(() => {
    try {
      const raw = localStorage.getItem(STORAGE_KEY)
      if (raw) {
        const parsed = JSON.parse(raw) as VarsMap
        if (parsed && typeof parsed === 'object') setVars(parsed)
      }
    } catch (e) {
      // ignore corrupted store and reset
      console.warn('VariableStore: failed to load, resetting.', e)
      setVars({})
    }
  }, [])

  // persist
  React.useEffect(() => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(vars))
    } catch (e) {
      // storage quota/full etc. We silently ignore.
      console.warn('VariableStore: failed to persist.', e)
    }
  }, [vars])

  const ctx = React.useMemo<Ctx>(() => ({
    vars,
    names: Object.keys(vars).sort(),
    get: (name) => vars[name],
    upsert: (name, v) => setVars((m) => ({ ...m, [name]: v })),
    remove: (name) => setVars((m) => { const n = { ...m }; delete n[name]; return n }),
    clear: () => setVars({}),
  }), [vars])

  return <VarContext.Provider value={ctx}>{children}</VarContext.Provider>
}

export function useVariableStore() {
  const ctx = React.useContext(VarContext)
  if (!ctx) throw new Error('useVariableStore must be used within VariableStoreProvider')
  return ctx
}
