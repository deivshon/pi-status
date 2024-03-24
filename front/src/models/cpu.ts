import { z } from "zod";

export const coreDataSchema = z.object({
    user: z.number(),
    nice: z.number(),
    system: z.number(),
    idle: z.number(),
    iowait: z.number(),
    irq: z.number(),
    softirq: z.number(),
    steal: z.number(),
    guest: z.number(),
    guest_nice: z.number(),
});

export type CoreData = z.infer<typeof coreDataSchema>;
