import { z } from "zod";

export const netInterfaceSchema = z.object({
    interface_path: z.string(),
    interface_name: z.string(),
});

export const netDataSchema = z.object({
    interface: netInterfaceSchema,
    upload_total: z.number(),
    download_total: z.number(),
    upload_speed: z.number(),
    download_speed: z.number(),
    timestamp: z.number(),
});

export const netValuesSchema = z.object({
    download: z.number(),
    upload: z.number(),
});

export enum NetTransferType {
    Download = "download",
    Upload = "upload",
}

export type NetInterface = z.infer<typeof netInterfaceSchema>;
export type NetData = z.infer<typeof netDataSchema>;
export type NetValues = z.infer<typeof netValuesSchema>;
