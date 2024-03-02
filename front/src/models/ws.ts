import { z } from "zod";
import { CoreDataSchema } from "./cpu";
import { DiskDataSchema } from "./disk";
import { HostDataSchema } from "./host";
import { NetDataSchema } from "./net";
import { ProcessDataSchema } from "./proc";
import { RamDataSchema } from "./ram";

export const StatusDataSchema = z.object({
    host: HostDataSchema.optional(),
    temp: z.number().optional(),
    net_stats: z.array(NetDataSchema).optional(),
    cpu_usage: z.array(CoreDataSchema).optional(),
    ram: RamDataSchema.optional(),
    disk: z.array(DiskDataSchema).optional(),
    proc: z.array(ProcessDataSchema).optional(),
});
