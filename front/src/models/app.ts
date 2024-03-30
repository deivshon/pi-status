import { z } from "zod";

export const tabSchema = z.enum(["cpu-tab", "mem-tab", "net-tab", "proc-tab"]);
export const Tab = tabSchema.Values;
export type Tab = z.infer<typeof tabSchema>;
