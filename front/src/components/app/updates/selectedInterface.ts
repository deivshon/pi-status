import { NetValues } from "@/models/net";
import { getMaxNetTotalsInterface } from "./net";

export enum SwitchDirection {
    Back = 0,
    Forward = 1,
}

export const getNewSelectedInterface = (
    prev: string,
    direction: SwitchDirection,
    netTotals: Record<string, NetValues>,
    interfaceNames: string[],
) => {
    if (!prev) {
        return getMaxNetTotalsInterface(netTotals);
    }

    const prevIndex = interfaceNames.indexOf(prev);
    let newSelectedInterface: string | null = null;
    if (prevIndex === -1) {
        newSelectedInterface = getMaxNetTotalsInterface(netTotals);
    } else if (
        (prevIndex === 0 && direction === SwitchDirection.Back) ||
        (prevIndex === interfaceNames.length - 1 &&
            direction === SwitchDirection.Forward)
    ) {
        newSelectedInterface = prev;
    } else if (direction === SwitchDirection.Back) {
        newSelectedInterface = interfaceNames[prevIndex - 1] ?? null;
    } else if (direction === SwitchDirection.Forward) {
        newSelectedInterface = interfaceNames[prevIndex + 1] ?? null;
    }

    return newSelectedInterface !== null &&
        interfaceNames.includes(newSelectedInterface)
        ? newSelectedInterface
        : prev;
};
