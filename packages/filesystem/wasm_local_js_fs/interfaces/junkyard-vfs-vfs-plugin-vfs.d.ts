declare module 'junkyard-vfs:vfs-plugin/vfs' {
  export { File };
  export { Filesystem };
  export function init(): Filesystem;
}
import type { AbsolutePath } from './junkyard-vfs-vfs-plugin-vfs-host.js';
export { AbsolutePath };
export type Seek = SeekStart | SeekCurrent | SeekEnd;
export interface SeekStart {
  tag: 'start',
  val: bigint,
}
export interface SeekCurrent {
  tag: 'current',
  val: bigint,
}
export interface SeekEnd {
  tag: 'end',
  val: bigint,
}
/**
 * # Variants
 * 
 * ## `"file"`
 * 
 * ## `"folder"`
 * 
 * ## `"link"`
 * 
 * ## `"unknown"`
 */
export type Kind = 'file' | 'folder' | 'link' | 'unknown';
export interface Stat {
  kind: Kind,
  size: bigint,
}

export class File {
  read(data: Uint8Array): bigint;
  write(data: Uint8Array): bigint;
  seek(s: Seek): bigint;
}

export class Filesystem {
  readDir(path: AbsolutePath): Array<AbsolutePath>;
  stat(path: AbsolutePath): Stat;
  open(path: AbsolutePath): File;
}
