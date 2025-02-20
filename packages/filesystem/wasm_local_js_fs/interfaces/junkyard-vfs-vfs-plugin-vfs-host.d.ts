declare module 'junkyard-vfs:vfs-plugin/vfs-host' {
  export { AbsolutePath };
  export function createAbsolutePath(s: string): AbsolutePath;
}

export class AbsolutePath {
  components(): Array<string>;
  isRoot(): boolean;
  parent(): AbsolutePath;
  fileName(): string;
  path(): string;
}
