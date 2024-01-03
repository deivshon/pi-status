export type NetData = {
    interface: string;
    upload_total: number;
    download_total: number;
    upload_speed: number;
    download_speed: number;
    timestamp: number;
};

export type NetValues = {
    download: number;
    upload: number;
};

export enum NetTransferType {
    DOWNLOAD = "download",
    UPLOAD = "upload",
}
