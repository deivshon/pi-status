import { HostData } from "@/models/host";

type HostViewData = {
    hostname: string;
    uptime: string;
};

export const computeHostData = (hostData: HostData): HostViewData => {
    return {
        hostname: hostData.hostname,
        uptime: getUptimeString(hostData.uptime),
    };
};

const getUptimeString = (uptime: number): string => {
    return uptime < 3600
        ? `${(uptime / 60).toFixed(0)} min`
        : `${(uptime / 3600).toFixed(0)} hours`;
};
