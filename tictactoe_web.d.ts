/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function getState(): string;
/**
* @param {number} i
*/
export function makeTurn(i: number): void;
/**
* @returns {boolean}
*/
export function isGameOver(): boolean;
/**
* @returns {string}
*/
export function getLastPlayed(): string;
/**
*/
export function restart(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly getState: (a: number) => void;
  readonly makeTurn: (a: number) => void;
  readonly isGameOver: () => number;
  readonly getLastPlayed: (a: number) => void;
  readonly restart: () => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
