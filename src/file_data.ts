export interface FileData {
    path: string,
    name: string,
    depth: number,
}

export interface FolderData {
    path: string,
    name: string,
    folders: FolderData[],
    files: FileData[]
}