import { z } from "zod";

export const diskDataSchema = z.object({
    filesystem: z.string(),
    mountpoint: z.string(),
    total: z.number(),
    available: z.number(),
});

export type DiskData = z.infer<typeof diskDataSchema>;
