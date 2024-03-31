import { formatBytes } from "@/lib/bytes";
import { CoreData } from "@/models/cpu";
import { ProcessData, ProcessOrder, ProcessProperty } from "@/models/proc";
import { useOrderStore } from "@/store/order";
import classNames from "classnames";
import { ChangeEvent, useState } from "react";
import "./Procs.css";

type ProcessOrderingFunction = (p1: ProcessData, p2: ProcessData) => number;
const orderingFromProperty = (
    processProperty: ProcessProperty,
): ProcessOrderingFunction => {
    switch (processProperty) {
        case ProcessProperty.PID:
            return (p1, p2) => p2.pid - p1.pid;
        case ProcessProperty.Name:
            return (p1, p2) => p1.name.localeCompare(p2.name);
        case ProcessProperty.Threads:
            return (p1, p2) => p2.threads - p1.threads;
        case ProcessProperty.Memory:
            return (p1, p2) => p2.mem - p1.mem;
        case ProcessProperty.CPU:
            return (p1, p2) => p2.cpu_usage - p1.cpu_usage;
        default:
            console.error(`Unknown ordering value passed: ${processProperty}`);
            return (_p1, _p2) => 0;
    }
};

type ProcProps = {
    procs: ProcessData[];
    mainCpuUsage: CoreData;
};

const pidLabelDefault = "PID";
const nameLabelDefault = "Name";
const threadsLabelDefault = "Thds";
const memLabelDefault = "Mem";
const cpuLabelDefault = "CPU";

export default function Procs(props: ProcProps) {
    const { order, setOrder } = useOrderStore();
    const [search, setSearch] = useState("");

    const total =
        props.mainCpuUsage.user +
        props.mainCpuUsage.nice +
        props.mainCpuUsage.system +
        props.mainCpuUsage.idle +
        props.mainCpuUsage.iowait +
        props.mainCpuUsage.irq +
        props.mainCpuUsage.softirq +
        props.mainCpuUsage.steal +
        props.mainCpuUsage.guest +
        props.mainCpuUsage.guest_nice;

    const visibleProcs = props.procs
        .filter(
            (p) =>
                p.name.toLowerCase().includes(search.toLowerCase()) ||
                p.pid.toString().startsWith(search),
        )
        .sort(orderingFromProperty(order.ord));

    if (order.rev) {
        visibleProcs.reverse();
    }

    const sortProcessesBy = (prop: ProcessProperty) => {
        setOrder({
            ord: prop,
            rev: order.ord === prop ? !order.rev : false,
        });
    };

    const propertyLabel = (
        prop: ProcessProperty,
        currentOrdering: ProcessOrder,
        defaultPropertyLabel: string,
        visibleProcs: ProcessData[],
    ): string => {
        if (visibleProcs.length === 0) {
            return "";
        }

        if (order.ord !== prop) {
            return defaultPropertyLabel;
        }

        if (prop === ProcessProperty.Name) {
            return `${defaultPropertyLabel}${currentOrdering.rev ? "▲" : "▼"}`;
        } else {
            return `${currentOrdering.rev ? "▲" : "▼"}${defaultPropertyLabel}`;
        }
    };

    const handleSearchChange = (event: ChangeEvent<HTMLInputElement>) => {
        setSearch(event.target.value);
    };

    const getResultsAmountText = (): string => {
        switch (visibleProcs.length) {
            case 0:
                return "No results";
            case 1:
                return "1 result";
            default:
                return `${visibleProcs.length} results`;
        }
    };

    return (
        <div className="flex flex-col gap-3">
            <div>
                <p>{props.procs.length} active processes</p>
                <div className="mt-3">
                    <input
                        type="text"
                        placeholder="Search..."
                        className="w-full rounded-none border-2 border-gray-400 bg-ayu-background px-2 py-1 text-start text-lightgrey placeholder:text-gray-500"
                        onChange={handleSearchChange}
                    />
                </div>
            </div>
            <div className="proc-container no-scrollbar flex">
                <div className="pid-col proc-col">
                    <button
                        className="text-nowrap text-end md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.PID)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline: order.ord === ProcessProperty.PID,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.PID,
                                order,
                                pidLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </button>
                    {visibleProcs.map((p, i) => (
                        <span className="col-content" key={i}>
                            {p.pid}
                        </span>
                    ))}
                </div>
                <div className="name-col proc-col">
                    <button
                        className="text-nowrap text-start md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.Name)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline: order.ord === ProcessProperty.Name,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.Name,
                                order,
                                nameLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </button>
                    {visibleProcs.map((p, i) => (
                        <span className="col-content" key={i}>
                            {p.name}
                        </span>
                    ))}
                </div>
                <div className="threads-col proc-col">
                    <button
                        className="threads-label text-nowrap text-end md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.Threads)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline:
                                    order.ord === ProcessProperty.Threads,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.Threads,
                                order,
                                threadsLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </button>
                    {visibleProcs.map((p, i) => (
                        <span className="col-content" key={i}>
                            {p.threads}
                        </span>
                    ))}
                </div>
                <div className="memory-col proc-col">
                    <button
                        className="mem-label text-nowrap text-end md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.Memory)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline: order.ord === ProcessProperty.Memory,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.Memory,
                                order,
                                memLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </button>
                    {visibleProcs.map((p, i) => (
                        <span className="col-content" key={i}>
                            {formatBytes(p.mem, {
                                short: true,
                                space: false,
                                roundTreshold: 10,
                                absoluteRoundTreshold: 1024 ** 3,
                                roundingDigits: 1,
                            })}
                        </span>
                    ))}
                </div>
                <div className="cpu-col proc-col">
                    <button
                        className="cpu-label text-nowrap text-end md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.CPU)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline: order.ord === ProcessProperty.CPU,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.CPU,
                                order,
                                cpuLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </button>
                    {visibleProcs.map((p, i) => (
                        <span className="col-content cpu-percs" key={i}>
                            {((p.cpu_usage / total) * 100)
                                .toFixed(1)
                                .padStart(5, " ")}
                            %
                        </span>
                    ))}
                </div>
            </div>
            <div className="flex w-full items-center justify-center md:justify-start">
                <p>{search.length > 0 && getResultsAmountText()}</p>
            </div>
        </div>
    );
}
