type RamData = {
    total: number;
    used: number;
    available: number;
    free: number;
    cached: number;
};

type DiskData = {
    filesystem: string;
    mountpoint: string;
    total: number;
    available: number;
};
