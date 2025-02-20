import { createAbsolutePath } from "junkyard-vfs:vfs-plugin/vfs-host";
class Vfs {
    fs = {
        "dir1": {},
        "dir2": {},
        "file1": ""
    };
    readDir(path) {
        let files = [];
        for (const file in this.fs) {
            files.push(createAbsolutePath("/" + file));
        }
        return files;
    }
    stat(path) {
        let parts = path.components();
        if (this.fs[parts[0]] instanceof Object) {
            return {
                kind: "folder",
                size: BigInt(0)
            };
        }
        else {
            return {
                kind: "file",
                size: BigInt(0)
            };
        }
    }
    open(path) {
        throw new Error("Method not implemented.");
    }
}
class MyFile {
    read(data) {
        throw new Error("Method not implemented.");
    }
    write(data) {
        throw new Error("Method not implemented.");
    }
    seek(s) {
        throw new Error("Method not implemented.");
    }
}
export const vfs = {
    init() {
        return new Vfs();
    },
    File: MyFile,
    Filesystem: Vfs,
};
