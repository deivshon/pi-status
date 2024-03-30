import { Tab, tabSchema } from "@/models/app";
import { CoreData } from "@/models/cpu";
import { DiskData } from "@/models/disk";
import { NetValues } from "@/models/net";
import { ProcessData } from "@/models/proc";
import { RamData } from "@/models/ram";
import { statusDataSchema } from "@/models/ws";
import { useTabStore } from "@/store/tab";
import * as Tabs from "@radix-ui/react-tabs";
import { useState } from "react";
import useWebSocket from "react-use-websocket";
import Cpu from "../cpu/Cpu";
import Mem from "../mem/Mem";
import Net from "../net/Net";
import Proc from "../procs/Procs";
import "./App.css";
import { ErrorBox } from "./ErrorBox";
import { emptyCoreData, emptyNetValues, emptyRamData } from "./defaults";
import { computeHostData } from "./updates/host";
import {
    computeUpdatedNetStats,
    getMaxNetTotalsInterface,
} from "./updates/net";
import {
    SwitchDirection,
    getNewSelectedInterface,
} from "./updates/selectedInterface";

const maxNetDataPoints = 30;

export default function App() {
    const [hostname, setHostname] = useState("");
    const [uptime, setUptime] = useState("");
    const [temp, setTemp] = useState(0);
    const [netSpeeds, setNetSpeeds] = useState<Record<string, NetValues[]>>({});
    const [netTotals, setNetTotals] = useState<Record<string, NetValues>>({});
    const [netMaxes, setNetMaxes] = useState<Record<string, number>>({});
    const [selectedNetInterface, setSelectedNetInterface] = useState<
        string | null
    >(null);
    const [cpuUsage, setCpuUsage] = useState<CoreData[]>([]);
    const [ramData, setRamData] = useState<RamData>(emptyRamData);
    const [disks, setDisks] = useState<DiskData[]>([]);
    const [processes, setProcesses] = useState<ProcessData[]>([]);
    const [error, setError] = useState<string | null>(null);
    const { selectedTab, setSelectedTab } = useTabStore();

    useWebSocket(`ws://${window.location.host}/ws_data`, {
        onError: () => {
            setError("Fatal error: unknown WebSocket error occurred");
        },
        onMessage: (event) => {
            if (!event) {
                return;
            }

            handleNewData(event);
        },
    });

    const handleTabChange = (rawTab: string) => {
        const result = tabSchema.safeParse(rawTab);
        if (!result.success) {
            setError(`Fatal error: switched to unknown tab: ${rawTab}`);
            return;
        }

        const tab = result.data;
        setSelectedTab(tab);
    };

    const selectedNet = (() => {
        if (!selectedNetInterface) {
            return {
                speeds: [],
                max: 0,
                totals: emptyNetValues,
                allowBack: false,
                allowForward: false,
            };
        }

        const selectedNetInterfaceIndex = Object.keys(netTotals)
            .sort()
            .indexOf(selectedNetInterface);
        return {
            speeds: netSpeeds[selectedNetInterface] ?? [],
            max: netMaxes[selectedNetInterface] ?? 0,
            totals: netTotals[selectedNetInterface] ?? emptyNetValues,
            allowBack: selectedNetInterfaceIndex !== 0,
            allowForward:
                selectedNetInterfaceIndex !== Object.keys(netTotals).length - 1,
        };
    })();

    const handleNewData = async (event: MessageEvent) => {
        if (error) {
            return;
        }

        let rawData;
        try {
            rawData = JSON.parse(event.data);
        } catch (error) {
            setError("Fatal error: WebSocket message is not valid JSON");
            return;
        }

        const parseResult = statusDataSchema.safeParse(rawData);
        if (!parseResult.success) {
            setError(`Fatal error: malformed message: ${parseResult.error}`);
            return;
        }

        const newData = parseResult.data;

        if (newData.host) {
            const computedHostData = computeHostData(newData.host);
            setHostname(computedHostData.hostname);
            setUptime(computedHostData.uptime);
        }

        const rawNetData = newData.net_stats;
        if (rawNetData) {
            setNetSpeeds((prevSpeeds) => {
                const newNetStats = computeUpdatedNetStats(
                    rawNetData,
                    prevSpeeds,
                    maxNetDataPoints,
                );

                setNetTotals(newNetStats.netTotals);
                setNetMaxes(newNetStats.netMaxes);
                setSelectedNetInterface((prevSelected) =>
                    prevSelected && prevSelected in newNetStats.netTotals
                        ? prevSelected
                        : getMaxNetTotalsInterface(newNetStats.netTotals),
                );

                return newNetStats.netSpeeds;
            });
        }

        if (newData.temp) {
            setTemp(Math.round(newData.temp));
        }

        if (newData.cpu_usage) {
            setCpuUsage(newData.cpu_usage);
        }

        if (newData.ram) {
            setRamData(newData.ram);
        }

        if (newData.disk) {
            setDisks(newData.disk);
        }

        if (newData.proc) {
            setProcesses(newData.proc);
        }
    };

    const switchInterface = (
        direction: SwitchDirection,
        netTotals: Record<string, NetValues>,
    ) => {
        const interfaceNames = Object.keys(netTotals).sort();
        setSelectedNetInterface((prev) => {
            if (!prev) {
                return getMaxNetTotalsInterface(netTotals);
            }

            return getNewSelectedInterface(
                prev,
                direction,
                netTotals,
                interfaceNames,
            );
        });
    };

    if (error) {
        return <ErrorBox error={error} />;
    }

    return (
        <Tabs.Root defaultValue={selectedTab} onValueChange={handleTabChange}>
            <div className="host-bar">
                <p>{hostname}</p>
                <p>Up {uptime}</p>
            </div>
            <Tabs.List className="mb-2 flex flex-wrap justify-between px-2 py-0 md:mb-4 md:justify-center md:gap-2">
                <Tabs.Trigger
                    className="radix-state-active:text-ayu-background radix-state-active:bg-ayu-purple radix-state-inactive:text-ayu-purple rounded-md px-3 py-2"
                    value={Tab["cpu-tab"]}
                >
                    CPU
                </Tabs.Trigger>
                <Tabs.Trigger
                    className="radix-state-active:text-ayu-background radix-state-active:bg-ayu-green radix-state-inactive:text-ayu-green rounded-md px-3 py-2"
                    value={Tab["mem-tab"]}
                >
                    MEM
                </Tabs.Trigger>
                <Tabs.Trigger
                    className="radix-state-active:text-ayu-background radix-state-active:bg-ayu-red radix-state-inactive:text-ayu-red rounded-md px-3 py-2"
                    value={Tab["net-tab"]}
                >
                    NET
                </Tabs.Trigger>
                <Tabs.Trigger
                    className="radix-state-active:text-ayu-background radix-state-active:bg-ayu-yellow radix-state-inactive:text-ayu-yellow rounded-md px-3 py-2"
                    value={Tab["proc-tab"]}
                >
                    PS
                </Tabs.Trigger>
            </Tabs.List>
            <div className="tab-content w-full">
                <Tabs.Content
                    className="m-0 w-full px-2 md:px-4"
                    value={Tab["cpu-tab"]}
                >
                    <Cpu temp={temp} cpuUsage={cpuUsage} />
                </Tabs.Content>
                <Tabs.Content
                    className="m-0 w-full px-2 md:px-4"
                    value={Tab["mem-tab"]}
                >
                    <Mem ram={ramData} disks={disks} />
                </Tabs.Content>
                <Tabs.Content
                    className="m-0 w-full px-2 md:px-4"
                    value={Tab["net-tab"]}
                >
                    <div className="mb-4 flex w-full items-center justify-between">
                        <button
                            className="transition-all duration-75 hover:scale-110 disabled:text-gray-500 disabled:hover:scale-100"
                            onClick={() =>
                                switchInterface(SwitchDirection.Back, netTotals)
                            }
                            disabled={!selectedNet.allowBack}
                        >
                            ᐸ
                        </button>
                        <p>{selectedNetInterface}</p>
                        <button
                            className="transition-all duration-75 hover:scale-110 disabled:text-gray-500 disabled:hover:scale-100"
                            onClick={() =>
                                switchInterface(
                                    SwitchDirection.Forward,
                                    netTotals,
                                )
                            }
                            disabled={!selectedNet.allowForward}
                        >
                            ᐳ
                        </button>
                    </div>
                    <Net
                        netSpeeds={selectedNet.speeds}
                        netMax={selectedNet.max}
                        netTotals={selectedNet.totals}
                    />
                </Tabs.Content>
                <Tabs.Content
                    className="m-0 w-full px-2 md:px-4"
                    value={Tab["proc-tab"]}
                >
                    <Proc
                        procs={processes}
                        mainCpuUsage={
                            cpuUsage.length > 0 ? cpuUsage[0] : emptyCoreData
                        }
                    />
                </Tabs.Content>
            </div>
        </Tabs.Root>
    );
}
