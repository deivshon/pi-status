import { z } from "zod";
import { coreDataSchema } from "./cpu";
import { diskDataSchema } from "./disk";
import { hostDataSchema } from "./host";
import { netDataSchema } from "./net";
import { processDataSchema } from "./proc";
import { ramDataSchema } from "./ram";

export const statusDataSchema = z.object({
    host: hostDataSchema.optional(),
    temp: z.number().optional(),
    net_stats: z.array(netDataSchema).optional(),
    cpu_usage: z.array(coreDataSchema).optional(),
    ram: ramDataSchema.optional(),
    disk: z.array(diskDataSchema).optional(),
    proc: z.array(processDataSchema).optional(),
});
