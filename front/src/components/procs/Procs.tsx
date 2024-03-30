import { formatBytes } from "@/lib/bytes";
import { CoreData } from "@/models/cpu";
import { ProcessData } from "@/models/proc";
import classNames from "classnames";
import { ChangeEvent, useState } from "react";
import "./Procs.css";

enum ProcessProperty {
    PID = 0,
    Name = 1,
    Threads = 2,
    Memory = 3,
    CPU = 4,
}

type ProcessOrdering = {
    ord: ProcessProperty;
    rev: boolean;
};

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
            console.log(`Unknown ordering value passed: ${processProperty}`);
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
    const [ordering, setOrdering] = useState<ProcessOrdering>({
        ord: ProcessProperty.Memory,
        rev: false,
    });
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
        .sort(orderingFromProperty(ordering.ord));

    if (ordering.rev) {
        visibleProcs.reverse();
    }

    const sortProcessesBy = (prop: ProcessProperty) => {
        setOrdering({
            ord: prop,
            rev: ordering.ord === prop ? !ordering.rev : false,
        });
    };

    const propertyLabel = (
        prop: ProcessProperty,
        currentOrdering: ProcessOrdering,
        defaultPropertyLabel: string,
        visibleProcs: ProcessData[],
    ): string => {
        if (visibleProcs.length === 0) {
            return "";
        }

        if (ordering.ord !== prop) {
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

    return (
        <div className="flex flex-col gap-3">
            <div>
                <p>{props.procs.length} active processes</p>
                <div className="mt-3">
                    <input
                        type="text"
                        placeholder="Search..."
                        className="bg-proc-search-bar text-lightgrey w-full rounded-none border-2 border-gray-400 px-2 py-1 text-start placeholder:text-gray-500"
                        onChange={handleSearchChange}
                    />
                </div>
            </div>
            <div className="proc-container flex">
                <div className="pid-col proc-col">
                    <div
                        className="text-nowrap md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.PID)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline: ordering.ord === ProcessProperty.PID,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.PID,
                                ordering,
                                pidLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    {visibleProcs.map((p, i) => (
                        <div className="col-content" key={i}>
                            {p.pid}
                        </div>
                    ))}
                </div>
                <div className="name-col proc-col">
                    <div
                        className="text-nowrap md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.Name)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline:
                                    ordering.ord === ProcessProperty.Name,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.Name,
                                ordering,
                                nameLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content" key={i}>
                            {p.name}
                        </p>
                    ))}
                </div>
                <div className="threads-col proc-col">
                    <div
                        className="threads-label text-nowrap md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.Threads)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline:
                                    ordering.ord === ProcessProperty.Threads,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.Threads,
                                ordering,
                                threadsLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content" key={i}>
                            {p.threads}
                        </p>
                    ))}
                </div>
                <div className="memory-col proc-col">
                    <div
                        className="mem-label text-nowrap md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.Memory)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline:
                                    ordering.ord === ProcessProperty.Memory,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.Memory,
                                ordering,
                                memLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content" key={i}>
                            {formatBytes(p.mem, {
                                short: true,
                                space: false,
                                roundTreshold: 10,
                                absoluteRoundTreshold: 1024 ** 3,
                                roundingDigits: 1,
                            })}
                        </p>
                    ))}
                </div>
                <div className="cpu-col proc-col">
                    <div
                        className="cpu-label text-nowrap md:mb-2"
                        onClick={() => sortProcessesBy(ProcessProperty.CPU)}
                    >
                        <span
                            className={classNames("col-content", {
                                underline: ordering.ord === ProcessProperty.CPU,
                            })}
                        >
                            {propertyLabel(
                                ProcessProperty.CPU,
                                ordering,
                                cpuLabelDefault,
                                visibleProcs,
                            )}
                        </span>
                    </div>
                    <div></div>
                    {visibleProcs.map((p, i) => (
                        <p className="col-content cpu-percs" key={i}>
                            {((p.cpu_usage / total) * 100)
                                .toFixed(1)
                                .padStart(5, " ")}
                            %
                        </p>
                    ))}
                </div>
            </div>
        </div>
    );
}
