import { CoreData } from "@/models/cpu";
import { NetValues } from "@/models/net";
import { RamData } from "@/models/ram";

export const emptyRamData: RamData = {
    available: 0,
    cached: 0,
    free: 0,
    total: 0,
    used: 0,
};

export const emptyCoreData: CoreData = {
    user: 0,
    nice: 0,
    system: 0,
    idle: 0,
    iowait: 0,
    irq: 0,
    softirq: 0,
    steal: 0,
    guest: 0,
    guest_nice: 0,
};

export const emptyNetValues: NetValues = {
    download: 0,
    upload: 0,
};
