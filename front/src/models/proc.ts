import { z } from "zod";

export const processDataSchema = z.object({
    pid: z.number(),
    name: z.string(),
    mem: z.number(),
    threads: z.number(),
    cpu_usage: z.number(),
    start_time: z.number(),
});

export type ProcessData = z.infer<typeof processDataSchema>;

export enum ProcessProperty {
    PID = 0,
    Name = 1,
    Threads = 2,
    Memory = 3,
    CPU = 4,
}

export type ProcessOrder = {
    ord: ProcessProperty;
    rev: boolean;
};
