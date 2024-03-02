export type NetData = {
    interface: NetInterface;
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

export type NetInterface = {
    interface_path: string;
    interface_name: string;
};

export enum NetTransferType {
    DOWNLOAD = "download",
    UPLOAD = "upload",
}
