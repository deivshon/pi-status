export type RamData = {
    total: number;
    used: number;
    available: number;
    free: number;
    cached: number;
};

export type DiskData = {
    filesystem: string;
    mountpoint: string;
    total: number;
    available: number;
};
