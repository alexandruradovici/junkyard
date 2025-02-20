import { AbsolutePath, File, Filesystem, Seek, Stat, Kind } from "./interfaces/junkyard-vfs-vfs-plugin-vfs";
import { createAbsolutePath } from "junkyard-vfs:vfs-plugin/vfs-host";

class Vfs implements Filesystem {
    fs = {
        "dir1": {

        },
        "dir2": {

        },
        "file1": ""
    }

    readDir(path: AbsolutePath): Array<AbsolutePath> {
        let files = [] as Array<AbsolutePath>;
        for (const file in this.fs) {
            files.push(createAbsolutePath("/"+file));
        }
        return files;
    }
    stat(path: AbsolutePath): Stat {
        let parts = path.components();
        if (this.fs[parts[0]] instanceof Object) {
            return {
                kind: "folder",
                size: BigInt(0)
            };
        }
        else
        {
            return {
                kind: "file",
                size: BigInt(0)
            };
        }
    }
    open(path: AbsolutePath): File {
        throw new Error("Method not implemented.");
    }

}

class MyFile implements File {
    read(data: Uint8Array): bigint {
        throw new Error("Method not implemented.");
    }
    write(data: Uint8Array): bigint {
        throw new Error("Method not implemented.");
    }
    seek(s: Seek): bigint {
        throw new Error("Method not implemented.");
    }

}

export const vfs = {
    init(): Vfs {
        return new Vfs();
    },
    File: MyFile,
    Filesystem: Vfs,
}