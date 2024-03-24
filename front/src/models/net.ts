import { z } from "zod";

export const NetInterfaceSchema = z.object({
    interface_path: z.string(),
    interface_name: z.string(),
});

export const NetDataSchema = z.object({
    interface: NetInterfaceSchema,
    upload_total: z.number(),
    download_total: z.number(),
    upload_speed: z.number(),
    download_speed: z.number(),
    timestamp: z.number(),
});

export const NetValuesSchema = z.object({
    download: z.number(),
    upload: z.number(),
});

export enum NetTransferType {
    Download = "download",
    Upload = "upload",
}

export type NetInterface = z.infer<typeof NetInterfaceSchema>;
export type NetData = z.infer<typeof NetDataSchema>;
export type NetValues = z.infer<typeof NetValuesSchema>;
