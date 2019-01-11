/* tslint:disable */
import * as wasm from './genby_bg';

/**
* @param {number} arg0
* @param {number} arg1
* @returns {void}
*/
export function create(arg0, arg1) {
    return wasm.create(arg0, arg1);
}

/**
* @param {number} arg0
* @returns {void}
*/
export function tick(arg0) {
    return wasm.tick(arg0);
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function getArrayU32FromWasm(ptr, len) {
    return getUint32Memory().subarray(ptr / 4, ptr / 4 + len);
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}
/**
* @returns {Uint32Array}
*/
export function size() {
    const retptr = globalArgumentPtr();
    wasm.size(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getArrayU32FromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 4);
    return realRet;

}

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}
/**
* @param {boolean} arg0
* @param {boolean} arg1
* @param {boolean} arg2
* @param {boolean} arg3
* @returns {Uint8Array}
*/
export function get_pixels(arg0, arg1, arg2, arg3) {
    const retptr = globalArgumentPtr();
    wasm.get_pixels(retptr, arg0, arg1, arg2, arg3);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getArrayU8FromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;

}

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null || cachegetFloat32Memory.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory;
}

function getArrayF32FromWasm(ptr, len) {
    return getFloat32Memory().subarray(ptr / 4, ptr / 4 + len);
}
/**
* @param {boolean} arg0
* @returns {Float32Array}
*/
export function get_heights(arg0) {
    const retptr = globalArgumentPtr();
    wasm.get_heights(retptr, arg0);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getArrayF32FromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 4);
    return realRet;

}

/**
* @returns {Float32Array}
*/
export function get_wind_directions() {
    const retptr = globalArgumentPtr();
    wasm.get_wind_directions(retptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getArrayF32FromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 4);
    return realRet;

}

/**
* @param {number} arg0
* @param {number} arg1
* @param {number} arg2
* @param {number} arg3
* @param {number} arg4
* @param {number} arg5
* @returns {void}
*/
export function alter_world(arg0, arg1, arg2, arg3, arg4, arg5) {
    return wasm.alter_world(arg0, arg1, arg2, arg3, arg4, arg5);
}

export function __wbg_random_2cc0c8d054a5c72a() {
    return Math.random();
}

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

export function __wbindgen_throw(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
}

