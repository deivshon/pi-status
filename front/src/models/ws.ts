import { z } from "zod";
import { coreDataSchema } from "./cpu";
import { diskDataSchema } from "./disk";
import { hostDataSchema } from "./host";
import { netDataSchema } from "./net";
import { processDataSchema } from "./proc";
import { ramDataSchema } from "./ram";

export const statusDataSchema = z.object({
    host: hostDataSchema.nullable(),
    temp: z.number().nullable(),
    net_stats: z.array(netDataSchema).nullable(),
    cpu_usage: z.array(coreDataSchema).nullable(),
    ram: ramDataSchema.nullable(),
    disk: z.array(diskDataSchema).nullable(),
    proc: z.array(processDataSchema).nullable(),
});
