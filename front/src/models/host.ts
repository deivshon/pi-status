import { z } from "zod";

export const HostDataSchema = z.object({
    hostname: z.string(),
    uptime: z.number(),
});

export type HostData = z.infer<typeof HostDataSchema>;
