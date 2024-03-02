import { z } from "zod";

export const ProcessDataSchema = z.object({
    pid: z.number(),
    name: z.string(),
    mem: z.number(),
    threads: z.number(),
    cpu_usage: z.number(),
    start_time: z.number(),
});

export type ProcessData = z.infer<typeof ProcessDataSchema>;
