let wasm;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_export_2.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

const cachedTextDecoder = (typeof TextDecoder !== 'undefined' ? new TextDecoder('utf-8', { ignoreBOM: true, fatal: true }) : { decode: () => { throw Error('TextDecoder not available') } } );

if (typeof TextDecoder !== 'undefined') { cachedTextDecoder.decode(); };

let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = (typeof TextEncoder !== 'undefined' ? new TextEncoder('utf-8') : { encode: () => { throw Error('TextEncoder not available') } } );

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches && builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let cachedFloat64ArrayMemory0 = null;

function getFloat64ArrayMemory0() {
    if (cachedFloat64ArrayMemory0 === null || cachedFloat64ArrayMemory0.byteLength === 0) {
        cachedFloat64ArrayMemory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachedFloat64ArrayMemory0;
}

function passArrayF64ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 8, 8) >>> 0;
    getFloat64ArrayMemory0().set(arg, ptr / 8);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_export_2.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

let cachedFloat32ArrayMemory0 = null;

function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function passArrayF32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getFloat32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

let cachedUint32ArrayMemory0 = null;

function getUint32ArrayMemory0() {
    if (cachedUint32ArrayMemory0 === null || cachedUint32ArrayMemory0.byteLength === 0) {
        cachedUint32ArrayMemory0 = new Uint32Array(wasm.memory.buffer);
    }
    return cachedUint32ArrayMemory0;
}

function passArray32ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 4, 4) >>> 0;
    getUint32ArrayMemory0().set(arg, ptr / 4);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_export_2.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1, 1) >>> 0;
    getUint8ArrayMemory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

function getArrayF64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
 * @param {number} size
 * @returns {Float64Array}
 */
export function window_hann(size) {
    const ret = wasm.window_hann(size);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} size
 * @returns {Float64Array}
 */
export function window_hamming(size) {
    const ret = wasm.window_hamming(size);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} size
 * @returns {Float64Array}
 */
export function window_blackman(size) {
    const ret = wasm.window_blackman(size);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} size
 * @returns {Float64Array}
 */
export function window_rectangular(size) {
    const ret = wasm.window_rectangular(size);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} size
 * @param {number} beta
 * @returns {Float64Array}
 */
export function window_kaiser(size, beta) {
    const ret = wasm.window_kaiser(size, beta);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} num_taps
 * @param {number} normalized_cutoff
 * @returns {Float64Array}
 */
export function sp_design_fir_lowpass(num_taps, normalized_cutoff) {
    const ret = wasm.sp_design_fir_lowpass(num_taps, normalized_cutoff);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} num_taps
 * @param {number} normalized_cutoff
 * @returns {Float64Array}
 */
export function sp_design_fir_highpass(num_taps, normalized_cutoff) {
    const ret = wasm.sp_design_fir_highpass(num_taps, normalized_cutoff);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} num_taps
 * @param {number} f1
 * @param {number} f2
 * @returns {Float64Array}
 */
export function sp_design_fir_bandpass(num_taps, f1, f2) {
    const ret = wasm.sp_design_fir_bandpass(num_taps, f1, f2);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} num_taps
 * @param {number} f1
 * @param {number} f2
 * @returns {Float64Array}
 */
export function sp_design_fir_bandstop(num_taps, f1, f2) {
    const ret = wasm.sp_design_fir_bandstop(num_taps, f1, f2);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} order
 * @param {number} fs
 * @param {number} fc_hz
 * @returns {DiscreteTF}
 */
export function sp_design_iir_butter_lowpass(order, fs, fc_hz) {
    const ret = wasm.sp_design_iir_butter_lowpass(order, fs, fc_hz);
    return DiscreteTF.__wrap(ret);
}

/**
 * @param {number} order
 * @param {number} fs
 * @param {number} fc_hz
 * @returns {DiscreteTF}
 */
export function sp_design_iir_butter_highpass(order, fs, fc_hz) {
    const ret = wasm.sp_design_iir_butter_highpass(order, fs, fc_hz);
    return DiscreteTF.__wrap(ret);
}

/**
 * @param {number} order
 * @param {number} fs
 * @param {number} f1_hz
 * @param {number} f2_hz
 * @returns {DiscreteTF}
 */
export function sp_design_iir_butter_bandpass(order, fs, f1_hz, f2_hz) {
    const ret = wasm.sp_design_iir_butter_bandpass(order, fs, f1_hz, f2_hz);
    return DiscreteTF.__wrap(ret);
}

/**
 * @param {number} order
 * @param {number} fs
 * @param {number} f1_hz
 * @param {number} f2_hz
 * @returns {DiscreteTF}
 */
export function sp_design_iir_butter_bandstop(order, fs, f1_hz, f2_hz) {
    const ret = wasm.sp_design_iir_butter_bandstop(order, fs, f1_hz, f2_hz);
    return DiscreteTF.__wrap(ret);
}

/**
 * @param {number} order
 * @param {number} ripple_db
 * @param {number} fs
 * @param {number} fc_hz
 * @returns {DiscreteTF}
 */
export function sp_design_iir_cheby1_lowpass(order, ripple_db, fs, fc_hz) {
    const ret = wasm.sp_design_iir_cheby1_lowpass(order, ripple_db, fs, fc_hz);
    return DiscreteTF.__wrap(ret);
}

/**
 * @param {number} order
 * @param {number} stop_atten_db
 * @param {number} fs
 * @param {number} fc_hz
 * @returns {DiscreteTF}
 */
