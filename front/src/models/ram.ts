import { z } from "zod";

export const ramDataSchema = z.object({
    total: z.number(),
    used: z.number(),
    available: z.number(),
    free: z.number(),
    cached: z.number(),
});

export type RamData = z.infer<typeof ramDataSchema>;
