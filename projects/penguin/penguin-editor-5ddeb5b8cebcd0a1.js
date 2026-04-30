import { getElementById, getValue, setAttr, setChecked, setClass, setFill, setHtml, setId, setPatternUnits, setPlaceholder, setPoints, setStrokeDasharray, setStroke, setStyle, setText, setType, setValue } from './snippets/penguin-editor-51132da1755e28de/src/dom/js.js';
import * as import1 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import2 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import3 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import4 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import5 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import6 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import7 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import8 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import9 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import10 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import11 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import12 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import13 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import14 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import15 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import16 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import17 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import18 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import19 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import20 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import21 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import22 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import23 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import24 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import25 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import26 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import27 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import28 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import29 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import30 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import31 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import32 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import33 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import34 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import35 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import36 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import37 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import38 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"
import * as import39 from "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js"


export function init() {
    wasm.init();
}

export function penguin_start() {
    wasm.penguin_start();
}

export function penguin_stop() {
    wasm.penguin_stop();
}
function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_debug_string_ab4b34d23d6778bd: function(arg0, arg1) {
            const ret = debugString(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_number_get_c7f42aed0525c451: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'number' ? obj : undefined;
            getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
        },
        __wbg___wbindgen_throw_6b64449b9b9ed33c: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg__wbg_cb_unref_b46c9b5a9f08ec37: function(arg0) {
            arg0._wbg_cb_unref();
        },
        __wbg_addEventListener_8176dab41b09531c: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            arg0.addEventListener(getStringFromWasm0(arg1, arg2), arg3);
        }, arguments); },
        __wbg_altKey_3116112ec764f316: function(arg0) {
            const ret = arg0.altKey;
            return ret;
        },
        __wbg_button_c794bf4b1dcd7c4c: function(arg0) {
            const ret = arg0.button;
            return ret;
        },
        __wbg_clientX_48ead8c93aa7a872: function(arg0) {
            const ret = arg0.clientX;
            return ret;
        },
        __wbg_clientY_ddcce7762c925e13: function(arg0) {
            const ret = arg0.clientY;
            return ret;
        },
        __wbg_clipboardData_f03e3b5606f47f6d: function(arg0) {
            const ret = arg0.clipboardData;
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_ctrlKey_31968cccd46bdef6: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_ctrlKey_a49693667722b909: function(arg0) {
            const ret = arg0.ctrlKey;
            return ret;
        },
        __wbg_debug_c014a160490283dc: function(arg0) {
            console.debug(arg0);
        },
        __wbg_deltaY_ca7744a8772482e1: function(arg0) {
            const ret = arg0.deltaY;
            return ret;
        },
        __wbg_disconnect_d173374266b80cfa: function(arg0) {
            arg0.disconnect();
        },
        __wbg_error_2001591ad2463697: function(arg0) {
            console.error(arg0);
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_from_0dbf29f09e7fb200: function(arg0) {
            const ret = Array.from(arg0);
            return ret;
        },
        __wbg_getData_a20c218e8ae28672: function() { return handleError(function (arg0, arg1, arg2, arg3) {
            const ret = arg1.getData(getStringFromWasm0(arg2, arg3));
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        }, arguments); },
        __wbg_getElementById_e8b3270afd453d88: function(arg0, arg1) {
            const ret = getElementById(getStringFromWasm0(arg0, arg1));
            return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
        },
        __wbg_getValue_03ff5489c896daad: function(arg0, arg1) {
            const ret = getValue(arg1);
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_get_8360291721e2339f: function(arg0, arg1) {
            const ret = arg0[arg1 >>> 0];
            return ret;
        },
        __wbg_info_7479429238bffbce: function(arg0) {
            console.info(arg0);
        },
        __wbg_key_2cbc38fa83cdb336: function(arg0, arg1) {
            const ret = arg1.key;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_log_7e1aa9064a1dbdbd: function(arg0) {
            console.log(arg0);
        },
        __wbg_metaKey_665498d01ebfd062: function(arg0) {
            const ret = arg0.metaKey;
            return ret;
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_new_ad8d9a2aa2624a65: function() { return handleError(function (arg0) {
            const ret = new ResizeObserver(arg0);
            return ret;
        }, arguments); },
        __wbg_observe_c79fbdfb1452af30: function(arg0, arg1) {
            arg0.observe(arg1);
        },
        __wbg_preventDefault_f55c01cb5fd2bcc0: function(arg0) {
            arg0.preventDefault();
        },
        __wbg_remove_48cb93cf7a6c4260: function(arg0) {
            arg0.remove();
        },
        __wbg_setAttr_e94d3219a0527f49: function(arg0, arg1, arg2, arg3, arg4) {
            setAttr(arg0, getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        },
        __wbg_setChecked_cc57443092866cc3: function(arg0, arg1) {
            setChecked(arg0, arg1 !== 0);
        },
        __wbg_setClass_fe35f81b0143395a: function(arg0, arg1, arg2) {
            setClass(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setData_4f4b39d90335ed4f: function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            arg0.setData(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        }, arguments); },
        __wbg_setFill_cb0fada8db88ef6a: function(arg0, arg1, arg2) {
            setFill(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setHtml_64836277541182e1: function(arg0, arg1, arg2) {
            setHtml(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setId_db5250c2b56e0915: function(arg0, arg1, arg2) {
            setId(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setPatternUnits_5887aacac7589cb6: function(arg0, arg1, arg2) {
            setPatternUnits(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setPlaceholder_0cfe08159295377b: function(arg0, arg1, arg2) {
            setPlaceholder(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setPoints_16307e64bc22c59a: function(arg0, arg1, arg2) {
            setPoints(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setStrokeDasharray_30effa116122408e: function(arg0, arg1, arg2) {
            setStrokeDasharray(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setStroke_90284dfec60f91e0: function(arg0, arg1, arg2) {
            setStroke(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setStyle_8cec29719d9048f7: function(arg0, arg1, arg2, arg3, arg4) {
            setStyle(arg0, getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        },
        __wbg_setText_3fa3a2d10b548fab: function(arg0, arg1, arg2) {
            setText(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setType_daa351320dedcd9e: function(arg0, arg1, arg2) {
            setType(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_setValue_8521b89b12524a20: function(arg0, arg1, arg2) {
            setValue(arg0, getStringFromWasm0(arg1, arg2));
        },
        __wbg_shiftKey_dcf8ee699c273ed2: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_shiftKey_e483c13c966878f6: function(arg0) {
            const ret = arg0.shiftKey;
            return ret;
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_stopPropagation_e088fca8231e68c4: function(arg0) {
            arg0.stopPropagation();
        },
        __wbg_warn_3cc416af27dbdc02: function(arg0) {
            console.warn(arg0);
        },
        __wbindgen_cast_0000000000000001: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("ClipboardEvent")], shim_idx: 36, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687);
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("Event")], shim_idx: 36, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_1);
            return ret;
        },
        __wbindgen_cast_0000000000000003: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("KeyboardEvent")], shim_idx: 36, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_2);
            return ret;
        },
        __wbindgen_cast_0000000000000004: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("MouseEvent")], shim_idx: 36, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_3);
            return ret;
        },
        __wbindgen_cast_0000000000000005: function(arg0, arg1) {
            // Cast intrinsic for `Closure(Closure { owned: true, function: Function { arguments: [NamedExternref("WheelEvent")], shim_idx: 36, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
            const ret = makeMutClosure(arg0, arg1, wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_4);
            return ret;
        },
        __wbindgen_cast_0000000000000006: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./penguin-editor_bg.js": import0,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import1,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import2,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import3,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import4,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import5,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import6,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import7,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import8,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import9,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import10,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import11,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import12,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import13,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import14,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import15,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import16,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import17,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import18,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import19,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import20,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import21,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import22,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import23,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import24,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import25,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import26,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import27,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import28,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import29,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import30,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import31,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import32,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import33,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import34,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import35,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import36,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import37,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import38,
        "./snippets/penguin-editor-51132da1755e28de/src/dom/js.js": import39,
    };
}

function wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_1(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_1(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_2(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_2(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_3(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_3(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_4(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h55977fb29eae1687_4(arg0, arg1, arg2);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => wasm.__wbindgen_destroy_closure(state.a, state.b));

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

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, f) {
    const state = { a: arg0, b: arg1, cnt: 1 };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            wasm.__wbindgen_destroy_closure(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

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
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
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

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('penguin-editor_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