export function sp_design_iir_cheby2_lowpass(order, stop_atten_db, fs, fc_hz) {
    const ret = wasm.sp_design_iir_cheby2_lowpass(order, stop_atten_db, fs, fc_hz);
    return DiscreteTF.__wrap(ret);
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {Float64Array} kernel
 * @param {number} kw
 * @param {number} kh
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_convolve2d_f32_simple(src, width, height, kernel, kw, kh, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(kernel, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.img_convolve2d_f32_simple(ptr0, len0, width, height, ptr1, len1, kw, kh, border);
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {Float64Array} kernel
 * @param {number} kw
 * @param {number} kh
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_convolve2d_f32(src, width, height, kernel, kw, kh, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(kernel, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.img_convolve2d_f32(ptr0, len0, width, height, ptr1, len1, kw, kh, border);
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {Float64Array} kernel
 * @param {number} kw
 * @param {number} kh
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_convolve2d_u8(src, width, height, kernel, kw, kh, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(kernel, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.img_convolve2d_u8(ptr0, len0, width, height, ptr1, len1, kw, kh, border);
    var v3 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v3;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @returns {Float64Array}
 */
export function img_dft2d(src, width, height) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_dft2d(ptr0, len0, width, height);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} spec_interleaved
 * @param {number} width
 * @param {number} height
 * @returns {Float64Array}
 */
export function img_idft2d(spec_interleaved, width, height) {
    const ptr0 = passArrayF64ToWasm0(spec_interleaved, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_idft2d(ptr0, len0, width, height);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} spec_interleaved
 * @param {number} width
 * @param {number} height
 * @returns {Float64Array}
 */
export function img_fftshift(spec_interleaved, width, height) {
    const ptr0 = passArrayF64ToWasm0(spec_interleaved, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_fftshift(ptr0, len0, width, height);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} spec_interleaved
 * @param {number} width
 * @param {number} height
 * @returns {Float64Array}
 */
export function img_magnitude(spec_interleaved, width, height) {
    const ptr0 = passArrayF64ToWasm0(spec_interleaved, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_magnitude(ptr0, len0, width, height);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} sigma
 * @param {number} radius
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_gaussian_blur_f32(src, width, height, sigma, radius, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_gaussian_blur_f32(ptr0, len0, width, height, sigma, radius, border);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} sigma
 * @param {number} radius
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_gaussian_blur_u8(src, width, height, sigma, radius, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_gaussian_blur_u8(ptr0, len0, width, height, sigma, radius, border);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} radius
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_box_filter_f32(src, width, height, radius, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_box_filter_f32(ptr0, len0, width, height, radius, border);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} radius
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_box_filter_u8(src, width, height, radius, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_box_filter_u8(ptr0, len0, width, height, radius, border);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} sigma
 * @param {number} radius
 * @param {number} amount
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_unsharp_mask_f32(src, width, height, sigma, radius, amount, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_unsharp_mask_f32(ptr0, len0, width, height, sigma, radius, amount, border);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} sigma
 * @param {number} radius
 * @param {number} amount
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_unsharp_mask_u8(src, width, height, sigma, radius, amount, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_unsharp_mask_u8(ptr0, len0, width, height, sigma, radius, amount, border);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_sobel_magnitude_f32(src, width, height, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_sobel_magnitude_f32(ptr0, len0, width, height, border);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_sobel_magnitude_u8(src, width, height, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_sobel_magnitude_u8(ptr0, len0, width, height, border);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_laplacian_f32(src, width, height, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_laplacian_f32(ptr0, len0, width, height, border);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_laplacian_u8(src, width, height, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_laplacian_u8(ptr0, len0, width, height, border);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} radius
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_median_filter_f32(src, width, height, radius, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_median_filter_f32(ptr0, len0, width, height, radius, border);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} radius
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_median_filter_u8(src, width, height, radius, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_median_filter_u8(ptr0, len0, width, height, radius, border);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Float64Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} radius
 * @param {number} sigma_s
 * @param {number} sigma_r
 * @param {WasmBorder} border
 * @returns {Float64Array}
 */
export function img_bilateral_filter_f32(src, width, height, radius, sigma_s, sigma_r, border) {
    const ptr0 = passArrayF64ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_bilateral_filter_f32(ptr0, len0, width, height, radius, sigma_s, sigma_r, border);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} radius
 * @param {number} sigma_s
 * @param {number} sigma_r
 * @param {WasmBorder} border
 * @returns {Uint8Array}
 */
export function img_bilateral_filter_u8(src, width, height, radius, sigma_s, sigma_r, border) {
    const ptr0 = passArray8ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_bilateral_filter_u8(ptr0, len0, width, height, radius, sigma_s, sigma_r, border);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Uint8Array} rgb
 * @param {number} width
 * @param {number} height
 * @returns {Float64Array}
 */
export function rgb_u8_to_gray_f64(rgb, width, height) {
    const ptr0 = passArray8ToWasm0(rgb, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.rgb_u8_to_gray_f64(ptr0, len0, width, height);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Uint8Array} rgba
 * @param {number} width
 * @param {number} height
 * @returns {Float64Array}
 */
export function rgba_u8_to_gray_f64(rgba, width, height) {
    const ptr0 = passArray8ToWasm0(rgba, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.rgba_u8_to_gray_f64(ptr0, len0, width, height);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} gray
 * @param {number} width
 * @param {number} height
 * @returns {Uint8Array}
 */
export function gray_f64_to_rgba_u8(gray, width, height) {
    const ptr0 = passArrayF64ToWasm0(gray, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.gray_f64_to_rgba_u8(ptr0, len0, width, height);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

/**
 * @param {Uint8Array} pixels
 * @returns {Float64Array}
 */
export function u8_to_gray_f64(pixels) {
    const ptr0 = passArray8ToWasm0(pixels, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.u8_to_gray_f64(ptr0, len0);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} gray
 * @returns {Uint8Array}
 */
export function gray_f64_to_u8_clamped(gray) {
    const ptr0 = passArrayF64ToWasm0(gray, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.gray_f64_to_u8_clamped(ptr0, len0);
    var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v2;
}

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * @param {Float32Array} src
 * @param {number} width
 * @param {number} height
 * @param {Float32Array} kernel
 * @param {number} kw
 * @param {number} kh
 * @param {WasmBorder} border
 * @returns {Float32Array}
 */
export function img_convolve2d_f32_io(src, width, height, kernel, kw, kh, border) {
    const ptr0 = passArrayF32ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF32ToWasm0(kernel, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.img_convolve2d_f32_io(ptr0, len0, width, height, ptr1, len1, kw, kh, border);
    var v3 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v3;
}

/**
 * @param {Float32Array} src
 * @param {number} width
 * @param {number} height
 * @param {number} sigma
 * @param {number} radius
 * @param {WasmBorder} border
 * @returns {Float32Array}
 */
export function img_gaussian_blur_f32_io(src, width, height, sigma, radius, border) {
    const ptr0 = passArrayF32ToWasm0(src, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.img_gaussian_blur_f32_io(ptr0, len0, width, height, sigma, radius, border);
    var v2 = getArrayF32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

export function init() {
    wasm.init();
}

/**
 * @returns {number}
 */
export function __probe() {
    const ret = wasm.__probe();
    return ret;
}

/**
 * @param {number} rows
 * @param {number} cols
 * @param {Float64Array} a_data
 * @param {Float64Array} b
 * @returns {Float64Array}
 */
export function solveLinearSystem(rows, cols, a_data, b) {
    const ptr0 = passArrayF64ToWasm0(a_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.solveLinearSystem(rows, cols, ptr0, len0, ptr1, len1);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {number} rows
 * @param {number} cols
 * @param {Float64Array} a_data
 * @param {Float64Array} b
 * @param {number} alpha
 * @returns {Float64Array}
 */
export function ridgeRegression(rows, cols, a_data, b, alpha) {
    const ptr0 = passArrayF64ToWasm0(a_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.ridgeRegression(rows, cols, ptr0, len0, ptr1, len1, alpha);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {number} rows
 * @param {number} cols
 * @param {Float64Array} a_data
 * @param {Float64Array} b
 * @param {number} alpha
 * @param {number} max_iter
 * @param {number} tol
 * @returns {Float64Array}
 */
export function lassoRegression(rows, cols, a_data, b, alpha, max_iter, tol) {
    const ptr0 = passArrayF64ToWasm0(a_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.lassoRegression(rows, cols, ptr0, len0, ptr1, len1, alpha, max_iter, tol);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {number} rows
 * @param {number} cols
 * @param {Float64Array} x_data
 * @param {Float64Array} y
 * @param {number} lr
 * @param {number} max_iter
 * @returns {Float64Array}
 */
export function logisticFit(rows, cols, x_data, y, lr, max_iter) {
    const ptr0 = passArrayF64ToWasm0(x_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(y, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.logisticFit(rows, cols, ptr0, len0, ptr1, len1, lr, max_iter);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {number} cols
 * @param {Float64Array} coeffs
 * @param {Float64Array} x
 * @returns {number}
 */
export function logisticPredictProba(cols, coeffs, x) {
    const ptr0 = passArrayF64ToWasm0(coeffs, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(x, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.logisticPredictProba(cols, ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ret[0];
}

/**
 * @param {number} n_samples
 * @param {number} n_features
 * @param {Float64Array} data
 * @param {number} k
 * @param {number} max_iter
 * @param {number} tol
 * @returns {Float64Array}
 */
export function gmmFit(n_samples, n_features, data, k, max_iter, tol) {
    const ptr0 = passArrayF64ToWasm0(data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.gmmFit(n_samples, n_features, ptr0, len0, k, max_iter, tol);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {number} n_features
 * @param {Float64Array} params
 * @param {Float64Array} x
 * @returns {Float64Array}
 */
export function gmmPredictProba(n_features, params, x) {
    const ptr0 = passArrayF64ToWasm0(params, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(x, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.gmmPredictProba(n_features, ptr0, len0, ptr1, len1);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {number} rows
 * @param {number} cols
 * @param {Float64Array} x_data
 * @param {Float64Array} y
 * @param {Float64Array} prior_mean
 * @param {Float64Array} prior_cov
 * @param {Float64Array} noise_cov
 * @returns {Float64Array}
 */
export function bayesianLinearPosterior(rows, cols, x_data, y, prior_mean, prior_cov, noise_cov) {
    const ptr0 = passArrayF64ToWasm0(x_data, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(y, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(prior_mean, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(prior_cov, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArrayF64ToWasm0(noise_cov, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.bayesianLinearPosterior(rows, cols, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v6 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v6;
}

/**
 * @param {number} n
 * @param {Float64Array} f_flat
 * @param {Float64Array} q_flat
 * @param {Float64Array} x_flat
 * @param {Float64Array} p_flat
 * @returns {Float64Array}
 */
export function kalmanPredict(n, f_flat, q_flat, x_flat, p_flat) {
    const ptr0 = passArrayF64ToWasm0(f_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(q_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(x_flat, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(p_flat, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ret = wasm.kalmanPredict(n, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v5 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v5;
}

/**
 * @param {number} n
 * @param {Float64Array} h_flat
 * @param {Float64Array} r_flat
 * @param {Float64Array} z_flat
 * @param {Float64Array} x_flat
 * @param {Float64Array} p_flat
 * @returns {Float64Array}
 */
export function kalmanUpdate(n, h_flat, r_flat, z_flat, x_flat, p_flat) {
    const ptr0 = passArrayF64ToWasm0(h_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(r_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArrayF64ToWasm0(z_flat, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(x_flat, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArrayF64ToWasm0(p_flat, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.kalmanUpdate(n, ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v6 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v6;
}

/**
 * @param {Float64Array} x
 * @param {Float64Array} h
 * @returns {Float64Array}
 */
export function convolveNaiveF64(x, h) {
    const ptr0 = passArrayF64ToWasm0(x, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(h, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.convolveNaiveF64(ptr0, len0, ptr1, len1);
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {Float64Array} x
 * @param {Float64Array} h
 * @returns {Float64Array}
 */
export function convolveFftF64(x, h) {
    const ptr0 = passArrayF64ToWasm0(x, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(h, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.convolveFftF64(ptr0, len0, ptr1, len1);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @param {Float64Array} x
 * @param {Float64Array} h
 * @param {number} threshold
 * @returns {Float64Array}
 */
export function convolveAutoF64(x, h, threshold) {
    const ptr0 = passArrayF64ToWasm0(x, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(h, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.convolveAutoF64(ptr0, len0, ptr1, len1, threshold);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v3;
}

/**
 * @returns {number}
 */
export function defaultConvolutionThreshold() {
    const ret = wasm.defaultConvolutionThreshold();
    return ret >>> 0;
}

/**
 * @param {Float64Array} x_flat
 * @returns {Float64Array}
 */
export function dftComplexF64(x_flat) {
    const ptr0 = passArrayF64ToWasm0(x_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.dftComplexF64(ptr0, len0);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} x_flat
 * @returns {Float64Array}
 */
export function iftComplexF64(x_flat) {
    const ptr0 = passArrayF64ToWasm0(x_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.iftComplexF64(ptr0, len0);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {number} n
 * @param {number} k
 * @returns {number}
 */
export function binom(n, k) {
    const ret = wasm.binom(n, k);
    return ret;
}

/**
 * @param {number} n
 * @param {number} k
 * @returns {number}
 */
export function stirling2(n, k) {
    const ret = wasm.stirling2(n, k);
    return ret;
}

/**
 * @param {number} m
 * @returns {Float64Array}
 */
export function fallingFactorialPoly(m) {
    const ret = wasm.fallingFactorialPoly(m);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {number} m
 * @returns {Float64Array}
 */
export function risingFactorialPoly(m) {
    const ret = wasm.risingFactorialPoly(m);
    var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {Float64Array} coeffs_flat
 * @param {number} h
 * @returns {Float64Array}
 */
export function shiftPolyXPlusH(coeffs_flat, h) {
    const ptr0 = passArrayF64ToWasm0(coeffs_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.shiftPolyXPlusH(ptr0, len0, h);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} coeffs_flat
 * @returns {Float64Array}
 */
export function discreteDiff(coeffs_flat) {
    const ptr0 = passArrayF64ToWasm0(coeffs_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.discreteDiff(ptr0, len0);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} coeffs_flat
 * @returns {Float64Array}
 */
export function discreteSum(coeffs_flat) {
    const ptr0 = passArrayF64ToWasm0(coeffs_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.discreteSum(ptr0, len0);
    var v2 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v2;
}

/**
 * @param {Float64Array} coeffs
 * @param {Float64Array} nh_polys_flat
 * @param {Uint32Array} nh_offsets
 * @param {Float64Array} nh_bases
 * @param {Float64Array} initial_values
 * @returns {ClosedForm}
 */
export function solveRecurrence(coeffs, nh_polys_flat, nh_offsets, nh_bases, initial_values) {
    const ptr0 = passArrayF64ToWasm0(coeffs, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArrayF64ToWasm0(nh_polys_flat, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ptr2 = passArray32ToWasm0(nh_offsets, wasm.__wbindgen_malloc);
    const len2 = WASM_VECTOR_LEN;
    const ptr3 = passArrayF64ToWasm0(nh_bases, wasm.__wbindgen_malloc);
    const len3 = WASM_VECTOR_LEN;
    const ptr4 = passArrayF64ToWasm0(initial_values, wasm.__wbindgen_malloc);
    const len4 = WASM_VECTOR_LEN;
    const ret = wasm.solveRecurrence(ptr0, len0, ptr1, len1, ptr2, len2, ptr3, len3, ptr4, len4);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return ClosedForm.__wrap(ret[0]);
}

function getArrayU32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}
/**
 * @param {Uint8Array} a
 * @param {Uint8Array} b
 * @returns {number}
 */
export function hammingDistanceGF2(a, b) {
    const ptr0 = passArray8ToWasm0(a, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passArray8ToWasm0(b, wasm.__wbindgen_malloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.hammingDistanceGF2(ptr0, len0, ptr1, len1);
    return ret >>> 0;
}

/**
 * @param {Uint8Array} codebook_flat
 * @param {number} n
 * @returns {Uint32Array}
 */
export function weightDistributionGF2(codebook_flat, n) {
    const ptr0 = passArray8ToWasm0(codebook_flat, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.weightDistributionGF2(ptr0, len0, n);
    if (ret[3]) {
        throw takeFromExternrefTable0(ret[2]);
    }
    var v2 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

let cachedBigUint64ArrayMemory0 = null;

function getBigUint64ArrayMemory0() {
    if (cachedBigUint64ArrayMemory0 === null || cachedBigUint64ArrayMemory0.byteLength === 0) {
        cachedBigUint64ArrayMemory0 = new BigUint64Array(wasm.memory.buffer);
    }
    return cachedBigUint64ArrayMemory0;
}

function getArrayU64FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getBigUint64ArrayMemory0().subarray(ptr / 8, ptr / 8 + len);
}
/**
 * @param {bigint} n
 * @returns {BigUint64Array}
 */
export function nt_factor_u64(n) {
    const ret = wasm.nt_factor_u64(n);
    var v1 = getArrayU64FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
    return v1;
}

/**
 * @param {string} n_str
 * @returns {string[]}
 */
export function nt_factor_bigint_str(n_str) {
    const ptr0 = passStringToWasm0(n_str, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.nt_factor_bigint_str(ptr0, len0);
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

/**
 * @param {number} x
 * @returns {number}
 */
export function erf(x) {
    const ret = wasm.erf(x);
    return ret;
}

/**
 * @param {number} x
 * @returns {number}
 */
export function erfc(x) {
    const ret = wasm.erfc(x);
    return ret;
}

/**
 * @param {number} y
 * @returns {number}
 */
export function erfInv(y) {
    const ret = wasm.erfInv(y);
    return ret;
}

/**
 * @param {number} x
 * @returns {number}
 */
export function gamma(x) {
    const ret = wasm.gamma(x);
    return ret;
}

/**
 * @param {number} x
 * @returns {number}
 */
export function logGamma(x) {
    const ret = wasm.logGamma(x);
    return ret;
}

/**
 * @param {number} s
 * @param {number} x
 * @returns {number}
 */
export function regularizedGamma(s, x) {
    const ret = wasm.regularizedGamma(s, x);
    return ret;
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function beta(a, b) {
    const ret = wasm.beta(a, b);
    return ret;
}

/**
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function logBeta(a, b) {
    const ret = wasm.logBeta(a, b);
    return ret;
}

/**
 * @param {number} a
 * @param {number} b
 * @param {number} x
 * @returns {number}
 */
export function regularizedBeta(a, b, x) {
    const ret = wasm.regularizedBeta(a, b, x);
    return ret;
}

/**
 * @enum {0 | 1 | 2}
 */
export const WasmBorder = Object.freeze({
    ConstantZero: 0, "0": "ConstantZero",
    Replicate: 1, "1": "Replicate",
    Reflect: 2, "2": "Reflect",
});

const BCHFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bch_free(ptr >>> 0, 1));

export class BCH {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BCHFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bch_free(ptr, 0);
    }
    /**
     * @param {number} n
     * @param {Uint8Array} g
     */
    constructor(n, g) {
        const ptr0 = passArray8ToWasm0(g, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.bch_new(n, ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        BCHFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} u
     * @returns {Uint8Array}
     */
    encode(u) {
        const ptr0 = passArray8ToWasm0(u, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.bch_encode(this.__wbg_ptr, ptr0, len0);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
    /**
     * @returns {number}
     */
    k() {
        const ret = wasm.bch_k(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    n() {
        const ret = wasm.bch_n(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    t() {
        const ret = wasm.bch_t(this.__wbg_ptr);
        return ret >>> 0;
    }
}

const BernoulliFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_bernoulli_free(ptr >>> 0, 1));

export class Bernoulli {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BernoulliFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_bernoulli_free(ptr, 0);
    }
    /**
     * @param {number} p
     */
    constructor(p) {
        const ret = wasm.bernoulli_new(p);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        BernoulliFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.bernoulli_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.bernoulli_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.bernoulli_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    pmf(k) {
        const ret = wasm.bernoulli_pmf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    cdf(k) {
        const ret = wasm.bernoulli_cdf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.bernoulli_quantile(this.__wbg_ptr, p);
        return ret >>> 0;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @returns {string}
     */
    pmf_svg(width, height) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.bernoulli_pmf_svg(this.__wbg_ptr, width, height);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const BinomialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_binomial_free(ptr >>> 0, 1));

export class Binomial {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        BinomialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_binomial_free(ptr, 0);
    }
    /**
     * @param {number} n
     * @param {number} p
     */
    constructor(n, p) {
        const ret = wasm.binomial_new(n, p);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        BinomialFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.binomial_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.binomial_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.binomial_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    pmf(k) {
        const ret = wasm.binomial_pmf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    cdf(k) {
        const ret = wasm.binomial_cdf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.binomial_quantile(this.__wbg_ptr, p);
        return ret >>> 0;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @returns {string}
     */
    pmf_svg(width, height) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.binomial_pmf_svg(this.__wbg_ptr, width, height);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const CategoricalFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_categorical_free(ptr >>> 0, 1));

export class Categorical {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CategoricalFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_categorical_free(ptr, 0);
    }
    /**
     * @param {Float64Array} probs
     */
    constructor(probs) {
        const ptr0 = passArrayF64ToWasm0(probs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.categorical_new(ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        CategoricalFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    pmf(k) {
        const ret = wasm.categorical_pmf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    log_pmf(k) {
        const ret = wasm.categorical_log_pmf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    cdf(k) {
        const ret = wasm.categorical_cdf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.categorical_quantile(this.__wbg_ptr, p);
        return ret >>> 0;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @returns {string}
     */
    pmf_svg(width, height) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.categorical_pmf_svg(this.__wbg_ptr, width, height);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const ChiSquareFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_chisquare_free(ptr >>> 0, 1));

export class ChiSquare {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ChiSquareFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chisquare_free(ptr, 0);
    }
    /**
     * @param {number} k
     */
    constructor(k) {
        const ret = wasm.chisquare_new(k);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        ChiSquareFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.chisquare_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.chisquare_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.chisquare_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    pdf(x) {
        const ret = wasm.chisquare_pdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    cdf(x) {
        const ret = wasm.chisquare_cdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.chisquare_quantile(this.__wbg_ptr, p);
        return ret;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} samples
     * @returns {string}
     */
    pdf_svg(width, height, samples) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.chisquare_pdf_svg(this.__wbg_ptr, width, height, samples);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const ClosedFormFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_closedform_free(ptr >>> 0, 1));

export class ClosedForm {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ClosedForm.prototype);
        obj.__wbg_ptr = ptr;
        ClosedFormFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ClosedFormFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_closedform_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    termsCount() {
        const ret = wasm.closedform_termsCount(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} i
     * @returns {Float64Array}
     */
    termPoly(i) {
        const ret = wasm.closedform_termPoly(this.__wbg_ptr, i);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} i
     * @returns {Float64Array}
     */
    termBase(i) {
        const ret = wasm.closedform_termBase(this.__wbg_ptr, i);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} n
     * @returns {Float64Array}
     */
    term(n) {
        const ret = wasm.closedform_term(this.__wbg_ptr, n);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
}

const ContinuousSSFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_continuousss_free(ptr >>> 0, 1));

export class ContinuousSS {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ContinuousSS.prototype);
        obj.__wbg_ptr = ptr;
        ContinuousSSFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ContinuousSSFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_continuousss_free(ptr, 0);
    }
}

const ContinuousTFFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_continuoustf_free(ptr >>> 0, 1));

export class ContinuousTF {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ContinuousTF.prototype);
        obj.__wbg_ptr = ptr;
        ContinuousTFFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ContinuousTFFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_continuoustf_free(ptr, 0);
    }
}

const ContinuousZpkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_continuouszpk_free(ptr >>> 0, 1));

export class ContinuousZpk {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(ContinuousZpk.prototype);
        obj.__wbg_ptr = ptr;
        ContinuousZpkFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ContinuousZpkFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_continuouszpk_free(ptr, 0);
    }
}

const CyclicCodeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_cycliccode_free(ptr >>> 0, 1));

export class CyclicCode {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CyclicCodeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_cycliccode_free(ptr, 0);
    }
    /**
     * @param {number} n
     * @param {Uint8Array} g
     */
    constructor(n, g) {
        const ptr0 = passArray8ToWasm0(g, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cycliccode_new(n, ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        CyclicCodeFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} u
     * @returns {Uint8Array}
     */
    encode(u) {
        const ptr0 = passArray8ToWasm0(u, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.cycliccode_encode(this.__wbg_ptr, ptr0, len0);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
    /**
     * @returns {number}
     */
    k() {
        const ret = wasm.bch_k(this.__wbg_ptr);
        return ret >>> 0;
    }
}

const DiscreteSSFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_discretess_free(ptr >>> 0, 1));

export class DiscreteSS {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DiscreteSS.prototype);
        obj.__wbg_ptr = ptr;
        DiscreteSSFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DiscreteSSFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_discretess_free(ptr, 0);
    }
}

const DiscreteTFFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_discretetf_free(ptr >>> 0, 1));

export class DiscreteTF {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DiscreteTF.prototype);
        obj.__wbg_ptr = ptr;
        DiscreteTFFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DiscreteTFFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_discretetf_free(ptr, 0);
    }
}

const DiscreteZpkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_discretezpk_free(ptr >>> 0, 1));

export class DiscreteZpk {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(DiscreteZpk.prototype);
        obj.__wbg_ptr = ptr;
        DiscreteZpkFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        DiscreteZpkFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_discretezpk_free(ptr, 0);
    }
}

const ExponentialFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_exponential_free(ptr >>> 0, 1));

export class Exponential {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ExponentialFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_exponential_free(ptr, 0);
    }
    /**
     * @param {number} lambda
     */
    constructor(lambda) {
        const ret = wasm.exponential_new(lambda);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        ExponentialFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.exponential_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.exponential_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.exponential_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    pdf(x) {
        const ret = wasm.exponential_pdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    cdf(x) {
        const ret = wasm.exponential_cdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.exponential_quantile(this.__wbg_ptr, p);
        return ret;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} samples
     * @returns {string}
     */
    pdf_svg(width, height, samples) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.exponential_pdf_svg(this.__wbg_ptr, width, height, samples);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const FFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_f_free(ptr >>> 0, 1));

export class F {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        FFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_f_free(ptr, 0);
    }
    /**
     * @param {number} d1
     * @param {number} d2
     */
    constructor(d1, d2) {
        const ret = wasm.f_new(d1, d2);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        FFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.f_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.f_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.f_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    pdf(x) {
        const ret = wasm.f_pdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    cdf(x) {
        const ret = wasm.f_cdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.f_quantile(this.__wbg_ptr, p);
        return ret;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} samples
     * @returns {string}
     */
    pdf_svg(width, height, samples) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.f_pdf_svg(this.__wbg_ptr, width, height, samples);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const GF2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gf2_free(ptr >>> 0, 1));

export class GF2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GF2.prototype);
        obj.__wbg_ptr = ptr;
        GF2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GF2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gf2_free(ptr, 0);
    }
    /**
     * @param {bigint} value
     */
    constructor(value) {
        const ret = wasm.gf2_new(value);
        this.__wbg_ptr = ret >>> 0;
        GF2Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    static modulus() {
        const ret = wasm.gf2_modulus();
        return ret;
    }
    /**
     * @returns {GF2}
     */
    inv() {
        const ret = wasm.gf2_inv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return GF2.__wrap(ret[0]);
    }
    /**
     * @returns {GF2}
     */
    static zero() {
        const ret = wasm.gf2_zero();
        return GF2.__wrap(ret);
    }
    /**
     * @returns {GF2}
     */
    static one() {
        const ret = wasm.gf2_one();
        return GF2.__wrap(ret);
    }
    /**
     * @param {GF2} rhs
     * @returns {GF2}
     */
    add(rhs) {
        _assertClass(rhs, GF2);
        const ret = wasm.gf2_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return GF2.__wrap(ret);
    }
    /**
     * @param {GF2} rhs
     * @returns {GF2}
     */
    sub(rhs) {
        _assertClass(rhs, GF2);
        const ret = wasm.gf2_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return GF2.__wrap(ret);
    }
    /**
     * @param {GF2} rhs
     * @returns {GF2}
     */
    mul(rhs) {
        _assertClass(rhs, GF2);
        const ret = wasm.gf2_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return GF2.__wrap(ret);
    }
    /**
     * @param {GF2} other
     * @returns {GF2}
     */
    div(other) {
        _assertClass(other, GF2);
        const ret = wasm.gf2_div(this.__wbg_ptr, other.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return GF2.__wrap(ret[0]);
    }
    /**
     * @returns {GF2}
     */
    neg() {
        const ret = wasm.gf2_neg(this.__wbg_ptr);
        return GF2.__wrap(ret);
    }
    /**
     * @returns {bigint}
     */
    get value() {
        const ret = wasm.gf2_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get isZero() {
        const ret = wasm.gf2_is_zero(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get isOne() {
        const ret = wasm.gf2_is_one(this.__wbg_ptr);
        return ret !== 0;
    }
}

const GF3Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gf3_free(ptr >>> 0, 1));

export class GF3 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GF3.prototype);
        obj.__wbg_ptr = ptr;
        GF3Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GF3Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gf3_free(ptr, 0);
    }
    /**
     * @param {bigint} value
     */
    constructor(value) {
        const ret = wasm.gf3_new(value);
        this.__wbg_ptr = ret >>> 0;
        GF3Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    static modulus() {
        const ret = wasm.gf3_modulus();
        return ret;
    }
    /**
     * @returns {GF3}
     */
    inv() {
        const ret = wasm.gf3_inv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return GF3.__wrap(ret[0]);
    }
    /**
     * @returns {GF3}
     */
    static zero() {
        const ret = wasm.gf2_zero();
        return GF3.__wrap(ret);
    }
    /**
     * @returns {GF3}
     */
    static one() {
        const ret = wasm.gf2_one();
        return GF3.__wrap(ret);
    }
    /**
     * @param {GF3} rhs
     * @returns {GF3}
     */
    add(rhs) {
        _assertClass(rhs, GF3);
        const ret = wasm.gf3_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return GF3.__wrap(ret);
    }
    /**
     * @param {GF3} rhs
     * @returns {GF3}
     */
    sub(rhs) {
        _assertClass(rhs, GF3);
        const ret = wasm.gf3_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return GF3.__wrap(ret);
    }
    /**
     * @param {GF3} rhs
     * @returns {GF3}
     */
    mul(rhs) {
        _assertClass(rhs, GF3);
        const ret = wasm.gf3_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return GF3.__wrap(ret);
    }
    /**
     * @param {GF3} other
     * @returns {GF3}
     */
    div(other) {
        _assertClass(other, GF3);
        const ret = wasm.gf3_div(this.__wbg_ptr, other.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return GF3.__wrap(ret[0]);
    }
    /**
     * @returns {GF3}
     */
    neg() {
        const ret = wasm.gf3_neg(this.__wbg_ptr);
        return GF3.__wrap(ret);
    }
    /**
     * @returns {bigint}
     */
    get value() {
        const ret = wasm.gf2_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {boolean}
     */
    get isZero() {
        const ret = wasm.gf2_is_zero(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get isOne() {
        const ret = wasm.gf2_is_one(this.__wbg_ptr);
        return ret !== 0;
    }
}

const GFExtGF2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gfextgf2_free(ptr >>> 0, 1));

export class GFExtGF2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GFExtGF2.prototype);
        obj.__wbg_ptr = ptr;
        GFExtGF2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GFExtGF2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gfextgf2_free(ptr, 0);
    }
    /**
     * @param {Uint8Array} px_coeffs
     * @param {Uint8Array} coeffs
     */
    constructor(px_coeffs, coeffs) {
        const ptr0 = passArray8ToWasm0(px_coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(coeffs, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.gfextgf2_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        GFExtGF2Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} px_coeffs
     * @param {number} base_value
     * @returns {GFExtGF2}
     */
    static fromBase(px_coeffs, base_value) {
        const ptr0 = passArray8ToWasm0(px_coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.gfextgf2_fromBase(ptr0, len0, base_value);
        return GFExtGF2.__wrap(ret);
    }
    /**
     * @returns {GFExtGF2}
     */
    inv() {
        const ret = wasm.gfextgf2_inv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return GFExtGF2.__wrap(ret[0]);
    }
    /**
     * @returns {GFExtGF2}
     */
    static zero() {
        const ret = wasm.gfextgf2_zero();
        return GFExtGF2.__wrap(ret);
    }
    /**
     * @returns {GFExtGF2}
     */
    static one() {
        const ret = wasm.gfextgf2_one();
        return GFExtGF2.__wrap(ret);
    }
    /**
     * @param {GFExtGF2} rhs
     * @returns {GFExtGF2}
     */
    add(rhs) {
        _assertClass(rhs, GFExtGF2);
        const ret = wasm.gfextgf2_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return GFExtGF2.__wrap(ret);
    }
    /**
     * @param {GFExtGF2} rhs
     * @returns {GFExtGF2}
     */
    sub(rhs) {
        _assertClass(rhs, GFExtGF2);
        const ret = wasm.gfextgf2_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return GFExtGF2.__wrap(ret);
    }
    /**
     * @param {GFExtGF2} rhs
     * @returns {GFExtGF2}
     */
    mul(rhs) {
        _assertClass(rhs, GFExtGF2);
        const ret = wasm.gfextgf2_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return GFExtGF2.__wrap(ret);
    }
    /**
     * @param {GFExtGF2} other
     * @returns {GFExtGF2}
     */
    div(other) {
        _assertClass(other, GFExtGF2);
        const ret = wasm.gfextgf2_div(this.__wbg_ptr, other.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return GFExtGF2.__wrap(ret[0]);
    }
    /**
     * @returns {GFExtGF2}
     */
    neg() {
        const ret = wasm.gfextgf2_neg(this.__wbg_ptr);
        return GFExtGF2.__wrap(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    get coeffs() {
        const ret = wasm.gfextgf2_coeffs(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {Uint8Array}
     */
    get px() {
        const ret = wasm.gfextgf2_px(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
}

const GammaFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gamma_free(ptr >>> 0, 1));

export class Gamma {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GammaFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gamma_free(ptr, 0);
    }
    /**
     * @param {number} shape
     * @param {number} rate
     */
    constructor(shape, rate) {
        const ret = wasm.gamma_new(shape, rate);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        GammaFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.gamma_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.gamma_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.gamma_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    pdf(x) {
        const ret = wasm.gamma_pdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    cdf(x) {
        const ret = wasm.gamma_cdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.gamma_quantile(this.__wbg_ptr, p);
        return ret;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} samples
     * @returns {string}
     */
    pdf_svg(width, height, samples) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.gamma_pdf_svg(this.__wbg_ptr, width, height, samples);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const Hamming74Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_hamming74_free(ptr >>> 0, 1));

export class Hamming74 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Hamming74.prototype);
        obj.__wbg_ptr = ptr;
        Hamming74Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        Hamming74Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_hamming74_free(ptr, 0);
    }
}

const LinearCodeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_linearcode_free(ptr >>> 0, 1));

export class LinearCode {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        LinearCodeFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_linearcode_free(ptr, 0);
    }
    /**
     * @param {number} k
     * @param {number} n
     * @param {Uint8Array} g_data
     */
    constructor(k, n, g_data) {
        const ptr0 = passArray8ToWasm0(g_data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.linearcode_new(k, n, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        LinearCodeFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} u
     * @returns {Uint8Array}
     */
    encode(u) {
        const ptr0 = passArray8ToWasm0(u, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.linearcode_encode(this.__wbg_ptr, ptr0, len0);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
}

const MatrixFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_matrix_free(ptr >>> 0, 1));

export class Matrix {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Matrix.prototype);
        obj.__wbg_ptr = ptr;
        MatrixFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MatrixFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_matrix_free(ptr, 0);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @param {Float64Array} data
     */
    constructor(rows, cols, data) {
        const ptr0 = passArrayF64ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.matrix_new(rows, cols, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        MatrixFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {Matrix}
     */
    static with_default(rows, cols) {
        const ret = wasm.matrix_with_default(rows, cols);
        return Matrix.__wrap(ret);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {Matrix}
     */
    static zeros(rows, cols) {
        const ret = wasm.matrix_zeros(rows, cols);
        return Matrix.__wrap(ret);
    }
    /**
     * @param {number} size
     * @returns {Matrix}
     */
    static identity(size) {
        const ret = wasm.matrix_identity(size);
        return Matrix.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    rows() {
        const ret = wasm.matrix_rows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    cols() {
        const ret = wasm.matrix_cols(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_square() {
        const ret = wasm.matrix_is_square(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {Matrix}
     */
    transpose() {
        const ret = wasm.matrix_transpose(this.__wbg_ptr);
        return Matrix.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    trace() {
        const ret = wasm.matrix_trace(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
    /**
     * @returns {number}
     */
    determinant() {
        const ret = wasm.matrix_determinant(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
    /**
     * @returns {number}
     */
    rank() {
        const ret = wasm.matrix_rank(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @returns {Matrix | undefined}
     */
    inverse() {
        const ret = wasm.matrix_inverse(this.__wbg_ptr);
        return ret === 0 ? undefined : Matrix.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    frobenius_norm() {
        const ret = wasm.matrix_frobenius_norm(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Matrix}
     */
    expm() {
        const ret = wasm.matrix_expm(this.__wbg_ptr);
        return Matrix.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    qr_decomposition() {
        const ret = wasm.matrix_qr_decomposition(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @returns {any}
     */
    svd() {
        const ret = wasm.matrix_svd(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @returns {any}
     */
    eigen_decomposition() {
        const ret = wasm.matrix_eigen_decomposition(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @returns {Matrix}
     */
    cholesky() {
        const ret = wasm.matrix_cholesky(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Matrix.__wrap(ret[0]);
    }
    /**
     * @returns {Matrix}
     */
    pinv() {
        const ret = wasm.matrix_pinv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Matrix.__wrap(ret[0]);
    }
}

const MatrixF32Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_matrixf32_free(ptr >>> 0, 1));

export class MatrixF32 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MatrixF32.prototype);
        obj.__wbg_ptr = ptr;
        MatrixF32Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MatrixF32Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_matrixf32_free(ptr, 0);
    }
    /**
     * @param {MatrixF32} rhs
     * @returns {MatrixF32}
     */
    add(rhs) {
        _assertClass(rhs, MatrixF32);
        const ret = wasm.matrixf32_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixF32.__wrap(ret);
    }
    /**
     * @param {MatrixF32} rhs
     * @returns {MatrixF32}
     */
    sub(rhs) {
        _assertClass(rhs, MatrixF32);
        const ret = wasm.matrixf32_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixF32.__wrap(ret);
    }
    /**
     * @param {MatrixF32} rhs
     * @returns {MatrixF32}
     */
    mul(rhs) {
        _assertClass(rhs, MatrixF32);
        const ret = wasm.matrixf32_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixF32.__wrap(ret);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @param {Float32Array} data
     */
    constructor(rows, cols, data) {
        const ptr0 = passArrayF32ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.matrixf32_new(rows, cols, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        MatrixF32Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {MatrixF32}
     */
    static with_default(rows, cols) {
        const ret = wasm.matrixf32_with_default(rows, cols);
        return MatrixF32.__wrap(ret);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {MatrixF32}
     */
    static zeros(rows, cols) {
        const ret = wasm.matrixf32_zeros(rows, cols);
        return MatrixF32.__wrap(ret);
    }
    /**
     * @param {number} size
     * @returns {MatrixF32}
     */
    static identity(size) {
        const ret = wasm.matrixf32_identity(size);
        return MatrixF32.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    rows() {
        const ret = wasm.matrixf32_rows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    cols() {
        const ret = wasm.matrixf32_cols(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_square() {
        const ret = wasm.matrixf32_is_square(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {MatrixF32}
     */
    transpose() {
        const ret = wasm.matrixf32_transpose(this.__wbg_ptr);
        return MatrixF32.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    trace() {
        const ret = wasm.matrixf32_trace(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
    /**
     * @returns {number}
     */
    determinant() {
        const ret = wasm.matrixf32_determinant(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
    /**
     * @returns {number}
     */
    rank() {
        const ret = wasm.matrixf32_rank(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @returns {MatrixF32 | undefined}
     */
    inverse() {
        const ret = wasm.matrixf32_inverse(this.__wbg_ptr);
        return ret === 0 ? undefined : MatrixF32.__wrap(ret);
    }
}

const MatrixF64Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_matrixf64_free(ptr >>> 0, 1));

export class MatrixF64 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MatrixF64.prototype);
        obj.__wbg_ptr = ptr;
        MatrixF64Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MatrixF64Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_matrixf64_free(ptr, 0);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @param {Float64Array} data
     */
    constructor(rows, cols, data) {
        const ptr0 = passArrayF64ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.matrixf64_new(rows, cols, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        MatrixF64Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {MatrixF64}
     */
    static with_default(rows, cols) {
        const ret = wasm.matrixf64_with_default(rows, cols);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {MatrixF64}
     */
    static zeros(rows, cols) {
        const ret = wasm.matrixf64_zeros(rows, cols);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @param {number} size
     * @returns {MatrixF64}
     */
    static identity(size) {
        const ret = wasm.matrixf64_identity(size);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    rows() {
        const ret = wasm.matrixf32_rows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    cols() {
        const ret = wasm.matrixf32_cols(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_square() {
        const ret = wasm.matrixf32_is_square(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {MatrixF64}
     */
    transpose() {
        const ret = wasm.matrixf64_transpose(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    trace() {
        const ret = wasm.matrixf64_trace(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
    /**
     * @returns {number}
     */
    determinant() {
        const ret = wasm.matrixf64_determinant(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
    /**
     * @returns {number}
     */
    rank() {
        const ret = wasm.matrixf64_rank(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0] >>> 0;
    }
    /**
     * @returns {MatrixF64 | undefined}
     */
    inverse() {
        const ret = wasm.matrixf64_inverse(this.__wbg_ptr);
        return ret === 0 ? undefined : MatrixF64.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    frobenius_norm() {
        const ret = wasm.matrixf64_frobenius_norm(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {MatrixF64}
     */
    expm() {
        const ret = wasm.matrixf64_expm(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @returns {any}
     */
    qr_decomposition() {
        const ret = wasm.matrixf64_qr_decomposition(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @returns {any}
     */
    svd() {
        const ret = wasm.matrixf64_svd(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @returns {any}
     */
    eigen_decomposition() {
        const ret = wasm.matrixf64_eigen_decomposition(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return takeFromExternrefTable0(ret[0]);
    }
    /**
     * @returns {MatrixF64}
     */
    cholesky() {
        const ret = wasm.matrixf64_cholesky(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return MatrixF64.__wrap(ret[0]);
    }
    /**
     * @returns {MatrixF64}
     */
    pinv() {
        const ret = wasm.matrixf64_pinv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return MatrixF64.__wrap(ret[0]);
    }
    /**
     * @param {MatrixF64} rhs
     * @returns {MatrixF64}
     */
    add(rhs) {
        _assertClass(rhs, MatrixF64);
        const ret = wasm.matrixf64_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @param {MatrixF64} rhs
     * @returns {MatrixF64}
     */
    sub(rhs) {
        _assertClass(rhs, MatrixF64);
        const ret = wasm.matrixf64_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @param {MatrixF64} rhs
     * @returns {MatrixF64}
     */
    mul(rhs) {
        _assertClass(rhs, MatrixF64);
        const ret = wasm.matrixf64_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @param {number} row
     * @param {number} col
     * @returns {number}
     */
    get(row, col) {
        const ret = wasm.matrixf64_get(this.__wbg_ptr, row, col);
        return ret;
    }
    /**
     * @param {number} index
     * @returns {VectorF64 | undefined}
     */
    row(index) {
        const ret = wasm.matrixf64_row(this.__wbg_ptr, index);
        return ret === 0 ? undefined : VectorF64.__wrap(ret);
    }
    /**
     * @param {number} index
     * @returns {VectorF64 | undefined}
     */
    col(index) {
        const ret = wasm.matrixf64_col(this.__wbg_ptr, index);
        return ret === 0 ? undefined : VectorF64.__wrap(ret);
    }
    /**
     * @param {VectorF64} vector
     * @returns {VectorF64 | undefined}
     */
    multiply_vector(vector) {
        _assertClass(vector, VectorF64);
        const ret = wasm.matrixf64_multiply_vector(this.__wbg_ptr, vector.__wbg_ptr);
        return ret === 0 ? undefined : VectorF64.__wrap(ret);
    }
    /**
     * @returns {VectorF64}
     */
    diagonal() {
        const ret = wasm.matrixf64_diagonal(this.__wbg_ptr);
        return VectorF64.__wrap(ret);
    }
    /**
     * @param {VectorF64} b
     * @returns {VectorF64 | undefined}
     */
    solve(b) {
        _assertClass(b, VectorF64);
        const ret = wasm.matrixf64_solve(this.__wbg_ptr, b.__wbg_ptr);
        return ret === 0 ? undefined : VectorF64.__wrap(ret);
    }
}

const MatrixI32Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_matrixi32_free(ptr >>> 0, 1));

export class MatrixI32 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(MatrixI32.prototype);
        obj.__wbg_ptr = ptr;
        MatrixI32Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        MatrixI32Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_matrixi32_free(ptr, 0);
    }
    /**
     * @param {MatrixI32} rhs
     * @returns {MatrixI32}
     */
    add(rhs) {
        _assertClass(rhs, MatrixI32);
        const ret = wasm.matrixi32_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixI32.__wrap(ret);
    }
    /**
     * @param {MatrixI32} rhs
     * @returns {MatrixI32}
     */
    sub(rhs) {
        _assertClass(rhs, MatrixI32);
        const ret = wasm.matrixi32_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixI32.__wrap(ret);
    }
    /**
     * @param {MatrixI32} rhs
     * @returns {MatrixI32}
     */
    mul(rhs) {
        _assertClass(rhs, MatrixI32);
        const ret = wasm.matrixi32_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return MatrixI32.__wrap(ret);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @param {Int32Array} data
     */
    constructor(rows, cols, data) {
        const ptr0 = passArray32ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.matrixi32_new(rows, cols, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        MatrixI32Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {MatrixI32}
     */
    static with_default(rows, cols) {
        const ret = wasm.matrixf32_with_default(rows, cols);
        return MatrixI32.__wrap(ret);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @returns {MatrixI32}
     */
    static zeros(rows, cols) {
        const ret = wasm.matrixf32_zeros(rows, cols);
        return MatrixI32.__wrap(ret);
    }
    /**
     * @param {number} size
     * @returns {MatrixI32}
     */
    static identity(size) {
        const ret = wasm.matrixi32_identity(size);
        return MatrixI32.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    rows() {
        const ret = wasm.matrixf32_rows(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    cols() {
        const ret = wasm.matrixf32_cols(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_square() {
        const ret = wasm.matrixf32_is_square(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {MatrixI32}
     */
    transpose() {
        const ret = wasm.matrixi32_transpose(this.__wbg_ptr);
        return MatrixI32.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    trace() {
        const ret = wasm.matrixi32_trace(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return ret[0];
    }
}

const NormalFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_normal_free(ptr >>> 0, 1));

export class Normal {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        NormalFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_normal_free(ptr, 0);
    }
    /**
     * @param {number} mu
     * @param {number} sigma
     */
    constructor(mu, sigma) {
        const ret = wasm.normal_new(mu, sigma);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        NormalFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.normal_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.normal_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.normal_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    pdf(x) {
        const ret = wasm.normal_pdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    cdf(x) {
        const ret = wasm.normal_cdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.normal_quantile(this.__wbg_ptr, p);
        return ret;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} samples
     * @returns {string}
     */
    pdf_svg(width, height, samples) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.normal_pdf_svg(this.__wbg_ptr, width, height, samples);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const PoissonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_poisson_free(ptr >>> 0, 1));

export class Poisson {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PoissonFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_poisson_free(ptr, 0);
    }
    /**
     * @param {number} lambda
     */
    constructor(lambda) {
        const ret = wasm.poisson_new(lambda);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        PoissonFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.bernoulli_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.bernoulli_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.poisson_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    pmf(k) {
        const ret = wasm.poisson_pmf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    log_pmf(k) {
        const ret = wasm.poisson_log_pmf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} k
     * @returns {number}
     */
    cdf(k) {
        const ret = wasm.poisson_cdf(this.__wbg_ptr, k);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.poisson_quantile(this.__wbg_ptr, p);
        return ret >>> 0;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @returns {string}
     */
    pmf_svg(width, height) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.poisson_pmf_svg(this.__wbg_ptr, width, height);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const PolynomialF64Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_polynomialf64_free(ptr >>> 0, 1));

export class PolynomialF64 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PolynomialF64.prototype);
        obj.__wbg_ptr = ptr;
        PolynomialF64Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PolynomialF64Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_polynomialf64_free(ptr, 0);
    }
    /**
     * @param {PolynomialF64} rhs
     * @returns {PolynomialF64}
     */
    add(rhs) {
        _assertClass(rhs, PolynomialF64);
        const ret = wasm.polynomialf64_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialF64.__wrap(ret);
    }
    /**
     * @param {PolynomialF64} rhs
     * @returns {PolynomialF64}
     */
    sub(rhs) {
        _assertClass(rhs, PolynomialF64);
        const ret = wasm.polynomialf64_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialF64.__wrap(ret);
    }
    /**
     * @param {PolynomialF64} rhs
     * @returns {PolynomialF64}
     */
    mul(rhs) {
        _assertClass(rhs, PolynomialF64);
        const ret = wasm.polynomialf64_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialF64.__wrap(ret);
    }
    /**
     * @param {PolynomialF64} rhs
     * @returns {PolynomialF64}
     */
    div(rhs) {
        _assertClass(rhs, PolynomialF64);
        const ret = wasm.polynomialf64_div(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialF64.__wrap(ret);
    }
    /**
     * @param {PolynomialF64} other
     * @returns {PolynomialF64[]}
     */
    divRem(other) {
        _assertClass(other, PolynomialF64);
        const ret = wasm.polynomialf64_divRem(this.__wbg_ptr, other.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Float64Array} coeffs
     */
    constructor(coeffs) {
        const ptr0 = passArrayF64ToWasm0(coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polynomialf64_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        PolynomialF64Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    deg() {
        const ret = wasm.polynomialf64_deg(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} i
     * @returns {number}
     */
    get(i) {
        const ret = wasm.polynomialf64_get(this.__wbg_ptr, i);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    eval(x) {
        const ret = wasm.polynomialf64_eval(this.__wbg_ptr, x);
        return ret;
    }
}

const PolynomialGF2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_polynomialgf2_free(ptr >>> 0, 1));

export class PolynomialGF2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PolynomialGF2.prototype);
        obj.__wbg_ptr = ptr;
        PolynomialGF2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PolynomialGF2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_polynomialgf2_free(ptr, 0);
    }
    /**
     * @param {PolynomialGF2} rhs
     * @returns {PolynomialGF2}
     */
    add(rhs) {
        _assertClass(rhs, PolynomialGF2);
        const ret = wasm.polynomialgf2_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGF2} rhs
     * @returns {PolynomialGF2}
     */
    sub(rhs) {
        _assertClass(rhs, PolynomialGF2);
        const ret = wasm.polynomialgf2_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGF2} rhs
     * @returns {PolynomialGF2}
     */
    mul(rhs) {
        _assertClass(rhs, PolynomialGF2);
        const ret = wasm.polynomialgf2_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGF2} rhs
     * @returns {PolynomialGF2}
     */
    div(rhs) {
        _assertClass(rhs, PolynomialGF2);
        const ret = wasm.polynomialgf2_div(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGF2} other
     * @returns {PolynomialGF2[]}
     */
    divRem(other) {
        _assertClass(other, PolynomialGF2);
        const ret = wasm.polynomialgf2_divRem(this.__wbg_ptr, other.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Uint8Array} coeffs
     */
    constructor(coeffs) {
        const ptr0 = passArray8ToWasm0(coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polynomialgf2_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        PolynomialGF2Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    deg() {
        const ret = wasm.polynomialgf2_deg(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} i
     * @returns {number}
     */
    get(i) {
        const ret = wasm.polynomialgf2_get(this.__wbg_ptr, i);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    eval(x) {
        const ret = wasm.polynomialgf2_eval(this.__wbg_ptr, x);
        return ret;
    }
}

const PolynomialGF256Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_polynomialgf256_free(ptr >>> 0, 1));

export class PolynomialGF256 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PolynomialGF256.prototype);
        obj.__wbg_ptr = ptr;
        PolynomialGF256Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PolynomialGF256Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_polynomialgf256_free(ptr, 0);
    }
    /**
     * @param {PolynomialGF256} rhs
     * @returns {PolynomialGF256}
     */
    add(rhs) {
        _assertClass(rhs, PolynomialGF256);
        const ret = wasm.polynomialgf256_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF256.__wrap(ret);
    }
    /**
     * @param {PolynomialGF256} rhs
     * @returns {PolynomialGF256}
     */
    sub(rhs) {
        _assertClass(rhs, PolynomialGF256);
        const ret = wasm.polynomialgf256_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF256.__wrap(ret);
    }
    /**
     * @param {PolynomialGF256} rhs
     * @returns {PolynomialGF256}
     */
    mul(rhs) {
        _assertClass(rhs, PolynomialGF256);
        const ret = wasm.polynomialgf256_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF256.__wrap(ret);
    }
    /**
     * @param {PolynomialGF256} rhs
     * @returns {PolynomialGF256}
     */
    div(rhs) {
        _assertClass(rhs, PolynomialGF256);
        const ret = wasm.polynomialgf256_div(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGF256.__wrap(ret);
    }
    /**
     * @param {PolynomialGF256} other
     * @returns {PolynomialGF256[]}
     */
    divRem(other) {
        _assertClass(other, PolynomialGF256);
        const ret = wasm.polynomialgf256_divRem(this.__wbg_ptr, other.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Uint8Array} coeffs
     */
    constructor(coeffs) {
        const ptr0 = passArray8ToWasm0(coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polynomialgf256_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        PolynomialGF256Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    deg() {
        const ret = wasm.polynomialgf256_deg(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} i
     * @returns {number}
     */
    get(i) {
        const ret = wasm.polynomialgf256_get(this.__wbg_ptr, i);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    eval(x) {
        const ret = wasm.polynomialgf256_eval(this.__wbg_ptr, x);
        return ret;
    }
}

const PolynomialGFExtGF2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_polynomialgfextgf2_free(ptr >>> 0, 1));

export class PolynomialGFExtGF2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(PolynomialGFExtGF2.prototype);
        obj.__wbg_ptr = ptr;
        PolynomialGFExtGF2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PolynomialGFExtGF2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_polynomialgfextgf2_free(ptr, 0);
    }
    /**
     * @param {PolynomialGFExtGF2} rhs
     * @returns {PolynomialGFExtGF2}
     */
    add(rhs) {
        _assertClass(rhs, PolynomialGFExtGF2);
        const ret = wasm.polynomialgfextgf2_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGFExtGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGFExtGF2} rhs
     * @returns {PolynomialGFExtGF2}
     */
    sub(rhs) {
        _assertClass(rhs, PolynomialGFExtGF2);
        const ret = wasm.polynomialgfextgf2_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGFExtGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGFExtGF2} rhs
     * @returns {PolynomialGFExtGF2}
     */
    mul(rhs) {
        _assertClass(rhs, PolynomialGFExtGF2);
        const ret = wasm.polynomialgfextgf2_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGFExtGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGFExtGF2} rhs
     * @returns {PolynomialGFExtGF2}
     */
    div(rhs) {
        _assertClass(rhs, PolynomialGFExtGF2);
        const ret = wasm.polynomialgfextgf2_div(this.__wbg_ptr, rhs.__wbg_ptr);
        return PolynomialGFExtGF2.__wrap(ret);
    }
    /**
     * @param {PolynomialGFExtGF2} other
     * @returns {PolynomialGFExtGF2[]}
     */
    divRem(other) {
        _assertClass(other, PolynomialGFExtGF2);
        const ret = wasm.polynomialgfextgf2_divRem(this.__wbg_ptr, other.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Uint8Array} px
     * @param {Uint8Array[]} coeffs
     */
    constructor(px, coeffs) {
        const ptr0 = passArray8ToWasm0(px, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayJsValueToWasm0(coeffs, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.polynomialgfextgf2_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        PolynomialGFExtGF2Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    deg() {
        const ret = wasm.polynomialgf256_deg(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} i
     * @returns {Uint8Array}
     */
    get(i) {
        const ret = wasm.polynomialgfextgf2_get(this.__wbg_ptr, i);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {Uint8Array} x_coeffs
     * @returns {Uint8Array}
     */
    eval(x_coeffs) {
        const ptr0 = passArray8ToWasm0(x_coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.polynomialgfextgf2_eval(this.__wbg_ptr, ptr0, len0);
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
}

const ReedSolomonFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_reedsolomon_free(ptr >>> 0, 1));

export class ReedSolomon {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        ReedSolomonFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_reedsolomon_free(ptr, 0);
    }
    /**
     * @param {number} k
     * @param {Uint8Array} alphas
     */
    constructor(k, alphas) {
        const ptr0 = passArray8ToWasm0(alphas, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.reedsolomon_new(k, ptr0, len0);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        ReedSolomonFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} f
     * @returns {Uint8Array}
     */
    encode(f) {
        const ptr0 = passArray8ToWasm0(f, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.reedsolomon_encode(this.__wbg_ptr, ptr0, len0);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
    /**
     * @param {Uint8Array} r
     * @returns {Uint8Array}
     */
    decode(r) {
        const ptr0 = passArray8ToWasm0(r, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.reedsolomon_decode(this.__wbg_ptr, ptr0, len0);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
    /**
     * @returns {number}
     */
    n() {
        const ret = wasm.reedsolomon_n(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    t() {
        const ret = wasm.reedsolomon_t(this.__wbg_ptr);
        return ret >>> 0;
    }
}

const StudentTFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_studentt_free(ptr >>> 0, 1));

export class StudentT {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        StudentTFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_studentt_free(ptr, 0);
    }
    /**
     * @param {number} df
     */
    constructor(df) {
        const ret = wasm.studentt_new(df);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        StudentTFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.studentt_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.studentt_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.studentt_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    pdf(x) {
        const ret = wasm.studentt_pdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    cdf(x) {
        const ret = wasm.studentt_cdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.studentt_quantile(this.__wbg_ptr, p);
        return ret;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} samples
     * @returns {string}
     */
    pdf_svg(width, height, samples) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.studentt_pdf_svg(this.__wbg_ptr, width, height, samples);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const UniformFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_uniform_free(ptr >>> 0, 1));

export class Uniform {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        UniformFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_uniform_free(ptr, 0);
    }
    /**
     * @param {number} a
     * @param {number} b
     */
    constructor(a, b) {
        const ret = wasm.uniform_new(a, b);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        UniformFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    mean() {
        const ret = wasm.uniform_mean(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    variance() {
        const ret = wasm.uniform_variance(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    std_dev() {
        const ret = wasm.uniform_std_dev(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    pdf(x) {
        const ret = wasm.uniform_pdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} x
     * @returns {number}
     */
    cdf(x) {
        const ret = wasm.uniform_cdf(this.__wbg_ptr, x);
        return ret;
    }
    /**
     * @param {number} p
     * @returns {number}
     */
    quantile(p) {
        const ret = wasm.uniform_quantile(this.__wbg_ptr, p);
        return ret;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} samples
     * @returns {string}
     */
    pdf_svg(width, height, samples) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.uniform_pdf_svg(this.__wbg_ptr, width, height, samples);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}

const VectorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vector_free(ptr >>> 0, 1));

export class Vector {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Vector.prototype);
        obj.__wbg_ptr = ptr;
        VectorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VectorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vector_free(ptr, 0);
    }
    /**
     * @param {Float64Array} data
     */
    constructor(data) {
        const ptr0 = passArrayF64ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.vector_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        VectorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} dim
     * @returns {Vector}
     */
    static zeros(dim) {
        const ret = wasm.vector_zeros(dim);
        return Vector.__wrap(ret);
    }
    /**
     * @param {number} dim
     * @returns {Vector}
     */
    static ones(dim) {
        const ret = wasm.vector_ones(dim);
        return Vector.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    dim() {
        const ret = wasm.vector_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    len() {
        const ret = wasm.vector_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_empty() {
        const ret = wasm.vector_is_empty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {Vector} other
     * @returns {number}
     */
    dot(other) {
        _assertClass(other, Vector);
        const ret = wasm.vector_dot(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmax() {
        const ret = wasm.vector_argmax(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmin() {
        const ret = wasm.vector_argmin(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    max() {
        const ret = wasm.vector_max(this.__wbg_ptr);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * @returns {number | undefined}
     */
    min() {
        const ret = wasm.vector_min(this.__wbg_ptr);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * @returns {number}
     */
    norm() {
        const ret = wasm.vector_norm(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Vector}
     */
    normalize() {
        const ret = wasm.vector_normalize(this.__wbg_ptr);
        return Vector.__wrap(ret);
    }
    /**
     * @param {Vector} other
     * @returns {number}
     */
    cosine_similarity(other) {
        _assertClass(other, Vector);
        const ret = wasm.vector_cosine_similarity(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number | undefined}
     */
    mean() {
        const ret = wasm.vector_mean(this.__wbg_ptr);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * @returns {number}
     */
    std() {
        const ret = wasm.vector_std(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} start
     * @param {number} end
     * @param {number} num
     * @returns {Vector}
     */
    static linspace(start, end, num) {
        const ret = wasm.vector_linspace(start, end, num);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return Vector.__wrap(ret[0]);
    }
    /**
     * @returns {number}
     */
    sum() {
        const ret = wasm.vector_sum(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {MatrixF64}
     */
    transpose() {
        const ret = wasm.vector_transpose(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @returns {MatrixF64}
     */
    to_column_matrix() {
        const ret = wasm.vector_to_column_matrix(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @returns {MatrixF64}
     */
    to_row_matrix() {
        const ret = wasm.vector_to_row_matrix(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
}

const VectorF32Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vectorf32_free(ptr >>> 0, 1));

export class VectorF32 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(VectorF32.prototype);
        obj.__wbg_ptr = ptr;
        VectorF32Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VectorF32Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vectorf32_free(ptr, 0);
    }
    /**
     * @param {VectorF32} rhs
     * @returns {VectorF32}
     */
    add(rhs) {
        _assertClass(rhs, VectorF32);
        const ret = wasm.vectorf32_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorF32.__wrap(ret);
    }
    /**
     * @param {VectorF32} rhs
     * @returns {VectorF32}
     */
    sub(rhs) {
        _assertClass(rhs, VectorF32);
        const ret = wasm.vectorf32_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorF32.__wrap(ret);
    }
    /**
     * @param {VectorF32} rhs
     * @returns {VectorF32}
     */
    mul(rhs) {
        _assertClass(rhs, VectorF32);
        const ret = wasm.vectorf32_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorF32.__wrap(ret);
    }
    /**
     * @param {Float32Array} data
     */
    constructor(data) {
        const ptr0 = passArrayF32ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.vectorf32_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        VectorF32Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} dim
     * @returns {VectorF32}
     */
    static zeros(dim) {
        const ret = wasm.vectorf32_zeros(dim);
        return VectorF32.__wrap(ret);
    }
    /**
     * @param {number} dim
     * @returns {VectorF32}
     */
    static ones(dim) {
        const ret = wasm.vectorf32_ones(dim);
        return VectorF32.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    dim() {
        const ret = wasm.vectorf32_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    len() {
        const ret = wasm.vectorf32_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_empty() {
        const ret = wasm.vectorf32_is_empty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {VectorF32} other
     * @returns {number}
     */
    dot(other) {
        _assertClass(other, VectorF32);
        const ret = wasm.vectorf32_dot(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmax() {
        const ret = wasm.vectorf32_argmax(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmin() {
        const ret = wasm.vectorf32_argmin(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    max() {
        const ret = wasm.vectorf32_max(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    min() {
        const ret = wasm.vectorf32_min(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
}

const VectorF64Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vectorf64_free(ptr >>> 0, 1));

export class VectorF64 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(VectorF64.prototype);
        obj.__wbg_ptr = ptr;
        VectorF64Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VectorF64Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vectorf64_free(ptr, 0);
    }
    /**
     * @param {Float64Array} data
     */
    constructor(data) {
        const ptr0 = passArrayF64ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.vectorf64_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        VectorF64Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} dim
     * @returns {VectorF64}
     */
    static zeros(dim) {
        const ret = wasm.vectorf64_zeros(dim);
        return VectorF64.__wrap(ret);
    }
    /**
     * @param {number} dim
     * @returns {VectorF64}
     */
    static ones(dim) {
        const ret = wasm.vectorf64_ones(dim);
        return VectorF64.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    dim() {
        const ret = wasm.vectorf64_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    len() {
        const ret = wasm.vectorf64_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_empty() {
        const ret = wasm.vectorf64_is_empty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {VectorF64} other
     * @returns {number}
     */
    dot(other) {
        _assertClass(other, VectorF64);
        const ret = wasm.vectorf64_dot(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmax() {
        const ret = wasm.vectorf64_argmax(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmin() {
        const ret = wasm.vectorf64_argmin(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    max() {
        const ret = wasm.vectorf64_max(this.__wbg_ptr);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * @returns {number | undefined}
     */
    min() {
        const ret = wasm.vectorf64_min(this.__wbg_ptr);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * @returns {number}
     */
    norm() {
        const ret = wasm.vectorf64_norm(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {VectorF64}
     */
    normalize() {
        const ret = wasm.vectorf64_normalize(this.__wbg_ptr);
        return VectorF64.__wrap(ret);
    }
    /**
     * @param {VectorF64} other
     * @returns {number}
     */
    cosine_similarity(other) {
        _assertClass(other, VectorF64);
        const ret = wasm.vectorf64_cosine_similarity(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number | undefined}
     */
    mean() {
        const ret = wasm.vectorf64_mean(this.__wbg_ptr);
        return ret[0] === 0 ? undefined : ret[1];
    }
    /**
     * @returns {number}
     */
    std() {
        const ret = wasm.vectorf64_std(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} start
     * @param {number} end
     * @param {number} num
     * @returns {VectorF64}
     */
    static linspace(start, end, num) {
        const ret = wasm.vectorf64_linspace(start, end, num);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return VectorF64.__wrap(ret[0]);
    }
    /**
     * @param {VectorF64} rhs
     * @returns {VectorF64}
     */
    add(rhs) {
        _assertClass(rhs, VectorF64);
        const ret = wasm.vectorf64_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorF64.__wrap(ret);
    }
    /**
     * @param {VectorF64} rhs
     * @returns {VectorF64}
     */
    sub(rhs) {
        _assertClass(rhs, VectorF64);
        const ret = wasm.vectorf64_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorF64.__wrap(ret);
    }
    /**
     * @param {VectorF64} rhs
     * @returns {VectorF64}
     */
    mul(rhs) {
        _assertClass(rhs, VectorF64);
        const ret = wasm.vectorf64_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorF64.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    sum() {
        const ret = wasm.vectorf64_sum(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {MatrixF64} matrix
     * @returns {MatrixF64}
     */
    multiply_matrix(matrix) {
        _assertClass(matrix, MatrixF64);
        const ret = wasm.vectorf64_multiply_matrix(this.__wbg_ptr, matrix.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return MatrixF64.__wrap(ret[0]);
    }
    /**
     * @returns {MatrixF64}
     */
    transpose() {
        const ret = wasm.vectorf64_transpose(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @returns {MatrixF64}
     */
    to_column_matrix() {
        const ret = wasm.vectorf64_to_column_matrix(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
    /**
     * @returns {MatrixF64}
     */
    to_row_matrix() {
        const ret = wasm.vectorf64_to_row_matrix(this.__wbg_ptr);
        return MatrixF64.__wrap(ret);
    }
}

const VectorI32Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_vectori32_free(ptr >>> 0, 1));

export class VectorI32 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(VectorI32.prototype);
        obj.__wbg_ptr = ptr;
        VectorI32Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        VectorI32Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_vectori32_free(ptr, 0);
    }
    /**
     * @param {VectorI32} rhs
     * @returns {VectorI32}
     */
    add(rhs) {
        _assertClass(rhs, VectorI32);
        const ret = wasm.vectori32_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorI32.__wrap(ret);
    }
    /**
     * @param {VectorI32} rhs
     * @returns {VectorI32}
     */
    sub(rhs) {
        _assertClass(rhs, VectorI32);
        const ret = wasm.vectori32_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorI32.__wrap(ret);
    }
    /**
     * @param {VectorI32} rhs
     * @returns {VectorI32}
     */
    mul(rhs) {
        _assertClass(rhs, VectorI32);
        const ret = wasm.vectori32_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return VectorI32.__wrap(ret);
    }
    /**
     * @param {Int32Array} data
     */
    constructor(data) {
        const ptr0 = passArray32ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.vectorf32_new(ptr0, len0);
        this.__wbg_ptr = ret >>> 0;
        VectorI32Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} dim
     * @returns {VectorI32}
     */
    static zeros(dim) {
        const ret = wasm.vectorf32_zeros(dim);
        return VectorI32.__wrap(ret);
    }
    /**
     * @param {number} dim
     * @returns {VectorI32}
     */
    static ones(dim) {
        const ret = wasm.vectori32_ones(dim);
        return VectorI32.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    dim() {
        const ret = wasm.vectorf32_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    len() {
        const ret = wasm.vectorf32_dim(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {boolean}
     */
    is_empty() {
        const ret = wasm.vectorf32_is_empty(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {VectorI32} other
     * @returns {number}
     */
    dot(other) {
        _assertClass(other, VectorI32);
        const ret = wasm.vectori32_dot(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmax() {
        const ret = wasm.vectori32_argmax(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    argmin() {
        const ret = wasm.vectori32_argmin(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    max() {
        const ret = wasm.vectori32_max(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @returns {number | undefined}
     */
    min() {
        const ret = wasm.vectori32_min(this.__wbg_ptr);
        return ret === 0x100000001 ? undefined : ret;
    }
}

const WasmContinuousSSFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcontinuousss_free(ptr >>> 0, 1));

export class WasmContinuousSS {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmContinuousSSFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcontinuousss_free(ptr, 0);
    }
    /**
     * @param {Float64Array} a
     * @param {number} na
     * @param {number} ma
     * @param {Float64Array} b
     * @param {number} nb
     * @param {number} mb
     * @param {Float64Array} c
     * @param {number} nc
     * @param {number} mc
     * @param {Float64Array} d
     * @param {number} nd
     * @param {number} md
     */
    constructor(a, na, ma, b, nb, mb, c, nc, mc, d, nd, md) {
        const ptr0 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArrayF64ToWasm0(c, wasm.__wbindgen_malloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passArrayF64ToWasm0(d, wasm.__wbindgen_malloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.wasmcontinuousss_new(ptr0, len0, na, ma, ptr1, len1, nb, mb, ptr2, len2, nc, mc, ptr3, len3, nd, md);
        return ContinuousSS.__wrap(ret);
    }
    /**
     * @param {Float64Array} num
     * @param {Float64Array} den
     * @returns {ContinuousSS}
     */
    static from_tf_siso(num, den) {
        const ptr0 = passArrayF64ToWasm0(num, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(den, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmcontinuousss_from_tf_siso(ptr0, len0, ptr1, len1);
        return ContinuousSS.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    to_tf_siso() {
        const ret = wasm.wasmcontinuousss_to_tf_siso(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} fs
     * @returns {DiscreteSS}
     */
    c2d_zoh(fs) {
        const ret = wasm.wasmcontinuousss_c2d_zoh(this.__wbg_ptr, fs);
        return DiscreteSS.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    a_flat() {
        const ret = wasm.wasmcontinuousss_a_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    b_flat() {
        const ret = wasm.wasmcontinuousss_b_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    c_flat() {
        const ret = wasm.wasmcontinuousss_c_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    d_flat() {
        const ret = wasm.wasmcontinuousss_d_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    a_shape() {
        const ret = wasm.wasmcontinuousss_a_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    b_shape() {
        const ret = wasm.wasmcontinuousss_b_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    c_shape() {
        const ret = wasm.wasmcontinuousss_c_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    d_shape() {
        const ret = wasm.wasmcontinuousss_d_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
}

const WasmContinuousTFFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcontinuoustf_free(ptr >>> 0, 1));

export class WasmContinuousTF {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmContinuousTFFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcontinuoustf_free(ptr, 0);
    }
    /**
     * @param {Float64Array} b
     * @param {Float64Array} a
     */
    constructor(b, a) {
        const ptr0 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmcontinuoustf_new(ptr0, len0, ptr1, len1);
        return ContinuousTF.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    b_coeffs() {
        const ret = wasm.wasmcontinuoustf_b_coeffs(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    a_coeffs() {
        const ret = wasm.wasmcontinuoustf_a_coeffs(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {boolean}
     */
    is_stable() {
        const ret = wasm.wasmcontinuoustf_is_stable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} fs
     * @param {number} len
     * @returns {Float64Array}
     */
    impulse_response(fs, len) {
        const ret = wasm.wasmcontinuoustf_impulse_response(this.__wbg_ptr, fs, len);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} fs
     * @param {number} len
     * @returns {Float64Array}
     */
    step_response(fs, len) {
        const ret = wasm.wasmcontinuoustf_step_response(this.__wbg_ptr, fs, len);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} omega_max
     * @param {number} n_freqs
     * @returns {Float64Array}
     */
    frequency_response_mag_phase(omega_max, n_freqs) {
        const ret = wasm.wasmcontinuoustf_frequency_response_mag_phase(this.__wbg_ptr, omega_max, n_freqs);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} fs
     * @returns {DiscreteTF}
     */
    to_discrete_bilinear(fs) {
        const ret = wasm.wasmcontinuoustf_to_discrete_bilinear(this.__wbg_ptr, fs);
        return DiscreteTF.__wrap(ret);
    }
    /**
     * @param {number} fs
     * @param {number} f_warp_hz
     * @returns {DiscreteTF}
     */
    to_discrete_bilinear_prewarp(fs, f_warp_hz) {
        const ret = wasm.wasmcontinuoustf_to_discrete_bilinear_prewarp(this.__wbg_ptr, fs, f_warp_hz);
        return DiscreteTF.__wrap(ret);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} f_min_hz
     * @param {number} f_max_hz
     * @param {number} n_points
     * @param {boolean} legend
     * @returns {string}
     */
    bode_svg(width, height, f_min_hz, f_max_hz, n_points, legend) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmcontinuoustf_bode_svg(this.__wbg_ptr, width, height, f_min_hz, f_max_hz, n_points, legend);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} f_min_hz
     * @param {number} f_max_hz
     * @param {number} n_points
     * @param {boolean} log_freq
     * @param {boolean} legend
     * @returns {string}
     */
    nyquist_svg(width, height, f_min_hz, f_max_hz, n_points, log_freq, legend) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmcontinuoustf_nyquist_svg(this.__wbg_ptr, width, height, f_min_hz, f_max_hz, n_points, log_freq, legend);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {boolean} negative_feedback
     * @param {string | null} [feedback_label]
     * @returns {string}
     */
    block_feedback_svg(width, height, negative_feedback, feedback_label) {
        let deferred2_0;
        let deferred2_1;
        try {
            var ptr0 = isLikeNone(feedback_label) ? 0 : passStringToWasm0(feedback_label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            const ret = wasm.wasmcontinuoustf_block_feedback_svg(this.__wbg_ptr, width, height, negative_feedback, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
}

const WasmContinuousZpkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmcontinuouszpk_free(ptr >>> 0, 1));

export class WasmContinuousZpk {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmContinuousZpkFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmcontinuouszpk_free(ptr, 0);
    }
    /**
     * @param {Float64Array} zeros_interleaved
     * @param {Float64Array} poles_interleaved
     * @param {number} gain
     */
    constructor(zeros_interleaved, poles_interleaved, gain) {
        const ptr0 = passArrayF64ToWasm0(zeros_interleaved, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(poles_interleaved, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmcontinuouszpk_new(ptr0, len0, ptr1, len1, gain);
        return ContinuousZpk.__wrap(ret);
    }
    /**
     * @param {ContinuousTF} tf
     * @returns {ContinuousZpk}
     */
    static from_tf(tf) {
        _assertClass(tf, ContinuousTF);
        const ret = wasm.wasmcontinuouszpk_from_tf(tf.__wbg_ptr);
        return ContinuousZpk.__wrap(ret);
    }
    /**
     * @returns {ContinuousTF}
     */
    to_tf() {
        const ret = wasm.wasmcontinuouszpk_to_tf(this.__wbg_ptr);
        return ContinuousTF.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    zeros_interleaved() {
        const ret = wasm.wasmcontinuouszpk_zeros_interleaved(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    poles_interleaved() {
        const ret = wasm.wasmcontinuouszpk_poles_interleaved(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {number}
     */
    gain() {
        const ret = wasm.wasmcontinuouszpk_gain(this.__wbg_ptr);
        return ret;
    }
}

const WasmDiscreteSSFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmdiscretess_free(ptr >>> 0, 1));

export class WasmDiscreteSS {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmDiscreteSSFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdiscretess_free(ptr, 0);
    }
    /**
     * @param {Float64Array} a
     * @param {number} na
     * @param {number} ma
     * @param {Float64Array} b
     * @param {number} nb
     * @param {number} mb
     * @param {Float64Array} c
     * @param {number} nc
     * @param {number} mc
     * @param {Float64Array} d
     * @param {number} nd
     * @param {number} md
     */
    constructor(a, na, ma, b, nb, mb, c, nc, mc, d, nd, md) {
        const ptr0 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ptr2 = passArrayF64ToWasm0(c, wasm.__wbindgen_malloc);
        const len2 = WASM_VECTOR_LEN;
        const ptr3 = passArrayF64ToWasm0(d, wasm.__wbindgen_malloc);
        const len3 = WASM_VECTOR_LEN;
        const ret = wasm.wasmcontinuousss_new(ptr0, len0, na, ma, ptr1, len1, nb, mb, ptr2, len2, nc, mc, ptr3, len3, nd, md);
        return DiscreteSS.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    to_tf_siso() {
        const ret = wasm.wasmdiscretess_to_tf_siso(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    a_flat() {
        const ret = wasm.wasmdiscretess_a_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    b_flat() {
        const ret = wasm.wasmdiscretess_b_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    c_flat() {
        const ret = wasm.wasmdiscretess_c_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    d_flat() {
        const ret = wasm.wasmdiscretess_d_flat(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    a_shape() {
        const ret = wasm.wasmdiscretess_a_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    b_shape() {
        const ret = wasm.wasmdiscretess_b_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    c_shape() {
        const ret = wasm.wasmdiscretess_c_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {Uint32Array}
     */
    d_shape() {
        const ret = wasm.wasmdiscretess_d_shape(this.__wbg_ptr);
        var v1 = getArrayU32FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
}

const WasmDiscreteTFFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmdiscretetf_free(ptr >>> 0, 1));

export class WasmDiscreteTF {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmDiscreteTFFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdiscretetf_free(ptr, 0);
    }
    /**
     * @param {Float64Array} b
     * @param {Float64Array} a
     * @param {number} sample_rate
     */
    constructor(b, a, sample_rate) {
        const ptr0 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(a, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmdiscretetf_new(ptr0, len0, ptr1, len1, sample_rate);
        return DiscreteTF.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    sample_rate() {
        const ret = wasm.wasmdiscretetf_sample_rate(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {number} fs
     */
    set_sample_rate(fs) {
        wasm.wasmdiscretetf_set_sample_rate(this.__wbg_ptr, fs);
    }
    /**
     * @returns {Float64Array}
     */
    b_coeffs() {
        const ret = wasm.wasmdiscretetf_b_coeffs(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    a_coeffs() {
        const ret = wasm.wasmdiscretetf_a_coeffs(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {boolean}
     */
    is_stable() {
        const ret = wasm.wasmdiscretetf_is_stable(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {number} len
     * @returns {Float64Array}
     */
    impulse_response(len) {
        const ret = wasm.wasmdiscretetf_impulse_response(this.__wbg_ptr, len);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} len
     * @returns {Float64Array}
     */
    step_response(len) {
        const ret = wasm.wasmdiscretetf_step_response(this.__wbg_ptr, len);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} n_freqs
     * @returns {Float64Array}
     */
    frequency_response_mag_phase(n_freqs) {
        const ret = wasm.wasmdiscretetf_frequency_response_mag_phase(this.__wbg_ptr, n_freqs);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} n_points
     * @param {boolean} hz_axis
     * @param {boolean} legend
     * @returns {string}
     */
    bode_svg(width, height, n_points, hz_axis, legend) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmdiscretetf_bode_svg(this.__wbg_ptr, width, height, n_points, hz_axis, legend);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {number} n_points
     * @param {boolean} show_minus_one
     * @param {boolean} legend
     * @returns {string}
     */
    nyquist_svg(width, height, n_points, show_minus_one, legend) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmdiscretetf_nyquist_svg(this.__wbg_ptr, width, height, n_points, show_minus_one, legend);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {DiscreteTF} other
     * @returns {DiscreteTF}
     */
    series(other) {
        _assertClass(other, DiscreteTF);
        const ret = wasm.wasmdiscretetf_series(this.__wbg_ptr, other.__wbg_ptr);
        return DiscreteTF.__wrap(ret);
    }
    /**
     * @param {DiscreteTF} other
     * @returns {DiscreteTF}
     */
    parallel(other) {
        _assertClass(other, DiscreteTF);
        const ret = wasm.wasmdiscretetf_parallel(this.__wbg_ptr, other.__wbg_ptr);
        return DiscreteTF.__wrap(ret);
    }
    /**
     * @returns {DiscreteTF}
     */
    feedback_unity() {
        const ret = wasm.wasmdiscretetf_feedback_unity(this.__wbg_ptr);
        return DiscreteTF.__wrap(ret);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {boolean} negative_feedback
     * @param {string | null} [feedback_label]
     * @returns {string}
     */
    block_feedback_svg(width, height, negative_feedback, feedback_label) {
        let deferred2_0;
        let deferred2_1;
        try {
            var ptr0 = isLikeNone(feedback_label) ? 0 : passStringToWasm0(feedback_label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            const ret = wasm.wasmdiscretetf_block_feedback_svg(this.__wbg_ptr, width, height, negative_feedback, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
}

const WasmDiscreteZpkFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmdiscretezpk_free(ptr >>> 0, 1));

export class WasmDiscreteZpk {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmDiscreteZpkFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmdiscretezpk_free(ptr, 0);
    }
    /**
     * @param {Float64Array} zeros_interleaved
     * @param {Float64Array} poles_interleaved
     * @param {number} gain
     * @param {number} sample_rate
     */
    constructor(zeros_interleaved, poles_interleaved, gain, sample_rate) {
        const ptr0 = passArrayF64ToWasm0(zeros_interleaved, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(poles_interleaved, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmdiscretezpk_new(ptr0, len0, ptr1, len1, gain, sample_rate);
        return DiscreteZpk.__wrap(ret);
    }
    /**
     * @param {DiscreteTF} tf
     * @returns {DiscreteZpk}
     */
    static from_tf(tf) {
        _assertClass(tf, DiscreteTF);
        const ret = wasm.wasmdiscretezpk_from_tf(tf.__wbg_ptr);
        return DiscreteZpk.__wrap(ret);
    }
    /**
     * @returns {DiscreteTF}
     */
    to_tf() {
        const ret = wasm.wasmdiscretezpk_to_tf(this.__wbg_ptr);
        return DiscreteTF.__wrap(ret);
    }
    /**
     * @returns {Float64Array}
     */
    zeros_interleaved() {
        const ret = wasm.wasmdiscretezpk_zeros_interleaved(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {Float64Array}
     */
    poles_interleaved() {
        const ret = wasm.wasmdiscretezpk_poles_interleaved(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {number}
     */
    gain() {
        const ret = wasm.wasmcontinuouszpk_gain(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    sample_rate() {
        const ret = wasm.wasmdiscretezpk_sample_rate(this.__wbg_ptr);
        return ret;
    }
}

const WasmGF2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgf2_free(ptr >>> 0, 1));

export class WasmGF2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmGF2.prototype);
        obj.__wbg_ptr = ptr;
        WasmGF2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGF2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgf2_free(ptr, 0);
    }
    /**
     * @param {bigint} value
     */
    constructor(value) {
        const ret = wasm.gf2_new(value);
        this.__wbg_ptr = ret >>> 0;
        WasmGF2Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    static modulus() {
        const ret = wasm.gf2_modulus();
        return ret;
    }
    /**
     * @returns {WasmGF2}
     */
    inv() {
        const ret = wasm.wasmgf2_inv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGF2.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGF2}
     */
    static zero() {
        const ret = wasm.gf2_zero();
        return WasmGF2.__wrap(ret);
    }
    /**
     * @returns {WasmGF2}
     */
    static one() {
        const ret = wasm.gf2_one();
        return WasmGF2.__wrap(ret);
    }
    /**
     * @param {WasmGF2} rhs
     * @returns {WasmGF2}
     */
    add(rhs) {
        _assertClass(rhs, WasmGF2);
        const ret = wasm.wasmgf2_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF2.__wrap(ret);
    }
    /**
     * @param {WasmGF2} rhs
     * @returns {WasmGF2}
     */
    sub(rhs) {
        _assertClass(rhs, WasmGF2);
        const ret = wasm.wasmgf2_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF2.__wrap(ret);
    }
    /**
     * @param {WasmGF2} rhs
     * @returns {WasmGF2}
     */
    mul(rhs) {
        _assertClass(rhs, WasmGF2);
        const ret = wasm.wasmgf2_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF2.__wrap(ret);
    }
    /**
     * @returns {bigint}
     */
    get value() {
        const ret = wasm.gf2_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {WasmGF2} other
     * @returns {WasmGF2}
     */
    div(other) {
        _assertClass(other, WasmGF2);
        const ret = wasm.wasmgf2_div(this.__wbg_ptr, other.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGF2.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGF2}
     */
    neg() {
        const ret = wasm.wasmgf2_neg(this.__wbg_ptr);
        return WasmGF2.__wrap(ret);
    }
    /**
     * @returns {boolean}
     */
    get isZero() {
        const ret = wasm.gf2_is_zero(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get isOne() {
        const ret = wasm.gf2_is_one(this.__wbg_ptr);
        return ret !== 0;
    }
}

const WasmGF256Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgf256_free(ptr >>> 0, 1));

export class WasmGF256 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmGF256.prototype);
        obj.__wbg_ptr = ptr;
        WasmGF256Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGF256Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgf256_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    get isZero() {
        const ret = wasm.wasmgf256_is_zero(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get isOne() {
        const ret = wasm.wasmgf256_is_one(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {number}
     */
    toU8() {
        const ret = wasm.wasmgf256_toU8(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {WasmGF256}
     */
    inv() {
        const ret = wasm.wasmgf256_inv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGF256.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGF256}
     */
    static zero() {
        const ret = wasm.gfextgf2_zero();
        return WasmGF256.__wrap(ret);
    }
    /**
     * @returns {WasmGF256}
     */
    static one() {
        const ret = wasm.gfextgf2_one();
        return WasmGF256.__wrap(ret);
    }
    /**
     * @param {WasmGF256} rhs
     * @returns {WasmGF256}
     */
    add(rhs) {
        _assertClass(rhs, WasmGF256);
        const ret = wasm.wasmgf256_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF256.__wrap(ret);
    }
    /**
     * @param {WasmGF256} rhs
     * @returns {WasmGF256}
     */
    sub(rhs) {
        _assertClass(rhs, WasmGF256);
        const ret = wasm.wasmgf256_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF256.__wrap(ret);
    }
    /**
     * @param {WasmGF256} rhs
     * @returns {WasmGF256}
     */
    mul(rhs) {
        _assertClass(rhs, WasmGF256);
        const ret = wasm.wasmgf256_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF256.__wrap(ret);
    }
    /**
     * @param {number} value
     */
    constructor(value) {
        const ret = wasm.wasmgf256_new(value);
        this.__wbg_ptr = ret >>> 0;
        WasmGF256Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    get value() {
        const ret = wasm.wasmgf256_toU8(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {Uint8Array}
     */
    get coeffs() {
        const ret = wasm.wasmgf256_coeffs(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {Uint8Array} coeffs
     * @returns {WasmGF256}
     */
    static fromCoeffs(coeffs) {
        const ptr0 = passArray8ToWasm0(coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmgf256_fromCoeffs(ptr0, len0);
        return WasmGF256.__wrap(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    static modulus() {
        const ret = wasm.wasmgf256_modulus();
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {WasmGF256} other
     * @returns {WasmGF256}
     */
    div(other) {
        _assertClass(other, WasmGF256);
        const ret = wasm.wasmgf256_div(this.__wbg_ptr, other.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGF256.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGF256}
     */
    neg() {
        const ret = wasm.wasmgf256_neg(this.__wbg_ptr);
        return WasmGF256.__wrap(ret);
    }
}

const WasmGF3Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgf3_free(ptr >>> 0, 1));

export class WasmGF3 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmGF3.prototype);
        obj.__wbg_ptr = ptr;
        WasmGF3Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGF3Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgf3_free(ptr, 0);
    }
    /**
     * @param {bigint} value
     */
    constructor(value) {
        const ret = wasm.gf3_new(value);
        this.__wbg_ptr = ret >>> 0;
        WasmGF3Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {number}
     */
    static modulus() {
        const ret = wasm.gf3_modulus();
        return ret;
    }
    /**
     * @returns {WasmGF3}
     */
    inv() {
        const ret = wasm.wasmgf3_inv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGF3.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGF3}
     */
    static zero() {
        const ret = wasm.gf2_zero();
        return WasmGF3.__wrap(ret);
    }
    /**
     * @returns {WasmGF3}
     */
    static one() {
        const ret = wasm.gf2_one();
        return WasmGF3.__wrap(ret);
    }
    /**
     * @param {WasmGF3} rhs
     * @returns {WasmGF3}
     */
    add(rhs) {
        _assertClass(rhs, WasmGF3);
        const ret = wasm.wasmgf3_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF3.__wrap(ret);
    }
    /**
     * @param {WasmGF3} rhs
     * @returns {WasmGF3}
     */
    sub(rhs) {
        _assertClass(rhs, WasmGF3);
        const ret = wasm.wasmgf3_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF3.__wrap(ret);
    }
    /**
     * @param {WasmGF3} rhs
     * @returns {WasmGF3}
     */
    mul(rhs) {
        _assertClass(rhs, WasmGF3);
        const ret = wasm.wasmgf3_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGF3.__wrap(ret);
    }
    /**
     * @returns {bigint}
     */
    get value() {
        const ret = wasm.gf2_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {WasmGF3} other
     * @returns {WasmGF3}
     */
    div(other) {
        _assertClass(other, WasmGF3);
        const ret = wasm.wasmgf3_div(this.__wbg_ptr, other.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGF3.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGF3}
     */
    neg() {
        const ret = wasm.wasmgf3_neg(this.__wbg_ptr);
        return WasmGF3.__wrap(ret);
    }
    /**
     * @returns {boolean}
     */
    get isZero() {
        const ret = wasm.gf2_is_zero(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get isOne() {
        const ret = wasm.gf2_is_one(this.__wbg_ptr);
        return ret !== 0;
    }
}

const WasmGFExtGF2Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmgfextgf2_free(ptr >>> 0, 1));

export class WasmGFExtGF2 {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmGFExtGF2.prototype);
        obj.__wbg_ptr = ptr;
        WasmGFExtGF2Finalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmGFExtGF2Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmgfextgf2_free(ptr, 0);
    }
    /**
     * @returns {boolean}
     */
    get isZero() {
        const ret = wasm.wasmgf256_is_zero(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {boolean}
     */
    get isOne() {
        const ret = wasm.wasmgf256_is_one(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @returns {WasmGFExtGF2}
     */
    inv() {
        const ret = wasm.wasmgfextgf2_inv(this.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGFExtGF2.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGFExtGF2}
     */
    static zero() {
        const ret = wasm.gfextgf2_zero();
        return WasmGFExtGF2.__wrap(ret);
    }
    /**
     * @returns {WasmGFExtGF2}
     */
    static one() {
        const ret = wasm.gfextgf2_one();
        return WasmGFExtGF2.__wrap(ret);
    }
    /**
     * @param {WasmGFExtGF2} rhs
     * @returns {WasmGFExtGF2}
     */
    add(rhs) {
        _assertClass(rhs, WasmGFExtGF2);
        const ret = wasm.wasmgfextgf2_add(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGFExtGF2.__wrap(ret);
    }
    /**
     * @param {WasmGFExtGF2} rhs
     * @returns {WasmGFExtGF2}
     */
    sub(rhs) {
        _assertClass(rhs, WasmGFExtGF2);
        const ret = wasm.wasmgfextgf2_sub(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGFExtGF2.__wrap(ret);
    }
    /**
     * @param {WasmGFExtGF2} rhs
     * @returns {WasmGFExtGF2}
     */
    mul(rhs) {
        _assertClass(rhs, WasmGFExtGF2);
        const ret = wasm.wasmgfextgf2_mul(this.__wbg_ptr, rhs.__wbg_ptr);
        return WasmGFExtGF2.__wrap(ret);
    }
    /**
     * @param {Uint8Array} px_coeffs
     * @param {Uint8Array} coeffs
     */
    constructor(px_coeffs, coeffs) {
        const ptr0 = passArray8ToWasm0(px_coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArray8ToWasm0(coeffs, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmgfextgf2_new(ptr0, len0, ptr1, len1);
        this.__wbg_ptr = ret >>> 0;
        WasmGFExtGF2Finalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {Uint8Array} px_coeffs
     * @param {number} base_value
     * @returns {WasmGFExtGF2}
     */
    static fromBase(px_coeffs, base_value) {
        const ptr0 = passArray8ToWasm0(px_coeffs, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmgfextgf2_fromBase(ptr0, len0, base_value);
        return WasmGFExtGF2.__wrap(ret);
    }
    /**
     * @returns {Uint8Array}
     */
    get coeffs() {
        const ret = wasm.wasmgfextgf2_coeffs(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @returns {Uint8Array}
     */
    get px() {
        const ret = wasm.wasmgfextgf2_px(this.__wbg_ptr);
        var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v1;
    }
    /**
     * @param {WasmGFExtGF2} other
     * @returns {WasmGFExtGF2}
     */
    div(other) {
        _assertClass(other, WasmGFExtGF2);
        const ret = wasm.wasmgfextgf2_div(this.__wbg_ptr, other.__wbg_ptr);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        return WasmGFExtGF2.__wrap(ret[0]);
    }
    /**
     * @returns {WasmGFExtGF2}
     */
    neg() {
        const ret = wasm.wasmgfextgf2_neg(this.__wbg_ptr);
        return WasmGFExtGF2.__wrap(ret);
    }
}

const WasmHamming74Finalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmhamming74_free(ptr >>> 0, 1));

export class WasmHamming74 {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmHamming74Finalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmhamming74_free(ptr, 0);
    }
    constructor() {
        const ret = wasm.wasmhamming74_new();
        return Hamming74.__wrap(ret);
    }
    /**
     * @param {Uint8Array} u
     * @returns {Uint8Array}
     */
    encode(u) {
        const ptr0 = passArray8ToWasm0(u, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmhamming74_encode(this.__wbg_ptr, ptr0, len0);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v2 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
        return v2;
    }
}

const WasmLMSFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlms_free(ptr >>> 0, 1));

export class WasmLMS {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmLMSFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmlms_free(ptr, 0);
    }
    /**
     * @param {number} taps
     * @param {number} step_size
     */
    constructor(taps, step_size) {
        const ret = wasm.wasmlms_new(taps, step_size);
        this.__wbg_ptr = ret >>> 0;
        WasmLMSFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} input
     * @param {number} desired
     * @returns {Float64Array}
     */
    process_sample(input, desired) {
        const ret = wasm.wasmlms_process_sample(this.__wbg_ptr, input, desired);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {Float64Array} input
     * @param {Float64Array} desired
     * @returns {Float64Array}
     */
    process_series(input, desired) {
        const ptr0 = passArrayF64ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(desired, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmlms_process_series(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v3;
    }
}

const WasmLinearModelFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmlinearmodel_free(ptr >>> 0, 1));

export class WasmLinearModel {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmLinearModelFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmlinearmodel_free(ptr, 0);
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @param {Float64Array} a_data
     * @param {Float64Array} b
     * @returns {Float64Array}
     */
    static solveLinearSystem(rows, cols, a_data, b) {
        const ptr0 = passArrayF64ToWasm0(a_data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmlinearmodel_solveLinearSystem(rows, cols, ptr0, len0, ptr1, len1);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v3;
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @param {Float64Array} a_data
     * @param {Float64Array} b
     * @param {number} alpha
     * @returns {Float64Array}
     */
    static ridgeRegression(rows, cols, a_data, b, alpha) {
        const ptr0 = passArrayF64ToWasm0(a_data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmlinearmodel_ridgeRegression(rows, cols, ptr0, len0, ptr1, len1, alpha);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v3;
    }
    /**
     * @param {number} rows
     * @param {number} cols
     * @param {Float64Array} a_data
     * @param {Float64Array} b
     * @param {number} alpha
     * @param {number} max_iter
     * @param {number} tol
     * @returns {Float64Array}
     */
    static lassoRegression(rows, cols, a_data, b, alpha, max_iter, tol) {
        const ptr0 = passArrayF64ToWasm0(a_data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(b, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmlinearmodel_lassoRegression(rows, cols, ptr0, len0, ptr1, len1, alpha, max_iter, tol);
        if (ret[3]) {
            throw takeFromExternrefTable0(ret[2]);
        }
        var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v3;
    }
}

const WasmNLMSFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmnlms_free(ptr >>> 0, 1));

export class WasmNLMS {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmNLMSFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmnlms_free(ptr, 0);
    }
    /**
     * @param {number} taps
     * @param {number} step_size
     * @param {number} epsilon
     */
    constructor(taps, step_size, epsilon) {
        const ret = wasm.wasmnlms_new(taps, step_size, epsilon);
        this.__wbg_ptr = ret >>> 0;
        WasmNLMSFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {number} input
     * @param {number} desired
     * @returns {Float64Array}
     */
    process_sample(input, desired) {
        const ret = wasm.wasmnlms_process_sample(this.__wbg_ptr, input, desired);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @param {Float64Array} input
     * @param {Float64Array} desired
     * @returns {Float64Array}
     */
    process_series(input, desired) {
        const ptr0 = passArrayF64ToWasm0(input, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ptr1 = passArrayF64ToWasm0(desired, wasm.__wbindgen_malloc);
        const len1 = WASM_VECTOR_LEN;
        const ret = wasm.wasmnlms_process_series(this.__wbg_ptr, ptr0, len0, ptr1, len1);
        var v3 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v3;
    }
}

const WasmSignalFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmsignal_free(ptr >>> 0, 1));

export class WasmSignal {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmSignal.prototype);
        obj.__wbg_ptr = ptr;
        WasmSignalFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmSignalFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmsignal_free(ptr, 0);
    }
    /**
     * @param {Float64Array} data
     * @param {number} sample_rate
     */
    constructor(data, sample_rate) {
        const ptr0 = passArrayF64ToWasm0(data, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmsignal_new(ptr0, len0, sample_rate);
        this.__wbg_ptr = ret >>> 0;
        WasmSignalFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Float64Array}
     */
    data() {
        const ret = wasm.wasmsignal_data(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {number}
     */
    sample_rate() {
        const ret = wasm.wasmsignal_sample_rate(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    len() {
        const ret = wasm.wasmsignal_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {WasmSpectrum}
     */
    dft() {
        const ret = wasm.wasmsignal_dft(this.__wbg_ptr);
        return WasmSpectrum.__wrap(ret);
    }
    /**
     * @param {WasmSignal} h
     * @returns {WasmSignal}
     */
    convolve(h) {
        _assertClass(h, WasmSignal);
        const ret = wasm.wasmsignal_convolve(this.__wbg_ptr, h.__wbg_ptr);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {Float64Array} taps
     * @returns {WasmSignal}
     */
    apply_fir(taps) {
        const ptr0 = passArrayF64ToWasm0(taps, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmsignal_apply_fir(this.__wbg_ptr, ptr0, len0);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {number} factor
     * @param {number} filter_taps
     * @returns {WasmSignal}
     */
    downsample(factor, filter_taps) {
        const ret = wasm.wasmsignal_downsample(this.__wbg_ptr, factor, filter_taps);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {number} factor
     * @param {number} filter_taps
     * @returns {WasmSignal}
     */
    upsample(factor, filter_taps) {
        const ret = wasm.wasmsignal_upsample(this.__wbg_ptr, factor, filter_taps);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {number} upsample_factor
     * @param {number} downsample_factor
     * @param {number} filter_taps
     * @returns {WasmSignal}
     */
    resample(upsample_factor, downsample_factor, filter_taps) {
        const ret = wasm.wasmsignal_resample(this.__wbg_ptr, upsample_factor, downsample_factor, filter_taps);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {number} factor
     * @returns {WasmSignal}
     */
    decimate(factor) {
        const ret = wasm.wasmsignal_decimate(this.__wbg_ptr, factor);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {number} factor
     * @returns {WasmSignal}
     */
    expand(factor) {
        const ret = wasm.wasmsignal_expand(this.__wbg_ptr, factor);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @returns {string}
     */
    save_svg_simple(width, height) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.wasmsignal_save_svg_simple(this.__wbg_ptr, width, height);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {string | null} [label]
     * @returns {string}
     */
    save_svg_with_axes(width, height, label) {
        let deferred2_0;
        let deferred2_1;
        try {
            var ptr0 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            const ret = wasm.wasmsignal_save_svg_with_axes(this.__wbg_ptr, width, height, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
}

const WasmSpectrumFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_wasmspectrum_free(ptr >>> 0, 1));

export class WasmSpectrum {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(WasmSpectrum.prototype);
        obj.__wbg_ptr = ptr;
        WasmSpectrumFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        WasmSpectrumFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_wasmspectrum_free(ptr, 0);
    }
    /**
     * @param {Float64Array} data_interleaved
     * @param {number} sample_rate
     */
    constructor(data_interleaved, sample_rate) {
        const ptr0 = passArrayF64ToWasm0(data_interleaved, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.wasmsignal_new(ptr0, len0, sample_rate);
        this.__wbg_ptr = ret >>> 0;
        WasmSpectrumFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @returns {Float64Array}
     */
    data_interleaved() {
        const ret = wasm.wasmspectrum_data_interleaved(this.__wbg_ptr);
        var v1 = getArrayF64FromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 8, 8);
        return v1;
    }
    /**
     * @returns {number}
     */
    sample_rate() {
        const ret = wasm.wasmsignal_sample_rate(this.__wbg_ptr);
        return ret;
    }
    /**
     * @returns {number}
     */
    len() {
        const ret = wasm.wasmspectrum_len(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {WasmSignal}
     */
    ift() {
        const ret = wasm.wasmspectrum_ift(this.__wbg_ptr);
        return WasmSignal.__wrap(ret);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @param {string | null} [label]
     * @returns {string}
     */
    magnitude_db_svg(width, height, label) {
        let deferred2_0;
        let deferred2_1;
        try {
            var ptr0 = isLikeNone(label) ? 0 : passStringToWasm0(label, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            const ret = wasm.wasmspectrum_magnitude_db_svg(this.__wbg_ptr, width, height, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_buffer_609cc3eee51ed158 = function(arg0) {
        const ret = arg0.buffer;
        return ret;
    };
    imports.wbg.__wbg_call_672a4d21634d4a24 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_7cccdd69e0791ae2 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_crypto_574e78ad8b13b65f = function(arg0) {
        const ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_error_7534b8e9a36f1ab4 = function(arg0, arg1) {
        let deferred0_0;
        let deferred0_1;
        try {
            deferred0_0 = arg0;
            deferred0_1 = arg1;
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
        }
    };
    imports.wbg.__wbg_getRandomValues_b8f5dbd5f3995a9e = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_length_a446193dc22c12f8 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_msCrypto_a61aeb35a24c1329 = function(arg0) {
        const ret = arg0.msCrypto;
        return ret;
    };
    imports.wbg.__wbg_new_405e22f390576ce2 = function() {
        const ret = new Object();
        return ret;
    };
    imports.wbg.__wbg_new_78feb108b6472713 = function() {
        const ret = new Array();
        return ret;
    };
    imports.wbg.__wbg_new_8a6f238a6ece86ea = function() {
        const ret = new Error();
        return ret;
    };
    imports.wbg.__wbg_new_a12002a7f91c75be = function(arg0) {
        const ret = new Uint8Array(arg0);
        return ret;
    };
    imports.wbg.__wbg_newnoargs_105ed471475aaf50 = function(arg0, arg1) {
        const ret = new Function(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_d97e637ebe145a9a = function(arg0, arg1, arg2) {
        const ret = new Uint8Array(arg0, arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_newwithlength_a381634e90c276d4 = function(arg0) {
        const ret = new Uint8Array(arg0 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_node_905d3e251edff8a2 = function(arg0) {
        const ret = arg0.node;
        return ret;
    };
    imports.wbg.__wbg_polynomialf64_new = function(arg0) {
        const ret = PolynomialF64.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_polynomialgf256_new = function(arg0) {
        const ret = PolynomialGF256.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_polynomialgf2_new = function(arg0) {
        const ret = PolynomialGF2.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_polynomialgfextgf2_new = function(arg0) {
        const ret = PolynomialGFExtGF2.__wrap(arg0);
        return ret;
    };
    imports.wbg.__wbg_process_dc0fbacc7c1c06f7 = function(arg0) {
        const ret = arg0.process;
        return ret;
    };
    imports.wbg.__wbg_randomFillSync_ac0988aba3254290 = function() { return handleError(function (arg0, arg1) {
        arg0.randomFillSync(arg1);
    }, arguments) };
    imports.wbg.__wbg_require_60cc747a6bc5215a = function() { return handleError(function () {
        const ret = module.require;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_set_37837023f3d740e8 = function(arg0, arg1, arg2) {
        arg0[arg1 >>> 0] = arg2;
    };
    imports.wbg.__wbg_set_3f1d0b984ed272ed = function(arg0, arg1, arg2) {
        arg0[arg1] = arg2;
    };
    imports.wbg.__wbg_set_65595bdd868b3009 = function(arg0, arg1, arg2) {
        arg0.set(arg1, arg2 >>> 0);
    };
    imports.wbg.__wbg_stack_0ed75d68575b0f3c = function(arg0, arg1) {
        const ret = arg1.stack;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_88a902d13a557d07 = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_56578be7e9f832b0 = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_37c5d418e4bf5819 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_5de37043a91a9c40 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_subarray_aa9065fa9dc5df96 = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_versions_c01dfd4722a88165 = function(arg0) {
        const ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
        const ret = BigInt.asUintN(64, arg0);
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        const ret = debugString(arg1);
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
        const ret = new Error(getStringFromWasm0(arg0, arg1));
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_export_2;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
        ;
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        const ret = typeof(arg0) === 'function';
        return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
        const ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_memory = function() {
        const ret = wasm.memory;
        return ret;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    return imports;
}

function __wbg_init_memory(imports, memory) {

}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedBigUint64ArrayMemory0 = null;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedFloat64ArrayMemory0 = null;
    cachedUint32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();

    __wbg_init_memory(imports);

    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }

    const instance = new WebAssembly.Instance(module, imports);

    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('wasm_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    __wbg_init_memory(imports);

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
