import { z } from "zod";

export const hostDataSchema = z.object({
    hostname: z.string(),
    uptime: z.number(),
});

export type HostData = z.infer<typeof hostDataSchema>;
