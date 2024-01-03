import { CoreData } from "../cpu/models";
import { DiskData, RamData } from "../mem/models";
import { NetData } from "../net/models";

export type StatusData = {
    host?: HostData;
    temp?: number;
    net_stats?: NetData;
    cpu_usage?: CoreData[];
    ram?: RamData;
    disk?: DiskData[];
    proc?: ProcessData[];
};

export type HostData = {
    hostname: string;
    uptime: number;
};
