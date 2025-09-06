"use client"
import React from 'react'
import type { Vector, Matrix } from '../src/widgets/dto/linalg'
import type { Complex } from '../src/widgets/dto/complex'
import type { Polynomial, RationalFunction } from '../src/widgets/dto/polynomial'
import type { TransferFunction, Zpk } from '../src/widgets/dto/lti-systems'
import type { Signal, Spectrum } from '../src/widgets/dto/signal_processing'

import { VectorInput, MatrixInput, ComplexInput, PolynomialInput, RationalFunctionInput, TransferFunctionInput, ZpkInput, SignalInput, SpectrumInput } from '../src/widgets/input'
import { VectorSizeControls, MatrixSizeControls } from '../src/widgets/input/SizeControls'
import { VectorView, MatrixView, ComplexView, PolynomialView, RationalFunctionView, TransferFunctionView, ZpkView, SignalView, SpectrumView } from '../src/widgets/display/index'

export default function Page() {
  // Vector
  const [vec, setVec] = React.useState<Vector>({ data: [1, 2, 3] })
  const [vecN, setVecN] = React.useState<number>(3)
  const applyVec = React.useCallback(() => setVec({ data: (vec.data.slice(0, vecN).concat(Array(Math.max(0, vecN - vec.data.length)).fill(0))) }), [vec, vecN])
  // Matrix
  const [mat, setMat] = React.useState<Matrix>({ rows: 2, cols: 2, data: [1, 0, 0, 1] })
  const [mRows, setMRows] = React.useState<number>(2)
  const [mCols, setMCols] = React.useState<number>(2)
  const applyMat = React.useCallback(() => {
    const r = mRows, c = mCols
    const size = r * c
    const next = mat.data.slice(0, size).concat(Array(Math.max(0, size - mat.data.length)).fill(0))
    setMat({ rows: r, cols: c, data: next })
  }, [mRows, mCols, mat])
  // Complex
  const [z, setZ] = React.useState<Complex>({ re: 1, im: 1 })
  // Polynomial / Rational
  const [poly, setPoly] = React.useState<Polynomial>({ coeffs: [1, -3, 2] })
  const [rat, setRat] = React.useState<RationalFunction>({ numerator: { coeffs: [1, 0] }, denominator: { coeffs: [1, -1] } })
  // LTI (TF / ZPK)
  const [tf, setTf] = React.useState<TransferFunction>({ num: [1, 0.5], den: [1, -0.3], sample_time: 1 })
  const [zpk, setZpk] = React.useState<Zpk>({ zeros: [0.2, 0], poles: [0.5, 0, -0.2, 0], gain: 1, sample_time: 1 })
  // Signal / Spectrum
  const [sig, setSig] = React.useState<Signal>({ data: [0, 1, 0, -1], sample_rate: 4 })
  const [spec, setSpec] = React.useState<Spectrum>({ data: [{ re: 1, im: 0 }, { re: 0, im: -1 }], sample_rate: 2 })

  return (
    <main style={{ display: 'grid', gap: 24 }}>
      <section>
        <h2 style={{ margin: '8px 0' }}>Vector</h2>
        <div style={{ display: 'grid', gap: 6 }}>
          <VectorSizeControls length={vecN} onChange={setVecN} onApply={applyVec} />
          <VectorInput value={vec} onChange={setVec} orientation="row" length={vecN} />
        </div>
        <div style={{ marginTop: 8 }}><VectorView value={vec} orientation="row" /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>Matrix</h2>
        <div style={{ display: 'grid', gap: 6 }}>
          <MatrixSizeControls rows={mRows} cols={mCols} onChange={(r, c) => { setMRows(r); setMCols(c) }} onApply={applyMat} />
          <MatrixInput value={mat} onChange={setMat} rows={mRows} cols={mCols} />
        </div>
        <div style={{ marginTop: 8 }}><MatrixView value={mat} /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>Complex</h2>
        <ComplexInput value={z} onChange={setZ} />
        <div style={{ marginTop: 8 }}><ComplexView value={z} /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>Polynomial</h2>
        <PolynomialInput value={poly} onChange={setPoly} />
        <div style={{ marginTop: 8 }}><PolynomialView value={poly} /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>Rational Function</h2>
        <RationalFunctionInput value={rat} onChange={setRat} />
        <div style={{ marginTop: 8 }}><RationalFunctionView value={rat} /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>Transfer Function</h2>
        <TransferFunctionInput value={tf} onChange={setTf} />
        <div style={{ marginTop: 8 }}><TransferFunctionView value={tf} /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>ZPK</h2>
        <ZpkInput value={zpk} onChange={setZpk} />
        <div style={{ marginTop: 8 }}><ZpkView value={zpk} /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>Signal</h2>
        <SignalInput value={sig} onChange={setSig} />
        <div style={{ marginTop: 8 }}><SignalView value={sig} /></div>
      </section>

      <section>
        <h2 style={{ margin: '8px 0' }}>Spectrum</h2>
        <SpectrumInput value={spec} onChange={setSpec} />
        <div style={{ marginTop: 8 }}><SpectrumView value={spec} /></div>
      </section>
    </main>
  )
}
