import { NetData, NetValues } from "@/models/net";

type NetViewData = {
    netSpeeds: Record<string, NetValues[]>;
    netTotals: Record<string, NetValues>;
    netMaxes: Record<string, number>;
};

export const computeUpdatedNetStats = (
    newNetStats: NetData[],
    previousSpeeds: Record<string, NetValues[]>,
    maxPerInterface: number,
): NetViewData => {
    const netSpeeds: Record<string, NetValues[]> = {};
    for (const interfaceData of newNetStats) {
        const interfaceName = interfaceData.interface.interface_name;
        const interfaceSpeeds = {
            download: interfaceData.download_speed,
            upload: interfaceData.upload_speed,
        };

        if (!(interfaceName in previousSpeeds)) {
            netSpeeds[interfaceName] = [interfaceSpeeds];
        } else {
            netSpeeds[interfaceName] = [
                ...previousSpeeds[interfaceName],
                interfaceSpeeds,
            ];
        }

        if (netSpeeds[interfaceName].length > maxPerInterface) {
            netSpeeds[interfaceName] = netSpeeds[interfaceName].slice(1);
        }
    }

    const netTotals: Record<string, NetValues> = {};
    for (const interfaceData of newNetStats) {
        netTotals[interfaceData.interface.interface_name] = {
            download: interfaceData.download_total,
            upload: interfaceData.upload_total,
        };
    }

    const netMaxes = getNetMaxes(netSpeeds);

    return {
        netMaxes,
        netTotals,
        netSpeeds,
    };
};

const getNetMaxes = (
    newNetSpeeds: Record<string, NetValues[]>,
): Record<string, number> => {
    const newNetMaxes: Record<string, number> = {};
    for (const interfaceName in newNetSpeeds) {
        const interfaceSpeeds = newNetSpeeds[interfaceName];

        const interfaceMax = Math.max(
            ...interfaceSpeeds.map((v) => v.download),
            ...interfaceSpeeds.map((v) => v.upload),
        );
        newNetMaxes[interfaceName] = interfaceMax + interfaceMax * (1 / 20);
    }

    return newNetMaxes;
};

export const getMaxNetTotalsInterface = (
    totals: Record<string, NetValues>,
): string | null => {
    let maxInterfaceName: string | null = null;
    let maxInterfaceSum: number | null = null;
    for (const interfaceName of Object.keys(totals)) {
        const interfaceTotals = totals[interfaceName];
        const interfaceSum = interfaceTotals.download + interfaceTotals.upload;

        if (maxInterfaceSum === null || interfaceSum > maxInterfaceSum) {
            maxInterfaceSum = interfaceSum;
            maxInterfaceName = interfaceName;
        }
    }

    return maxInterfaceName;
};
